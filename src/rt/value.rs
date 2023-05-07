//! Wrappers for field values.

use std::fmt;
use std::marker::PhantomData;
use std::ops;
use std::ptr::NonNull;

use crate::rt::arena::AVec;
use crate::rt::arena::RawArena;
use crate::rt::ptr::Proxied;
use crate::rt::ptr::ScalarMut;
use crate::Mut;
use crate::Str;
use crate::StrBuf;
use crate::View;

pub trait Type: Proxied {
  #[doc(hidden)]
  type __Storage;
  #[doc(hidden)]
  unsafe fn __make_view<'a>(ptr: *mut u8) -> View<'a, Self>;
  #[doc(hidden)]
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: RawArena) -> Mut<'a, Self>;
  #[doc(hidden)]
  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: RawArena);
}

impl<T> Type for T
where
  T: Copy + for<'a> Proxied<View<'a> = T, Mut<'a> = ScalarMut<'a, T>>,
{
  type __Storage = T;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> View<'a, Self> {
    ptr.cast::<T>().read()
  }

  unsafe fn __make_mut<'a>(ptr: *mut u8, _arena: RawArena) -> Mut<'a, Self> {
    ScalarMut::__wrap(&mut *ptr.cast::<T>())
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: RawArena) {
    (&mut *ptr.cast::<AVec<T>>()).resize(new_len, arena)
  }
}

impl Type for Str {
  type __Storage = (*mut u8, usize);

  unsafe fn __make_view<'a>(ptr: *mut u8) -> View<'a, Self> {
    let (mut ptr, len) = ptr.cast::<(*mut u8, usize)>().read();
    if ptr.is_null() {
      ptr = 1 as *mut u8;
    }

    Str::from_raw_parts(ptr, len)
  }

  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: RawArena) -> Mut<'a, Self> {
    let data = &mut *ptr.cast::<(*mut u8, usize)>();
    StrBuf::__wrap(data, arena)
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: RawArena) {
    (&mut *ptr.cast::<AVec<(*mut u8, usize)>>()).resize(new_len, arena)
  }
}

pub struct OptMut<'a, T: Type + ?Sized> {
  ptr: *mut u8,
  arena: RawArena,
  hazzer: unsafe fn(*mut u8, RawArena, Option<bool>) -> bool,
  _ph: PhantomData<&'a mut T>,
}

impl<'a, T: Type + ?Sized> OptMut<'a, T> {
  #[doc(hidden)]
  pub unsafe fn __wrap(
    ptr: *mut u8,
    arena: RawArena,
    hazzer: unsafe fn(*mut u8, RawArena, Option<bool>) -> bool,
  ) -> Self {
    Self {
      ptr,
      arena,
      hazzer,
      _ph: PhantomData,
    }
  }

  /// Returns whether this value is present.
  pub fn has(&self) -> bool {
    unsafe { (self.hazzer)(self.ptr, self.arena, None) }
  }

  /// Initializes the field, if it isn't already.
  pub fn init(&mut self) {
    unsafe {
      (self.hazzer)(self.ptr, self.arena, Some(true));
    }
  }

  /// Clears the field.
  pub fn clear(&mut self) {
    unsafe {
      (self.hazzer)(self.ptr, self.arena, Some(false));
    }
  }

  /// Converts this mutator into a view, returning a view of the default value
  /// if the view it isn't present.
  pub fn as_view(&self) -> View<T> {
    if !self.has() {
      todo!()
    }

    unsafe { T::__make_view(self.ptr) }
  }

  /// Converts this mutator into a view, returning a view of the default value
  /// if the view it isn't present.
  ///
  /// This version consumes the mutator to make the returned view's lifetime as
  /// long as possible.
  pub fn into_view(self) -> View<'a, T> {
    if !self.has() {
      todo!()
    }

