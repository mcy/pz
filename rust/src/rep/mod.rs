//! Wrappers for field values.

use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::arena::AVec;
use crate::arena::RawArena;
use crate::reflect;
use crate::reflect::MutView;
use crate::reflect::RefView;
use crate::reflect::Rep;
use crate::reflect::Set;
use crate::seal::Seal;
use crate::Mut;
use crate::Type;

mod eq;
mod index;
mod iter;

pub use index::RepIndex;
pub use iter::Iter;
pub use iter::IterMut;

type Elem<T> = <T as reflect::private::Type>::__Storage<Seal>;

/// An immutable slice of a [`Repeated`] field. This is roughly equivalent to
/// a `&[T]`.
pub struct Slice<'a, T: Type + ?Sized> {
  ptr: NonNull<Elem<T>>,
  len: usize,
  _ph: PhantomData<&'a T>,
}

impl<'a, T: Type + ?Sized> Slice<'a, T> {
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
  pub fn get<Range: RepIndex>(self, idx: Range) -> Option<Range::Ref<'a, T>> {
    unsafe { idx.__get(self.ptr, self.len) }
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// # Panics
  ///
  /// Panics if the index is out of bounds.
  pub fn at<Range: RepIndex>(self, idx: Range) -> Range::Ref<'a, T> {
    match self.get(idx.clone()) {
      Some(v) => v,
      None => panic!(
        "slice index out of bounds: got {idx:?} but expected 0..{}",
        self.len()
      ),
    }
  }

  /// Returns an iterator over this slice.
  pub fn iter(&self) -> Iter<T> {
    self.into_iter()
  }

  #[doc(hidden)]
  pub(crate) unsafe fn from_raw_parts(
    ptr: NonNull<Elem<T>>,
    len: usize,
  ) -> Self {
    Self {
      ptr,
      len,
      _ph: PhantomData,
    }
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
      ptr: NonNull::dangling(),
      len: 0,
      _ph: PhantomData,
    }
  }
}

impl<'a, T: Type> RefView<'a> for Slice<'a, T> {
  type Target = Rep<T>;

  fn as_ref(&self) -> Slice<T> {
    *self
  }
}

/// An mutable slice of a [`Repeated`] field. This is roughly equivalent to
/// a `&mut [T]`.
pub struct SliceMut<'a, T: Type + ?Sized> {
  slice: Slice<'a, T>,
  arena: RawArena,
  _ph: PhantomData<&'a mut T>,
}

impl<'a, T: Type + ?Sized> SliceMut<'a, T> {
  /// Reborrows this slice as a shared slice.
  pub fn as_ref(&self) -> Slice<'_, T> {
    self.slice
  }

  /// Converts this slice into a shared slice.
  ///
  /// Note that unlike `as_ref()`, this function consumes the slice to give it
  /// the longest possible lifetime.
  pub fn into_ref(self) -> Slice<'a, T> {
    self.slice
  }

  /// Reborrows this slice as a mutable slice.
  pub fn as_mut(&mut self) -> SliceMut<'_, T> {
    SliceMut {
      slice: self.slice,
      arena: self.arena,
      _ph: PhantomData,
    }
  }

  /// Returns whether this `SliceMut` has a length of zero.
  pub fn is_empty(&self) -> bool {
    self.as_ref().is_empty()
  }

  /// Returns the length of this `SliceMut`.
  pub fn len(&self) -> usize {
    self.as_ref().len()
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get<Range: RepIndex>(self, idx: Range) -> Option<Range::Ref<'a, T>> {
    self.into_ref().get(idx)
  }

  /// Gets a mutable subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get_mut<Range: RepIndex>(
    self,
    idx: Range,
  ) -> Option<Range::Mut<'a, T>> {
    unsafe { idx.__get_mut(self.slice.ptr, self.slice.len, self.arena) }
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// # Panics
  ///
  /// Panics if the index is out of bounds.
  pub fn at<Range: RepIndex>(self, idx: Range) -> Range::Ref<'a, T> {
    self.into_ref().at(idx)
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
  pub fn iter(&self) -> Iter<T> {
    self.as_ref().into_iter()
  }

  /// Returns a mutable iterator over this slice.
  pub fn iter_mut(&mut self) -> IterMut<T> {
    self.as_mut().into_iter()
  }

  #[doc(hidden)]
  pub(crate) unsafe fn from_raw_parts(
    ptr: NonNull<Elem<T>>,
    len: usize,
    arena: RawArena,
  ) -> Self {
    Self {
      slice: Slice::from_raw_parts(ptr, len),
      arena,
      _ph: PhantomData,
    }
  }
}

/// An mutable slice of a [`Repeated`] field. This is roughly equivalent to
/// a `&mut Vec<T>`.
pub struct Repeated<'a, T: Type + ?Sized> {
  raw: &'a mut AVec<Elem<T>>,
  arena: RawArena,
}

impl<'a, T: Type + ?Sized> Repeated<'a, T> {
  /// Returns whether this repeated field has a length of zero.
  pub fn is_empty(&self) -> bool {
    self.as_ref().is_empty()
  }

  /// Returns the length of this repeated field.
  pub fn len(&self) -> usize {
    self.as_ref().len()
  }

