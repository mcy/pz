//!

use std::fmt;

use crate::pz;
use crate::report::Report;

mod parse;

#[derive(Debug)]
pub struct PzFile<'file> {
  pub file: &'file pz::File,
  pub edition: Edition,
  pub package: Package,
}

impl<'file> PzFile<'file> {
  pub fn parse(file: &'file pz::File, report: &mut Report) -> Option<Self> {
    parse::parse(file, report)
  }
}

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

#[derive(Debug)]
pub struct Package {
  pub span: Span,
  pub components: Vec<Ident>,
}

impl Spanned for Package {
  fn span(&self) -> Span {
    self.span
  }
}

#[derive(Copy, Clone)]
pub struct Span {
  start: u32,
  end: u32,
}

pub trait Spanned {
  fn span(&self) -> Span;

  fn text<'file>(&self, file: &'file pz::File) -> &'file str {
    self.span().text(file)
  }
}

impl Spanned for Span {
  fn span(&self) -> Span {
    *self
  }
}

impl Span {
  pub fn text<'file>(&self, file: &'file pz::File) -> &'file str {
    file
      .text
      .as_ref()
      .and_then(|text| text.get(self.start as usize..self.end as usize))
      .unwrap()
  }
}

impl fmt::Debug for Span {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}..{}", self.start, self.end)
  }
}

impl From<Span> for pz::Span {
  fn from(Span { start, end }: Span) -> Self {
    pz::Span {
      start: Some(start),
      end: Some(end),
    }
  }
}

#[derive(Debug)]
pub struct Ident(Span);
impl Ident {
  pub fn name<'file>(&self, file: &'file pz::File) -> &'file str {
    self.text(file).trim_start_matches("#")
  }
}

impl Spanned for Ident {
  fn span(&self) -> Span {
    self.0
  }
}

#[derive(Debug)]
pub struct StrLit(Span);

impl Spanned for StrLit {
  fn span(&self) -> Span {
    self.0
  }
}

pub const KEYWORDS: &[&str] = &["edition", "package"];
