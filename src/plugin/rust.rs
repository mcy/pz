//! Rust codegen backend.

use std::fmt;

use crate::plugin::emit;
use crate::plugin::emit::SourceWriter;
use crate::plugin::exec_plugin;
use crate::plugin::Field;
use crate::plugin::Type;
use crate::proto::field::Type as TypeEnum;
use crate::proto::plugin;
use crate::proto::r#type::Kind;

const INESCAPABLE_KWS: &[&str] = &["crate", "self", "super", "Self"];

const KWS: &[&str] = &[
  "as", "break", "const", "continue", "crate", "else", "enum", "extern",
  "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
  "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
  "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async",
  "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
  "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

pub fn ident(name: &str) -> impl fmt::Display + '_ {
  emit::display(move |f| {
    if INESCAPABLE_KWS.contains(&name) {
      write!(f, "{name}_")
    } else if KWS.contains(&name) {
      write!(f, "r#{name}")
    } else {
      f.write_str(name)
    }
  })
}

fn type_name<'ccx>(ty: Type<'ccx>) -> impl fmt::Display + 'ccx {
  emit::display(move |f| {
    fmt::Display::fmt(&ident(&ty.name().replace(".", "_")), f)
  })
}

fn field_type_name<'ccx>(field: Field<'ccx>) -> impl fmt::Display + 'ccx {
  emit::display(move |f| {
    if let (TypeEnum::Type, Some(ty)) = field.ty() {
      write!(f, "{}", type_name(ty))?;
      return Ok(());
    }

    let name = match field.ty() {
      (TypeEnum::I32, _) => "i32",
      (TypeEnum::U32, _) => "u32",
      (TypeEnum::F32, _) => "f32",
      (TypeEnum::I64, _) => "i64",
      (TypeEnum::U64, _) => "u64",
      (TypeEnum::F64, _) => "f64",
      (TypeEnum::Bool, _) => "bool",
      (TypeEnum::String, _) => "Vec<u8>",
      _ => todo!(),
    };

    write!(f, "{name}")
  })
}

fn storage_for<'ccx>(field: Field<'ccx>) -> impl fmt::Display + 'ccx {
  emit::display(move |f| {
    let name = field_type_name(field);
    if field.is_repeated() {
      write!(f, "Vec<{name}>")
    } else {
      write!(f, "{name}")
    }
  })
}

fn deprecated<'a>(reason: Option<&'a str>) -> impl fmt::Display + 'a {
  emit::display(move |f| match reason {
    Some(value) => write!(f, "#[deprecated = {value:?}]"),
    _ => Ok(()),
  })
}

pub fn rust_plugin() -> ! {
  exec_plugin(
    |_| plugin::AboutResponse {
      name: Some("rust".into()),
      version: Some(env!("CARGO_PKG_VERSION").into()),
      options: vec![plugin::about_response::Option {
        name: Some("rt-crate".into()),
        help: Some(
          "Rust crate name to use for importing the pz runtime".into(),
        ),
      }],
    },
    |ctx| {
      let mut w = emit::SourceWriter::new(emit::Options::default());
      w.emit(
        [],
        r"
        // ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
        #![rustfmt::skip]
      ",
      );

      let rt = ctx.option("rt-crate").unwrap_or("pz");
      if rt == "crate" {
        w.emit(
          vars! {},
          r"
          use crate as __rt;
        ",
        );
      } else {
        w.emit(
          vars! { rt },
          r"
          extern $rt as __rt;
        ",
        );
      }
      w.new_line();

      for ty in ctx.types_to_generate() {
        w.with_vars(
          vars! {
            deprecated: deprecated(
              ty.proto().attrs.as_ref().and_then(|a| a.deprecated.as_deref())),
          },
          |w| match ty.kind() {
            crate::proto::r#type::Kind::Message => emit_message(ty, w),
            crate::proto::r#type::Kind::Struct => {
              ctx
                .warn("sorry: can't emit this kind of type yet")
                .at(ty.span().unwrap());
            }
            crate::proto::r#type::Kind::Choice => {
              ctx
                .warn("sorry: can't emit this kind of type yet")
                .at(ty.span().unwrap());
            }
            crate::proto::r#type::Kind::Enum => emit_enum(ty, w),
          },
        )
      }

      ctx.add_file(plugin::codegen_response::File {
        path: Some("lib.pz.rs".into()),
        content: Some(w.to_string().into_bytes()),
      })
    },
  )
}

fn emit_message(ty: Type, w: &mut SourceWriter) {
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
            vars! { name: ident(field.name()) },
            r"
              $name: todo!(),
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
            fn $name(&self) -> $Type {
              self.${name}_opt().unwrap_or_default()
            }
            $deprecated
            fn ${name}_opt(&self) -> Option<$Type> {
              if self.__hasbits[$hasbit_word] & $hasbit_bit != 0 {
                Some(self.$name)
              } else {
                None
              }
            }
            $deprecated
            fn ${name}_set(&mut self, value: impl Into<Option<$Type>>) {
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
            fn $name(&self) -> &[$Type] {
              &self.$name
            }
            $deprecated
            fn ${name}_mut(&mut self) -> &mut [$Type] {
              &mut self.$name
            }
            $deprecated
            fn ${name}_set(&mut self, that: &[$Type]) {
              self.$name.clear();
              self.${name}_extend(that)
            }
            $deprecated
            fn ${name}_extend(&mut self, that: &[$Type]) {
              self.$name.extend_from_slice(that)
            }
          ",
          );
        }
      //} else if let (TypeEnum::String, _) = field.ty() {
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

fn emit_enum(ty: Type, w: &mut SourceWriter) {
  w.emit(
    vars! {
      package: ident(ty.package()),
      Name: ident(ty.name()),
      Enum: type_name(ty),
      "Enum::VARIANTS": |w| for field in ty.fields() {
        w.emit(
          vars! {
            NAME: ident(field.name()),
            NUMBER: field.number().unwrap(),
            deprecated: deprecated(
              field.proto().attrs.as_ref().and_then(|a| a.deprecated.as_deref())),
          },
          r"
            $deprecated
            pub const $NAME: Self = Self($NUMBER);
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
      pub struct $Enum(pub i32);

      impl $Enum {
        ${Enum::VARIANTS}
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
