//! Type codegen.

#![allow(unused)]

use std::collections::HashMap;

use pz::proto;
use pz::proto::field::Type as TypeEnum;
use pz::proto::r#type::Kind;

use crate::emit::SourceWriter;
use crate::rust::fields::FieldGenerators;
use crate::rust::fields::Where;
use crate::rust::names::ident;
use crate::rust::names::type_name;
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
      package: ident(ty.package()),
      Name: ident(ty.name()),
      Type: type_name(ty),
      NUM_FIELDS: ty.fields().count(),
      NUM_TYS: ty_ptrs.len(),
      priv: format!("__priv_{}", type_name(ty)),
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
      "Type::debug": |w| for field in &gen.fields {
        field.in_debug(w);
      },
      "View::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::ViewImpl, w);
        w.new_line();
      },
      "Mut::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::MutImpl, w);
        field.in_mut_methods(Where::MutImpl, w);
        w.new_line();
      },
      self_to_output_arms: |w| for field in ty.fields() {
        w.emit(
          vars! {Name: ident(heck::AsPascalCase(field.name())) },
          "
            ${Type}Cases::$Name(val) => ${Type}Cases::$Name($rt::ptr::ViewFor::as_view(val)),
          "
        )
      },
      make_view_arms: |w| for field in ty.fields() {
        w.emit(
          vars! {
            number: field.number().unwrap(),
            index: field.index,
            Name: ident(heck::AsPascalCase(field.name())),
            Field: |w| {match field.ty() {
              (TypeEnum::I32, _) => w.write("i32"),
              (TypeEnum::I64, _) => w.write("i64"),
              (TypeEnum::U32, _) => w.write("u32"),
              (TypeEnum::U64, _) => w.write("u64"),
              (TypeEnum::F32, _) => w.write("f32"),
              (TypeEnum::F64, _) => w.write("f64"),
              (TypeEnum::Bool, _) => w.write("bool"),
              (TypeEnum::String, _) => w.write("$rt::Str"),
              (TypeEnum::Type, Some(ty)) => w.write(&type_name(ty).to_string()),
              _ => unreachable!(),
            }},
            make_view: |w| {
              if !field.is_repeated() {
                w.write("$Type::__tdp_info().field($index).make_view::<$Field>(self.ptr.as_ptr())");
              } else {
                w.write("$Type::__tdp_info().field($index).make_slice::<$Field>(self.ptr.as_ptr())");
              }
            },
          },
          "
            $number => ${Type}Cases::$Name($make_view),
          "
        )
      },
      make_mut_arms: |w| for field in ty.fields() {
        w.emit(
          vars! {
            number: field.number().unwrap(),
            index: field.index,
            Name: ident(heck::AsPascalCase(field.name())),
            Field: |w| {match field.ty() {
              (TypeEnum::I32, _) => w.write("i32"),
              (TypeEnum::I64, _) => w.write("i64"),
              (TypeEnum::U32, _) => w.write("u32"),
              (TypeEnum::U64, _) => w.write("u64"),
              (TypeEnum::F32, _) => w.write("f32"),
              (TypeEnum::F64, _) => w.write("f64"),
              (TypeEnum::Bool, _) => w.write("bool"),
              (TypeEnum::String, _) => w.write("$rt::Str"),
              (TypeEnum::Type, Some(ty)) => w.write(&type_name(ty).to_string()),
              _ => unreachable!(),
            }},
            make_view: |w| {
              if !field.is_repeated() {
                w.write("$Type::__tdp_info().field($index).make_mut::<$Field>(self.ptr.as_ptr(), self.arena)");
              } else {
                w.write("$Type::__tdp_info().field($index).make_rep::<$Field>(self.ptr.as_ptr(), self.arena)");
              }
            },
          },
          "
            $number => ${Type}Cases::$Name($make_view),
          "
        )
      },
      tdp_descs: |w| for &ty in &ty_ptrs {
        w.emit(
          vars!{ Submsg: type_name(ty) },
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
              name: ident(field.name()),
              number: field.number().unwrap(),
              raw_name: field.name(),
              repeated: field.is_repeated() as u32,
            },
            "
              $z::tdp::FieldStorage {
                number: $number,
                flags: 
                  $z::tdp::Kind::$tdp_kind.raw() << $z::tdp::Field::KIND_SHIFT |
                  $repeated << $z::tdp::Field::REP_SHIFT,
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
      /// choice `$package.$Name`
      $deprecated
      pub struct $Type {
        ptr: $z::ABox<$priv::Storage>,
        arena: $z::RawArena,
      }

      const _: () = {
        assert!(
          std::mem::size_of::<$priv::Storage>() < (u32::MAX as usize),
          "storage size excees 4GB",
        );
      };

      impl $Type {
        pub const DEFAULT: $rt::View<'static, Self> = unsafe {
          const VALUE: $priv::Storage = $priv::Storage {
            which: 0,
            union: $priv::Union { __unset: () },
          };
          $rt::View::<Self> {
            ptr: $z::ABox::from_ptr(&VALUE as *const $priv::Storage as *mut $priv::Storage as *mut u8),
            _ph: $PhantomData,
          }
        };
        
        pub fn new() -> Self {
          let arena = $z::RawArena::new();
          let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
          unsafe {
            ptr.write_bytes(0, Self::__LAYOUT.size());
            Self { ptr: $z::ABox::from_ptr(ptr), arena }
          }
        }

        pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, $rt::Error> {
          let mut new = Self::new();
          new.parse_pb(input)?;
          Ok(new)
        }

        pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), $rt::Error> {
          self.as_mut().parse_pb(input)
        }

        pub fn as_view(&self) -> $rt::View<Self> {
          $priv::View { ptr: self.ptr, _ph: $PhantomData }
        }

        pub fn as_mut(&mut self) -> $rt::Mut<Self> {
          $priv::Mut { ptr: self.ptr, _ph: $PhantomData, arena: self.arena }
        }

        pub fn cases(&self) -> ${Type}Cases<'_, $rt::ptr::select::View> {
          self.as_view().cases()
        }

        pub fn cases_mut(&mut self) -> ${Type}Cases<'_, $rt::ptr::select::Mut> {
          self.as_mut().cases_mut()
        }

        pub fn clear(&mut self) {
          unsafe { $Type::__raw_clear(self.ptr.as_ptr()) }
        }

        pub fn into_raw(self) -> *mut u8 {
          self.ptr.as_ptr()
        }

        ${Type::access}

        #[doc(hidden)]
        pub const __LAYOUT: $Layout = $Layout::new::<$priv::Storage>();
        #[doc(hidden)]
        pub unsafe fn __raw_clear(raw: *mut u8) {
          (&mut *raw.cast::<$priv::Storage>()).which = 0;
        }
        #[doc(hidden)]
        pub fn __tdp_info() -> $z::tdp::Desc {
          unsafe { $priv::TDP_INFO.get() }
        }
        #[doc(hidden)]
        pub unsafe fn __raw_data(&self) -> &[u8] {
          std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
        }
      }

      pub enum ${Type}Cases<'proto, Which: $rt::ptr::select::Select> {
        Unset($PhantomData<&'proto Which>),
        ${Type::Variants}
      }

      impl Default for $Type {
        fn default() -> Self {
          Self::new()
        }
      }

      impl $rt::ptr::Proxied for $Type {
        type View<'proto> = $priv::View<'proto>;
        type Mut<'proto> = $priv::Mut<'proto>;
      }
      
      impl<'proto> $priv::View<'proto> {
        pub fn as_view(&self) -> $rt::View<$Type> {
          $priv::View { ptr: self.ptr, _ph: $PhantomData }
        }

        pub fn cases(self) -> ${Type}Cases<'proto, $rt::ptr::select::View> {
          unsafe {
            let number = self.ptr.as_ptr().cast::<u32>().read();
            match number {
              0 => ${Type}Cases::Unset($PhantomData),
              $make_view_arms
              _ => unreachable!(),
            }
          }
        }

        ${View::access}

        #[doc(hidden)]
        pub fn __debug(self, debug: &mut $z::Debug) -> std::fmt::Result {
          let mut count = 0;
          debug.start_block()?;
          ${Type::debug}
          if count != 0 {
            debug.comma(true)?;
          }
          debug.end_block()?;
          Ok(())
        }
      }

      impl Default for $priv::View<'_> {
        fn default() -> Self {
          $Type::DEFAULT
        }
      }

      impl<'proto> $priv::Mut<'proto>  {
        pub fn as_view(&self) -> $rt::View<$Type> {
          $priv::View { ptr: self.ptr, _ph: $PhantomData }
        }

        pub fn into_view(self) -> $rt::View<'proto, $Type> {
          $priv::View { ptr: self.ptr, _ph: $PhantomData }
        }

        pub fn as_mut(&mut self) -> $rt::Mut<$Type> {
          $priv::Mut { ptr: self.ptr, _ph: $PhantomData, arena: self.arena }
        }

        pub fn cases(self) -> ${Type}Cases<'proto, $rt::ptr::select::View> {
          self.into_view().cases()
        }

        pub fn cases_mut(self) -> ${Type}Cases<'proto, $rt::ptr::select::Mut> {
          unsafe {
            let number = self.ptr.as_ptr().cast::<u32>().read();
            match number {
              0 => ${Type}Cases::Unset($PhantomData),
              $make_mut_arms
              _ => unreachable!(),
            }
          }
        }

        pub fn clear(self) {
          unsafe { $Type::__raw_clear(self.ptr.as_ptr()) }
        }

        pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), $rt::Error> {
          let mut ctx = $z::tdp::parse::Context::new(input, self.arena);
          ctx.parse(self.ptr.as_ptr() as *mut u8, $Type::__tdp_info())
        }

        ${Mut::access}
      }

      impl Drop for $Type {
        fn drop(&mut self) {
          unsafe { self.arena.destroy() }
        }
      }

      impl std::fmt::Debug for $priv::View<'_> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
          fmt.write_str("$package.$Name ")?;
          let mut debug = $z::Debug::new(fmt);
          self.__debug(&mut debug)
        }
      }

      impl std::fmt::Debug for $priv::Mut<'_> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
          use $rt::ptr::ViewFor;
          std::fmt::Debug::fmt(&self.as_view(), fmt)
        }
      }

      impl std::fmt::Debug for $Type {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
          std::fmt::Debug::fmt(&self.as_view(), fmt)
        }
      }

      impl $rt::value::Type for $Type {
        type __Storage = *mut u8;

        unsafe fn __make_view<'a>(ptr: *const *mut u8) -> $rt::View<'a, Self> {
          $priv::View {
            ptr: $z::ABox::from_ptr(ptr.read()),
            _ph: $PhantomData,
          }
        }
        unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: $z::RawArena) -> $rt::Mut<'a, Self> {
          $priv::Mut {
            ptr: $z::ABox::from_ptr(ptr.read()),
            arena,
            _ph: $PhantomData,
          }
        }

        unsafe fn __resize(vec: &mut $z::AVec<*mut u8>, new_len: usize, arena: $z::RawArena) {
          vec.resize_msg(new_len, arena, Self::__LAYOUT)
        }
      }

      mod __priv_$Type {
        pub use super::*;

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
          let align = std::mem::align_of::<$priv::Union>();
          if align < 4 { 4 } else { align }
        };

        pub static TDP_INFO: $z::tdp::DescStorage<{$NUM_FIELDS + 1}> =
          $z::tdp::DescStorage::<{$NUM_FIELDS + 1}> {
            header: $z::tdp::DescHeader {
              size: {
                let size = $Type::__LAYOUT.size();
                assert!(size <= (u32::MAX as usize));
                size as u32
              },
              descs: {
                const DESCS: &[fn() -> $z::tdp::Desc] = &[
                  $tdp_descs
                ];
                DESCS.as_ptr()
              },
              num_hasbit_words: 0,
              kind: $z::tdp::DescKind::Choice,
            },
            fields: [
              $tdp_fields
              $z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
            ],
          };
       
        #[derive(Copy, Clone)]
        pub struct View<'proto> {
          pub(in super) ptr: $z::ABox<$priv::Storage>,
          pub(in super) _ph: $PhantomData<&'proto $Type>,
        }
       
        impl<'proto> $rt::ptr::ViewFor<'proto, super::$Type> for View<'proto> {
          fn as_view(&self) -> View {
            View { ptr: self.ptr, _ph: $PhantomData }
          }
        }

        pub struct Mut<'proto> {
          pub(in super) ptr: $z::ABox<$priv::Storage>,
          pub(in super) _ph: $PhantomData<&'proto mut $Type>,
          pub(in super) arena: $z::RawArena,
        }
       
        impl<'proto> $rt::ptr::ViewFor<'proto, super::$Type> for Mut<'proto> {
          fn as_view(&self) -> View {
            View { ptr: self.ptr, _ph: $PhantomData }
          }
        }

        impl<'proto> $rt::ptr::MutFor<'proto, super::$Type> for Mut<'proto> {
          fn into_view(self) -> View<'proto> {
            View { ptr: self.ptr, _ph: $PhantomData }
          }

          fn as_mut(&mut self) -> Mut {
            Mut { ptr: self.ptr, _ph: $PhantomData, arena: self.arena }
          }
        }
      }
    "#,
  );
  w.new_line();
}
