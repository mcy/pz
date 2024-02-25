//! Debug printing helpers.

use std::fmt;
use std::fmt::Write;

pub struct Debug<'a, 'b> {
  fmt: &'a mut fmt::Formatter<'b>,
  indent: usize,
  open_brace: bool,
}

impl<'a, 'b> Debug<'a, 'b> {
  pub fn new(fmt: &'a mut fmt::Formatter<'b>) -> Self {
    Self {
      fmt,
      indent: 0,
      open_brace: false,
    }
  }

  fn new_line(&mut self, include_space: bool) -> fmt::Result {
    if !self.fmt.alternate() {
      if include_space {
        return Ok(());
      }
      return self.fmt.write_char(' ');
    }

    self.fmt.write_char('\n')?;
    for _ in 0..self.indent {
      self.fmt.write_char(' ')?;
    }

    Ok(())
  }

  pub fn start_block(&mut self) -> fmt::Result {
    self.indent += 2;
    self.open_brace = true;
    self.fmt.write_char('{')
  }

  pub fn end_block(&mut self) -> fmt::Result {
    self.indent -= 2;
    if !self.open_brace {
      self.new_line(true)?;
    }
    self.open_brace = false;
    self.fmt.write_char('}')
  }

  pub fn field(&mut self, name: &str) -> fmt::Result {
    self.open_brace = false;
    self.new_line(true)?;
    write!(self.fmt, "{name}: ")
  }

  pub fn write_debug(&mut self, val: impl fmt::Debug) -> fmt::Result {
    write!(self.fmt, "{val:?}")
  }

  pub fn comma(&mut self, is_last: bool) -> fmt::Result {
    if !is_last || self.fmt.alternate() {
      self.fmt.write_char(',')?;
    }

    Ok(())
  }

  pub fn iter<T: fmt::Debug>(
    &mut self,
    values: impl IntoIterator<Item = T>,
  ) -> fmt::Result {
    let iter = values.into_iter();
    if iter.size_hint().1.filter(|&n| n <= 8).is_some() {
      self.fmt.write_char('[')?;
      for (i, v) in iter.enumerate() {
        if i != 0 {
          self.fmt.write_str(", ")?;
        }
        v.fmt(self.fmt)?;
      }
      self.fmt.write_char(']')?;
      return Ok(());
    }

    self.fmt.write_char('[')?;
    self.indent += 2;
    for (i, v) in iter.enumerate() {
      if i != 0 {
        self.comma(false)?;
        self.new_line(false)?;
      } else {
        self.new_line(true)?;
      }
      v.fmt(self.fmt)?;
    }
    self.comma(true)?;
    self.indent -= 2;
    self.new_line(false)?;
    self.fmt.write_char(']')?;

    Ok(())
  }
}
