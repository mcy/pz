//! Enum codegen.

use crate::emit::SourceWriter;
use crate::rust::names::deprecated;
use crate::rust::names::ident;
use crate::Type;


pub fn emit(ty: Type, w: &mut SourceWriter) {
  w.emit(
    vars! {
      "Enum::Variants": |w| for field in ty.fields() {
        w.emit(
          vars! {
            Name: ident(&heck::AsPascalCase(field.name()).to_string()),
            NUMBER: field.number().unwrap(),
            deprecated: deprecated(
              field.proto().attrs.as_ref().and_then(|a| a.deprecated.as_deref())),
          },
          r"
            $deprecated
            pub const $Name: Self = Self($NUMBER);
          "
        );
      },
      debug_arms: |w| for field in ty.fields() {
        w.emit(
          vars! {
            Name: ident(&heck::AsPascalCase(field.name()).to_string()),
          },
          r#"
            Self::$Name => __s::write!(fmt, "$Name"),
          "#,
        );
      },
      "DEFAULT": match ty.fields().next() {
        Some(f) => f.number().unwrap(),
        _ => 0,
      },
    },
    r#"
      /// enum `$package.$Name`
      $deprecated
      #[derive(Copy, Clone, PartialEq, Eq, Hash)]
      #[repr(transparent)]
      pub struct $Ident(pub i32);

      impl $Type {
        ${Enum::Variants}

        pub const fn new() -> Self {
          Self($DEFAULT)
        }
      }

      impl __s::default::Default for $Type {
        fn default() -> Self {
          Self($DEFAULT)
        }
      }

      impl __rt::ptr::Proxied for $Type {
        type View<'a> = Self;
        type Mut<'a> = __rt::ptr::ScalarMut<'a, Self>;
      }

      impl __s::fmt::Debug for $Type {
        fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
          match *self {
            $debug_arms
            Self(n) => __s::write!(fmt, "$package.$Name({n})"),
          }
        }
      }
    "#,
  );
  w.new_line();
}
