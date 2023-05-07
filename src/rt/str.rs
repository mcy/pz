//! String field support.

use std::alloc::Layout;
use std::fmt;
use std::fmt::Write;
use std::hash;
use std::iter;
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;
use std::slice;
use std::slice::SliceIndex;
use std::str;

use super::arena::RawArena;

/// A string field.
///
/// Strings are "UTF-8 by convention", which means they are byte slices that can
/// usually be interpreted as UTF-8 strings, although they can be any old random
/// bytes, too.
///
/// This type is a wrapper over `[u8]` that makes Unicode-ey operations easier.
#[repr(transparent)]
pub struct Str([u8]);

impl Str {
  pub fn new(bytes: &(impl AsRef<[u8]> + ?Sized)) -> &Self {
    unsafe { mem::transmute::<&[u8], &Self>(bytes.as_ref()) }
  }

  pub fn new_mut(bytes: &mut (impl AsMut<[u8]> + ?Sized)) -> &mut Self {
    unsafe { mem::transmute::<&mut [u8], &mut Self>(bytes.as_mut()) }
  }

  pub unsafe fn from_raw_parts<'a>(ptr: *const u8, len: usize) -> &'a Self {
    Self::new(slice::from_raw_parts(ptr, len))
  }

  pub unsafe fn from_raw_parts_mut<'a>(
    ptr: *mut u8,
    len: usize,
  ) -> &'a mut Self {
    Self::new_mut(slice::from_raw_parts_mut(ptr, len))
  }

  pub fn as_bytes(&self) -> &[u8] {
    &self.0
  }

  pub fn as_mut_bytes(&mut self) -> &mut [u8] {
    &mut self.0
  }

  pub fn is_empty(&self) -> bool {
    self.as_bytes().is_empty()
  }

  pub fn len(&self) -> usize {
    self.as_bytes().len()
  }

  pub fn get<Idx>(&self, idx: Idx) -> Option<&Self>
  where
    Idx: SliceIndex<[u8], Output = [u8]>,
  {
    self.as_bytes().get(idx).map(|bytes| Str::new(bytes))
  }

  pub fn get_mut<Idx>(&mut self, idx: Idx) -> Option<&mut Self>
  where
    Idx: SliceIndex<[u8], Output = [u8]>,
  {
    self.as_mut_bytes().get_mut(idx).map(Self::new_mut)
  }

  pub fn as_utf8(&self) -> Result<&str, str::Utf8Error> {
    str::from_utf8(self.as_bytes())
  }

  pub fn expect_utf8(&self) -> &str {
    str::from_utf8(self.as_bytes()).expect("expected a UTF-8 string")
  }

  pub fn chunks(&self) -> impl Iterator<Item = Result<&str, &[u8]>> + '_ {
    let mut str = self.as_bytes();
    let mut error = None;
    iter::from_fn(move || {
      if let Some(error) = error.take() {
        return Some(Err(error));
      } else if str.is_empty() {
        return None;
      }

      match str::from_utf8(str) {
        Ok(s) => {
          str = b"";
          Some(Ok(s))
        }
        Err(e) => {
          let (utf8, rest) = str.split_at(e.valid_up_to());
          let utf8 = unsafe { str::from_utf8_unchecked(utf8) };
          error = match e.error_len() {
            Some(len) => {
              let (bad, rest) = rest.split_at(len);
              str = rest;
              Some(bad)
            }
            None => {
              str = b"";
              Some(rest)
            }
          };

          Some(Ok(utf8))
        }
      }
    })
  }

  pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
    self
      .chunks()
      .map(|chunk| chunk.unwrap_or("\u{fffd}").chars())
      .flatten()
  }
}

/// A string field that can be grown.
///
/// This type provides a few additional methods over `Str`, which it derefs to.
pub struct StrBuf<'a> {
  data: &'a mut (*mut u8, usize),
  arena: RawArena,
}

impl<'a> StrBuf<'a> {
  #[doc(hidden)]
  pub fn __wrap(data: &'a mut (*mut u8, usize), arena: RawArena) -> Self {
    Self { data, arena }
  }

  pub fn clear(&mut self) {
    self.truncate(0);
  }

  pub fn truncate(&mut self, len: usize) {
    self.data.1 = self.len().min(len);
  }

  pub fn set(&mut self, data: &(impl AsRef<[u8]> + ?Sized)) {
    let data = data.as_ref();
    if data.len() > self.len() || self.data.0.is_null() {
      self.data.0 = self.arena.alloc(Layout::for_value(data)).as_ptr();
    }

    unsafe {
      self
        .data
        .0
        .copy_from_nonoverlapping(data.as_ptr(), data.len());
    }
    self.data.1 = data.len();
  }
}

