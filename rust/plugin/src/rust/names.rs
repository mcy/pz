//! Naming helper functions.

use std::fmt;

use crate::emit;
use crate::Field;
use crate::Type;

const INESCAPABLE_KWS: &[&str] = &["crate", "self", "super", "Self"];

const KWS: &[&str] = &[
  "as", "break", "const", "continue", "crate", "else", "enum", "extern",
  "false", "pub fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
  "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
  "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async",
  "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
  "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];

pub fn ident(name: impl fmt::Display) -> impl fmt::Display {
  emit::display(move |f| {
    let name = name.to_string();
    if INESCAPABLE_KWS.contains(&name.as_str()) {
      write!(f, "{name}_")
    } else if KWS.contains(&name.as_str()) {
      write!(f, "r#{name}")
    } else {
      f.write_str(name.as_str())
    }
  })
}

pub fn type_name(ty: Type) -> impl fmt::Display + '_ {
  emit::display(move |f| {
    f.write_str("__")?;
    for component in ty.package().split(".") {
      f.write_fmt(format_args!("::{}", ident(component)))?;
    }

    f.write_fmt(format_args!("::{}", ident(&ty.name().replace('.', "_"))))
  })
}

pub fn type_ident(ty: Type) -> impl fmt::Display + '_ {
  emit::display(move |f| {
    f.write_fmt(format_args!("{}", ident(&ty.name().replace('.', "_"))))
  })
}

pub fn field_name_type_name(field: Field) -> impl fmt::Display + '_ {
  emit::display(move |f| {
    write!(
      f,
      "__field_{}__{}",
      type_ident(field.parent()),
      field.name()
    )
  })
}

pub fn deprecated(reason: Option<&str>) -> impl fmt::Display + '_ {
  emit::display(move |f| match reason {
    Some(value) => write!(f, "#[deprecated = {value:?}]"),
    _ => Ok(()),
  })
}
