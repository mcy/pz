//! Wrappers for field values.

use std::marker::PhantomData;

use crate::arena::AVec;
use crate::arena::RawArena;
use crate::ptr::Proxied;
use crate::ptr::ScalarMut;
use crate::tdp;
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
  field: tdp::Field,
  _ph: PhantomData<&'a mut T>,
}

impl<'a, T: Type + ?Sized> OptMut<'a, T> {
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

  /// Returns whether this value is present.
  #[inline(always)]
  pub fn has(&self) -> bool {
    unsafe { self.field.has(self.ptr) }
  }

  /// Initializes the field, if it isn't already.
  #[inline(always)]
  pub fn init(&mut self) {
    unsafe { self.field.init(self.ptr, self.arena) }
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

    unsafe { T::__make_view(self.ptr.add(self.field.offset())) }
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

    unsafe { T::__make_view(self.ptr.add(self.field.offset())) }
  }

  /// Converts this mutator into a mutator of the underlying type, initializing
  /// the field if it isn't already.
  #[inline(always)]
  pub fn as_mut(&mut self) -> Mut<T> {
    self.init();
    unsafe { T::__make_mut(self.ptr.add(self.field.offset()), self.arena) }
  }

  /// Converts this mutator into a mutator of the underlying type, initializing
  /// the field if it isn't already.
  ///
  /// This version consumes the mutator to make the returned mutator's lifetime
  /// as long as possible.
  #[inline(always)]
  pub fn into_mut(mut self) -> Mut<'a, T> {
    self.init();
    unsafe { T::__make_mut(self.ptr.add(self.field.offset()), self.arena) }
  }

  /// Converts this mutator into a mutator of the underlying type, or returns
  /// `None` if it isn't present.
  #[inline(always)]
  pub fn as_mut_or(&mut self) -> Option<Mut<T>> {
    if !self.has() {
      return None;
    }
    unsafe {
      Some(T::__make_mut(self.ptr.add(self.field.offset()), self.arena))
    }
  }

  /// Converts this mutator into a mutator of the underlying type, or returns
  /// `None` if it isn't present.
  ///
  /// This version consumes the mutator to make the returned mutator's lifetime
  /// as long as possible.
  #[inline(always)]
  pub fn into_mut_or(self) -> Option<Mut<'a, T>> {
    if !self.has() {
      return None;
    }
    unsafe {
      Some(T::__make_mut(self.ptr.add(self.field.offset()), self.arena))
    }
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
}