    unsafe { T::__make_view(self.ptr) }
  }

  /// Converts this mutator into a mutator of the underlying type, initializing
  /// the field if it isn't already.
  pub fn as_mut(&mut self) -> Mut<T> {
    self.init();
    unsafe { T::__make_mut(self.ptr, self.arena) }
  }

  /// Converts this mutator into a mutator of the underlying type, initializing
  /// the field if it isn't already.
  ///
  /// This version consumes the mutator to make the returned mutator's lifetime
  /// as long as possible.
  pub fn into_mut(mut self) -> Mut<'a, T> {
    self.init();
    unsafe { T::__make_mut(self.ptr, self.arena) }
  }

  /// Converts this mutator into a mutator of the underlying type, or returns
  /// `None` if it isn't present.
  pub fn as_mut_or(&mut self) -> Option<Mut<T>> {
    if !self.has() {
      return None;
    }
    unsafe { Some(T::__make_mut(self.ptr, self.arena)) }
  }

  /// Converts this mutator into a mutator of the underlying type, or returns
  /// `None` if it isn't present.
  ///
  /// This version consumes the mutator to make the returned mutator's lifetime
  /// as long as possible.
  pub fn into_mut_or(self) -> Option<Mut<'a, T>> {
    if !self.has() {
      return None;
    }
    unsafe { Some(T::__make_mut(self.ptr, self.arena)) }
  }
}

/// An immutable slice of a repeated field.
pub struct Slice<'a, T: Type + ?Sized> {
  ptr: *mut T::__Storage,
  len: usize,
  _ph: PhantomData<&'a T>,
}

impl<'a, T: Type + ?Sized> Slice<'a, T> {
  #[doc(hidden)]
  pub unsafe fn __wrap(ptr: *mut T::__Storage, len: usize) -> Self {
    Self {
      ptr,
      len,
      _ph: PhantomData,
    }
  }

  /// Returns whether this `Slice` has a length of zero.
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// Returns the length of this `Slice`.
  pub fn len(&self) -> usize {
    self.len
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get<Range: RepIndex>(self, idx: Range) -> Option<Range::View<'a, T>>
  where
    Range: RepIndex,
  {
    unsafe { idx.__get(self.ptr, self.len) }
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// # Panics
  ///
  /// Panics if the index is out of bounds.
  pub fn at<Range: RepIndex>(self, idx: Range) -> Range::View<'a, T> {
    match self.get(idx.clone()) {
      Some(v) => v,
      None => panic!(
        "slice index out of bounds: got {idx:?} but expected 0..{}",
        self.len()
      ),
    }
  }

  /// Returns an iterator over this slice.
  pub fn iter(&self) -> Iter<'_, T> {
    self.into_iter()
  }
}

impl<T: Type + ?Sized> Clone for Slice<'_, T> {
  fn clone(&self) -> Self {
    *self
  }
}
impl<T: Type + ?Sized> Copy for Slice<'_, T> {}

impl<T: Type + ?Sized> Default for Slice<'_, T> {
  fn default() -> Self {
    Self {
      ptr: NonNull::dangling().as_ptr(),
      len: 0,
      _ph: PhantomData,
    }
  }
}

pub struct Iter<'a, T: Type + ?Sized> {
  start: *mut T::__Storage,
  end: *mut T::__Storage,
  _ph: PhantomData<&'a T>,
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a Slice<'_, T> {
  type Item = View<'a, T>;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (*self).into_iter()
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for Slice<'a, T> {
  type Item = View<'a, T>;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    Iter {
      start: self.ptr,
      end: unsafe { self.ptr.add(self.len) },
      _ph: PhantomData,
    }
  }
}

impl<'a, T: Type + ?Sized> Iterator for Iter<'a, T> {
  type Item = View<'a, T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.start == self.end {
      return None;
    }
    let ptr = self.start;
    unsafe {
      self.start = ptr.add(1);
      Some(T::__make_view(ptr as *mut u8))
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let len = unsafe { self.end.offset_from(self.start) } as usize;
    (len, Some(len))
  }
}

impl<T: Type + ?Sized> fmt::Debug for Slice<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut dbg = f.debug_list();
    for value in self {
      dbg.entry(&value);
    }
    Ok(())
  }
}

