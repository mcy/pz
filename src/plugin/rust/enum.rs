//! Enum codegen.

use crate::plugin::emit::SourceWriter;
use crate::plugin::rust::names::deprecated;
use crate::plugin::rust::names::ident;
use crate::plugin::rust::names::type_name;
use crate::plugin::Type;

pub fn emit(ty: Type, w: &mut SourceWriter) {
  w.emit(
    vars! {
      package: ident(ty.package()),
      Name: ident(ty.name()),
      Enum: type_name(ty),
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
      "DEFAULT": match ty.fields().next() {
        Some(f) => f.number().unwrap(),
        _ => 0,
      },
    },
    r#"
      /// enum `$package.$Name`
      $deprecated
      #[derive(Copy, Clone, PartialEq, Eq, Hash)]
      pub struct $Enum(pub i32);

      impl $Enum {
        ${Enum::Variants}

        pub const fn new() -> Self {
          Self($DEFAULT)
        }
      }

      impl Default for $Enum {
        fn default() -> Self {
          Self($DEFAULT)
        }
      }
    "#,
  );
  w.new_line();
}
