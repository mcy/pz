//! Error printing facilities.
//!
//! These functions are used to simplify the display of various SNASM errors to
//! the user. The [`Error`] trait describes how a Rust error type can be
//! converted into a simple diagonstic.

use core::fmt;
use std::cell::Cell;
use std::io;
use std::ops::Range;
use std::panic;

use annotate_snippets::display_list::DisplayList;
use annotate_snippets::display_list::FormatOptions;
use annotate_snippets::snippet;

use pz::proto::plugin;
use pz::proto::plugin::diagnostic::Kind;

use crate::syn;
use crate::syn::SourceCtx;

/// A diagnostic that is being built up.
///
/// See [`Report::diagnose()`].
pub struct Diagnostic {
  kind: Kind,
  message: String,
  snippets: Vec<Vec<(syn::Span, String, bool)>>,
  notes: Vec<String>,
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
    let snips = match self.snippets.last_mut() {
      Some(s) => s,
      None => {
        self.snippets.push(Vec::new());
        &mut self.snippets[0]
      }
    };
    snips.push((span.span(), message.to_string(), false));
    self
  }

  /// Like `saying`, but the underline is as for a "note" rather than the
  /// overall diagnostic.
  pub fn remark(
    &mut self,
    span: impl syn::Spanned,
    message: impl fmt::Display,
  ) -> &mut Self {
    self.saying(span, message);
    self.snippets.last_mut().unwrap().last_mut().unwrap().2 = true;
    self
  }

  /// Starts a new snippet, even if the next span is in a different file.
  pub fn new_snippet(&mut self) -> &mut Self {
    self.snippets.push(Vec::new());
    self
  }

  /// Appends a note to the bottom of the diagnostic.
  pub fn note(&mut self, message: impl fmt::Display) -> &mut Self {
    self.notes.push(message.to_string());
    self
  }
}

/// Options for [`Report::render()`].
pub struct RenderOptions {
  /// Whether to color the output.
  pub color: bool,
  /// Whether to add a note to each diagnostic showing where it was
  /// reported.
  pub show_report_locations: bool,
}

impl Default for RenderOptions {
  fn default() -> Self {
    Self {
      color: true,
      show_report_locations: std::env::var_os("PZ_DEBUG").is_some(),
    }
  }
}

/// A collection of errors that may built up over the course of an action.
pub struct Report {
  opts: RenderOptions,
  errors: Vec<Diagnostic>,
  rendered: Cell<usize>,
}

impl Report {
  /// Creates a new report.
  pub fn new() -> Self {
    Self::with_options(RenderOptions::default())
  }

  /// Creates a report with the given options for rendering.
  pub fn with_options(opts: RenderOptions) -> Self {
    Self {
      opts,
      errors: Vec::new(),
      rendered: Cell::new(0),
    }
  }

  pub fn has_errors(&self) -> bool {
    self.errors.iter().any(|e| e.kind == Kind::Error)
  }

  /// Adds a new fatal error to this report, and then kills the program with
  /// the given exit code.
  #[track_caller]
  pub fn fatal(
    &mut self,
    scx: &SourceCtx,
    exit_code: i32,
    message: impl fmt::Display,
  ) -> ! {
    self.error(message);
    self.dump_and_die(scx, exit_code);
    unreachable!();
  }

  /// Adds a new error to this report.
  #[track_caller]
  pub fn error(&mut self, message: impl fmt::Display) -> &mut Diagnostic {
    self.errors.push(Diagnostic {
      message: message.to_string(),
      kind: Kind::Error,
      snippets: Vec::new(),
      notes: Vec::new(),
      reported_at: Some(panic::Location::caller()),
    });

    self.errors.last_mut().unwrap()
  }

  /// Adds a new warning to this report.
  #[track_caller]
  pub fn warn(&mut self, message: impl fmt::Display) -> &mut Diagnostic {
    self.errors.push(Diagnostic {
      message: message.to_string(),
      kind: Kind::Warning,
      snippets: Vec::new(),
      notes: Vec::new(),
      reported_at: Some(panic::Location::caller()),
    });

    self.errors.last_mut().unwrap()
  }

