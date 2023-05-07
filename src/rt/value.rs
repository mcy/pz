//! Wrappers for field values.

use std::marker::PhantomData;

use super::arena::RawArena;
use super::ptr::Proxied;
use super::ptr::ScalarMut;
use crate::Mut;
use crate::Str;
use crate::StrBuf;
use crate::View;

pub trait Type: Proxied {
  #[doc(hidden)]
  unsafe fn __make_view<'a>(ptr: *mut u8) -> View<'a, Self>;
  #[doc(hidden)]
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: RawArena) -> Mut<'a, Self>;
}

impl<T> Type for T
where
  T: Copy + for<'a> Proxied<View<'a> = T, Mut<'a> = ScalarMut<'a, T>>,
{
  unsafe fn __make_view<'a>(ptr: *mut u8) -> View<'a, Self> {
    ptr.cast::<T>().read()
  }

  unsafe fn __make_mut<'a>(ptr: *mut u8, _arena: RawArena) -> Mut<'a, Self> {
    ScalarMut::__wrap(&mut *ptr.cast::<T>())
  }
}

impl Type for Str {
  unsafe fn __make_view<'a>(ptr: *mut u8) -> View<'a, Self> {
    let (ptr, len) = ptr.cast::<(*mut u8, usize)>().read();
    Str::from_raw_parts(ptr, len)
  }

  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: RawArena) -> Mut<'a, Self> {
    let data = &mut *ptr.cast::<(*mut u8, usize)>();
    StrBuf::__wrap(data, arena)
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
