//! The parser.

use std::fmt;

use crate::report::Report;
use crate::syn;
use crate::syn::lex::Kind;
use crate::syn::lex::Lexer;
use crate::syn::lex::Result;
use crate::syn::lex::Token;
use crate::syn::File;
use crate::syn::SourceCtx;
use crate::syn::Span;
use crate::syn::Spanned;

fn parse_edition(lexer: &mut Lexer) -> Result<syn::Edition> {
  let kw = lexer.keyword("edition")?;
  lexer.keyword("=")?;
  let edition = match lexer.expect(&[Kind::Str])? {
    Token::Str(s) => s,
    wrong => syn::StrLit(wrong.span()),
  };
  let semi = lexer.keyword(";")?;
  Ok(syn::Edition {
    span: kw.with_end(semi.span(), lexer.scx_mut()),
    value: edition,
  })
}

fn parse_path(lexer: &mut Lexer) -> Result<syn::Path> {
  let mut components = Vec::new();
  match lexer.expect(&[Kind::Ident])? {
    Token::Ident(id) => components.push(id),
    _ => {
      return Ok(syn::Path {
        span: lexer.zero_width_span(),
        components,
      })
    }
  }

  while let Some(_) = lexer.take_exact(".")? {
    match lexer.expect(&[Kind::Ident])? {
      Token::Ident(id) => components.push(id),
      _ => break,
    }
  }

  let span = components[0]
    .span()
    .with_end(components.last().unwrap().span(), lexer.scx_mut());
  Ok(syn::Path { span, components })
}

fn parse_ident(lexer: &mut Lexer) -> Result<syn::Ident> {
  // We go through parse_path to catch a path where we wanted a single
  // identifier.
  let idents = parse_path(lexer)?;
  if idents.components.len() == 0 {
    return Ok(syn::Ident(lexer.zero_width_span()));
  }

  if idents.components.len() > 1 {
    lexer.error("expected identifier, got path").at(&idents);
  }

  Ok(idents.components[0])
}

fn parse_package(lexer: &mut Lexer) -> Result<syn::Package> {
  let kw = lexer.keyword("package")?;

  let path = match lexer.peek()?.text(lexer.scx()) {
    ";" => syn::Path {
      span: lexer.zero_width_span(),
      components: Vec::new(),
    },
    _ => parse_path(lexer)?,
  };

  let semi = lexer.keyword(";")?;
  Ok(syn::Package {
    span: kw.with_end(semi.span(), lexer.scx_mut()),
    path,
  })
}

fn parse_type(lexer: &mut Lexer) -> Result<syn::Type> {
  let repeated = lexer.take_exact("repeated")?.map(|s| s.span());
  let name = lexer.expect(&[
    Kind::Exact("i32"),
    Kind::Exact("u32"),
    Kind::Exact("f32"),
    Kind::Exact("i64"),
    Kind::Exact("u64"),
    Kind::Exact("f64"),
    Kind::Exact("bool"),
    Kind::Exact("str"),
    Kind::Ident,
  ])?;

  let kind = match name.text(lexer.scx()) {
    "i32" => syn::TypeKind::I32,
    "u32" => syn::TypeKind::U32,
    "f32" => syn::TypeKind::F32,
    "i64" => syn::TypeKind::I64,
    "u64" => syn::TypeKind::U64,
    "f64" => syn::TypeKind::F64,
    "bool" => syn::TypeKind::Bool,
    "str" => syn::TypeKind::String,
    _ => {
      // Unlex the identifier token so that the path function finds it.
      lexer.unlex(1);
      syn::TypeKind::Path(parse_path(lexer)?)
    }
  };

  let span = repeated
    .unwrap_or(name.span())
    .with_end(name.span(), lexer.scx_mut());
  Ok(syn::Type {
    span,
    repeated,
    kind,
  })
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Container {
  File,
  Message,
  Enum,
  Struct,
  Choice,
}

impl fmt::Display for Container {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::File => write!(f, "file"),
      Self::Message => write!(f, "message"),
      Self::Enum => write!(f, "enum"),
      Self::Struct => write!(f, "struct"),
      Self::Choice => write!(f, "choice"),
    }
  }
}

