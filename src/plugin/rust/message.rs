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
      "Msg::fields": |w| for field in &gen.fields {
        field.in_storage(w);
      },
      "Msg::field_init": |w| for field in &gen.fields {
        field.in_storage_init(w);
      },
      "Msg::clear": |w| for field in &gen.fields {
        field.in_clear(w);
      },
      "Msg::drop": |w| for field in &gen.fields {
        field.in_drop(w);
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
      #[derive(Clone)]
      pub struct $Msg {
        ptr: Box<__priv_$Msg::Storage>,
      }

      impl $Msg {
        pub const DEFAULT: $rt::View<'static, Self> = {
          const VALUE: __priv_$Msg::Storage = __priv_$Msg::Storage {
            __hasbits: [0; $hasbit_words],
            ${Msg::field_init}
          };
          $rt::View::<Self> { ptr: &VALUE }
        };
        
        pub fn new() -> Self {
          Self {
            ptr: Box::new(__priv_$Msg::Storage {
              __hasbits: [0; $hasbit_words],
              ${Msg::field_init}
            }),
          }
        }

        pub fn as_view(&self) -> $rt::View<Self> {
          __priv_$Msg::View { ptr: &self.ptr }
        }

        pub fn as_mut(&mut self) -> $rt::Mut<Self> {
          __priv_$Msg::Mut { ptr: &mut self.ptr }
        }

        pub fn clear(&mut self) {
          self.as_mut().clear();
        }

        ${Msg::access}
      }

      impl Default for $Msg {
        fn default() -> Self {
          Self::new()
        }
      }

      impl $rt::rt::ptr::Proxied for $Msg {
        type View<'msg> = __priv_$Msg::View<'msg>;
        type Mut<'msg> = __priv_$Msg::Mut<'msg>;
      }

      impl<'msg> __priv_$Msg::View<'msg> {
        ${View::access}
      }

      impl<'msg> __priv_$Msg::Mut<'msg>  {
        pub fn clear(&mut self) {
          self.ptr.__hasbits = [0; $hasbit_words];
          ${Msg::clear}
        }

        ${Mut::access}
      }

      impl Drop for $Msg {
        fn drop(&mut self) {
          ${Msg::drop}
        }
      }

      mod __priv_$Msg {
        pub use super::*;

        #[derive(Clone)]
        pub struct Storage {
          pub(crate) __hasbits: [u32; $hasbit_words],
          ${Msg::fields}    
        }
       
        #[derive(Copy, Clone)]
        pub struct View<'msg> {
          pub(crate) ptr: &'msg Storage,
        }
       
        impl<'msg> $rt::rt::ptr::ViewFor<'msg, super::$Msg> for View<'msg> {
          fn as_view(&self) -> View {
            View { ptr: self.ptr }
          }
        }

        pub struct Mut<'msg> {
          pub(crate) ptr: &'msg mut Storage,
        }
       
        impl<'msg> $rt::rt::ptr::ViewFor<'msg, super::$Msg> for Mut<'msg> {
          fn as_view(&self) -> View {
            View { ptr: &self.ptr }
          }
        }

        impl<'msg> $rt::rt::ptr::MutFor<'msg, super::$Msg> for Mut<'msg> {
          fn into_view(self) -> View<'msg> {
            View { ptr: self.ptr }
          }

          fn as_mut(&mut self) -> Mut {
            Mut { ptr: self.ptr }
          }
        }
      }
    "#,
  );
  w.new_line();
}