impl<T, U> PartialEq<[U]> for Slice<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U]) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == *b)
  }
}

impl<T, U, const N: usize> PartialEq<[U; N]> for Slice<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U; N]) -> bool {
    self.eq(other.as_slice())
  }
}

impl<T, U> PartialEq<Slice<'_, U>> for Slice<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Slice<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<SliceMut<'_, U>> for Slice<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &SliceMut<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<Repeated<'_, U>> for Slice<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Repeated<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T: Type + ?Sized> Eq for Slice<'_, T> where for<'a> View<'a, T>: PartialEq {}

/// An mutable slice of a repeated field.
pub struct SliceMut<'a, T: Type + ?Sized> {
  ptr: *mut T::__Storage,
  len: usize,
  arena: RawArena,
  _ph: PhantomData<&'a T>,
}

impl<'a, T: Type + ?Sized> SliceMut<'a, T> {
  #[doc(hidden)]
  pub unsafe fn __wrap(
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Self {
    Self {
      ptr,
      len,
      arena,
      _ph: PhantomData,
    }
  }

  /// Reborrows this slice as a shared slice.
  pub fn as_view(&self) -> Slice<'_, T> {
    Slice {
      ptr: self.ptr,
      len: self.len,
      _ph: PhantomData,
    }
  }

  /// Converts this slice into a shared slice.
  ///
  /// Note that unlike `as_view()`, this function consumes the slice to give it
  /// the longest possible lifetime.
  pub fn into_view(self) -> Slice<'a, T> {
    Slice {
      ptr: self.ptr,
      len: self.len,
      _ph: PhantomData,
    }
  }

  /// Reborrows this slice as a mutable slice.
  pub fn as_mut(&mut self) -> SliceMut<'_, T> {
    SliceMut {
      ptr: self.ptr,
      len: self.len,
      arena: self.arena,
      _ph: PhantomData,
    }
  }

  /// Returns whether this `SliceMut` has a length of zero.
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// Returns the length of this `SliceMut`.
  pub fn len(&self) -> usize {
    self.len
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get<Range: RepIndex>(self, idx: Range) -> Option<Range::View<'a, T>>
  where
    Range: RepIndex,
  {
    self.into_view().get(idx)
  }

  /// Gets a mutable subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get_mut<Range: RepIndex>(self, idx: Range) -> Option<Range::Mut<'a, T>>
  where
    Range: RepIndex,
  {
    unsafe { idx.__get_mut(self.ptr, self.len, self.arena) }
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// # Panics
  ///
  /// Panics if the index is out of bounds.
  pub fn at<Range: RepIndex>(self, idx: Range) -> Range::View<'a, T> {
    self.into_view().at(idx)
  }

  /// Gets a mutable subslice of this slice (or a single element).
  ///
  /// # Panics
  ///
  /// Panics if the index is out of bounds.
  pub fn at_mut<Range: RepIndex>(self, idx: Range) -> Range::Mut<'a, T> {
    let len = self.len();
    match self.get_mut(idx.clone()) {
      Some(v) => v,
      None => {
        panic!("slice index out of range: got {idx:?} but expected 0..{len}")
      }
    }
  }

  /// Returns an iterator over this slice.
  pub fn iter(&self) -> Iter<'_, T> {
    self.as_view().into_iter()
  }

  /// Returns a mutable iterator over this slice.
  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    self.as_mut().into_iter()
  }
}

pub struct IterMut<'a, T: Type + ?Sized> {
  start: *mut T::__Storage,
  end: *mut T::__Storage,
  arena: RawArena,
  _ph: PhantomData<&'a mut T>,
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a SliceMut<'_, T> {
  type Item = View<'a, T>;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a mut SliceMut<'_, T> {
  type Item = Mut<'a, T>;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for SliceMut<'a, T> {
  type Item = Mut<'a, T>;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    IterMut {
      start: self.ptr,
      end: unsafe { self.ptr.add(self.len) },
      arena: self.arena,
      _ph: PhantomData,
    }
  }
}

