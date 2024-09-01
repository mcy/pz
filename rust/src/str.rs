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
///
/// When printed as `Debug`, invalid bytes are printed as `\xNN`. When printed
/// as `Display`, invalid bytes are printed as replacement characters
/// (`U+FFFD`). See `[Str::chars()]`.
#[repr(transparent)]
pub struct Str([u8]);

impl Str {
  /// Constructs a new `Str` from any byte slice.
  pub fn new(bytes: &(impl AsRef<[u8]> + ?Sized)) -> &Self {
    unsafe { mem::transmute::<&[u8], &Self>(bytes.as_ref()) }
  }

  /// Constructs a new mutable `Str` from any mutable byte slice.
  pub fn new_mut(bytes: &mut (impl AsMut<[u8]> + ?Sized)) -> &mut Self {
    unsafe { mem::transmute::<&mut [u8], &mut Self>(bytes.as_mut()) }
  }

  /// Constructs a new `Str` from a pointer and a length.
  ///
  /// # Safety
  ///
  /// `ptr` must be valid and point to at least `len` bytes.
  pub unsafe fn from_raw_parts<'a>(ptr: *const u8, len: usize) -> &'a Self {
    Self::new(slice::from_raw_parts(ptr, len))
  }

  /// Constructs a new mutable `Str` from a pointer and a length.
  ///
  /// # Safety
  ///
  /// `ptr` must be valid (for mutation) and point to at least `len` bytes.
  pub unsafe fn from_raw_parts_mut<'a>(
    ptr: *mut u8,
    len: usize,
  ) -> &'a mut Self {
    Self::new_mut(slice::from_raw_parts_mut(ptr, len))
  }

  /// Returns the underlying byte slice.
  pub fn as_bytes(&self) -> &[u8] {
    &self.0
  }

  /// Returns the underlying mutable byte slice.
  pub fn as_mut_bytes(&mut self) -> &mut [u8] {
    &mut self.0
  }

  /// Returns whether this `Str` has a length of zero.
  pub fn is_empty(&self) -> bool {
    self.as_bytes().is_empty()
  }

  /// Returns the length of this `Str`.
  pub fn len(&self) -> usize {
    self.as_bytes().len()
  }

  /// Gets a substring given by an range.
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get<Range>(&self, idx: Range) -> Option<&Self>
  where
    Range: SliceIndex<[u8], Output = [u8]>,
  {
    self.as_bytes().get(idx).map(Str::new)
  }

  /// Gets a mutable substring given by an range.
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get_mut<Range>(&mut self, idx: Range) -> Option<&mut Self>
  where
    Range: SliceIndex<[u8], Output = [u8]>,
  {
    self.as_mut_bytes().get_mut(idx).map(Self::new_mut)
  }

  /// Attempts to convert this string to UTF-8.
  ///
  /// Note that this operation is not free, since it must traverse the whole
  /// string.
  pub fn as_utf8(&self) -> Result<&str, str::Utf8Error> {
    str::from_utf8(self.as_bytes())
  }

  /// Converts this string to UTF-8.
  ///
  /// Note that this operation is not free, since it must traverse the whole
  /// string.
  ///
  /// # Panics
  ///
  /// This function will panic if this string does not contain actual UTF-8.
  pub fn expect_utf8(&self) -> &str {
    str::from_utf8(self.as_bytes()).expect("expected a UTF-8 string")
  }

  /// Partitions the string into valid and invalid UTF-8 chunks.
  ///
  /// Each valid chunk is returned as an `Ok(str)` variant of the result; any
  /// bytes that cannot be interpreted as UTF-8 are returned in the `Err([u8])`
  /// result.
  ///
  /// The UTF-8 chunks are guaranteed to be maximal, but how non-UTF-8 chunks
  /// are partitioned is unspecified. In particular, the iterator will never
  /// return consecutive `Ok` values, but may return consecutive `Err` values.
  pub fn utf8_chunks(&self) -> impl Iterator<Item = Result<&str, &[u8]>> + '_ {
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

  /// Returns an iterator over this string's characters (i.e., UTF-8 scalars).
  ///
  /// Invalid byte sequences are replaced with a replacement character
  /// (`U+FFFD`). The number of replacement characters each invalid byte
  /// sequence becomes is unspecified.
  pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
    self
      .utf8_chunks()
      .flat_map(|chunk| chunk.unwrap_or("\u{fffd}").chars())
  }
}

