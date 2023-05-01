//! Naming helper functions.

use std::fmt;

use crate::plugin::emit;
use crate::plugin::Field;
use crate::plugin::Type;
use crate::proto::field::Type as TypeEnum;

const INESCAPABLE_KWS: &[&str] = &["crate", "self", "super", "Self"];

const KWS: &[&str] = &[
  "as", "break", "const", "continue", "crate", "else", "enum", "extern",
  "false", "pub fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
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

pub fn type_name<'ccx>(ty: Type<'ccx>) -> impl fmt::Display + 'ccx {
  emit::display(move |f| {
    fmt::Display::fmt(&ident(&ty.name().replace(".", "_")), f)
  })
}

pub fn field_type_name<'ccx>(field: Field<'ccx>) -> impl fmt::Display + 'ccx {
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
      _ => unreachable!(),
    };

    write!(f, "{name}")
  })
}

pub fn storage_for<'ccx>(field: Field<'ccx>) -> impl fmt::Display + 'ccx {
  emit::display(move |f| {
    let name = field_type_name(field);
    if field.is_repeated() {
      write!(f, "Vec<{name}>")
    } else {
      write!(f, "{name}")
    }
  })
}

pub fn default_for<'ccx>(field: Field<'ccx>) -> impl fmt::Display + 'ccx {
  emit::display(move |f| {
    if field.is_repeated() {
      return write!(f, "Vec::new()");
    }

    match field.ty() {
      (TypeEnum::I32 | TypeEnum::U32 | TypeEnum::I64 | TypeEnum::U64, _) => {
        write!(f, "0")
      }
      (TypeEnum::F32 | TypeEnum::F64, _) => write!(f, "0.0"),
      (TypeEnum::Bool, _) => write!(f, "false"),
      (TypeEnum::String, _) => write!(f, "Vec::new()"),
      (TypeEnum::Type, Some(ty)) => write!(f, "{}::new()", type_name(ty)),
      _ => unreachable!(),
    }
  })
}

pub fn deprecated<'a>(reason: Option<&'a str>) -> impl fmt::Display + 'a {
  emit::display(move |f| match reason {
    Some(value) => write!(f, "#[deprecated = {value:?}]"),
    _ => Ok(()),
  })
}
