//! Naming helper functions.

use std::fmt;

use crate::emit;
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

pub fn deprecated<'a>(reason: Option<&'a str>) -> impl fmt::Display + 'a {
  emit::display(move |f| match reason {
    Some(value) => write!(f, "#[deprecated = {value:?}]"),
    _ => Ok(()),
  })
}
