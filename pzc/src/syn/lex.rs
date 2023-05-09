//! Lexing and tokens.

use std::fmt;
use std::io;
use std::mem;

use crate::report;
use crate::report::Report;
use crate::syn;
use crate::syn::File;
use crate::syn::Ident;
use crate::syn::IntLit;
use crate::syn::SourceCtx;
use crate::syn::Span;
use crate::syn::Spanned;
use crate::syn::StrLit;

/// A token returned by the lexer.
#[derive(Copy, Clone)]
pub enum Token {
  Ident(Ident),
  Str(StrLit),
  Int(IntLit),
  Punct(Span),
  Doc(Span),
  Unknown(Span),
  Eof(Span),
}

/// A kind of token, for use with [`Token::expect()`].
#[allow(unused)]
pub enum Kind<'a> {
  Exact(&'a str),
  Doc,
  Ident,
  Str,
  Int,
  Eof,
}

impl fmt::Display for Kind<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Kind::Exact(str) => write!(f, "`{str}`"),
      Kind::Doc => write!(f, "doc comment"),
      Kind::Ident => write!(f, "identifier"),
      Kind::Int => write!(f, "integer"),
      Kind::Str => write!(f, "quoted string"),
      Kind::Eof => write!(f, "EOF"),
    }
  }
}

impl Token {
  fn display<'a>(self, scx: &'a SourceCtx) -> impl fmt::Display + 'a {
    struct Display<'a>(&'a SourceCtx, Token);
    impl fmt::Display for Display<'_> {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(scx, tok) = self;
        match tok {
          Token::Punct(x) => write!(f, "`{}`", x.text(scx)),
          Token::Ident(x) if syn::KEYWORDS.contains(&x.text(scx)) => {
            write!(f, "`{}`", x.text(scx))
          }
          Token::Doc(..) => write!(f, "doc comment"),
          Token::Ident(..) => write!(f, "identifier"),
          Token::Int(..) => write!(f, "integer"),
          Token::Str(..) => write!(f, "quoted string"),
          Token::Unknown(x) => {
            write!(f, "unexpected character `{}`", x.text(scx))
          }
          Token::Eof(..) => write!(f, "EOF"),
        }
      }
    }

    Display(scx, self)
  }

  #[track_caller]
  fn expect(
    self,
    scx: &mut SourceCtx,
    report: &mut Report,
    kinds: &[Kind],
  ) -> Token {
    assert!(!kinds.is_empty());

    let ok = 'check: {
      for kind in kinds {
        match (self, kind) {
          (
            Self::Ident(Ident(span)) | Self::Punct(span),
            Kind::Exact(expected),
          ) if span.text(scx) == *expected => break 'check true,
          (Self::Ident(..), Kind::Ident) => break 'check true,
          (Self::Str(..), Kind::Str) => break 'check true,
          (Self::Int(..), Kind::Int) => break 'check true,
          (Self::Doc(..), Kind::Doc) => break 'check true,
          (Self::Eof(..), Kind::Eof) => break 'check true,
          _ => continue,
        }
      }
      false
    };

    if !ok {
      use std::fmt::Write;
      let mut msg = "expected ".to_string();
      for (i, kind) in kinds.iter().enumerate() {
        if i > 0 {
          if kinds.len() > 2 {
            msg.push(',');
          }
          msg.push(' ');
          if i == kinds.len() - 1 {
            msg.push_str("or ");
          }
        }

        let _ = write!(msg, "{}", kind);
      }

      let _ = write!(msg, ", got {}", self.display(scx));
      report.error(msg).at(self);
    }

    self
  }
}

impl Spanned for Token {
  fn span(&self) -> Span {
    match self {
      Self::Punct(x) => x.span(),
      Self::Ident(x) => x.span(),
      Self::Str(x) => x.span(),
      Self::Int(x) => x.span(),
      Self::Doc(x) => x.span(),
      Self::Unknown(x) => x.span(),
      Self::Eof(x) => x.span(),
    }
  }
}

pub struct Lexer<'scx, 'report> {
  scx: &'scx mut SourceCtx,
  report: &'report mut Report,
  file: File,
  cursor: Option<u32>,
  unprocessed_comments: Vec<Span>,

  tokens: Vec<Token>,
  token_cursor: usize,
}

/// A catastrophic error that the parser can't recover from.
pub struct Fatal;
pub type Result<T> = std::result::Result<T, Fatal>;

impl<'scx, 'report> Lexer<'scx, 'report> {
  pub fn new(
    file: File,
    scx: &'scx mut SourceCtx,
    report: &'report mut Report,
  ) -> Self {
    Self {
      scx,
      report,
      file,
      cursor: Some(0),
      unprocessed_comments: Vec::new(),
      tokens: Vec::new(),
      token_cursor: 0,
    }
  }

  pub fn scx(&self) -> &SourceCtx {
    &self.scx
  }

  pub fn scx_mut(&mut self) -> &mut SourceCtx {
    &mut self.scx
  }

  pub fn error(&mut self, msg: impl fmt::Display) -> &mut report::Diagnostic {
    self.report.error(msg)
  }

