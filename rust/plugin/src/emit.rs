//! Source code printing utilities.

use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Write;
use std::mem;
use std::panic;

macro_rules! vars_inner {
  (($($args:tt)*) $(,)?) => {
    [$($args),*]
  };
  (($($args:tt)*) $name:tt: |$x:tt| $expr:literal $(, $($rest:tt)*)?) => {
    vars_inner!(
      ($($args)* (vars_inner!(@stringify $name),
        $crate::emit::Sub::Text($expr)))
      $($($rest)*)?
    )
  };
  (($($args:tt)*) $name:tt: |$x:tt| $expr:expr $(, $($rest:tt)*)?) => {
    vars_inner!(
      ($($args)* (vars_inner!(@stringify $name),
        $crate::emit::Sub::Cb(&|$x| $expr)))
      $($($rest)*)?
    )
  };
  (($($args:tt)*) $name:tt: $expr:expr $(, $($rest:tt)*)?) => {
    vars_inner!(
      ($($args)* (vars_inner!(@stringify $name),
        $crate::emit::Sub::Fmt(&$expr as &dyn std::fmt::Display)))
      $($($rest)*)?
    )
  };

  (($($args:tt)*) $name:tt $(, $($rest:tt)*)?) => {
    vars_inner!(
      ($($args)*) $name: $name $(, $($rest)*)?
    )
  };

  (@stringify $name:ident) => {stringify!($name)};
  (@stringify $name:expr) => {$name};
}

macro_rules! vars {
  ($($tt:tt)*) => {vars_inner!(() $($tt)*)}
}

pub fn display(
  body: impl Fn(&mut fmt::Formatter) -> fmt::Result,
) -> impl Display {
  pub struct Display<F>(F);
  impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for Display<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      (self.0)(f)
    }
  }
  Display(body)
}

pub struct SourceWriter {
  lines: Vec<Line>,
  indent: u32,
  opts: Options,
  frames: Vec<HashMap<&'static str, Sub<'static>>>,
}

pub struct Options {
  #[allow(unused)]
  pub comment_start: &'static str,
}

#[allow(unused)]
pub enum Sub<'a> {
  Text(&'a str),
  Fmt(&'a dyn fmt::Display),
  Cb(&'a dyn Fn(&mut SourceWriter)),
}

impl Default for Options {
  fn default() -> Self {
    Self {
      comment_start: "//",
    }
  }
}

struct Line {
  indent: u32,
  data: String,
}

impl SourceWriter {
  pub fn new(opts: Options) -> Self {
    Self {
      lines: Vec::new(),
      indent: 0,
      opts,
      frames: Vec::new(),
    }
  }

  pub fn new_line(&mut self) {
    self.lines.push(Line {
      indent: self.indent,
      data: String::new(),
    })
  }

  fn buf_mut(&mut self) -> &mut String {
    if self.lines.is_empty() {
      self.new_line();
    }
    &mut self.lines.last_mut().unwrap().data
  }

  pub fn with_vars<const N: usize>(
    &mut self,
    vars: [(&str, Sub); N],
    cb: impl FnOnce(&mut Self),
  ) {
    let map: HashMap<&str, Sub> = vars.into();
    unsafe {
      self.frames.push(mem::transmute::<
        HashMap<&str, Sub>,
        HashMap<&'static str, Sub<'static>>,
      >(map));
    }

    // This assert-unwind-safe is actually safe, because all we do
    // while the panic is paused is pop from `self.frames` and then
    // resume the panic.
    let err = panic::catch_unwind(panic::AssertUnwindSafe(|| cb(self)));

    self.frames.pop();
    if let Err(p) = err {
      panic::resume_unwind(p);
    }
  }

  #[track_caller]
  pub fn emit<const N: usize>(&mut self, vars: [(&str, Sub); N], tpl: &str) {
    self.with_vars(vars, |e| {
      e.exec(Template::parse(tpl, &e.opts));
    })
  }

  pub fn write(&mut self, tpl: &str) {
    self.emit(vars! {}, tpl)
  }

  #[track_caller]
  fn exec(&mut self, tpl: Template) {
    let ambient_indent = self.indent;
    let mut was_newline = true;
    for tok in &tpl.tokens {
      match tok {
        Token::Newline { indent } => {
          // Non-consecutive newlines that *appear* to have been printed
          // consecutively indicate that the previous line consisted only of
          // templates that expanded to the empty string, so we can safely
          // delete such a line.
          if !was_newline {
            if let Some(cur) = self.lines.last_mut() {
              if cur.data.is_empty() {
                self.lines.pop();
              }
            }
          }
          self.indent = ambient_indent + indent;
          self.new_line();
          was_newline = true;
          continue;
        }
        Token::Text(data) => {
          was_newline = false;
          self.buf_mut().push_str(data);
        }
        Token::Var(var) => {
          was_newline = false;
          let mut sub = None;
          for frame in self.frames.iter().rev() {
            if let Some(s) = frame.get(var) {
              sub = Some(s);
              break;
            }
          }

          assert!(sub.is_some(), "unknown variable {var}");
          let sub = unsafe { mem::transmute::<&Sub, &Sub>(sub.unwrap()) };
          match sub {
            Sub::Text(text) => self.buf_mut().push_str(text),
            Sub::Fmt(value) => {
              let _ = write!(self.buf_mut(), "{value}");
            }
            Sub::Cb(cb) => cb(self),
          }
        }
      }
    }
    self.indent = ambient_indent;
  }
}

impl fmt::Display for SourceWriter {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut last_was_empty = true;
    for Line { indent, data } in &self.lines {
      if !data.is_empty() {
        for _ in 0..*indent {
          f.write_char(' ')?;
        }
        f.write_str(data)?;
        f.write_char('\n')?;

        last_was_empty = false;
      } else {
        if !last_was_empty {
          f.write_char('\n')?;
        }
        last_was_empty = true;
      }
    }

    Ok(())
  }
}

struct Template<'tpl> {
  tokens: Vec<Token<'tpl>>,
}

