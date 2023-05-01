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

fn parse_attr_kv(start: Option<Span>, lexer: &mut Lexer) -> Result<syn::Attr> {
  let name = parse_path(lexer)?;
  let mut end = name.span();
  let value = if lexer.peek()?.text(lexer.scx()) == "=" {
    lexer.keyword("=")?;
    let tok = lexer.expect(&[Kind::Ident, Kind::Int, Kind::Str])?;
    end = tok.span();
    match tok {
      Token::Ident(x) => syn::AttrValue::Ident(x),
      Token::Int(x) => syn::AttrValue::Int(x),
      Token::Str(x) => syn::AttrValue::Str(x),
      _ => unreachable!(),
    }
  } else {
    syn::AttrValue::None
  };

  let span = start
    .unwrap_or(name.span())
    .span()
    .with_end(end, lexer.scx_mut());
  Ok(syn::Attr {
    span,
    kind: syn::AttrKind::At(name),
    value,
  })
}

fn parse_wheres(lexer: &mut Lexer, attrs: &mut Vec<syn::Attr>) -> Result<()> {
  while lexer.take_exact("where")?.is_some() {
    if lexer.take_exact("{")?.is_some() {
      while lexer.take_exact("}")?.is_none() {}
      lexer.keyword("}")?;
    } else {
      attrs.push(parse_attr_kv(None, lexer)?);
    }
  }
  Ok(())
}

fn parse_item(
  lexer: &mut Lexer,
  inside: Container,
  outer_name: Span,
  outer_attrs: &mut Vec<syn::Attr>,
) -> Result<syn::Item> {
  let mut attrs = Vec::new();
  loop {
    let kw = lexer.expect(&[
      Kind::Ident,
      Kind::Int,
      Kind::Doc,
      Kind::Exact("@"),
      Kind::Exact("message"),
      Kind::Exact("enum"),
      Kind::Exact("struct"),
      Kind::Exact("choice"),
    ])?;

    break match kw.text(lexer.scx()) {
      "@" => {
        attrs.push(parse_attr_kv(Some(kw.span()), lexer)?);
        continue;
      }
      comment if comment.starts_with("///") => {
        attrs.push(syn::Attr {
          span: kw.span(),
          kind: syn::AttrKind::Doc,
          value: syn::AttrValue::None,
        });
        continue;
      }
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

        parse_wheres(lexer, &mut attrs)?;

        let mut items = Vec::new();
        while !lexer.at_eof()? {
          if lexer.peek()?.text(lexer.scx()) == "}" {
            break;
          }

          items.push(parse_item(lexer, container, span, &mut attrs)?);
        }
        lexer.keyword("}")?;

        Ok(syn::Item::Decl(syn::Decl {
          span,
          kind,
          name,
          items,
          attrs,
        }))
      }

      _ => {
        lexer.unlex(1);
        let mut first = None;
        let mut last;

        let mut number = None;
        if let Token::Int(n) = kw {
          lexer.next()?;
          if inside == Container::Struct {
            lexer
              .error(format_args!("{inside} fields cannot have numbers"))
              .saying(&n, "remove this number")
              .remark(
                outer_name,
                format_args!("declared within this {inside}"),
              );
          }

          first = Some(n.span());
          number = Some(n);
          lexer.keyword(".")?;
        }

        let name = parse_ident(lexer)?;
        first.get_or_insert(name.span());
        last = Some(name.span());

        // This needs to be here so we can moor the diagnostic onto the `name`.
        // (This may be a sign I should move where these diagnostics are
        // generated...)
        if number.is_none() && inside != Container::Struct {
          lexer
            .error(format_args!("{inside} fields must have numbers"))
            .saying(name, "expected number")
            .remark(outer_name, format_args!("declared within this {inside}"));
        }

        let mut ty = None;
        if let Some(_) = lexer.take_exact(":")? {
          let typ = parse_type(lexer)?;
          last = Some(typ.span());
          if inside == Container::Enum {
            lexer
              .error(format_args!("{inside} fields cannot have types"))
              .saying(&typ, "remove this type")
              .remark(
                outer_name,
                format_args!("declared within this {inside}"),
              );
          }

          ty = Some(typ);
        } else if inside != Container::Enum {
          lexer
            .error(format_args!("{inside} fields must have types"))
            .saying(name, "expected type")
            .remark(outer_name, format_args!("declared within this {inside}"));
        }

        parse_wheres(lexer, &mut attrs)?;

        let span = first.unwrap().with_end(last.unwrap(), lexer.scx_mut());
        Ok(syn::Item::Field(syn::Field {
          span,
          name,
          number,
          ty,
          attrs,
        }))
      }
    };
  }
}

fn parse_file<'scx, 'file>(
  lexer: &mut Lexer,
) -> Result<(syn::Edition, syn::Package, Vec<syn::Item>)> {
  let edition = parse_edition(lexer)?;
  let package = parse_package(lexer)?;

  let mut items = Vec::new();
  let mut attrs = Vec::new();
  while !lexer.at_eof()? {
    items.push(parse_item(
      lexer,
      Container::File,
      edition.span(),
      &mut attrs,
    )?);
  }

  for attr in attrs {
    lexer
      .error("sorry: attributes not supported at file scope yet")
      .at(attr);
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
