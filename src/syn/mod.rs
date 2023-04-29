//! Syntax trees for pz.

mod lex;
mod parse;

pub use lex::Context;
pub use lex::Span;
pub use lex::Spanned;

const PUNCTUATION: &[&str] = &[";", ".", "=", "{", "}"];

const KEYWORDS: &[&str] = &["edition", "package", "message", "enum"];

/// A single `.pz` file.
#[derive(Debug)]
pub struct PzFile<'ctx, 'file> {
  pub ctx: &'ctx mut Context<'file>,
  pub edition: Edition,
  pub package: Package,
  pub items: Vec<Item>,
}

impl<'ctx, 'file> PzFile<'ctx, 'file> {
  pub fn parse(
    ctx: &'ctx mut Context<'file>,
  ) -> Result<Self, &'ctx mut Context<'file>> {
    parse::parse(ctx)
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

impl Spanned for Path {
  fn span(&self) -> Span {
    self.span
  }
}

/// A top-level declaration (which may be nested within other some declarations).
#[derive(Debug)]
pub enum Item {
  Message(Message),
  Enum(Enum),
}

impl Spanned for Item {
  fn span(&self) -> Span {
    match self {
      Self::Message(x) => x.span,
      Self::Enum(x) => x.span,
    }
  }
}

/// A `message Foo { ... }` declaration.
#[derive(Debug)]
pub struct Message {
  pub span: Span,
  pub name: Ident,
}

impl Spanned for Message {
  fn span(&self) -> Span {
    self.span
  }
}

/// An `enum Foo { ... }` declaration.
#[derive(Debug)]
pub struct Enum {
  pub span: Span,
  pub name: Ident,
}

impl Spanned for Enum {
  fn span(&self) -> Span {
    self.span
  }
}

/// An identifier.
///
/// Keywords may not be used as identifiers directly; instead, they must be
/// prefixed with a `#`, e.g. `#package`.
#[derive(Copy, Clone, Debug)]
pub struct Ident(Span);
impl Ident {
  /// Returns the name of this identifier (i.e., the text with an optional
  /// leading `#` stripped).
  pub fn name<'ctx>(&self, ctx: &'ctx Context) -> &'ctx str {
    self.text(ctx).trim_start_matches("#")
  }
}

impl Spanned for Ident {
  fn span(&self) -> Span {
    self.0
  }
}

/// A quoted string literal.
#[derive(Copy, Clone, Debug)]
pub struct StrLit(Span);

impl Spanned for StrLit {
  fn span(&self) -> Span {
    self.0
  }
}