  /// Returns the capacity of this repeated field.
  pub fn capacity(&self) -> usize {
    self.raw.cap()
  }

  /// Reserves space in this repeated field for adding `n` more elements without
  /// resizing the underlying buffer.
  pub fn reserve(&mut self, n: usize) {
    let wanted_cap = self.len().saturating_add(n);
    if self.capacity() >= wanted_cap {
      return;
    }

    self.raw.grow(Some(wanted_cap), self.arena)
  }

  /// Adds a new, default element to this repeated field, returning a mutator
  /// for it.
  ///
  /// # Memory Usage Footguns
  ///
  /// This function and [`Repeated::push()`] are generally not recommended
  /// without first calling [`Repeated::reserve()`] first. This is because
  /// although a [`Repeated`] works like a [`Vec`], resized memory generally
  /// isn't freed until the whole message is destroyed. Naive use of these
  /// functions can result in quadratic memory usage.
  pub fn add(&mut self) -> Mut<'_, T> {
    let new_len = self.len() + 1;
    unsafe {
      T::__resize(Seal, self.raw, new_len, self.arena);
      T::__mut(
        Seal,
        self.raw.as_ptr().unwrap_unchecked().add(new_len - 1).cast(),
        self.arena,
      )
    }
  }

  /// Pushes a new value to this repeated field, returning a mutator for it.
  ///
  /// # Memory Usage Footguns
  ///
  /// This function generally isn't what you want. See related docs on
  /// [`Repeated::add()`].
  pub fn push(&mut self, value: impl Set<T>) -> Mut<'_, T> {
    let mut m = self.add();
    value.apply_to(m.as_mut());
    m
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
      self.raw.set_len(new_len);
    }
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get<Range: RepIndex>(self, idx: Range) -> Option<Range::Ref<'a, T>> {
    self.into_slice().get(idx)
  }

  /// Gets a mutable subslice of this slice (or a single element).
  ///
  /// Returns `None` if the range is out of bounds.
  pub fn get_mut<Range: RepIndex>(
    self,
    idx: Range,
  ) -> Option<Range::Mut<'a, T>> {
    self.into_mut_slice().get_mut(idx)
  }

  /// Gets a subslice of this slice (or a single element).
  ///
  /// # Panics
  ///
  /// Panics if the index is out of bounds.
  pub fn at<Range: RepIndex>(self, idx: Range) -> Range::Ref<'a, T> {
    self.into_slice().at(idx)
  }

  /// Gets a mutable subslice of this slice (or a single element).
  ///
  /// # Panics
  ///
  /// Panics if the index is out of bounds.
  pub fn at_mut<Range: RepIndex>(self, idx: Range) -> Range::Mut<'a, T> {
    self.into_mut_slice().at_mut(idx)
  }

  /// Returns an iterator over this slice.
  pub fn iter(&self) -> Iter<T> {
    self.as_slice().into_iter()
  }

  /// Returns a mutable iterator over this slice.
  pub fn iter_mut(&mut self) -> IterMut<T> {
    self.as_mut_slice().into_iter()
  }

  /// Borrows this repeated field as a shared slice.
  pub fn as_slice(&self) -> Slice<T> {
    Slice {
      ptr: self.raw.as_ptr().unwrap_or(NonNull::dangling()),
      len: self.raw.len(),
      _ph: PhantomData,
    }
  }

  /// Borrows this repeated field as a mutable slice.
  pub fn as_mut_slice(&mut self) -> SliceMut<T> {
    SliceMut {
      slice: self.as_slice(),
      arena: self.arena,
      _ph: PhantomData,
    }
  }

  /// Converts this repeated field as a shared slice.
  pub fn into_slice(self) -> Slice<'a, T> {
    Slice {
      ptr: self.raw.as_ptr().unwrap_or(NonNull::dangling()),
      len: self.raw.len(),
      _ph: PhantomData,
    }
  }

  /// Converts this repeated field as a mutable slice.
  pub fn into_mut_slice(self) -> SliceMut<'a, T> {
    SliceMut {
      arena: self.arena,
      _ph: PhantomData,
      slice: self.into_slice(),
    }
  }

  pub(crate) unsafe fn from_raw_parts(
    raw: &'a mut AVec<Elem<T>>,
    arena: RawArena,
  ) -> Self {
    Self { raw, arena }
  }
}

impl<'a, T: Type> MutView<'a> for Repeated<'a, T> {
  type Target = Rep<T>;

  fn as_ref(&self) -> Slice<T> {
    self.as_slice()
  }

  fn into_ref(self) -> Slice<'a, T> {
    self.into_slice()
  }

  fn as_mut(&mut self) -> Mut<Self::Target> {
    Repeated {
      raw: self.raw,
      arena: self.arena,
    }
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
    fmt::Debug::fmt(&self.as_ref(), f)
  }
}

impl<T: Type + ?Sized> fmt::Debug for Repeated<'_, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Debug::fmt(&self.as_ref(), f)
  }
}

impl<T: Type, U: IntoIterator<Item: Set<T>>> Set<Rep<T>> for U {
  fn apply_to(self, mut m: Mut<Rep<T>>) {
    m.clear();

    let it = self.into_iter();
    let (low, _) = it.size_hint();
    m.reserve(low);

    for value in it {
      m.push(value);
    }
  }
}
