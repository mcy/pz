//! Lexing and tokens.

use std::cell::Cell;
use std::fmt;
use std::mem;
use std::ops::Range;
use std::ptr;

use crate::pz;
use crate::report::Report;
use crate::syn;
use crate::syn::Ident;

use super::StrLit;

/// A source code span.
///
/// This does not contain a pointer to the original file, and as such does not
/// contain any text; it must be paired with a [`pz::File`] to get the actual
/// text.
#[derive(Copy, Clone)]
pub struct Span(u32);

#[derive(Default)]
pub struct SpanInfo {
  /// All comments leading up to this span.
  ///
  /// Note that a zero-width span represents a "blank line"; real comments will
  /// include their `//` or equivalent characters in their span.
  pub comments: Vec<Span>,
}

/// A thing that contains a span, such as an AST node.
pub trait Spanned {
  /// Returns this thing's overall span.
  fn span(&self) -> Span;

  /// Returns the text this thing was parsed from, using the file that it came
  /// from.
  fn text<'ctx>(&self, ctx: &'ctx Context) -> &'ctx str {
    self.span().text(ctx)
  }
}

impl Spanned for Span {
  fn span(&self) -> Span {
    *self
  }
}

impl Span {
  /// Returns the byte range for this span.
  pub fn range(self, ctx: &Context) -> Range<usize> {
    let SpanOffsets { start, end, .. } = ctx.spans[self.0 as usize];
    start as usize..end as usize
  }

  /// Converts this span into a [`pz::Span`].
  pub fn to_pz(self, ctx: &Context) -> pz::Span {
    let SpanOffsets { start, end, .. } = ctx.spans[self.0 as usize];
    pz::Span {
      start: Some(start),
      end: Some(end),
    }
  }

  /// Returns the text this span was parsed from, using the file that it came
  /// from.
  pub fn text<'ctx>(self, ctx: &'ctx Context) -> &'ctx str {
    ctx.text().get(self.range(ctx)).unwrap()
  }

  /// Returns ancillary information attached to this span.
  pub fn info<'ctx>(self, ctx: &'ctx Context) -> &'ctx SpanInfo {
    let SpanOffsets { info, .. } = ctx.spans[self.0 as usize];
    if info == !0 {
      const EMPTY: &SpanInfo = &SpanInfo {
        comments: Vec::new(),
      };
      return EMPTY;
    }

    &ctx.span_info[info as usize]
  }

  /// Returns a mutable reference to ancillary information attached to this
  /// span.
  pub fn info_mut<'ctx>(self, ctx: &'ctx mut Context) -> &'ctx mut SpanInfo {
    let SpanOffsets { info, .. } = &mut ctx.spans[self.0 as usize];
    if *info == !0 {
      ctx.span_info.push(Default::default());
      *info = ctx.span_info.len() as u32 - 1;
    }

    &mut ctx.span_info[*info as usize]
  }

  /// Makes a new span that aliases `self` but with a longer end.
  ///
  /// This is useful for creating an "overall span" for a node.
  pub fn with_end(self, end: Span, ctx: &mut Context) -> Span {
    let start = self.range(ctx).start as u32;
    let end = end.range(ctx).end as u32;
    let span = ctx.new_span(start, end);
    ctx.spans[span.0 as usize].info = ctx.spans[self.0 as usize].info;
    span
  }
}

#[cfg(debug_assertions)]
thread_local! {
  static CONTEXT: Cell<*const Context<'static>> = Cell::new(ptr::null());
}

impl fmt::Debug for Span {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    #[cfg(debug_assertions)]
    unsafe {
      let ptr = CONTEXT.with(|x| x.get());
      if !ptr.is_null() {
        return write!(f, "{:?}#{}", self.text(&*ptr), self.0);
      }
    }
    write!(f, "#{}", self.0)
  }
}

/// A source code span context.
///
/// The actual data of [`Span`]s is stored in this struct.
pub struct Context<'file> {
  file: &'file pz::File,
  report: Report<'file>,
  spans: Vec<SpanOffsets>,
  span_info: Vec<SpanInfo>,
}

