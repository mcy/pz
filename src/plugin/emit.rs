//! Source code printing utilities.

use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;
use std::mem;
use std::panic;

pub struct SourceWriter {
  lines: Vec<Line>,
  indent: u32,
  opts: Options,
  frames: Vec<HashMap<&'static str, Sub<'static>>>,
}

pub struct Options {
  pub comment_start: &'static str,
}

pub enum Sub<'a> {
  Text(&'a str),
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

  fn new_line(&mut self) {
    self.lines.push(Line {
      indent: self.indent,
      data: String::new(),
    })
  }

  fn write_raw(&mut self, text: &str) {
    if self.lines.is_empty() {
      self.new_line();
    }
    self.lines.last_mut().unwrap().data.push_str(text);
  }

  pub fn with_vars<const N: usize>(
    &mut self,
    vars: [(&str, Sub); N],
    cb: impl FnOnce(&mut Self) + panic::UnwindSafe,
  ) {
    let map: HashMap<&str, Sub> = vars.into();
    unsafe {
      self.frames.push(mem::transmute(map));
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
  pub fn emit<const N: usize>(&mut self, vars: [(&str, &str); N], tpl: &str) {
    self.emit_with(vars.map(|(k, v)| (k, Sub::Text(v))), tpl)
  }

  #[track_caller]
  pub fn emit_with<const N: usize>(
    &mut self,
    vars: [(&str, Sub); N],
    tpl: &str,
  ) {
    self.with_vars(vars, |e| {
      e.exec(Template::parse(tpl, &e.opts));
    })
  }

  #[track_caller]
  fn exec(&mut self, tpl: Template) {
    let ambient_indent = self.indent;
    for tok in &tpl.lines {
      match tok {
        Token::Newline { indent } => {
          self.indent = ambient_indent + indent;
          self.new_line();
        }
        Token::Text(data) => self.write_raw(data),
        Token::Var(var) => {
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
            Sub::Text(text) => self.write_raw(text),
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
    for Line { indent, data } in &self.lines {
      for _ in 0..*indent {
        f.write_char(' ')?;
      }
      f.write_str(&data)?;
      f.write_char('\n')?;
    }

    Ok(())
  }
}

struct Template<'tpl> {
  lines: Vec<Token<'tpl>>,
}

enum Token<'tpl> {
  Newline { indent: u32 },
  Text(&'tpl str),
  Var(&'tpl str),
}

impl<'tpl> Template<'tpl> {
  fn parse(mut tpl: &'tpl str, opts: &Options) -> Template<'tpl> {
    if !tpl.starts_with("\n") {
      assert!(
        !tpl.contains("\n"),
        "non-multiline templates must not contain newlines"
      );
      return Template {
        lines: vec![Token::Text(tpl)],
      };
    }
    tpl = tpl.trim_matches('\n');

    let indent = tpl.find(|c| c != ' ').unwrap_or(tpl.len());
    let indent_text = &tpl[..indent];

    let mut tokens = Vec::new();
    for (i, mut line) in tpl.split("\n").enumerate() {
      // Delete the first `n` spaces.
      assert!(line.starts_with(indent_text));
      line = &line[indent..];

      let indent = line.find(|c| c != ' ').unwrap_or(line.len());
      line = &line[indent..];
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

                tokens.push(Token::Var(&line[..end - 1]));
                line = &line[end..];
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

    todo!()
  }
}
