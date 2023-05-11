//! Message codegen.

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
      "Type::field_init": |w| for field in &gen.fields {
        field.in_storage_init(w);
      },
      "Type::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::TypeImpl, w);
        field.in_mut_methods(Where::TypeImpl, w);
        w.new_line();
      },
      "Type::debug": |w| for field in &gen.fields {
        field.in_debug(w);
      },
      "Type::tables": |w| for gen in &gen.fields {
        w.emit(
          vars! {
            hasbit: gen.hasbit.map(|x| x as i32).unwrap_or(i32::MIN),
            name: ident(gen.field.name()),
            raw_name: gen.field.name(),
            size: |w| {
              if gen.field.is_repeated() {
                // This will clear only the FIRST pointer in the AVec that backs
                // this repeated field. This will set the length, which is
                // intentionally the first field, to zero.
                w.write("(usize::BITS / 8) as i32");
                return;
              }

              match gen.field.ty() {
                (TypeEnum::I32 | TypeEnum::U32 | TypeEnum::F32, _) => w.write("4"),
                (TypeEnum::I64 | TypeEnum::U64 | TypeEnum::F64, _) => w.write("8"),
                (TypeEnum::Bool, _) => w.write("1"),
                (TypeEnum::String, _) => w.write("(usize::BITS / 8 * 2) as i32"),
                (TypeEnum::Type, Some(ty)) => match ty.kind() {
                    Kind::Enum => w.write("4"),
                    _ => w.emit(
                      vars! { Type: type_name(ty) },
                      "-($Type::__LAYOUT.size() as i32)",
                    ),
                },
                _ => unreachable!(),
              }
            }
          },
          "
            #[doc(hidden)]
            pub const __OFFSET_${raw_name}: u32 = unsafe {
              let msg = $Type::DEFAULT;
              let top = msg.ptr.as_ptr().cast::<u8>();
              let field = &msg.ptr.as_ref().$name as *const _ as *const u8;
              field.offset_from(top) as u32
            };
            #[doc(hidden)]
            pub const __HAZZER_$raw_name: &$z::Hazzer = &$z::Hazzer {
              hasbit_or_number: $hasbit,
              offset: Self::__OFFSET_${raw_name},
              size: $size,
            };
          "
        )
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
      tdp_tys: |w| for &ty in &ty_ptrs {
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
              $z::tdp::Field {
                number: $number,
                flags: ($z::tdp::Kind::$tdp_kind as u8 as u32) | ($repeated << 4),
                offset: $Type::__OFFSET_${raw_name},
                ty: $ty_idx,
                hasbit: $hasbit_index,
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
      /// message `$package.$Name`
      $deprecated
      pub struct $Type {
        ptr: $z::ABox<$priv::Storage>,
        arena: $z::RawArena,
      }

      impl $Type {
        pub const DEFAULT: $rt::View<'static, Self> = unsafe {
          const VALUE: $priv::Storage = $priv::Storage {
            __hasbits: [0; $hasbit_words],
            ${Type::field_init}
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
          (&mut *raw.cast::<$priv::Storage>()).__hasbits = [0; $hasbit_words];
        }
        #[doc(hidden)]
        pub fn __tdp_info() -> *const $z::tdp::Type {
          &$priv::TDP_INFO as *const _ as *const $z::tdp::Type
        }
        #[doc(hidden)]
        pub unsafe fn __raw_data(&self) -> &[u8] {
          std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
        }
        ${Type::tables}
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

      impl $rt::value::Type for $Type {
        type __Storage = *mut u8;

        unsafe fn __make_view<'a>(ptr: *mut u8) -> $rt::View<'a, Self> {
          $priv::View {
            ptr: $z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
            _ph: $PhantomData,
          }
        }
        unsafe fn __make_mut<'a>(ptr: *mut u8, arena: $z::RawArena) -> $rt::Mut<'a, Self> {
          $priv::Mut {
            ptr: $z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
            arena,
            _ph: $PhantomData,
          }
        }

        unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: $z::RawArena) {
          (&mut *ptr.cast::<$z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
        }
      }

      impl<'proto> $priv::View<'proto> {
        pub fn as_view(&self) -> $rt::View<$Type> {
          $priv::View { ptr: self.ptr, _ph: $PhantomData }
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

        pub fn clear(self) {
          unsafe { $Type::__raw_clear(self.ptr.as_ptr()) }
        }

        pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), $rt::Error> {
          let mut ctx = $z::tdp::ParseCtx::new(input, self.arena);
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

      mod __priv_$Type {
        pub use super::*;

        #[repr(C)]
        pub struct Storage {
          pub(crate) __hasbits: [u32; $hasbit_words],
          ${Type::fields}    
        }
       
        pub static TDP_INFO: $z::tdp::TypeAndFields<{$NUM_FIELDS + 1}> =
          $z::tdp::TypeAndFields::<{$NUM_FIELDS + 1}> {
            msg: $z::tdp::Type {
              size: {
                let size = $Type::__LAYOUT.size();
                assert!(size <= (u32::MAX as usize));
                size as u32
              },
              tys: {
                const TYS: &[fn() -> *const $z::tdp::Type] = &[
                  $tdp_tys
                ];
                TYS.as_ptr()
              },
              kind: $z::tdp::TyKind::Msg,
            },
            fields: [
              $tdp_fields
              $z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
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
