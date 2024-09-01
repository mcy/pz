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
      NUM_TYS: ty_ptrs.len(),

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
      "View::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::ViewImpl, w);
        w.new_line();
      },
      "Mut::access": |w| for field in &gen.fields {
        field.in_ref_methods(Where::MutImpl, w);
        field.in_mut_methods(Where::MutImpl, w);
        w.new_line();
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
              __z::tdp::FieldStorage {
                number: $number,
                flags: 
                  __z::tdp::Kind::$tdp_kind.raw() << __z::tdp::Field::KIND_SHIFT |
                  $repeated << __z::tdp::Field::REP_SHIFT,
                offset: unsafe {
                  let msg = $Type::DEFAULT;
                  let top = msg.ptr.as_ptr().cast::<u8>();
                  let field = &msg.ptr.as_ref().$name as *const _ as *const u8;
                  field.offset_from(top) as u32
                },
                desc: $ty_idx,
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
      pub struct $Ident {
        ptr: __z::ABox<$priv::Storage>,
        arena: __z::RawArena,
      }

      const _: () = {
        assert!(
          $size_of::<$priv::Storage>() < (u32::MAX as usize),
          "storage size excees 4GB",
        );
      };

      impl $Type {
        pub const DEFAULT: $View<'static, Self> = unsafe {
          const VALUE: $priv::Storage = $priv::Storage {
            __hasbits: [0; $hasbit_words],
            ${Type::field_init}
          };
          $View::<Self> {
            ptr: __z::ABox::from_ptr(&VALUE as *const $priv::Storage as *mut $priv::Storage as *mut u8),
            _ph: $PhantomData,
          }
        };
        
        pub fn new() -> Self {
          let arena = __z::RawArena::new();
          let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
          unsafe {
            ptr.write_bytes(0, Self::__LAYOUT.size());
            Self { ptr: __z::ABox::from_ptr(ptr), arena }
          }
        }

        pub fn from_pb(input: &mut dyn $Read) -> $Result<Self, __rt::Error> {
          let mut new = Self::new();
          new.parse_pb(input)?;
          $Ok(new)
        }

        pub fn parse_pb(&mut self, input: &mut dyn $Read) -> $Result<(), __rt::Error> {
          self.as_mut().parse_pb(input)
        }

        pub fn as_view(&self) -> $View<Self> {
          $priv::View { ptr: self.ptr, _ph: $PhantomData }
        }

        pub fn as_mut(&mut self) -> $Mut<Self> {
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
        pub fn __tdp_info() -> __z::tdp::Desc {
          unsafe { $priv::TDP_INFO.get() }
        }
        #[doc(hidden)]
        pub unsafe fn __raw_data(&self) -> &[u8] {
          __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
        }
      }

      impl $Default for $Type {
        fn default() -> Self {
          Self::new()
        }
      }

      impl __rt::ptr::Proxied for $Type {
        type View<'proto> = $priv::View<'proto>;
        type Mut<'proto> = $priv::Mut<'proto>;
      }

      impl __rt::Type for $Type {
        type __Storage = *mut u8;

        unsafe fn __make_view<'a>(ptr: *const *mut u8) -> $View<'a, Self> {
          $priv::View {
            ptr: __z::ABox::from_ptr(ptr.read()),
            _ph: $PhantomData,
          }
        }
        unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> $Mut<'a, Self> {
          $priv::Mut {
            ptr: __z::ABox::from_ptr(ptr.read()),
            arena,
            _ph: $PhantomData,
          }
        }

        unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
          vec.resize_msg(new_len, arena, Self::__LAYOUT)
        }
      }

      impl<'proto> $priv::View<'proto> {
        pub fn as_view(&self) -> $View<$Type> {
          $priv::View { ptr: self.ptr, _ph: $PhantomData }
        }
        
        ${View::access}

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

      impl $Default for $priv::View<'_> {
        fn default() -> Self {
          $Type::DEFAULT
        }
      }

      impl<'proto> $priv::Mut<'proto>  {
        pub fn as_view(&self) -> $View<$Type> {
          $priv::View { ptr: self.ptr, _ph: $PhantomData }
        }

        pub fn into_view(self) -> $View<'proto, $Type> {
          $priv::View { ptr: self.ptr, _ph: $PhantomData }
        }

        pub fn as_mut(&mut self) -> $Mut<$Type> {
          $priv::Mut { ptr: self.ptr, _ph: $PhantomData, arena: self.arena }
        }

        pub fn clear(self) {
          unsafe { $Type::__raw_clear(self.ptr.as_ptr()) }
        }

        pub fn parse_pb(self, input: &mut dyn $Read) -> $Result<(), __rt::Error> {
          let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
          ctx.parse(self.ptr.as_ptr() as *mut u8, $TDP)
        }

        ${Mut::access}
      }

      impl __s::ops::Drop for $Type {
        fn drop(&mut self) {
          unsafe { self.arena.destroy() }
        }
      }

      impl $fmt::Debug for $priv::View<'_> {
        fn fmt(&self, fmt: &mut $fmt::Formatter) -> $fmt::Result {
          fmt.write_str("$package.$Name ")?;
          let mut debug = __z::Debug::new(fmt);
          self.__debug(&mut debug)
        }
      }

      impl $fmt::Debug for $priv::Mut<'_> {
        fn fmt(&self, fmt: &mut $fmt::Formatter) -> $fmt::Result {
          use __rt::ptr::ViewFor;
          $fmt::Debug::fmt(&self.as_view(), fmt)
        }
      }

      impl $fmt::Debug for $Type {
        fn fmt(&self, fmt: &mut $fmt::Formatter) -> $fmt::Result {
          $fmt::Debug::fmt(&self.as_view(), fmt)
        }
      }

      mod $priv {
        pub use super::*;

        #[repr(C)]
        pub struct Storage {
          pub(crate) __hasbits: [u32; $hasbit_words],
          ${Type::fields}    
        }
       
        pub static TDP_INFO: __z::tdp::DescStorage<{$NUM_FIELDS + 1}> =
          __z::tdp::DescStorage::<{$NUM_FIELDS + 1}> {
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
              num_hasbit_words: $hasbit_words,
              kind: __z::tdp::DescKind::Message,
            },
            fields: [
              $tdp_fields
              __z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
            ],
          };

        #[derive(Copy, Clone)]
        pub struct View<'proto> {
          pub(in super) ptr: __z::ABox<$priv::Storage>,
          pub(in super) _ph: $PhantomData<&'proto $Type>,
        }
       
        impl<'proto> __rt::ptr::ViewFor<'proto, super::$Type> for View<'proto> {
          fn as_view(&self) -> View {
            View { ptr: self.ptr, _ph: $PhantomData }
          }
        }

        pub struct Mut<'proto> {
          pub(in super) ptr: __z::ABox<$priv::Storage>,
          pub(in super) _ph: $PhantomData<&'proto mut $Type>,
          pub(in super) arena: __z::RawArena,
        }
       
        impl<'proto> __rt::ptr::ViewFor<'proto, super::$Type> for Mut<'proto> {
          fn as_view(&self) -> View {
            View { ptr: self.ptr, _ph: $PhantomData }
          }
        }

        impl<'proto> __rt::ptr::MutFor<'proto, super::$Type> for Mut<'proto> {
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
