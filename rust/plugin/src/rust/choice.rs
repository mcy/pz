//! Type codegen.

#![allow(unused)]

use std::collections::HashMap;

use pz::proto;
use pz::proto::field::Type as TypeEnum;
use pz::proto::r#type::Kind;

use crate::rust::message;
use crate::emit::SourceWriter;
use crate::rust::fields::FieldGenerators;
use crate::rust::fields::Where;
use crate::rust::names;
use crate::Type;

pub fn emit(ty: Type, w: &mut SourceWriter) {
  let gen = FieldGenerators::build(ty.fields());
  let hasbit_words = gen.num_hasbits / 32 + (gen.num_hasbits % 32 != 0) as u32;

  let mut ty_ptrs = ty
    .fields()
    .filter_map(|f| f.ty().1.filter(|ty| ty.kind() != Kind::Enum))
    .collect::<Vec<_>>();
  ty_ptrs.sort_by(|a, b| (a.package(), a.name()).cmp(&(b.package(), b.name())));
  ty_ptrs.dedup_by(|a, b| (a.package(), a.name()) == (b.package(), b.name()));

  w.emit(
    vars! {
      hasbit_words,
      NUM_TYS: ty_ptrs.len(),
      "Type::struct": |w| message::emit_main_struct(w),
      "Type::common": |w| message::emit_common_methods(w),
      "Type::impls": |w| message::emit_impls(w),
      "Type::views": |w| message::emit_view_types(w),
      "Type::fields": |w| for field in &gen.fields {
        field.in_storage(w);
      },
      "Type::Variants": |w| for field in &gen.fields {
        field.in_variants(w);
      },
      "Type::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::TypeImpl, w);
        field.in_mut_methods(Where::TypeImpl, w);
        w.new_line();
      },
      "Type::memcpys": |w| for field in &gen.fields {
        w.emit(
          vars! {
            n: field.field.number().unwrap(),
            __name: names::field_name_type_name(field.field),
          },
          r#"
            $n => __r::Set::<<$Type as __r::Field<$__name>>::Type>::apply_to(src.get($__name{}), dst.get_mut($__name{})),
          "#
        );
      },
      "Type::field_impls": |w| for field in &gen.fields {
        field.in_impls(w);
        w.new_line();
      },
      "Type::debug": |w| for field in &gen.fields {
        field.in_debug(w);
      },
      "Ref::common": |w| message::emit_common_ref_methods(w),
      "Ref::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::ViewImpl, w);
        w.new_line();
      },
      "Mut::common": |w| message::emit_common_mut_methods(w),
      "Mut::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::MutImpl, w);
        field.in_mut_methods(Where::MutImpl, w);
        w.new_line();
      },
      self_to_output_arms: |w| for field in ty.fields() {
        w.emit(
          vars! {Name: names::ident(heck::AsPascalCase(field.name())) },
          "
            ${Type}Cases::$Name(val) => ${Type}Cases::$Name(__rt::ptr::ViewFor::as_view(val)),
          "
        )
      },
      make_view_arms: |w| for field in ty.fields() {
        w.emit(
          vars! {
            number: field.number().unwrap(),
            index: field.index,
            Name: names::ident(heck::AsPascalCase(field.name())),
            __name: names::field_name_type_name(field),
            Field: |w| {match field.ty() {
              (TypeEnum::I32, _) => w.write("i32"),
              (TypeEnum::I64, _) => w.write("i64"),
              (TypeEnum::U32, _) => w.write("u32"),
              (TypeEnum::U64, _) => w.write("u64"),
              (TypeEnum::F32, _) => w.write("f32"),
              (TypeEnum::F64, _) => w.write("f64"),
              (TypeEnum::Bool, _) => w.write("bool"),
              (TypeEnum::String, _) => w.write("__rt::Str"),
              (TypeEnum::Type, Some(ty)) => w.write(&names::type_name(ty).to_string()),
              _ => unreachable!(),
            }},
            suffix: if field.is_repeated() { "" } else {".unwrap_unchecked()"},
          },
          "
            $number => ${Type}Cases::$Name(self.get($__name{})$suffix),
          "
        )
      },
      make_mut_arms: |w| for field in ty.fields() {
        w.emit(
          vars! {
            number: field.number().unwrap(),
            index: field.index,
            Name: names::ident(heck::AsPascalCase(field.name())),
            __name: names::field_name_type_name(field),
            Field: |w| {match field.ty() {
              (TypeEnum::I32, _) => w.write("i32"),
              (TypeEnum::I64, _) => w.write("i64"),
              (TypeEnum::U32, _) => w.write("u32"),
              (TypeEnum::U64, _) => w.write("u64"),
              (TypeEnum::F32, _) => w.write("f32"),
              (TypeEnum::F64, _) => w.write("f64"),
              (TypeEnum::Bool, _) => w.write("bool"),
              (TypeEnum::String, _) => w.write("__rt::Str"),
              (TypeEnum::Type, Some(ty)) => w.write(&names::type_name(ty).to_string()),
              _ => unreachable!(),
            }},
            suffix: if field.is_repeated() { "" } else {".into_option().unwrap_unchecked()"},
          },
          "
            $number => ${Type}Cases::$Name(self.get_mut($__name{})$suffix),
          "
        )
      },
      tdp_descs: |w| for &ty in &ty_ptrs {
        w.emit(
          vars!{ Submsg: names::type_name(ty) },
          "
            $Submsg::__tdp_info,
          "
        );
      },
      tdp_fields: |w| {
        let mut hasbit_index = 0u32;
        let ty_map = ty_ptrs.iter().enumerate()
          .map(|(i, t)| (t.proto() as *const proto::Type, i))
          .collect::<HashMap<_, _>>();

        for field in ty.fields() {
          let mut ty_idx = 0;
          let tdp_kind = match field.ty() {
            (TypeEnum::I32 | TypeEnum::U32, _) => "I32",
            (TypeEnum::I64 | TypeEnum::U64, _) => "I64",
            (TypeEnum::F32, _) => "F32",
            (TypeEnum::F64, _) => "F64",
            (TypeEnum::Bool, _) => "Bool",
            (TypeEnum::String, _) => "Str",
            (TypeEnum::Type, Some(ty)) => match ty.kind() {
              Kind::Message | Kind::Choice => {
                ty_idx = ty_map[&(ty.proto() as *const _)];
                "Type"
              }
              Kind::Struct => todo!(),
              Kind::Enum => "I32",
            },
            _ => unreachable!(),
          };

          w.emit(
            vars! {
              hasbit_index,
              tdp_kind,
              ty_idx,
              name: names::ident(field.name()),
              number: field.number().unwrap(),
              raw_name: field.name(),
              repeated: field.is_repeated() as u32,
            },
            "
              __z::tdp::FieldStorage {
                number: $number,
                flags: 
                  __z::tdp::Kind::$tdp_kind.raw() << __z::tdp::Field::KIND_SHIFT |
                  $repeated << __z::tdp::Field::REP_SHIFT,
                offset: $priv::UNION_OFFSET as u32,
                desc: $ty_idx,
                hasbit: 0,
              },
            "
          );
          if !field.is_repeated() {
            hasbit_index += 1;
          }
        }
      },
    },
    r#"
      ${Type::struct}

      mod $priv {
        pub use super::*;

        ${Type::views}

        #[repr(C)]
        pub struct Storage {
          pub(super) which: u32,
          pub(super) union: Union,    
        }

        #[repr(C)]
        pub union Union {
          pub(super) __unset: (),
          ${Type::fields}
        }

        pub const UNION_OFFSET: usize = {
          let align = __s::mem::align_of::<$priv::Union>();
          if align < 4 { 4 } else { align }
        };
      }

      impl $Type {
        /// The default value for [`$Type`], provided as a static constant.
        ///
        /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
        pub const DEFAULT: &'static Self = unsafe { &Self {
          ptr: $NonNull::new_unchecked(&const { $priv::Storage {
            which: 0,
            union: $priv::Union { __unset: () },
          }} as *const $priv::Storage as *mut $priv::Storage),
          arena: $None,
        }};
        
        ${Type::common}

        pub fn which(&self) -> i32 {
          self.as_ref().which()
        }

        pub fn cases(&self) -> ${Type}Cases<__r::SelectRef> {
          self.as_ref().cases()
        }

        pub fn cases_mut(&mut self) -> ${Type}Cases<__r::SelectMut> {
          self.as_mut().cases_mut()
        }

        ${Type::access}

        #[doc(hidden)]
        pub const __LAYOUT: $Layout = $Layout::new::<$priv::Storage>();
        #[doc(hidden)]
        pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
          raw.cast::<$priv::Storage>().as_mut().which = 0;
        }
        #[doc(hidden)]
        pub fn __tdp_info() -> __z::tdp::Desc {
          <Self as __z::Message>::__TDP
        }

        #[doc(hidden)]
        fn __memcpy(mut dst: $Mut<$Type>, src: $Ref<$Type>) {
          match src.which() {
            0 => dst.clear(),
            ${Type::memcpys}
            _ => __s::unreachable!(),
          }
        }
      }
      
      #[non_exhaustive]
      pub enum ${Ident}Cases<'proto, Which: __r::Select = __r::SelectRef> {
        Unset,
        ${Type::Variants}

        #[doc(hidden)]
        __PhantomData($PhantomData<&'proto Which>, __z::Void),
      }

      impl __z::Message for $Type {
        const __TDP: __z::tdp::Desc = {
          type Tdp = __z::tdp::DescStorage<{$NUM_FIELDS + 1}>;
          const STATIC: Tdp = Tdp {
            header: __z::tdp::DescHeader {
              size: {
                let size = $Type::__LAYOUT.size();
                assert!(size <= (u32::MAX as usize));
                size as u32
              },

              descs: {
                const DESCS: &[fn() -> __z::tdp::Desc] = &[
                  $tdp_descs
                ];
                DESCS.as_ptr()
              },
              
              num_hasbit_words: 0,
              kind: __z::tdp::DescKind::Choice,
            },

            fields: [
              $tdp_fields
              __z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
            ],
          };

          unsafe { STATIC.get() }
        };

        fn __is_null(&self, _: impl __z::Sealed) -> bool {
          self.ptr == $NonNull::dangling()
        }
        fn __raw(_: impl __z::Sealed, ptr: __r::Ref<Self>) -> __z::tdp::Opaque { ptr.ptr.cast() }
        fn __arena(_: impl __z::Sealed, ptr: &mut __r::Mut<Self>) -> __z::RawArena { ptr.arena }
      }

      ${Type::impls}

      ${Type::field_impls}

      impl<'proto> $priv::Ref<'proto> {
        pub fn which(self) -> i32 {
          unsafe { self.ptr.as_ref() }.which as i32
        }

        pub fn cases(self) -> ${Type}Cases<'proto> {
          unsafe {
            match self.which() {
              0 => ${Type}Cases::Unset,
              $make_view_arms
              _ => __s::unreachable!(),
            }
          }
        }

        ${Ref::common}

        ${Ref::access}

        #[doc(hidden)]
        pub fn __debug(self, debug: &mut __z::Debug) -> $fmt::Result {
          let mut count = 0;
          debug.start_block()?;
          ${Type::debug}
          if count != 0 {
            debug.comma(true)?;
          }
          debug.end_block()?;
          $Ok(())
        }
      }

      impl<'proto> $priv::Mut<'proto>  {
        pub fn which(&self) -> i32 {
          self.as_ref().which()
        }

        pub fn cases(self) -> ${Type}Cases<'proto> {
          self.into_ref().cases()
        }

        pub fn cases_mut(self) -> ${Type}Cases<'proto, __r::SelectMut> {
          unsafe {
            match self.which() {
              0 => ${Type}Cases::Unset,
              $make_mut_arms
              _ => __s::unreachable!(),
            }
          }
        }

        ${Mut::common}

        ${Mut::access}
      }
    "#,
  );
  w.new_line();
}