fn parse_item(
  lexer: &mut Lexer,
  inside: Container,
  outer_name: Span,
) -> Result<syn::Item> {
  let kw = lexer.expect(&[
    Kind::Ident,
    Kind::Exact("message"),
    Kind::Exact("enum"),
    Kind::Exact("struct"),
    Kind::Exact("choice"),
  ])?;

  match kw.text(lexer.scx()) {
    kind @ ("message" | "enum" | "struct" | "choice") => {
      let (kind, container) = match kind {
        "message" => (syn::DeclKind::Message, Container::Message),
        "enum" => (syn::DeclKind::Enum, Container::Enum),
        "struct" => (syn::DeclKind::Struct, Container::Struct),
        "choice" => (syn::DeclKind::Choice, Container::Choice),
        _ => unreachable!(),
      };

      let name = parse_ident(lexer)?;
      let span = kw.span().with_end(name.span(), lexer.scx_mut());

      if inside == Container::Enum {
        lexer
          .error("declarations not permitted inside an `enum`")
          .at(span)
          .remark(outer_name, "declared within this  enum");
      }

      lexer.keyword("{")?;
      let mut items = Vec::new();
      while !lexer.at_eof()? {
        if lexer.peek()?.text(lexer.scx()) == "}" {
          break;
        }

        if let Some(syn::Item::Field(..)) = items.last() {
          lexer.keyword(",")?;
          if lexer.peek()?.text(lexer.scx()) == "}" {
            break;
          }
        }

        items.push(parse_item(lexer, container, span)?);
      }
      lexer.keyword("}")?;

      Ok(syn::Item::Decl(syn::Decl {
        span,
        kind,
        name,
        items,
      }))
    }

    _ => {
      lexer.unlex(1);
      let name = parse_ident(lexer)?;

      let mut ty = None;
      if let Some(_) = lexer.take_exact(":")? {
        let typ = parse_type(lexer)?;
        if inside == Container::Enum {
          lexer
            .error(format_args!("{inside} fields cannot have types"))
            .saying(&typ, "remove this type")
            .remark(outer_name, format_args!("declared within this {inside}"));
        }

        ty = Some(typ);
      } else if inside != Container::Enum {
        lexer
          .error(format_args!("{inside} fields must have types"))
          .saying(name, "expected type")
          .remark(outer_name, format_args!("declared within this {inside}"));
      }

      let mut number = None;
      if let Some(_) = lexer.take_exact("=")? {
        if let Token::Int(n) = lexer.expect(&[Kind::Int])? {
          if inside == Container::Struct {
            lexer
              .error(format_args!("{inside} fields cannot have numbers"))
              .saying(&n, "remove this number")
              .remark(
                outer_name,
                format_args!("declared within this {inside}"),
              );
          }

          number = Some(n)
        }
      } else if inside != Container::Struct {
        lexer
          .error(format_args!("{inside} fields must have numbers"))
          .saying(name, "expected number")
          .remark(outer_name, format_args!("declared within this {inside}"));
      }

      let last = [
        number.as_ref().map(|x| x.span()),
        ty.as_ref().map(|x| x.span()),
        Some(name.span()),
      ]
      .iter()
      .find(|x| x.is_some())
      .unwrap()
      .unwrap();

      let span = name.span().with_end(last, lexer.scx_mut());
      Ok(syn::Item::Field(syn::Field {
        span,
        name,
        number,
        ty,
      }))
    }
  }
}

fn parse_file<'scx, 'file>(
  lexer: &mut Lexer,
) -> Result<(syn::Edition, syn::Package, Vec<syn::Item>)> {
  let edition = parse_edition(lexer)?;
  let package = parse_package(lexer)?;

  let mut items = Vec::new();
  while !lexer.at_eof()? {
    items.push(parse_item(lexer, Container::File, edition.span())?);
  }

  Ok((edition, package, items))
}

pub fn parse(
  file: File,
  scx: &mut SourceCtx,
  report: &mut Report,
) -> Option<syn::PzFile> {
  let mut lexer = Lexer::new(file, scx, report);
  let (edition, package, items) = parse_file(&mut lexer).ok()?;

  Some(syn::PzFile {
    edition,
    package,
    items,
  })
}
