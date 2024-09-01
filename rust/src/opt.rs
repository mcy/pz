//! Wrappers for field values.

use std::marker::PhantomData;

use crate::arena::RawArena;
use crate::tdp;
use crate::Mut;
use crate::Type;
use crate::View;

/// A mutator for optional (singular) fields.
pub struct OptMut<'a, T: Type + ?Sized> {
  ptr: *mut u8,
  arena: RawArena,
  field: tdp::Field,
  _ph: PhantomData<&'a mut T>,
}

impl<'a, T: Type + ?Sized> OptMut<'a, T> {
  /// Returns whether this value is present.
  #[inline(always)]
  pub fn has(&self) -> bool {
    unsafe { self.field.has(self.ptr) }
  }

  /// Clears the field.
  #[inline(always)]
  pub fn clear(&mut self) {
    unsafe { self.field.clear(self.ptr) }
  }

  /// Converts this mutator into a view, returning a view of the default value
  /// if the view it isn't present.
  #[inline(always)]
  pub fn as_view(&self) -> View<T> {
    if !self.has() {
      todo!()
    }

    unsafe { self.make_view() }
  }

  /// Converts this mutator into a view, returning a view of the default value
  /// if the view it isn't present.
  ///
  /// This version consumes the mutator to make the returned view's lifetime as
  /// long as possible.
  #[inline(always)]
  pub fn into_view(self) -> View<'a, T> {
    if !self.has() {
      todo!()
    }

    unsafe { self.make_view() }
  }

  /// Converts this mutator into a mutator of the underlying type, initializing
  /// the field if it isn't already.
  #[inline(always)]
  pub fn as_mut(&mut self) -> Mut<T> {
    self.reborrow().into_mut()
  }

  /// Converts this mutator into a mutator of the underlying type, initializing
  /// the field if it isn't already.
  ///
  /// This version consumes the mutator to make the returned mutator's lifetime
  /// as long as possible.
  #[inline(always)]
  pub fn into_mut(mut self) -> Mut<'a, T> {
    unsafe {
      self.field.init(self.ptr, self.arena);
      self.make_mut()
    }
  }

  /// Converts this mutator into a mutator of the underlying type, or returns
  /// `None` if it isn't present.
  #[inline(always)]
  pub fn as_mut_or(&mut self) -> Option<Mut<T>> {
    if !self.has() {
      return None;
    }
    unsafe { Some(self.make_mut()) }
  }

  /// Converts this mutator into a mutator of the underlying type, or returns
  /// `None` if it isn't present.
  ///
  /// This version consumes the mutator to make the returned mutator's lifetime
  /// as long as possible.
  #[inline(always)]
  pub fn into_mut_or(mut self) -> Option<Mut<'a, T>> {
    if !self.has() {
      return None;
    }
    unsafe { Some(self.make_mut()) }
  }

  /// Reborrows this mutator with a shorter lifetime.
  #[inline(always)]
  pub fn reborrow(&mut self) -> OptMut<T> {
    OptMut {
      ptr: self.ptr,
      arena: self.arena,
      field: self.field,
      _ph: PhantomData,
    }
  }

  /// Wraps a pointer to a message type, its arena, and a field to make an
  /// `OptMut`.
  #[doc(hidden)]
  pub unsafe fn __wrap(
    ptr: *mut u8,
    arena: RawArena,
    field: tdp::Field,
  ) -> Self {
    Self {
      ptr,
      arena,
      field,
      _ph: PhantomData,
    }
  }

  #[inline(always)]
  unsafe fn make_view<'b>(&self) -> View<'b, T> {
    unsafe { T::__make_view(self.field.cast(self.ptr)) }
  }

  #[inline(always)]
  unsafe fn make_mut<'b>(&mut self) -> Mut<'b, T> {
    unsafe { T::__make_mut(self.field.cast(self.ptr), self.arena) }
  }
}