impl<'a, T: Type + ?Sized> Iterator for IterMut<'a, T> {
  type Item = Mut<'a, T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.start == self.end {
      return None;
    }
    let ptr = self.start;
    unsafe {
      self.start = ptr.add(1);
      Some(T::__make_mut(ptr as *mut u8, self.arena))
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let len = unsafe { self.end.offset_from(self.start) } as usize;
    (len, Some(len))
  }
}

impl<T: Type + ?Sized> fmt::Debug for SliceMut<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.as_view(), f)
  }
}

impl<T, U> PartialEq<[U]> for SliceMut<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U]) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == *b)
  }
}

impl<T, U, const N: usize> PartialEq<[U; N]> for SliceMut<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U; N]) -> bool {
    self.eq(other.as_slice())
  }
}

impl<T, U> PartialEq<Slice<'_, U>> for SliceMut<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Slice<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<SliceMut<'_, U>> for SliceMut<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &SliceMut<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<Repeated<'_, U>> for SliceMut<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Repeated<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T: Type + ?Sized> Eq for SliceMut<'_, T> where
  for<'a> View<'a, T>: PartialEq
{
}

pub struct Repeated<'a, T: Type + ?Sized> {
  ptr: *mut u8,
  arena: RawArena,
  _ph: PhantomData<&'a mut T>,
}

impl<'a, T: Type + ?Sized> Repeated<'a, T> {
  #[doc(hidden)]
  pub unsafe fn __wrap(ptr: *mut u8, arena: RawArena) -> Self {
    Self {
      ptr,
      arena,
      _ph: PhantomData,
    }
  }

  fn raw(&self) -> &AVec<T::__Storage> {
    unsafe { &*self.ptr.cast::<AVec<T::__Storage>>() }
  }

  fn raw_mut(&mut self) -> &mut AVec<T::__Storage> {
    unsafe { &mut *self.ptr.cast::<AVec<T::__Storage>>() }
  }

  /// Reborrows this repeated field as a shared slice.
  pub fn as_view(&self) -> Slice<'_, T> {
    Slice {
      ptr: self.raw().as_ptr(),
      len: self.raw().len(),
      _ph: PhantomData,
    }
  }

  /// Reborrows this repeated field as a mutable slice.
  pub fn as_mut(&mut self) -> SliceMut<'_, T> {
    SliceMut {
      ptr: self.raw().as_ptr(),
      len: self.raw().len(),
      arena: self.arena,
      _ph: PhantomData,
    }
  }

  /// Converts this repeated field as a shared slice.
  pub fn into_view(self) -> Slice<'a, T> {
    Slice {
      ptr: self.raw().as_ptr(),
      len: self.raw().len(),
      _ph: PhantomData,
    }
  }

  /// Converts this repeated field as a mutable slice.
  pub fn into_mut(self) -> SliceMut<'a, T> {
    SliceMut {
      ptr: self.raw().as_ptr(),
      len: self.raw().len(),
      arena: self.arena,
      _ph: PhantomData,
    }
  }

  /// Returns whether this repeated field has a length of zero.
  pub fn is_empty(&self) -> bool {
    self.as_view().is_empty()
  }

  /// Returns the length of this repeated field.
  pub fn len(&self) -> usize {
    self.as_view().len()
  }

  /// Adds a new, default element to this repeated field, returning a mutator
  /// for it.
  pub fn add(&mut self) -> Mut<'_, T> {
    let new_len = self.len() + 1;
    unsafe {
      T::__resize(self.ptr, new_len, self.arena);
      T::__make_mut(self.raw().as_ptr().add(new_len - 1).cast(), self.arena)
    }
  }

  /// Clears this repeated field.
  pub fn clear(&mut self) {
    self.truncate(0);
  }

  /// Truncates this repeated field.
  ///
  /// If `len > self.len()`, the field is unaffected.
  pub fn truncate(&mut self, len: usize) {
    let new_len = usize::min(self.len(), len);
    unsafe {
      self.raw_mut().set_len(new_len);
    }
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get<Range: RepIndex>(self, idx: Range) -> Option<Range::View<'a, T>>
  where
    Range: RepIndex,
  {
    self.into_view().get(idx)
  }

  /// Gets a mutable subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get_mut<Range: RepIndex>(self, idx: Range) -> Option<Range::Mut<'a, T>>
  where
    Range: RepIndex,
  {
    self.into_mut().get_mut(idx)
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// # Panics
  ///
  /// Panics if the index is out of bounds.
  pub fn at<Range: RepIndex>(self, idx: Range) -> Range::View<'a, T> {
    self.into_view().at(idx)
  }

  /// Gets a mutable subslice of this slice (or a single element).
  ///
  /// # Panics
  ///
  /// Panics if the index is out of bounds.
  pub fn at_mut<Range: RepIndex>(self, idx: Range) -> Range::Mut<'a, T> {
    self.into_mut().at_mut(idx)
  }

  /// Returns an iterator over this slice.
  pub fn iter(&self) -> Iter<'_, T> {
    self.as_view().into_iter()
  }

  /// Returns a mutable iterator over this slice.
  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    self.as_mut().into_iter()
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a Repeated<'_, T> {
  type Item = View<'a, T>;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a mut Repeated<'_, T> {
  type Item = Mut<'a, T>;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<T: Type + ?Sized> fmt::Debug for Repeated<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.as_view(), f)
  }
}