/// A string field that can be grown.
///
/// This type provides a few additional methods over `Str`, which it derefs to.
pub struct StrBuf<'a> {
  data: &'a mut Storage,
  arena: RawArena,
}

impl<'a> StrBuf<'a> {
  #[doc(hidden)]
  pub fn __wrap(data: &'a mut Storage, arena: RawArena) -> Self {
    Self { data, arena }
  }

  pub(crate) fn reborrow(&mut self) -> StrBuf<'_> {
    StrBuf {
      data: self.data,
      arena: self.arena,
    }
  }

  /// Clears the underlying buffer.
  pub fn clear(&mut self) {
    self.truncate(0);
  }

  /// Truncated the underlying buffer.
  ///
  /// If `len > self.len()`, the buffer is unaffected.
  pub fn truncate(&mut self, len: usize) {
    self.data.len = self.len().min(len);
  }

  /// Sets the contents of the underlying buffer to `data`.
  ///
  /// If `data` is longer than the underlying buffer, this may trigger
  /// re-allocation. Note that resizes cannot be amortized, so `StrBuf` does not
  /// provide append or write features.
  pub fn set(&mut self, data: &(impl AsRef<[u8]> + ?Sized)) {
    let data = data.as_ref();
    if data.len() > self.len() || self.data.ptr.is_null() {
      self.data.ptr = self.arena.alloc(Layout::for_value(data)).as_ptr();
    }

    unsafe {
      self
        .data
        .ptr
        .copy_from_nonoverlapping(data.as_ptr(), data.len());
    }
    self.data.len = data.len();
  }
}

impl Deref for StrBuf<'_> {
  type Target = Str;

  fn deref(&self) -> &Str {
    Str::new(unsafe { self.data.as_slice() })
  }
}

impl DerefMut for StrBuf<'_> {
  fn deref_mut(&mut self) -> &mut Str {
    Str::new_mut(unsafe { self.data.as_mut_slice() })
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

impl<'a> From<StrBuf<'a>> for &'a Str {
  fn from(value: StrBuf<'a>) -> Self {
    Str::new(unsafe { value.data.as_slice() })
  }
}

impl fmt::Debug for Str {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char('"')?;
    for chunk in self.utf8_chunks() {
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
    for chunk in self.utf8_chunks() {
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
    Some(self.cmp(other))
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

pub(crate) use private::*;
pub(crate) mod private {
  use super::*;

  /// Storage for a string field in a message.
  ///
  /// This is for all intents and purposes a `*mut [u8]` whose layout we
  /// control.
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct Storage {
    pub ptr: *mut u8,
    pub len: usize,
  }

  impl Storage {
    /// Return s a new empty raw string.
    pub const fn new() -> Storage {
      Storage {
        ptr: 0 as *mut u8,
        len: 0,
      }
    }

    /// Dereferences this storage, producing an unbound reference. If the
    /// pointer part is null, returns an empty slice.
    ///
    /// # Safety
    ///
    /// `self.ptr` must either be null or dereferenceable for `self.len` bytes.
    pub unsafe fn as_slice<'a>(self) -> &'a [u8] {
      if self.ptr.is_null() {
        return &[];
      }
      return slice::from_raw_parts(self.ptr, self.len);
    }
    /// Dereferences this storage, producing an unbound reference. If the
    /// pointer part is null, returns an empty slice.
    ///
    /// # Safety
    ///
    /// `self.ptr` must either be null or dereferenceable for `self.len` bytes.
    pub unsafe fn as_mut_slice<'a>(self) -> &'a mut [u8] {
      if self.ptr.is_null() {
        return &mut [];
      }
      return slice::from_raw_parts_mut(self.ptr, self.len);
    }
  }

  impl Default for Storage {
    fn default() -> Self {
      Self::new()
    }
  }
}
