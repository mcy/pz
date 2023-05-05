//! Message codegen.

use crate::plugin::emit::SourceWriter;
use crate::plugin::Type;

use crate::plugin::rust::fields::FieldGenerators;
use crate::plugin::rust::fields::Where;
use crate::plugin::rust::names::ident;
use crate::plugin::rust::names::type_name;

pub fn emit(ty: Type, w: &mut SourceWriter) {
  let gen = FieldGenerators::build(ty.fields());
  let hasbit_words = gen.num_hasbits / 32 + (gen.num_hasbits % 32 != 0) as u32;

  w.emit(
    vars! {
      hasbit_words,
      package: ident(ty.package()),
      Name: ident(ty.name()),
      Msg: type_name(ty),
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
    },
    r#"
      /// message `$package.$Name`
      $deprecated
      pub struct $Msg {
        ptr: $z::ABox<$priv::Storage>,
        arena: $z::RawArena,
      }

      impl $Msg {
        #[doc(hidden)]
        pub const __LAYOUT: $Layout = $Layout::new::<$priv::Storage>();
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
        pub unsafe fn __raw_clear(raw: *mut u8) {
          (&mut *raw.cast::<$priv::Storage>()).__hasbits = [0; $hasbit_words];
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
        pub fn clear(&mut self) {
          unsafe { $Msg::__raw_clear(self.ptr.as_ptr()) }
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

        pub struct Storage {
          pub(crate) __hasbits: [u32; $hasbit_words],
          ${Msg::fields}    
        }
       
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
