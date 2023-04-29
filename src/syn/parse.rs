//! The parser.

use crate::syn;
use crate::syn::lex::Fatal;
use crate::syn::lex::Kind;
use crate::syn::lex::Lexer;
use crate::syn::lex::Result;
use crate::syn::lex::Token;
use crate::syn::Context;
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
    span: kw.with_end(semi.span(), lexer.ctx_mut()),
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
    .with_end(components.last().unwrap().span(), lexer.ctx_mut());
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
    let span = idents.span().to_pz(lexer.ctx());
    lexer
      .ctx_mut()
      .report()
      .error("expected identifier, got path")
      .saying(span, "only a single identifier is allowed");
  }

  Ok(idents.components[0])
}

fn parse_package(lexer: &mut Lexer) -> Result<syn::Package> {
  let kw = lexer.keyword("package")?;

  let path = match lexer.peek()?.text(lexer.ctx()) {
    ";" => syn::Path {
      span: lexer.zero_width_span(),
      components: Vec::new(),
    },
    _ => parse_path(lexer)?,
  };

  let semi = lexer.keyword(";")?;
  Ok(syn::Package {
    span: kw.with_end(semi.span(), lexer.ctx_mut()),
    path,
  })
}

fn parse_item(lexer: &mut Lexer) -> Result<syn::Item> {
  let kw = lexer.expect(&[Kind::Exact("message"), Kind::Exact("enum")])?;
  match kw.text(lexer.ctx()) {
    "message" => {
      let name = parse_ident(lexer)?;
      lexer.keyword("{")?;
      // TODO
      let end = lexer.keyword("}")?;

      let span = kw.span().with_end(end, lexer.ctx_mut());
      Ok(syn::Item::Message(syn::Message { span, name }))
    }
    "enum" => {
      let name = parse_ident(lexer)?;
      lexer.keyword("{")?;
      // TODO
      let end = lexer.keyword("}")?;

      let span = kw.span().with_end(end, lexer.ctx_mut());
      Ok(syn::Item::Enum(syn::Enum { span, name }))
    }

    // No idea what this is, so we should probably give up.
    _ => Err(Fatal),
  }
}

fn parse_file<'ctx, 'file>(
  lexer: &mut Lexer,
) -> Result<(syn::Edition, syn::Package, Vec<syn::Item>)> {
  let edition = parse_edition(lexer)?;
  let package = parse_package(lexer)?;

  let mut items = Vec::new();
  while !lexer.at_eof()? {
    items.push(parse_item(lexer)?);
  }

  Ok((edition, package, items))
}

pub fn parse<'ctx, 'file>(
  ctx: &'ctx mut Context<'file>,
) -> std::result::Result<syn::PzFile<'ctx, 'file>, &'ctx mut Context<'file>> {
  let mut lexer = Lexer::new(ctx);
  let Ok((edition, package, items)) = parse_file(&mut lexer) else {
    return Err(ctx);
  };

  Ok(syn::PzFile {
    ctx,
    edition,
    package,
    items,
  })
}