impl Deref for StrBuf<'_> {
  type Target = Str;

  fn deref(&self) -> &Str {
    unsafe { Str::from_raw_parts(self.data.0, self.data.1) }
  }
}

impl DerefMut for StrBuf<'_> {
  fn deref_mut(&mut self) -> &mut Str {
    unsafe { Str::from_raw_parts_mut(self.data.0, self.data.1) }
  }
}

impl Default for &Str {
  fn default() -> Self {
    Str::new("")
  }
}

impl AsRef<[u8]> for Str {
  fn as_ref(&self) -> &[u8] {
    self.as_bytes()
  }
}

impl AsMut<[u8]> for Str {
  fn as_mut(&mut self) -> &mut [u8] {
    self.as_mut_bytes()
  }
}

impl<'a> From<&'a [u8]> for &'a Str {
  fn from(value: &'a [u8]) -> Self {
    Str::new(value)
  }
}

impl<'a> From<&'a str> for &'a Str {
  fn from(value: &'a str) -> Self {
    Str::new(value)
  }
}

impl<'a> From<&'a Vec<u8>> for &'a Str {
  fn from(value: &'a Vec<u8>) -> Self {
    Str::new(value)
  }
}

impl<'a> From<&'a String> for &'a Str {
  fn from(value: &'a String) -> Self {
    Str::new(value)
  }
}

impl fmt::Debug for Str {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('"')?;
    for chunk in self.chunks() {
      match chunk {
        Ok(str) => fmt::Display::fmt(&str.escape_debug(), f)?,
        Err(bytes) => {
          for b in bytes {
            write!(f, "\\x{b:02x}")?;
          }
        }
      }
    }
    f.write_char('"')
  }
}

impl fmt::Display for Str {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for chunk in self.chunks() {
      match chunk {
        Ok(str) => fmt::Display::fmt(str, f)?,
        Err(..) => f.write_char(char::REPLACEMENT_CHARACTER)?,
      }
    }
    Ok(())
  }
}

impl Eq for Str {}
impl PartialEq<Str> for Str {
  fn eq(&self, other: &Str) -> bool {
    self.as_bytes() == other.as_bytes()
  }
}

impl PartialEq<str> for Str {
  fn eq(&self, other: &str) -> bool {
    self.as_bytes() == other.as_bytes()
  }
}

impl PartialEq<[u8]> for Str {
  fn eq(&self, other: &[u8]) -> bool {
    self.as_bytes() == other
  }
}

impl<const N: usize> PartialEq<[u8; N]> for Str {
  fn eq(&self, other: &[u8; N]) -> bool {
    self.as_bytes() == other.as_slice()
  }
}

impl PartialEq<String> for Str {
  fn eq(&self, other: &String) -> bool {
    self.as_bytes() == other.as_bytes()
  }
}

impl PartialEq<Vec<u8>> for Str {
  fn eq(&self, other: &Vec<u8>) -> bool {
    self.as_bytes() == other
  }
}

impl hash::Hash for Str {
  fn hash<H: hash::Hasher>(&self, state: &mut H) {
    self.as_bytes().hash(state);
  }
}

impl Ord for Str {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.as_bytes().cmp(other.as_bytes())
  }
}

impl PartialOrd<Str> for Str {
  fn partial_cmp(&self, other: &Str) -> Option<std::cmp::Ordering> {
    self.as_bytes().partial_cmp(other.as_bytes())
  }
}

impl PartialOrd<str> for Str {
  fn partial_cmp(&self, other: &str) -> Option<std::cmp::Ordering> {
    self.as_bytes().partial_cmp(other.as_bytes())
  }
}

impl PartialOrd<[u8]> for Str {
  fn partial_cmp(&self, other: &[u8]) -> Option<std::cmp::Ordering> {
    self.as_bytes().partial_cmp(other)
  }
}

impl PartialOrd<String> for Str {
  fn partial_cmp(&self, other: &String) -> Option<std::cmp::Ordering> {
    self.as_bytes().partial_cmp(other.as_bytes())
  }
}

impl PartialOrd<Vec<u8>> for Str {
  fn partial_cmp(&self, other: &Vec<u8>) -> Option<std::cmp::Ordering> {
    self.as_bytes().partial_cmp(other.as_slice())
  }
}

/// Used by the generated API for setters.
pub trait IntoStrOpt<'a> {
  fn into_str_opt(self) -> Option<&'a Str>;
}

impl<'a, Bytes: AsRef<[u8]> + ?Sized> IntoStrOpt<'a> for &'a Bytes {
  fn into_str_opt(self) -> Option<&'a Str> {
    Some(Str::new(self))
  }
}

impl<'a> IntoStrOpt<'a> for Option<&'a Str> {
  fn into_str_opt(self) -> Option<&'a Str> {
    self
  }
}
