//! Syntax trees for pz.

mod lex;
mod parse;
mod src;

use std::fmt;

pub use src::File;
pub use src::SourceCtx;
pub use src::Span;
pub use src::Spanned;

use crate::report::Report;

const PUNCTUATION: &[&str] = &[";", ".", "=", "{", "}", ":", "/", ","];

const KEYWORDS: &[&str] = &[
  "edition", "package", "message", "enum", "struct", "choice", "i32", "u32",
  "f32", "i64", "u64", "f64", "str", "bool",
];

/// A single `.pz` file.
#[derive(Debug)]
pub struct PzFile {
  pub edition: Edition,
  pub package: Package,
  pub items: Vec<Item>,
}

impl PzFile {
  pub fn parse(
    file: File,
    scx: &mut SourceCtx,
    report: &mut Report,
  ) -> Option<Self> {
    parse::parse(file, scx, report)
  }
}

/// An `edition = "...";` declaration.
///
/// This is the first thing in the file.
#[derive(Debug)]
pub struct Edition {
  pub span: Span,
  pub value: StrLit,
}

impl Spanned for Edition {
  fn span(&self) -> Span {
    self.span
  }
}

/// A `package = foo.bar.baz;` declaration.
///
/// This is the second thing in the file, after the [`Edition`].
#[derive(Debug)]
pub struct Package {
  pub span: Span,
  pub path: Path,
}

impl Spanned for Package {
  fn span(&self) -> Span {
    self.span
  }
}

/// A period-delimited path, e.g. `foo.bar.Msg`.
#[derive(Debug)]
pub struct Path {
  pub span: Span,
  pub components: Vec<Ident>,
}

impl Path {
  pub fn join(&self, scx: &SourceCtx) -> String {
    let mut name = String::new();
    for (i, id) in self.components.iter().enumerate() {
      if i != 0 {
        name.push('.');
      }
      name.push_str(id.name(scx));
    }

    name
  }
}

impl Spanned for Path {
  fn span(&self) -> Span {
    self.span
  }
}

/// Any kind of delcaration.
#[derive(Debug)]
pub enum Item {
  Decl(Decl),
  Field(Field),
}

impl Spanned for Item {
  fn span(&self) -> Span {
    match self {
      Self::Decl(x) => x.span,
      Self::Field(x) => x.span,
    }
  }
}

/// A declaration. This is anything of the form `keyword Name { items }`.
#[derive(Debug)]
pub struct Decl {
  pub span: Span,
  pub kind: DeclKind,
  pub name: Ident,
  pub items: Vec<Item>,
}

impl Spanned for Decl {
  fn span(&self) -> Span {
    self.span
  }
}

/// A kind of [`Decl`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DeclKind {
  Message,
  Struct,
  Choice,
  Enum,
}

/// A `my_field: type = 5,` declaration.
///
/// All declarations use the same field production, except that some might
/// not use a number and some might not use a type.
#[derive(Debug)]
pub struct Field {
  pub span: Span,
  pub name: Ident,
  pub ty: Option<Type>,
  pub number: Option<IntLit>,
}

impl Spanned for Field {
  fn span(&self) -> Span {
    self.span
  }
}

/// A type, such as on a field declaration.
#[derive(Debug)]
pub struct Type {
  pub span: Span,
  pub repeated: Option<Span>,
  pub kind: TypeKind,
}

impl Spanned for Type {
  fn span(&self) -> Span {
    self.span
  }
}

/// A type kind. See [`Type`].
#[derive(Debug)]
pub enum TypeKind {
  Path(Path),
  I32,
  U32,
  F32,
  I64,
  U64,
  F64,
  String,
  Bool,
}

/// An identifier.
///
/// Keywords may not be used as identifiers directly; instead, they must be
/// prefixed with a `#`, e.g. `#package`.
#[derive(Copy, Clone)]
pub struct Ident(Span);
impl Ident {
  /// Returns the name of this identifier (i.e., the text with an optional
  /// leading `#` stripped).
  pub fn name<'scx>(&self, scx: &'scx SourceCtx) -> &'scx str {
    self.text(scx).trim_start_matches("#")
  }
}

impl Spanned for Ident {
  fn span(&self) -> Span {
    self.0
  }
}

impl fmt::Debug for Ident {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.0, f)
  }
}

/// A quoted string literal.
#[derive(Copy, Clone)]
pub struct StrLit(Span);

impl Spanned for StrLit {
  fn span(&self) -> Span {
    self.0
  }
}

impl fmt::Debug for StrLit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.0, f)
  }
}

/// An integer literal.
#[derive(Copy, Clone, Debug)]
pub struct IntLit {
  span: Span,
  // A missing value indicates out-of-range-ness.
  value: Option<i128>,
}

impl IntLit {
  pub fn value(self) -> Option<i128> {
    self.value
  }
}

impl Spanned for IntLit {
  fn span(&self) -> Span {
    self.span
  }
}
