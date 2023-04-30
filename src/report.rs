//! Error printing facilities.
//!
//! These functions are used to simplify the display of various SNASM errors to
//! the user. The [`Error`] trait describes how a Rust error type can be
//! converted into a simple diagonstic.

use core::fmt;
use std::io;
use std::mem;
use std::ops::Range;
use std::panic;

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::display_list::FormatOptions;
use annotate_snippets::snippet;

use crate::pz;
use crate::syn;

/// A diagnostic that is being built up.
///
/// See [`Report::diagnose()`].
pub struct Diagnostic {
  message: String,
  snippets: Vec<(syn::Span, String)>,
  kind: pz::diagnostic::Kind,
  reported_at: Option<&'static panic::Location<'static>>,
}

impl Diagnostic {
  /// Adds a new relevant snippet at the given location.
  pub fn at(&mut self, span: impl syn::Spanned) -> &mut Self {
    self.saying(span, "")
  }

  /// Adds a new relevant snippet at the given location, with the given message
  /// attached to it.
  pub fn saying(
    &mut self,
    span: impl syn::Spanned,
    message: impl fmt::Display,
  ) -> &mut Self {
    self.snippets.push((span.span(), message.to_string()));
    self
  }
}

impl From<(u32, u32)> for pz::Span {
  fn from((start, end): (u32, u32)) -> Self {
    pz::Span {
      start: Some(start),
      end: Some(end),
      ..Default::default()
    }
  }
}

/// Options for [`Report::render()`].
#[derive(Default)]
pub struct RenderOptions {
  /// Whether to color the output.
  pub color: bool,
  /// Whether to add a note to each diagnostic showing where it was
  /// reported.
  pub show_report_locations: bool,
}

/// A collection of errors that may built up over the course of an action.
pub struct Report {
  errors: Vec<Diagnostic>,
  errors_since_last_check: usize,
}

impl Report {
  /// Creates a new, empty report.
  pub fn new() -> Self {
    Self {
      errors: Vec::new(),
      errors_since_last_check: 0,
    }
  }

  /// Returns whether there have been any new errors since this function was
  /// last called.
  pub fn has_new_errors(&mut self) -> bool {
    let error_count = self
      .errors
      .iter()
      .filter(|e| matches!(e.kind, pz::diagnostic::Kind::Error))
      .count();
    let old = mem::replace(&mut self.errors_since_last_check, error_count);
    old != self.errors.len()
  }

  /// Adds a new error to this report.
  #[track_caller]
  pub fn error(&mut self, message: impl fmt::Display) -> &mut Diagnostic {
    self.errors.push(Diagnostic {
      message: message.to_string(),
      kind: pz::diagnostic::Kind::Error.into(),
      snippets: Vec::new(),
      reported_at: Some(panic::Location::caller()),
    });

    self.errors.last_mut().unwrap()
  }

  /// Calls `dump_to()` on `stderr`, exiting the process with the given
  /// `exit_code` if any errors are present.
  pub fn dump_and_die(
    &self,
    ctx: &syn::Context,
    opts: &RenderOptions,
    code: i32,
  ) {
    // Writing to stderr is fairly unlikely to fail, so panicking is a fine
    // response here.
    if self.render(ctx, opts, &mut io::stderr()).unwrap() {
      std::process::exit(code)
    }
  }

  /// Dumps this collection of errors as user-displayable text into `sink`.
  ///
  /// Returns `Ok(true)` if any errors were fatal.
  pub fn render(
    &self,
    ctx: &syn::Context,
    opts: &RenderOptions,
    sink: &mut dyn io::Write,
  ) -> io::Result<bool> {
    if self.errors.is_empty() {
      return Ok(false);
    }

    for (i, e) in self.errors.iter().enumerate() {
      let kind = match e.kind {
        pz::diagnostic::Kind::Warning => snippet::AnnotationType::Warning,
        pz::diagnostic::Kind::Error => snippet::AnnotationType::Error,
      };

      let mut snippet = snippet::Snippet {
        title: Some(snippet::Annotation {
          id: None,
          label: Some(&e.message),
          annotation_type: kind,
        }),
        opt: FormatOptions {
          color: opts.color,
          anonymized_line_numbers: false,
          margin: None,
        },
        ..Default::default()
      };

      let mut slice = snippet::Slice {
        source: ctx.text(),
        line_start: 1,
        origin: ctx.file().path.as_deref(),
        annotations: Vec::new(),
        fold: true,
      };

      for (span, text) in &e.snippets {
        let Range { mut start, mut end } = span.range(ctx);
        if start == end && !slice.source.is_empty() {
          // Normalize the range so that it is never just one space long.
          // If this would cause range.1 to go past the end of the input length,
          // we swap them around instead.
          if end as usize == slice.source.len() {
            start = end - 1;
          } else {
            end = start + 1;
          }
        }

        slice.annotations.push(snippet::SourceAnnotation {
          range: (start as usize, end as usize),
          label: text,
          annotation_type: kind,
        });
      }

      // Crop the starts of each slice to only incorporate the annotations.
      let earliest_start = slice
        .annotations
        .iter()
        .map(|a| a.range.0)
        .min()
        .unwrap_or(0);

      let (count, idx) = slice.source[..earliest_start]
        .bytes()
        .enumerate()
        .filter_map(|(i, c)| (c == b'\n').then_some(i + 1))
        .enumerate()
        .map(|(i, j)| (i + 1, j))
        .last()
        .unwrap_or_default();

      slice.line_start = count + 1;
      slice.source = &slice.source[idx..];
      for a in &mut slice.annotations {
        a.range.0 -= idx;
        a.range.1 -= idx;
      }

      snippet.slices.push(slice);

      let footer;
      if opts.show_report_locations {
        if let Some(at) = e.reported_at {
          footer = format!("reported at: {}", at);
          snippet.footer.push(snippet::Annotation {
            id: None,
            label: Some(&footer),
            annotation_type: snippet::AnnotationType::Note,
          });
        }
      }

      if i != 0 {
        writeln!(sink, "")?;
      }
      writeln!(sink, "{}", DisplayList::from(snippet))?;
    }

    writeln!(sink, "")?;
    let message = format!("there were {} errors", self.errors.len());
    let snippet = snippet::Snippet {
      title: Some(snippet::Annotation {
        id: None,
        label: Some(&message),
        annotation_type: snippet::AnnotationType::Error,
      }),
      opt: FormatOptions {
        color: opts.color,
        ..Default::default()
      },
      ..Default::default()
    };
    writeln!(sink, "{}", DisplayList::from(snippet))?;

    Ok(true)
  }
}
