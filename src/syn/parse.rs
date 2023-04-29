//! The parser.

use crate::syn;
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

fn parse_path(lexer: &mut Lexer) -> Result<Vec<syn::Ident>> {
  let mut path = Vec::new();
  match lexer.expect(&[Kind::Ident])? {
    Token::Ident(id) => path.push(id),
    _ => return Ok(path),
  }

  while let Some(_) = lexer.take_exact(".")? {
    match lexer.expect(&[Kind::Ident])? {
      Token::Ident(id) => path.push(id),
      _ => return Ok(path),
    }
  }

  Ok(path)
}

fn parse_package(lexer: &mut Lexer) -> Result<syn::Package> {
  let kw = lexer.keyword("package")?;
  let mut components = Vec::new();

  if lexer.peek()?.text(lexer.ctx()) != ";" {
    components = parse_path(lexer)?;
  }

  let semi = lexer.keyword(";")?;
  Ok(syn::Package {
    span: kw.with_end(semi.span(), lexer.ctx_mut()),
    components,
  })
}

pub fn parse<'ctx, 'file>(
  ctx: &'ctx mut Context<'file>,
) -> std::result::Result<syn::PzFile<'ctx, 'file>, &'ctx mut Context<'file>> {
  let mut lexer = Lexer::new(ctx);
  let Ok(edition) = parse_edition(&mut lexer) else {
    return Err(ctx);
  };
  let Ok(package) = parse_package(&mut lexer) else {
    return Err(ctx);
  };

  Ok(syn::PzFile {
    ctx,
    edition,
    package,
  })
}
