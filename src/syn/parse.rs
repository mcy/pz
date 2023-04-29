//!

use crate::pz;
use crate::report::Report;
use crate::syn;
use crate::syn::Ident;
use crate::syn::Span;
use crate::syn::Spanned;

use super::StrLit;

struct ParseCtx<'file> {
  text: &'file str,
  cursor: Option<u32>,
}

struct Fatal;
type Result<T> = std::result::Result<T, Fatal>;

impl<'file> ParseCtx<'file> {
  fn exhausted(&self) -> bool {
    self.cursor.is_none()
  }

  fn cursor(&self) -> u32 {
    self.cursor.unwrap_or(self.text.len() as u32)
  }

  fn advance(&mut self, value: u32) {
    self.cursor = self.cursor.and_then(|c| {
      let c = c.checked_add(value)?;
      if c as usize >= self.text.len() {
        return None;
      }

      Some(c)
    });
  }

  fn zero_width_span(&self) -> Span {
    self.span(self.cursor())
  }

  fn span(&self, start: u32) -> Span {
    Span {
      start,
      end: self.cursor(),
    }
  }

  fn rest(&self) -> &'file str {
    &self.text[self.cursor() as usize..]
  }

  fn starts_with(&self, pat: &str) -> bool {
    self.rest().starts_with(pat)
  }

  fn take_str(&mut self, pat: &str) -> Option<Span> {
    if !self.starts_with(pat) {
      return None;
    }

    let start = self.cursor();
    self.advance(pat.len() as u32);
    Some(self.span(start))
  }

  fn find(&self, pat: &str) -> Option<Span> {
    self.rest().find(pat).map(|p| {
      let start = p as u32 + self.cursor();
      Span {
        start,
        end: pat.len() as u32 + start,
      }
    })
  }

  fn next_char(&self) -> Option<char> {
    self.rest().chars().next()
  }

  fn skip_whitespace(&mut self, report: &mut Report) -> Result<Option<char>> {
    let mut block_comment_starts = Vec::new();
    while !self.exhausted() {
      if let Some(_) = self.take_str("//") {
        self.cursor = self.find("\n").map(|s| s.end);
      }

      if let Some(start) = self.take_str("/*") {
        block_comment_starts.push(start);
      }

      if let Some(end) = self.take_str("*/") {
        if let None = block_comment_starts.pop() {
          report.error("unexpected closing block comment").at(end);
          return Err(Fatal);
        }
      }

      if let Some(next) = self.next_char() {
        if block_comment_starts.is_empty() && !next.is_whitespace() {
          return Ok(self.next_char());
        }

        self.advance(next.len_utf8() as u32);
      }
    }

    if let Some(unclosed) = block_comment_starts.pop() {
      report.error("unclosed block comment").at(unclosed);
    }

    Ok(self.next_char())
  }

  fn skip_lexeme(&mut self, report: &mut Report) -> Result<Span> {
    let span = match self.next_char() {
      Some('a'..='z' | 'A'..='Z' | '_') => self.parse_ident(report)?.span(),
      Some('"') => self.parse_quoted(report)?.span(),
      Some(any) => {
        let start = self.cursor();
        self.advance(any.len_utf8() as u32);
        self.span(start)
      }
      _ => self.zero_width_span(),
    };

    Ok(span)
  }

  fn parse_exact(&mut self, pat: &str, report: &mut Report) -> Result<Span> {
    self.skip_whitespace(report)?;

    let start = self.cursor();
    if !self.starts_with(pat) {
      let span = self.skip_lexeme(report)?;
      report.error(format_args!("expected `{pat}`")).at(span);
      return Ok(span);
    }

    self.advance(pat.len() as u32);
    Ok(self.span(start))
  }

  fn parse_ident(&mut self, report: &mut Report) -> Result<Ident> {
    self.skip_whitespace(report)?;

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

    if ident_start == self.cursor() {
      if let Some(next) = self.next_char() {
        report
          .error(format_args!("expected identifier, got {}", next))
          .at(self.zero_width_span());
      } else {
        report
          .error("expected identifier, got EOF")
          .at(self.zero_width_span());
      }

      if start == self.cursor() {
        self.skip_lexeme(report)?;
      }
    }

    let span = self.span(start);
    let text = &self.text[span.start as usize..span.end as usize];
    if syn::KEYWORDS.contains(&text) {
      report
        .error("expected identifier, got `{text}`")
        .saying(span, "add a leading `#` to escape this keyword");
    }

    Ok(Ident(self.span(start)))
  }

  fn parse_quoted(&mut self, report: &mut Report) -> Result<StrLit> {
    self.skip_whitespace(report)?;

    let start = self.cursor();
    let open = self.parse_exact("\"", report)?;

    while let Some(next) = self.next_char() {
      if let Some(_) = self.take_str("\\\"") {
        continue;
      }

      self.advance(next.len_utf8() as u32);
      if next == '"' {
        return Ok(StrLit(self.span(start)));
      }
    }

    report
      .error("unclosed string literal")
      .saying(open, "opened here");
    Err(Fatal)
  }
}

pub fn parse<'file>(
  file: &'file pz::File,
  report: &mut Report,
) -> Option<syn::PzFile<'file>> {
  let mut parser = ParseCtx {
    text: file.text.as_deref().unwrap_or(""),
    cursor: Some(0),
  };

  let start = parser.cursor();
  parser.parse_exact("edition", report).ok()?;
  parser.parse_exact("=", report).ok()?;
  let edition = parser.parse_quoted(report).ok()?;
  parser.parse_exact(";", report).ok()?;
  let edition = syn::Edition {
    span: parser.span(start),
    value: edition,
  };

  let start = parser.cursor();
  parser.parse_exact("package", report).ok()?;
  let mut components = Vec::new();
  if parser.skip_whitespace(report).ok()? != Some(';') {
    while !parser.exhausted() {
      components.push(parser.parse_ident(report).ok()?);

      match parser.skip_whitespace(report).ok()? {
        Some(';') => {
          parser.advance(1);
          break;
        }
        Some('.') => {
          parser.advance(1);
          continue;
        }
        Some(next) => {
          report
            .error(format_args!("expected `;` or `.`, got {next}"))
            .at(parser.zero_width_span());
          break;
        }
        None => {
          report
            .error("expected `;` or `.`, got EOF")
            .at(parser.zero_width_span());
          break;
        }
      }
    }
  } else {
    parser.advance(1); // For the `;`.
  }
  let package = syn::Package {
    span: parser.span(start),
    components,
  };

  Some(syn::PzFile {
    file,
    edition,
    package,
  })
}
