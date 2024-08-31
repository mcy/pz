//! Wrappers for field values.

use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::arena::AVec;
use crate::arena::RawArena;
use crate::value::Type;
use crate::Mut;

mod eq;
mod index;
mod iter;

pub use index::RepIndex;
pub use iter::Iter;
pub use iter::IterMut;

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
  pub fn get<Range: RepIndex>(self, idx: Range) -> Option<Range::View<'a, T>> {
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
  pub fn get<Range: RepIndex>(self, idx: Range) -> Option<Range::View<'a, T>> {
    self.into_view().get(idx)
  }

  /// Gets a mutable subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get_mut<Range: RepIndex>(
    self,
    idx: Range,
  ) -> Option<Range::Mut<'a, T>> {
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

  /// Reborrows this repeated field with a shorter lifetime.
  pub fn reborrow(&mut self) -> Repeated<T> {
    Repeated {
      ptr: self.ptr,
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
  pub fn get<Range: RepIndex>(self, idx: Range) -> Option<Range::View<'a, T>> {
    self.into_view().get(idx)
  }

  /// Gets a mutable subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get_mut<Range: RepIndex>(
    self,
    idx: Range,
  ) -> Option<Range::Mut<'a, T>> {
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

impl<T: Type + ?Sized> fmt::Debug for Slice<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut dbg = f.debug_list();
    for value in self {
      dbg.entry(&value);
    }
    dbg.finish()
  }
}

impl<T: Type + ?Sized> fmt::Debug for SliceMut<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.as_view(), f)
  }
}

impl<T: Type + ?Sized> fmt::Debug for Repeated<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.as_view(), f)
  }
}
