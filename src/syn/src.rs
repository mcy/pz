//! Source file management.

use std::borrow::Cow;
use std::cell::Cell;
use std::fmt;
use std::fs;
use std::ops::Range;
use std::path::Path;
use std::ptr;

use crate::report::Report;

/// A source code file.
///
/// This does not contain a pointer to the original file, which is instead
/// stored in a [`SourceCtx`].
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct File(u32);

impl File {
  /// Returns the path of this file.
  pub fn path<'scx>(self, scx: &'scx SourceCtx) -> &'scx str {
    &scx.files[self.0 as usize].0
  }

  /// Returns the fill text of this file.
  pub fn text<'scx>(self, scx: &'scx SourceCtx) -> &'scx str {
    &scx.files[self.0 as usize].1
  }
}

/// A source code span.
///
/// This does not contain a pointer to the original file, and as such does not
/// contain any text; it must be paired with a [`SourceCtx`] to get the text
/// it referrs to.
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
  fn text<'scx>(&self, scx: &'scx SourceCtx) -> &'scx str {
    self.span().text(scx)
  }
}

impl Spanned for Span {
  fn span(&self) -> Span {
    *self
  }
}

impl<S: Spanned> Spanned for &S {
  fn span(&self) -> Span {
    S::span(self)
  }
}

impl Span {
  /// Returns this span's opaque ID, for encoding into codegen protos.
  pub fn id(self) -> u32 {
    self.0
  }

  /// Returns the byte range for this span.
  pub fn range(self, scx: &SourceCtx) -> Range<usize> {
    let SpanOffsets { start, end, .. } = scx.spans[self.0 as usize];
    start as usize..end as usize
  }

  /// Returns the file this span points into.
  pub fn file<'scx>(self, scx: &'scx SourceCtx) -> File {
    scx.spans[self.0 as usize].file
  }

  /// Returns the text this span was parsed from.
  pub fn text<'scx>(self, scx: &'scx SourceCtx) -> &'scx str {
    &self.file(scx).text(scx)[self.range(scx)]
  }

  /// Returns ancillary information attached to this span.
  pub fn info<'scx>(self, scx: &'scx SourceCtx) -> &'scx SpanInfo {
    let SpanOffsets { info, .. } = scx.spans[self.0 as usize];
    if info == !0 {
      const EMPTY: &SpanInfo = &SpanInfo {
        comments: Vec::new(),
      };
      return EMPTY;
    }

    &scx.span_info[info as usize]
  }

  /// Returns a mutable reference to ancillary information attached to this
  /// span.
  pub fn info_mut<'scx>(self, scx: &'scx mut SourceCtx) -> &'scx mut SpanInfo {
    let SpanOffsets { info, .. } = &mut scx.spans[self.0 as usize];
    if *info == !0 {
      scx.span_info.push(Default::default());
      *info = scx.span_info.len() as u32 - 1;
    }

    &mut scx.span_info[*info as usize]
  }

  /// Makes a new span that aliases `self` but with a longer end.
  ///
  /// This is useful for creating an "overall span" for a node.
  pub fn with_end(self, end: Span, scx: &mut SourceCtx) -> Span {
    let start = self.range(scx).start as u32;
    let end = end.range(scx).end as u32;
    let span = scx.new_span(self.file(scx), start, end);
    scx.spans[span.0 as usize].info = scx.spans[self.0 as usize].info;
    span
  }
}

#[cfg(debug_assertions)]
thread_local! {
  static CONTEXT: Cell<*const SourceCtx> = Cell::new(ptr::null());
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
pub struct SourceCtx {
  files: Vec<(String, String)>,
  spans: Vec<SpanOffsets>,
  span_info: Vec<SpanInfo>,
}

impl fmt::Debug for SourceCtx {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "SourceCtx {{ .. }}")
  }
}

#[derive(Copy, Clone)]
struct SpanOffsets {
  file: File,
  start: u32,
  end: u32,
  // Set to !0 if no associated info.
  info: u32,
}

impl SourceCtx {
  /// Creates a new context.
  pub fn new() -> Self {
    Self {
      files: Vec::new(),
      spans: Vec::new(),
      span_info: Vec::new(),
    }
  }

  /// Adds a file to this context.
  pub fn add_file(&mut self, path: String, text: String) -> File {
    let file = File(self.files.len() as u32);
    self.files.push((path, text));
    file
  }

  /// Tries to read a file, logging an error if that doesn't work.
  pub fn open_file(
    &mut self,
    path: &Path,
    report: &mut Report,
  ) -> Option<File> {
    let file = File(self.files.len() as u32);

    let utf8_path = path.to_string_lossy();
    if let Cow::Owned(..) = &utf8_path {
      report.error(format_args!("non-Unicode path: {}", path.display()));
      return None;
    }

    let text = match fs::read(path).map(String::from_utf8) {
      Ok(Ok(text)) => text,
      Ok(Err(_not_utf8)) => {
        report.error(format_args!(
          "non-UTF-8 file contents in {}",
          path.display()
        ));
        return None;
      }
      Err(e) => {
        report.error(format_args!("couldn't read {}: {e}", path.display()));
        return None;
      }
    };

    self.files.push((utf8_path.into_owned(), text));
    Some(file)
  }

  #[doc(hidden)]
  #[cfg(debug_assertions)]
  pub fn enable_printing(&self) -> impl Drop + '_ {
    struct Swapper<'a>(&'a SourceCtx, *const SourceCtx);
    impl Drop for Swapper<'_> {
      fn drop(&mut self) {
        CONTEXT.with(|x| x.set(self.1));
      }
    }

    Swapper(
      self,
      CONTEXT.with(|x| x.replace((self as *const Self).cast())),
    )
  }

  #[doc(hidden)]
  #[cfg(not(debug_assertions))]
  pub fn enable_printing(&self) {}

  pub(in crate::syn) fn new_span(
    &mut self,
    file: File,
    start: u32,
    end: u32,
  ) -> Span {
    let span = Span(self.spans.len() as u32);
    self.spans.push(SpanOffsets {
      file,
      start,
      end,
      info: !0,
    });
    span
  }
}