  pub fn at_eof(&mut self) -> Result<bool> {
    Ok(matches!(self.peek()?, Token::Eof(..)))
  }

  pub fn peek(&mut self) -> Result<Token> {
    let next = self.next();
    self.token_cursor -= 1;
    next
  }

  pub fn take_exact(&mut self, text: &str) -> Result<Option<Token>> {
    let next = self.next()?;
    if next.text(self.scx()) == text {
      return Ok(Some(next));
    }

    self.token_cursor -= 1;
    Ok(None)
  }

  pub fn unlex(&mut self, count: usize) {
    self.token_cursor -= count;
  }

  pub fn next(&mut self) -> Result<Token> {
    if let Some(next) = self.tokens.get(self.token_cursor) {
      self.token_cursor += 1;
      return Ok(*next);
    }

    self.skip_whitespace()?;

    let tok = match self.next_char() {
      Some('#' | '_' | 'a'..='z' | 'A'..='Z') => {
        Token::Ident(self.lex_ident()?)
      }
      Some('-' | '0'..='9') => Token::Int(self.lex_int()?),
      Some('"') => Token::Str(self.lex_quoted()?),
      Some('/') => {
        // Doc comment. Simply take everything until the next newline.
        let start = self.cursor();
        let len = self.rest().find("\n").unwrap_or(self.rest().len());
        self.advance(len as u32);
        Token::Doc(self.span(start))
      }
      Some(c) => {
        let start = self.cursor();
        match syn::PUNCTUATION.iter().find(|p| self.starts_with(p)) {
          Some(punct) => {
            self.advance(punct.len() as u32);
            Token::Punct(self.span(start))
          }
          None => {
            self.advance(c.len_utf8() as u32);
            let span = self.span(start);
            self
              .error(format_args!("unexpected chracter: `{c}`"))
              .at(span);

            Token::Unknown(span)
          }
        }
      }
      _ => Token::Eof(self.zero_width_span()),
    };

    if !self.unprocessed_comments.is_empty() {
      tok.span().info_mut(&mut self.scx).comments =
        mem::take(&mut self.unprocessed_comments);
    }

    self.tokens.push(tok);
    self.token_cursor += 1;

    Ok(tok)
  }

  #[track_caller]
  pub fn expect(&mut self, kinds: &[Kind]) -> Result<Token> {
    Ok(self.next()?.expect(&mut self.scx, self.report, kinds))
  }

  #[track_caller]
  pub fn keyword(&mut self, text: &str) -> Result<Span> {
    let next = self.next()?;
    if next.text(self.scx()) != text {
      let msg = format!("expected `{text}`, got {}", next.display(self.scx()));
      self.error(msg).at(next);
      self.token_cursor -= 1;
      return Ok(self.zero_width_span());
    }

    Ok(next.span())
  }

  fn exhausted(&self) -> bool {
    self.cursor.is_none()
  }

  fn cursor(&self) -> u32 {
    self
      .cursor
      .unwrap_or(self.file.text(&self.scx).len() as u32)
  }

  fn advance(&mut self, value: u32) {
    self.cursor = self.cursor.and_then(|c| {
      let c = c.checked_add(value)?;
      if c as usize >= self.file.text(&self.scx).len() {
        return None;
      }

      Some(c)
    });
  }

  pub fn zero_width_span(&mut self) -> Span {
    self.span(self.cursor())
  }

  fn span(&mut self, start: u32) -> Span {
    self.scx.new_span(self.file, start, self.cursor())
  }

  fn rest(&self) -> &str {
    &self.file.text(&self.scx)[self.cursor() as usize..]
  }

  fn starts_with(&self, pat: &str) -> bool {
    self.rest().starts_with(pat)
  }

  fn take_str(&mut self, pat: &str) -> Option<(u32, u32)> {
    if !self.starts_with(pat) {
      return None;
    }

    let start = self.cursor();
    self.advance(pat.len() as u32);
    Some((start, self.cursor()))
  }

  fn find(&self, pat: &str) -> Option<(u32, u32)> {
    self.rest().find(pat).map(|p| {
      let start = p as u32 + self.cursor();
      (start, pat.len() as u32 + start)
    })
  }

  fn next_char(&self) -> Option<char> {
    self.rest().chars().next()
  }

