//! Message codegen.

use std::collections::HashMap;

use crate::plugin::emit::SourceWriter;
use crate::plugin::rust::fields::FieldGenerators;
use crate::plugin::rust::fields::Where;
use crate::plugin::rust::names::ident;
use crate::plugin::rust::names::type_name;
use crate::plugin::Type;
use crate::proto;
use crate::proto::field::Type as TypeEnum;
use crate::proto::r#type::Kind;

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
      Msg: type_name(ty),
      NUM_FIELDS: ty.fields().count(),
      NUM_TYS: ty_ptrs.len(),
      priv: format!("__priv_{}", type_name(ty)),
      "Msg::fields": |w| for field in &gen.fields {
        field.in_storage(w);
      },
      "Msg::field_init": |w| for field in &gen.fields {
        field.in_storage_init(w);
      },
      "Msg::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::MsgImpl, w);
        field.in_mut_methods(Where::MsgImpl, w);
        w.new_line();
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
      "Msg::OFFSETS": |w| for field in ty.fields() {
        w.emit(
          vars! {
            name: ident(field.name()),
            raw_name: field.name(),
          },
          "
            pub const FIELD_OFFSET_${raw_name}: u32 = unsafe {
              let msg = $Msg::DEFAULT;
              let top = msg.ptr.as_ptr().cast::<u8>();
              let field = &msg.ptr.as_ref().$name as *const _ as *const u8;
              field.offset_from(top) as u32
            };
          "
        );
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
            (TypeEnum::I64 | TypeEnum::U64, _) => "I32",
            (TypeEnum::F32, _) => "F32",
            (TypeEnum::F64, _) => "F64",
            (TypeEnum::Bool, _) => "Bool",
            (TypeEnum::String, _) => "Str",
            (TypeEnum::Type, Some(ty)) => match ty.kind() {
              Kind::Message => {
                ty_idx = ty_map[&(ty.proto() as *const _)];
                "Msg"
              }
              Kind::Struct => todo!(),
              Kind::Choice => todo!(),
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
                offset: $priv::FIELD_OFFSET_${raw_name},
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
      pub struct $Msg {
        ptr: $z::ABox<$priv::Storage>,
        arena: $z::RawArena,
      }

      impl $Msg {
        pub const DEFAULT: $rt::View<'static, Self> = unsafe {
          const VALUE: $priv::Storage = $priv::Storage {
            __hasbits: [0; $hasbit_words],
            ${Msg::field_init}
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
            Self {
              ptr: $z::ABox::from_ptr(ptr),
              arena,
            }
          }
        }

        pub fn parsed(input: &mut dyn std::io::Read) -> Result<Self, $rt::Error> {
          let mut new = Self::new();
          new.parse(input)?;
          Ok(new)
        }

        pub fn parse(&mut self, input: &mut dyn std::io::Read) -> Result<(), $rt::Error> {
          self.as_mut().parse(input)
        }

        pub fn as_view(&self) -> $rt::View<Self> {
          $priv::View { ptr: self.ptr, _ph: $PhantomData }
        }

        pub fn as_mut(&mut self) -> $rt::Mut<Self> {
          $priv::Mut { ptr: self.ptr, _ph: $PhantomData, arena: self.arena }
        }

        pub fn clear(&mut self) {
          unsafe { $Msg::__raw_clear(self.ptr.as_ptr()) }
        }

        pub fn into_raw(self) -> *mut u8 {
          self.ptr.as_ptr()
        }

        ${Msg::access}

        #[doc(hidden)]
        pub const __LAYOUT: $Layout = $Layout::new::<$priv::Storage>();
        #[doc(hidden)]
        pub unsafe fn __raw_clear(raw: *mut u8) {
          (&mut *raw.cast::<$priv::Storage>()).__hasbits = [0; $hasbit_words];
        }
        #[doc(hidden)]
        pub fn __tdp_info() -> *const $z::tdp::Message {
          &$priv::TDP_INFO as *const _ as *const $z::tdp::Message
        }
      }

      impl Default for $Msg {
        fn default() -> Self {
          Self::new()
        }
      }

      impl $rt::ptr::Proxied for $Msg {
        type View<'msg> = $priv::View<'msg>;
        type Mut<'msg> = $priv::Mut<'msg>;
      }

      impl<'msg> $priv::View<'msg> {
        ${View::access}
      }

      impl<'msg> $priv::Mut<'msg>  {
        pub fn clear(self) {
          unsafe { $Msg::__raw_clear(self.ptr.as_ptr()) }
        }

        pub fn parse(self, input: &mut dyn std::io::Read) -> Result<(), $rt::Error> {
          dbg!(&$priv::TDP_INFO);
          let mut ctx = $z::tdp::ParseCtx::new(input, self.arena);
          ctx.parse(self.ptr.as_ptr() as *mut u8, $Msg::__tdp_info())
        }

        ${Mut::access}
      }

      impl Drop for $Msg {
        fn drop(&mut self) {
          unsafe { self.arena.destroy() }
        }
      }

      mod __priv_$Msg {
        pub use super::*;

        #[repr(C)]
        pub struct Storage {
          pub(crate) __hasbits: [u32; $hasbit_words],
          ${Msg::fields}    
        }

        ${Msg::OFFSETS}
       
        pub static TDP_INFO: $z::tdp::MessageAndFields<{$NUM_FIELDS + 1}> =
          $z::tdp::MessageAndFields::<{$NUM_FIELDS + 1}> {
            msg: $z::tdp::Message {
              size: {
                let size = $Msg::__LAYOUT.size();
                assert!(size <= (u32::MAX as usize));
                size as u32
              },
              tys: {
                const TYS: &[fn() -> *const $z::tdp::Message] = &[
                  $tdp_tys
                ];
                TYS.as_ptr()
              },
              raw_clear: $Msg::__raw_clear,
            },
            fields: [
              $tdp_fields
              $z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
            ],
          };

        #[derive(Copy, Clone)]
        pub struct View<'msg> {
          pub(in super) ptr: $z::ABox<$priv::Storage>,
          pub(in super) _ph: $PhantomData<&'msg $Msg>,
        }
       
        impl<'msg> $rt::ptr::ViewFor<'msg, super::$Msg> for View<'msg> {
          fn as_view(&self) -> View {
            View { ptr: self.ptr, _ph: $PhantomData }
          }
        }

        pub struct Mut<'msg> {
          pub(in super) ptr: $z::ABox<$priv::Storage>,
          pub(in super) _ph: $PhantomData<&'msg mut $Msg>,
          pub(in super) arena: $z::RawArena,
        }
       
        impl<'msg> $rt::ptr::ViewFor<'msg, super::$Msg> for Mut<'msg> {
          fn as_view(&self) -> View {
            View { ptr: self.ptr, _ph: $PhantomData }
          }
        }

        impl<'msg> $rt::ptr::MutFor<'msg, super::$Msg> for Mut<'msg> {
          fn into_view(self) -> View<'msg> {
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
