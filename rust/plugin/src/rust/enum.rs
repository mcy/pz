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
      pub struct $Ident(pub __s::primitive::i32);

      impl $Type {
        ${Enum::Variants}

        pub const fn new() -> Self {
          Self($DEFAULT)
        }
      }

      impl $Default for $Type {
        fn default() -> Self {
          Self($DEFAULT)
        }
      }

      impl __z::Type for $Type {
        type __Storage<S: __z::Sealed> = Self;
  
        unsafe fn __ref<'a, S: __z::Sealed>(_: S, ptr: $NonNull<Self>) -> __rt::Ref<'a, Self> {
          ptr.read()
        }
  
        unsafe fn __mut<'a, S: __z::Sealed>(
          _: S,
          mut ptr: $NonNull<Self>,
          _: __z::RawArena,
        ) -> __rt::Mut<'a, Self> {
          __rt::ScalarMut::__wrap(ptr.as_mut())
        }
      }
  
      impl __r::Views for $Type {
        type Ref<'a> = Self;
        type Mut<'a> = __rt::ScalarMut<'a, Self>;
      }
  
      impl __r::RefView<'_> for $Type {
        type Target = Self;
  
        fn as_ref(&self) -> Self {
          *self
        }
      }

      impl __r::Set<$Type> for $Type {
        fn apply_to(self, mut m: __r::Mut<$Type>) {
          m.set(self)
        }
      }

      impl __r::Set<__r::Opt<$Type>> for $Type {
        fn apply_to(self, m: __r::Mut<__r::Opt<$Type>>) {
          m.into_inner().set(self)
        }
      }

      impl $fmt::Debug for $Type {
        fn fmt(&self, fmt: &mut $fmt::Formatter) -> $fmt::Result {
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