  fn skip_whitespace(&mut self) -> Result<Option<char>> {
    let mut saw_newline = false;
    let mut block_comment_starts = Vec::new();
    while !self.exhausted() {
      if block_comment_starts.is_empty() && self.starts_with("///") {
        // Doc comments do *not* count as whitespace.
        return Ok(self.next_char());
      }

      if let Some((start, _)) = self.take_str("//") {
        saw_newline = false;
        self.cursor = self.find("\n").map(|(_, end)| end);
        self.unprocessed_comments.push(self.scx.new_span(
          self.file,
          start,
          self.cursor(),
        ));
        continue;
      }

      if let Some(range) = self.take_str("/*") {
        saw_newline = false;
        block_comment_starts.push(range);
        continue;
      }

      if let Some(end) = self.take_str("*/") {
        saw_newline = false;
        if let Some((start, _)) = block_comment_starts.pop() {
          if block_comment_starts.is_empty() {
            self.unprocessed_comments.push(self.scx.new_span(
              self.file,
              start,
              self.cursor(),
            ));
          }
          continue;
        }

        let span = self.scx.new_span(self.file, end.0, end.1);
        self.error("unexpected closing block comment").at(span);
        return Err(Fatal);
      }

      if let Some(next) = self.next_char() {
        if next == '\n' && block_comment_starts.is_empty() {
          if saw_newline {
            let span = self.zero_width_span();
            self.unprocessed_comments.push(span);
          }

          saw_newline = true;
        }

        if block_comment_starts.is_empty() && !next.is_whitespace() {
          return Ok(self.next_char());
        }

        self.advance(next.len_utf8() as u32);
      }
    }

    if let Some(unclosed) = block_comment_starts.pop() {
      let span = self.scx.new_span(self.file, unclosed.0, unclosed.1);
      self.error("unclosed block comment").at(span);
    }

    Ok(self.next_char())
  }

  fn lex_ident(&mut self) -> Result<Ident> {
    let start = self.cursor();
    if self.starts_with("#") {
      self.advance(1);
    }

    let ident_start = self.cursor();
    while let Some(next) = self.next_char() {
      if next != '_'
        && !next.is_ascii_alphanumeric()
        && !(ident_start == self.cursor() && next.is_ascii_digit())
      {
        break;
      }
      self.advance(next.len_utf8() as u32);
    }

    let span = self.span(start);
    if ident_start == self.cursor() {
      // This is a single `#` pretending to be an identifier.
      self.error("`#` must be followed by an identifier").at(span);
    }

    /*let text = &self.text[span.start as usize..span.end as usize];
    if syn::KEYWORDS.contains(&text) {
      report
        .error("expected identifier, got `{text}`")
        .saying(span, "add a leading `#` to escape this keyword");
    }*/

    Ok(Ident(span))
  }

  fn lex_quoted(&mut self) -> Result<StrLit> {
    let start = self.cursor();
    let open = self.take_str("\"").unwrap();

    while let Some(next) = self.next_char() {
      if let Some(_) = self.take_str("\\\"") {
        continue;
      }

      self.advance(next.len_utf8() as u32);
      if next == '"' {
        let lit = StrLit(self.span(start));
        // Work around `new_span` requiring an &mut self.
        let text = unsafe { mem::transmute::<&str, &str>(lit.text(&self.scx)) };
        let _ = unquote(text, &mut io::sink(), |start, end, msg| {
          let span = self.scx.new_span(self.file, start, end);
          self.report.error(msg).at(span);
        });

        return Ok(lit);
      }
    }

    let span = self.scx.new_span(self.file, open.0, open.1);
    self
      .error("unclosed string literal")
      .saying(span, "opened here");
    Err(Fatal)
  }

  fn lex_int(&mut self) -> Result<IntLit> {
    let start = self.cursor();
    let is_negative = self.take_str("-").is_some();
    let base = if self.take_str("0x").is_some() {
      16
    } else if self.take_str("0b").is_some() {
      2
    } else {
      10
    };

    let digits_start = self.cursor();
    while let Some(next @ ('0'..='9' | 'a'..='f' | 'A'..='F' | '_')) =
      self.next_char()
    {
      self.advance(next.len_utf8() as u32);
    }
    let span = self.span(start);
    let digits =
      &self.file.text(&self.scx)[digits_start as usize..self.cursor() as usize];

    // Don't bother checking for overflow; other code can complain this is out
    // of range elsewhere.
    let mut value = i128::from_str_radix(digits, base).ok();
    if is_negative {
      value = value.map(|x| -x);
    }

    Ok(syn::IntLit { span, value })
  }
}

pub fn unquote(
  quoted: &str,
  out: &mut impl io::Write,
  mut error: impl FnMut(u32, u32, &dyn fmt::Display),
) -> io::Result<()> {
  let mut iter = quoted[1..quoted.len() - 1].chars();
  let mut offset = 1;
  while let Some(c) = iter.next() {
    offset += c.len_utf16() as u32;
    if c != '\\' {
      write!(out, "{c}")?;
      continue;
    }

    let Some(c) = iter.next() else {
      error(offset - 1, offset, &"unterminated escape sequence");
      break;
    };
    offset += c.len_utf16() as u32;

    let byte = match c {
      '0' => b'\0',
      'n' => b'\n',
      '\\' => b'\\',
      '"' => b'"',
      'x' => {
        let Some(digits) = quoted.get(offset as usize..offset as usize + 2) else {
          error(offset - 2, offset, &"unterminated escape sequence");
          break;
        };
        offset += 2;

        match u8::from_str_radix(digits, 16) {
          Ok(b) => b,
          Err(e) => {
            error(
              offset - 4,
              offset,
              &format_args!("invalid hex in \\x escape: {e}"),
            );
            break;
          }
        }
      }
      c => {
        error(
          offset - 2,
          offset,
          &format_args!("unknown escape sequence \\{c}"),
        );
        break;
      }
    };

    out.write_all(&[byte])?;
  }

  Ok(())
}