impl<T, U> PartialEq<[U]> for Repeated<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U]) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == *b)
  }
}

impl<T, U, const N: usize> PartialEq<[U; N]> for Repeated<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U; N]) -> bool {
    self.eq(other.as_slice())
  }
}

impl<T, U> PartialEq<Slice<'_, U>> for Repeated<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Slice<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<SliceMut<'_, U>> for Repeated<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &SliceMut<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<Repeated<'_, U>> for Repeated<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Repeated<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T: Type + ?Sized> Eq for Repeated<'_, T> where
  for<'a> View<'a, T>: PartialEq
{
}

pub trait RepIndex: Clone + fmt::Debug {
  type View<'a, T: Type + ?Sized>
  where
    T: 'a;
  type Mut<'a, T: Type + ?Sized>
  where
    T: 'a;

  #[doc(hidden)]
  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>>;

  #[doc(hidden)]
  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>>;
}

impl RepIndex for usize {
  type View<'a, T: Type + ?Sized> = View<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = Mut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    if self >= len {
      return None;
    }

    Some(T::__make_view(ptr.add(self).cast()))
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    if self >= len {
      return None;
    }

    Some(T::__make_mut(ptr.add(self).cast(), arena))
  }
}

impl RepIndex for ops::Range<usize> {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    if self.start > self.end || self.end > len {
      return None;
    }

    Some(Slice::__wrap(ptr.add(self.start), self.end - self.start))
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    if self.start > self.end || self.end > len {
      return None;
    }

    Some(SliceMut::__wrap(
      ptr.add(self.start),
      self.end - self.start,
      arena,
    ))
  }
}

impl RepIndex for ops::RangeTo<usize> {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    RepIndex::__get(0..self.end, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    RepIndex::__get_mut(0..self.end, ptr, len, arena)
  }
}

impl RepIndex for ops::RangeFrom<usize> {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    RepIndex::__get(self.start..len, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    RepIndex::__get_mut(self.start..len, ptr, len, arena)
  }
}

impl RepIndex for ops::RangeFull {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    Some(Slice::__wrap(ptr, len))
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    Some(SliceMut::__wrap(ptr, len, arena))
  }
}

impl RepIndex for ops::RangeInclusive<usize> {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    if *self.end() == usize::MAX {
      return None;
    }
    RepIndex::__get(*self.start()..*self.end() + 1, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    if *self.end() == usize::MAX {
      return None;
    }
    RepIndex::__get_mut(*self.start()..*self.end() + 1, ptr, len, arena)
  }
}

impl RepIndex for ops::RangeToInclusive<usize> {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    RepIndex::__get(0..=self.end, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    RepIndex::__get_mut(0..=self.end, ptr, len, arena)
  }
}