enum Token<'tpl> {
  Newline { indent: u32 },
  Text(&'tpl str),
  Var(&'tpl str),
}

impl<'tpl> Template<'tpl> {
  fn parse(tpl: &'tpl str, _opts: &Options) -> Template<'tpl> {
    let (tpl, indent) = match tpl.strip_prefix('\n') {
      Some(tpl) => {
        let indent = tpl.find(|c| c != ' ').unwrap_or(tpl.len());
        (tpl, indent)
      }
      None => {
        assert!(
          !tpl.contains('\n'),
          "non-multiline templates must not contain newlines"
        );
        (tpl, 0)
      }
    };

    let mut tokens = Vec::new();
    for (i, mut line) in tpl.split('\n').enumerate() {
      let indent = line
        .find(|c| c != ' ')
        .unwrap_or(line.len())
        .saturating_sub(indent);
      line = line[indent..].trim();
      if i != 0 {
        tokens.push(Token::Newline {
          indent: indent as u32,
        });
      }

      while !line.is_empty() {
        match line.find('$') {
          Some(dollar) => {
            if line[dollar + 1..].starts_with('$') {
              tokens.push(Token::Text(&line[dollar + 1..]));
              line = &line[dollar + 2..];
              continue;
            }

            let text = &line[..dollar];
            line = &line[dollar + 1..];
            if !text.is_empty() {
              tokens.push(Token::Text(text))
            }

            match line.chars().next() {
              None => panic!("expected character after $"),
              Some('a'..='z' | 'A'..='Z' | '_') => {
                let end = line
                  .find(|c: char| !(c == '_' || c.is_ascii_alphanumeric()))
                  .unwrap_or(line.len());

                tokens.push(Token::Var(&line[..end]));
                line = &line[end..];
              }
              Some('{') => {
                let end = line.find('}').expect("unclosed ${...}");

                let var = &line[1..end - 1];
                assert!(!var.is_empty(), "missing var name in ${{...}}");

                tokens.push(Token::Var(&line[1..end]));
                line = &line[end + 1..];
              }
              Some(c) => panic!("unexpected character after $: {c:?}"),
            }
          }
          None => {
            tokens.push(Token::Text(line));
            line = "";
          }
        };
      }
    }

    Template { tokens }
  }
}
