//! Syntax trees for pz.

mod parse;

use ilex::token;
use ilex::Span;
use ilex::Spanned;
use ilex::Token;

pub fn lex<'t>(
  file: ilex::File<'t>,
  report: &ilex::Report,
) -> Result<token::Stream<'t>, ilex::Fatal> {
  parse::lex(file, report)
}

/// A single `.pz` file.
#[derive(Debug)]
pub struct PzFile<'ast> {
  pub attrs: Vec<Attr<'ast>>,
  pub package: Package<'ast>,
  pub imports: Vec<Import<'ast>>,
  pub items: Vec<Item<'ast>>,
}

impl<'ast> PzFile<'ast> {
  pub fn parse(
    stream: &'ast token::Stream<'ast>,
    report: &ilex::Report,
  ) -> Result<Self, ilex::Fatal> {
    parse::parse(stream, report)
  }
}

/// A `package = foo.bar.baz` declaration.
///
/// This is the first thing in the file.
#[derive(Debug)]
pub struct Package<'ast> {
  pub package: token::Keyword<'ast>,
  pub path: Path<'ast>,
}

impl Spanned for Package<'_> {
  fn span(&self, icx: &ilex::Context) -> Span {
    Span::union([self.package.span(icx), self.path.span(icx)])
  }
}

/// An `import foo.bar { Type.Sub, Enum as Rename }` declaration.
///
/// This is the first thing in the file.
#[derive(Debug)]
pub struct Import<'ast> {
  pub import: token::Keyword<'ast>,
  pub attrs: Vec<Attr<'ast>>,
  pub package: Path<'ast>,

  /// Pairs of imported paths, and an optional rename.
  pub braces: token::Bracket<'ast>,
  pub symbols: Vec<ImportedSymbol<'ast>>,
}

#[derive(Debug)]
pub struct ImportedSymbol<'ast> {
  pub symbol: Path<'ast>,
  pub rename: Option<(token::Keyword<'ast>, token::Ident<'ast>)>,
}

impl Spanned for Import<'_> {
  fn span(&self, icx: &ilex::Context) -> Span {
    Span::union([self.import.span(icx), self.braces.span(icx)])
  }
}

/// A period-delimited path, e.g. `foo.bar.Msg`.
#[derive(Debug)]
pub struct Path<'ast> {
  /// List of components and the dots that follow them, if any.
  pub components: Vec<(token::Ident<'ast>, Option<token::Keyword<'ast>>)>,
}

impl<'ast> Path<'ast> {
  pub fn last(&self) -> token::Ident<'ast> {
    self.components.last().unwrap().0
  }

  pub fn join(&self) -> String {
    let mut name = String::new();
    for (id, dot) in &self.components {
      name.push_str(id.name().text(id.context()));
      if dot.is_some() {
        name.push('.');
      }
    }

    name
  }

  pub fn is_exactly<'a>(
    &self,
    path: impl IntoIterator<Item = &'a str>,
  ) -> bool {
    self
      .components
      .iter()
      .zip(path)
      .map(|((a, _), b)| a.name().text(a.context()) == b)
      .all(|x| x)
  }
}

impl Spanned for Path<'_> {
  fn span(&self, icx: &ilex::Context) -> Span {
    Span::union([
      self.components.first().unwrap().0.span(icx),
      self.components.last().unwrap().0.span(icx),
    ])
  }
}

/// Any kind of delcaration.
#[derive(Debug)]
pub enum Item<'ast> {
  Decl(Decl<'ast>),
  Field(Field<'ast>),
}

impl Spanned for Item<'_> {
  fn span(&self, icx: &ilex::Context) -> Span {
    match self {
      Self::Decl(x) => x.span(icx),
      Self::Field(x) => x.span(icx),
    }
  }
}

/// An attribute on something, e.g. `@deprecated`.
#[derive(Debug)]
pub enum Attr<'ast> {
  At {
    at: Option<token::Keyword<'ast>>,
    name: Path<'ast>,
    value: Option<(token::Keyword<'ast>, AttrValue<'ast>)>,
  },
  Doc(token::Quoted<'ast>),
}

impl Spanned for Attr<'_> {
  fn span(&self, icx: &ilex::Context) -> Span {
    match self {
      Attr::At { at, name, value } => {
        let start = at.map(|s| s.span(icx)).unwrap_or_else(|| name.span(icx));
        let end = value
          .as_ref()
          .map(|(_, v)| v.span(icx))
          .unwrap_or_else(|| name.span(icx));
        Span::union([start, end])
      }
      Attr::Doc(doc) => doc.span(icx),
    }
  }
}

/// An attribute value.
#[derive(Debug)]
pub enum AttrValue<'ast> {
  Path(Path<'ast>),
  Int(token::Digital<'ast>),
  Str(token::Quoted<'ast>),
}

impl Spanned for AttrValue<'_> {
  fn span(&self, icx: &ilex::Context) -> Span {
    match self {
      AttrValue::Path(p) => p.span(icx),
      AttrValue::Int(i) => i.span(icx),
      AttrValue::Str(s) => s.span(icx),
    }
  }
}

/// A declaration. This is anything of the form `keyword Name { items }`.
#[derive(Debug)]
pub struct Decl<'ast> {
  pub kw: token::Keyword<'ast>,
  pub braces: token::Bracket<'ast>,
  pub kind: DeclKind,
  pub name: token::Ident<'ast>,
  pub items: Vec<Item<'ast>>,
  pub attrs: Vec<Attr<'ast>>,
}

impl Spanned for Decl<'_> {
  fn span(&self, icx: &ilex::Context) -> Span {
    Span::union([self.kw.span(icx), self.braces.span(icx)])
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
pub struct Field<'ast> {
  pub number: Option<token::Digital<'ast>>,
  pub name: token::Ident<'ast>,
  pub ty: Option<Type<'ast>>,
  pub attrs: Vec<Attr<'ast>>,
}

impl Spanned for Field<'_> {
  fn span(&self, icx: &ilex::Context) -> Span {
    let start = self
      .number
      .map(|n| n.span(icx))
      .unwrap_or_else(|| self.name.span(icx));
    let end = self
      .ty
      .as_ref()
      .map(|n| n.span(icx))
      .unwrap_or_else(|| self.name.span(icx));
    Span::union([start, end])
  }
}

/// A type, such as on a field declaration.
#[derive(Debug)]
pub enum Type<'ast> {
  Repeated {
    repeated: token::Keyword<'ast>,
    element: Box<Type<'ast>>,
  },
  Path(Path<'ast>),
}

impl Spanned for Type<'_> {
  fn span(&self, icx: &ilex::Context) -> Span {
    match self {
      Type::Repeated { repeated, element } => {
        Span::union([repeated.span(icx), element.span(icx)])
      }
      Type::Path(path) => path.span(icx),
    }
  }
}

pub fn unescape(str: token::Quoted) -> String {
  str.to_utf8(|esc, _, out| match esc.text(str.context()) {
    "\\\"" => out.push('"'),
    "\\\\" => out.push('\\'),
    _ => unreachable!(),
  })
}
