//! Message codegen.

use crate::plugin::emit::SourceWriter;
use crate::plugin::Field;
use crate::plugin::Type;
use crate::proto::field::Type as TypeEnum;
use crate::proto::r#type::Kind;

use crate::plugin::rust::names::default_for;
use crate::plugin::rust::names::deprecated;
use crate::plugin::rust::names::field_type_name;
use crate::plugin::rust::names::ident;
use crate::plugin::rust::names::storage_for;
use crate::plugin::rust::names::type_name;

pub fn emit(ty: Type, w: &mut SourceWriter) {
  let singular_count = ty.fields().filter(|f| !f.is_repeated()).count();
  let hasbit_words = singular_count / 32 + (singular_count % 32 != 0) as usize;

  w.emit(
    vars! {
      package: ident(ty.package()),
      Name: ident(ty.name()),
      Msg: type_name(ty),
      "Msg::fields": |w| {
        w.emit(vars! { hasbit_words }, r"
          __hasbits: [u32; $hasbit_words],
        ");
        for field in ty.fields() {
          w.emit(
            vars! {
              name: ident(field.name()),
              ty: storage_for(field),
            },
            r"
              $name: $ty,
            "
          );
        }
      },
      "Msg::field_init": |w| {
        w.emit(vars! { hasbit_words }, r"
          __hasbits: [0; $hasbit_words],
        ");
        for field in ty.fields() {
          w.emit(
            vars! {
              name: ident(field.name()),
              default: default_for(field),
            },
            r"
              $name: $default,
            "
          );
        }
      },
      "Msg::accessors": |w| {
        let mut hasbit_index = 0;
        for field in ty.fields() {
          emit_message_accessors(field, hasbit_index, w);
          if !field.is_repeated() {
            hasbit_index += 1;
          }
        }
      },
    },
    r#"
      /// message `$package.$Name`
      $deprecated
      #[derive(Clone)]
      pub struct $Msg {
        ${Msg::fields}
      }

      impl $Msg {
        pub const fn new() -> Self {
          Self {
            ${Msg::field_init}
          }
        }

        ${Msg::accessors}
      }

      impl Default for $Msg {
        fn default() -> Self {
          Self::new()
        }
      }
    "#,
  );
  w.new_line();
}

fn emit_message_accessors(
  field: Field,
  hasbit_index: u32,
  w: &mut SourceWriter,
) {
  w.with_vars(
    vars! {
      deprecated: deprecated(
        field.proto().attrs.as_ref().and_then(|a| a.deprecated.as_deref())),
    },
    |w| {
      let hasbit_word = hasbit_index / 32;
      let hasbit_bit = 1 << (hasbit_index % 32);

      let is_scalar = match field.ty() {
        (
          TypeEnum::I32
          | TypeEnum::U32
          | TypeEnum::F32
          | TypeEnum::I64
          | TypeEnum::U64
          | TypeEnum::F64
          | TypeEnum::Bool,
          _,
        ) => true,
        (TypeEnum::Type, Some(ty)) => ty.kind() == Kind::Enum,
        _ => false,
      };

      if is_scalar {
        if !field.is_repeated() {
          w.emit(
            vars! {
              hasbit_word,
              hasbit_bit,
              name: ident(field.name()),
              Type: field_type_name(field),
            },
            r"
              $deprecated
              pub fn $name(&self) -> $Type {
                self.${name}_opt().unwrap_or_default()
              }
              $deprecated
              pub fn ${name}_opt(&self) -> Option<$Type> {
                if self.__hasbits[$hasbit_word] & $hasbit_bit != 0 {
                  Some(self.$name)
                } else {
                  None
                }
              }
              $deprecated
              pub fn ${name}_set(&mut self, value: impl Into<Option<$Type>>) {
                match value.into() {
                  Some(value) => {
                    self.__hasbits[$hasbit_word] |= $hasbit_bit;
                    self.$name = value;
                  }
                  None => {
                    self.__hasbits[$hasbit_word] &= !$hasbit_bit;
                  }
                }
              }
            ",
          );
        } else {
          w.emit(
            vars! {
              name: ident(field.name()),
              Type: field_type_name(field),
            },
            r"
              $deprecated
              pub fn $name(&self) -> &[$Type] {
                &self.$name
              }
              $deprecated
              pub fn ${name}_mut(&mut self) -> &mut [$Type] {
                &mut self.$name
              }
              $deprecated
              pub fn ${name}_set(&mut self, that: &[$Type]) {
                self.$name.clear();
                self.${name}_extend(that)
              }
              $deprecated
              pub fn ${name}_extend(&mut self, that: &[$Type]) {
                self.$name.extend_from_slice(that)
              }
            ",
          );
        }
      } else if let (TypeEnum::String, _) = field.ty() {
        if !field.is_repeated() {
          w.emit(
            vars! {
              hasbit_word,
              hasbit_bit,
              name: ident(field.name()),
              Type: field_type_name(field),
            },
            r"
              $deprecated
              pub fn $name(&self) -> &__rt::Str {
                self.${name}_opt().unwrap_or_default()
              }
              $deprecated
              pub fn ${name}_opt(&self) -> Option<&__rt::Str> {
                if self.__hasbits[$hasbit_word] & $hasbit_bit != 0 {
                  Some(__rt::Str::new(&*self.$name))
                } else {
                  None
                }
              }
              $deprecated
              pub fn ${name}_mut(&mut self) -> __rt::StrBuf {
                self.__hasbits[$hasbit_word] |= $hasbit_bit;
                __rt::StrBuf::__wrap(&mut self.$name)
              }
              $deprecated
              pub fn ${name}_opt_mut(&mut self) -> Option<__rt::StrBuf> {
                if self.__hasbits[$hasbit_word] & $hasbit_bit != 0 {
                  Some(__rt::StrBuf::__wrap(&mut self.$name))
                } else {
                  None
                }
              }
              $deprecated
              pub fn ${name}_set<'a>(&mut self, value: impl __rt::rt::str::IntoStrOpt<'a>) {
                match value.into_str_opt() {
                  Some(value) => {
                    self.__hasbits[$hasbit_word] |= $hasbit_bit;
                    self.$name.clear();
                    self.$name.extend_from_slice(value.as_bytes())
                  }
                  None => {
                    self.__hasbits[$hasbit_word] &= !$hasbit_bit;
                    self.$name.clear();
                  }
                }
              }
            ",
          );
        } else {
          w.emit(
            vars! { name: ident(field.name()), },
            r"
              $deprecated
              pub fn ${name}_len(&self) -> usize {
                self.${name}.len()
              }
              $deprecated
              pub fn $name(&self, n: usize) -> Option<&__rt::Str> {
                self.${name}.get(n).map(__rt::Str::new)
              }
              $deprecated
              pub fn ${name}_mut(&mut self, n: usize) -> Option<__rt::StrBuf> {
                self.${name}.get_mut(n).map(__rt::StrBuf::__wrap)
              }
              $deprecated
              pub fn ${name}_add(&mut self) -> __rt::StrBuf {
                self.${name}.push(Default::default());
                self.${name}.last_mut().map(__rt::StrBuf::__wrap).unwrap()
              }
              $deprecated
              pub fn ${name}_iter(&self) -> impl Iterator<Item = &__rt::Str> + '_ {
                self.${name}.iter().map(__rt::Str::new)
              }
              $deprecated
              pub fn ${name}_resize(&mut self, n: usize) {
                self.${name}.resize(n, Default::default())
              }
            ",
          );
        }
      } else if let (TypeEnum::Type, Some(ty)) = field.ty() {
        if !field.is_repeated() {
          w.emit(
            vars! {
              hasbit_word,
              hasbit_bit,
              name: ident(field.name()),
              Submsg: type_name(ty),
            },
            r#"
              $deprecated
              pub fn $name(&self) -> &$Submsg {
                self.${name}_opt().expect("no message defaults yet...")
              }
              $deprecated
              pub fn ${name}_opt(&self) -> Option<&$Submsg> {
                if self.__hasbits[$hasbit_word] & $hasbit_bit != 0 {
                  Some(&self.$name)
                } else {
                  None
                }
              }
              $deprecated
              pub fn ${name}_mut(&mut self) -> &mut $Submsg {
                self.__hasbits[$hasbit_word] |= $hasbit_bit;
                &mut self.$name
              }
              $deprecated
              pub fn ${name}_opt_mut(&mut self) -> Option<$Submsg> {
                if self.__hasbits[$hasbit_word] & $hasbit_bit != 0 {
                  Some(&mut self.$name)
                } else {
                  None
                }
              }
              $deprecated
              pub fn ${name}_clear(&mut self) {
                self.__hasbits[$hasbit_word] &= !$hasbit_bit;
                self.$name = Default::default();
              }
            "#,
          );
        } else {
          w.emit(
            vars! {
              name: ident(field.name()),
              Submsg: type_name(ty),
            },
            r"
                $deprecated
                pub fn ${name}_len(&self) -> usize {
                  self.${name}.len()
                }
                $deprecated
                pub fn $name(&self, n: usize) -> Option<&$Submsg> {
                  self.${name}.get(n)
                }
                $deprecated
                pub fn ${name}_mut(&mut self, n: usize) -> Option<&mut $Submsg> {
                  self.${name}.get_mut(n)
                }
                $deprecated
                pub fn ${name}_add(&mut self) -> &mut $Submsg {
                  self.${name}.push(Default::default());
                  self.${name}.last_mut().unwrap()
                }
                $deprecated
                pub fn ${name}_iter(&self) -> impl Iterator<Item = &$Submsg> + '_ {
                  self.${name}.iter()
                }
                $deprecated
                pub fn ${name}_resize(&mut self, n: usize) {
                  self.${name}.resize(n, Default::default())
                }
              ",
          );
        }
      } else {
        field
          .ccx()
          .warn("no support for this field type yet")
          .at(field.span().unwrap());
      }

      w.new_line()
    },
  )
}