  pub fn from_proto(&mut self, diagnostic: &plugin::Diagnostic) {
    self.errors.push(Diagnostic {
      message: diagnostic.message().to_string(),
      kind: diagnostic.kind(),
      snippets: vec![diagnostic
        .snippets
        .iter()
        .map(|s| {
          (
            syn::Span::from_id(s.span()),
            s.message().to_string(),
            s.is_remark(),
          )
        })
        .collect()],
      notes: diagnostic.notes.iter().map(|s| s.to_string()).collect(),
      reported_at: None,
    });
  }

  /// Calls `dump_to()` on `stderr`, exiting the process with the given
  /// `exit_code` if any errors are present.
  pub fn dump_and_die(&self, scx: &syn::SourceCtx, code: i32) {
    // Writing to stderr is fairly unlikely to fail, so panicking is a fine
    // response here.
    if self.render(scx, &mut io::stderr()).unwrap() {
      std::process::exit(code)
    }
  }

  /// Dumps this collection of errors as user-displayable text into `sink`.
  ///
  /// Returns `Ok(true)` if any errors were fatal.
  pub fn render(
    &self,
    scx: &syn::SourceCtx,
    sink: &mut dyn io::Write,
  ) -> io::Result<bool> {
    let mut errors = 0;
    for (i, e) in self.errors[self.rendered.get()..].iter().enumerate() {
      let kind = match e.kind {
        Kind::Warning => snippet::AnnotationType::Warning,
        Kind::Error => {
          errors += 1;
          snippet::AnnotationType::Error
        }
      };

      let mut snippet = snippet::Snippet {
        title: Some(snippet::Annotation {
          id: None,
          label: Some(&e.message),
          annotation_type: kind,
        }),
        opt: FormatOptions {
          color: self.opts.color,
          anonymized_line_numbers: false,
          margin: None,
        },
        ..Default::default()
      };

      for snips in &e.snippets {
        let mut cur_file = None;
        let mut cur_slice = None;
        for (span, text, is_remark) in snips {
          let file = span.file(scx);
          if cur_file != Some(file) {
            cur_file = Some(file);
            if let Some(slice) = cur_slice.take() {
              snippet.slices.push(slice);
            }

            cur_slice = Some(snippet::Slice {
              source: file.text(scx),
              line_start: 1,
              origin: Some(file.path(scx)),
              annotations: Vec::new(),
              fold: true,
            });
          }

          let slice = cur_slice.as_mut().unwrap();
          let Range { mut start, mut end } = span.range(scx);
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
            annotation_type: if *is_remark {
              snippet::AnnotationType::Info
            } else {
              kind
            },
          });
        }

        if let Some(slice) = cur_slice.take() {
          snippet.slices.push(slice);
        }
      }

      // Crop the starts of each slice to only incorporate the annotations.
      for slice in &mut snippet.slices {
        let earliest_start = slice
          .annotations
          .iter()
          .map(|a| a.range.0)
          .min()
          .unwrap_or(0);
        let (count, start_idx) = slice.source[..earliest_start]
          .bytes()
          .enumerate()
          .filter_map(|(i, c)| (c == b'\n').then_some(i + 1))
          .enumerate()
          .map(|(i, j)| (i + 1, j))
          .last()
          .unwrap_or_default();

        slice.line_start = count + 1;
        slice.source = &slice.source[start_idx..];
        for a in &mut slice.annotations {
          a.range.0 -= start_idx;
          a.range.1 -= start_idx;
        }
      }

      for note in &e.notes {
        snippet.footer.push(snippet::Annotation {
          id: None,
          label: Some(note),
          annotation_type: snippet::AnnotationType::Note,
        });
      }

      let footer;
      if self.opts.show_report_locations {
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

    self.rendered.set(self.errors.len());
    if errors == 0 {
      return Ok(false);
    }

    writeln!(sink, "")?;
    let message = match errors {
      1 => "aborted due to previous error".into(),
      n => format!("aborted due to {n} errors"),
    };
    let snippet = snippet::Snippet {
      title: Some(snippet::Annotation {
        id: None,
        label: Some(&message),
        annotation_type: snippet::AnnotationType::Error,
      }),
      opt: FormatOptions {
        color: self.opts.color,
        ..Default::default()
      },
      ..Default::default()
    };
    writeln!(sink, "{}", DisplayList::from(snippet))?;

    Ok(true)
  }
}