impl fmt::Debug for Context<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.file, f)
  }
}

#[derive(Copy, Clone)]
struct SpanOffsets {
  start: u32,
  end: u32,
  // Set to !0 if no associated info.
  info: u32,
}

impl<'file> Context<'file> {
  /// Creates a new context.
  pub fn new(file: &'file pz::File) -> Self {
    Self {
      file,
      report: Report::new(file),
      spans: Vec::new(),
      span_info: Vec::new(),
    }
  }

  /// Gets this context's file.
  pub fn file(&self) -> &'file pz::File {
    &self.file
  }

  pub fn report(&mut self) -> &mut Report<'file> {
    &mut self.report
  }

  /// Gets the text of this context's file.
  pub fn text(&self) -> &str {
    self.file.text()
  }

  #[doc(hidden)]
  #[cfg(debug_assertions)]
  pub fn enable_printing(&self) -> impl Drop + '_ {
    struct Swapper<'a>(&'a Context<'a>, *const Context<'static>);
    impl Drop for Swapper<'_> {
      fn drop(&mut self) {
        CONTEXT.with(|x| x.set(self.1));
      }
    }
    
    Swapper(self, CONTEXT.with(|x| x.replace((self as *const Self).cast())))
  }

  #[doc(hidden)]
  #[cfg(not(debug_assertions))]
  pub fn enable_printing(&self) {}

  fn new_span(&mut self, start: u32, end: u32) -> Span {
    let span = Span(self.spans.len() as u32);
    self.spans.push(SpanOffsets {
      start,
      end,
      info: !0,
    });
    span
  }
}
/// A token returned by the lexer.
#[derive(Copy, Clone)]
pub enum Token {
  Ident(Ident),
  Str(StrLit),
  Punct(Span),
  Unknown(Span),
  Eof(Span),
}

/// A kind of token, for use with [`Token::expect()`].
pub enum Kind<'a> {
  Exact(&'a str),
  Ident,
  Str,
  Eof,
}

impl fmt::Display for Kind<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Kind::Exact(str) => write!(f, "`{str}`"),
      Kind::Ident => write!(f, "identifier"),
      Kind::Str => write!(f, "quoted string"),
      Kind::Eof => write!(f, "EOF"),
    }
  }
}

impl Token {
  fn display<'a>(self, ctx: &'a Context<'a>) -> impl fmt::Display + 'a {
    struct Display<'a>(&'a Context<'a>, Token);
    impl fmt::Display for Display<'_> {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(ctx, tok) = self;
        match tok {
          Token::Punct(x) => write!(f, "`{}`", x.text(ctx)),
          Token::Ident(x) if syn::KEYWORDS.contains(&x.text(ctx)) => {
            write!(f, "`{}`", x.text(ctx))
          }
          Token::Ident(..) => write!(f, "identifier"),
          Token::Str(..) => write!(f, "quoted string"),
          Token::Unknown(x) => {
            write!(f, "unexpected character `{}`", x.text(ctx))
          }
          Token::Eof(..) => write!(f, "EOF"),
        }
      }
    }

    Display(ctx, self)
  }

  fn expect(self, ctx: &mut Context, kinds: &[Kind]) -> Token {
    assert!(!kinds.is_empty());

    let ok = 'check: {
      for kind in kinds {
        match (self, kind) {
          (
            Self::Ident(Ident(span)) | Self::Punct(span),
            Kind::Exact(expected),
          ) if span.text(ctx) == *expected => break 'check true,
          (Self::Ident(..), Kind::Ident) => break 'check true,
          (Self::Str(..), Kind::Str) => break 'check true,
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

      let _ = write!(msg, ", got {}", self.display(ctx));
      let span = self.span().to_pz(ctx);
      ctx.report().error(msg).at(span);
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
      Self::Unknown(x) => x.span(),
      Self::Eof(x) => x.span(),
    }
  }
}

