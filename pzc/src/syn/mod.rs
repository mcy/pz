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

const PUNCTUATION: &[&str] = &[";", ".", "=", "{", "}", ":", "/", ",", "@"];

const HARD_KEYWORDS: &[&str] = &["message", "enum", "struct", "choice"];

const KEYWORDS: &[&str] = &[
  "edition", "package", "message", "enum", "struct", "choice", "i32", "u32",
  "f32", "i64", "u64", "f64", "str", "bool", "where", "as",
];

/// A single `.pz` file.
#[derive(Debug)]
pub struct PzFile {
  pub attrs: Vec<Attr>,
  pub package: Package,
  pub imports: Vec<Import>,
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

/// A `package = foo.bar.baz` declaration.
///
/// This is the first thing in the file.
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

/// An `import foo.bar { Type.Sub, Enum as Rename }` declaration.
///
/// This is the first thing in the file.
#[derive(Debug)]
pub struct Import {
  pub span: Span,
  pub attrs: Vec<Attr>,
  pub package: Path,

  /// Pairs of imported paths, and an optional rename.
  pub symbols: Vec<(Path, Option<Ident>)>,
}

impl Spanned for Import {
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

  pub fn is_exactly(&self, scx: &SourceCtx, path: &[&str]) -> bool {
    if self.components.len() != path.len() {
      return false;
    }

    self
      .components
      .iter()
      .zip(path)
      .map(|(a, b)| &a.name(scx) == b)
      .all(|x| x)
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

/// An attribute on something, e.g. `@deprecated`.
#[derive(Debug)]
pub struct Attr {
  pub span: Span,
  pub kind: AttrKind,
  pub value: AttrValue,
}

impl Spanned for Attr {
  fn span(&self) -> Span {
    self.span
  }
}

/// A kind of attribute.
#[derive(Debug)]
pub enum AttrKind {
  At(Path),
  Doc,
}
/// An attribute value.
#[derive(Debug)]
pub enum AttrValue {
  None,
  Ident(Ident),
  Int(IntLit),
  Str(StrLit),
}

impl AttrValue {
  pub fn is_none(&self) -> bool {
    matches!(self, Self::None)
  }
}

/// A declaration. This is anything of the form `keyword Name { items }`.
#[derive(Debug)]
pub struct Decl {
  pub span: Span,
  pub kind: DeclKind,
  pub name: Ident,
  pub items: Vec<Item>,
  pub attrs: Vec<Attr>,
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
  pub attrs: Vec<Attr>,
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
    self.text(scx).trim_start_matches('#')
  }

  /// Returns whether this is a "hard" keyword, which is not allowed in some
  /// contexts for ambiguity reasons.
  pub fn is_hard_keyword(&self, scx: &SourceCtx) -> bool {
    HARD_KEYWORDS.contains(&self.text(scx))
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

impl StrLit {
  pub fn unescape(&self, scx: &SourceCtx) -> Vec<u8> {
    let mut vec = Vec::new();
    let _ = lex::unquote(self.text(scx), &mut vec, |_, _, _| unreachable!());
    vec
  }

  pub fn unescape_utf8(self, scx: &SourceCtx, report: &mut Report) -> String {
    match String::from_utf8(self.unescape(scx)) {
      Ok(s) => s,
      Err(_) => {
        // TODO(mcyoung): attempt to produce a more detailed error?
        report.error("expected Unicode string").at(self);
        String::new()
      }
    }
  }
}

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