pub struct Lexer<'ctx, 'file> {
  ctx: &'ctx mut Context<'file>,
  cursor: Option<u32>,
  unprocessed_comments: Vec<Span>,

  tokens: Vec<Token>,
  token_cursor: usize,
}

/// A catastrophic error that the parser can't recover from.
pub struct Fatal;
pub type Result<T> = std::result::Result<T, Fatal>;

impl<'ctx, 'file> Lexer<'ctx, 'file> {
  pub fn new(ctx: &'ctx mut Context<'file>) -> Self {
    Self {
      ctx,
      cursor: Some(0),
      unprocessed_comments: Vec::new(),
      tokens: Vec::new(),
      token_cursor: 0,
    }
  }

  pub fn ctx(&self) -> &Context<'file> {
    &self.ctx
  }

  pub fn ctx_mut(&mut self) -> &mut Context<'file> {
    &mut self.ctx
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
    if next.text(self.ctx()) == text {
      return Ok(Some(next));
    }

    self.token_cursor -= 1;
    Ok(None)
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
      Some('"') => Token::Str(self.lex_quoted()?),
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
            let pz_span = span.to_pz(&self.ctx);
            self
              .ctx
              .report()
              .error(format_args!("unexpected chracter: `{c}`"))
              .at(pz_span);

            Token::Unknown(span)
          }
        }
      }
      _ => Token::Eof(self.zero_width_span()),
    };

    if !self.unprocessed_comments.is_empty() {
      tok.span().info_mut(&mut self.ctx).comments =
        mem::take(&mut self.unprocessed_comments);
    }

    self.tokens.push(tok);
    self.token_cursor += 1;

    Ok(tok)
  }

  pub fn expect(&mut self, kinds: &[Kind]) -> Result<Token> {
    Ok(self.next()?.expect(&mut self.ctx, kinds))
  }

  pub fn keyword(&mut self, text: &str) -> Result<Span> {
    let next = self.next()?;
    if next.text(self.ctx()) != text {
      let msg = format!("expected `{text}`, got {}", next.display(self.ctx()));
      let span = next.span().to_pz(&mut self.ctx);
      self.ctx.report().error(msg).at(span);
      self.token_cursor -= 1;
      return Ok(self.zero_width_span());
    }

    Ok(next.span())
  }

  fn exhausted(&self) -> bool {
    self.cursor.is_none()
  }

  fn cursor(&self) -> u32 {
    self.cursor.unwrap_or(self.ctx.text().len() as u32)
  }

  fn advance(&mut self, value: u32) {
    self.cursor = self.cursor.and_then(|c| {
      let c = c.checked_add(value)?;
      if c as usize >= self.ctx.text().len() {
        return None;
      }

      Some(c)
    });
  }

  pub fn zero_width_span(&mut self) -> Span {
    self.span(self.cursor())
  }

  fn span(&mut self, start: u32) -> Span {
    self.ctx.new_span(start, self.cursor())
  }

  fn rest(&self) -> &str {
    &self.ctx.text()[self.cursor() as usize..]
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
      if let Some((start, _)) = self.take_str("//") {
        saw_newline = false;
        self.cursor = self.find("\n").map(|(_, end)| end);
        self
          .unprocessed_comments
          .push(self.ctx.new_span(start, self.cursor()));
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
            self
              .unprocessed_comments
              .push(self.ctx.new_span(start, self.cursor()));
          }
          continue;
        }
        self
          .ctx
          .report()
          .error("unexpected closing block comment")
          .at(end);
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
      self
        .ctx
        .report()
        .error("unclosed block comment")
        .at(unclosed);
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
      let span = span.to_pz(&self.ctx);
      self
        .ctx
        .report()
        .error("`#` must be followed by an identifier")
        .at(span);
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
        return Ok(StrLit(self.span(start)));
      }
    }

    self
      .ctx
      .report()
      .error("unclosed string literal")
      .saying(open, "opened here");
    Err(Fatal)
  }
}
