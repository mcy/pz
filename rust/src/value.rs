//! Wrappers for field values.

use std::marker::PhantomData;

use crate::arena::AVec;
use crate::arena::RawArena;
use crate::ptr::Proxied;
use crate::ptr::ScalarMut;
use crate::str;
use crate::tdp;
use crate::Mut;
use crate::Str;
use crate::StrBuf;
use crate::View;

/// A field type.
///
/// This type is implemented by all `pz` generated types, as well as [`bool`],
/// [`i32`], [`i64`], [`u32`], [`u64`], [`f32`], [`f64`], and [`Str`].
///
/// # Safety
///
/// This trait should not be implemented by users, only by the `pz` compiler.
pub trait Type: Proxied {
  /// The actual underlying storage type for this type. This is used in
  /// the implementation of repeated fields.
  #[doc(hidden)]
  type __Storage;

  /// Constructs a view out of a pointer to storage for this type.
  ///
  /// # Safety
  ///
  /// The pointer must be dereferenceable for this type.
  #[doc(hidden)]
  unsafe fn __make_view<'a>(ptr: *const Self::__Storage) -> View<'a, Self>;

  /// Constructs a view out of a pointer to storage for this type.
  ///
  /// # Safety
  ///
  /// The pointer must be uniquely dereferenceable for this type.
  #[doc(hidden)]
  unsafe fn __make_mut<'a>(
    ptr: *mut Self::__Storage,
    arena: RawArena,
  ) -> Mut<'a, Self>;

  /// Resizes storage for a repeated field for this type.
  ///
  /// # Safety
  ///
  /// The arena vector must be dereferenceable and belong to the given arena.
  #[doc(hidden)]
  unsafe fn __resize(
    vec: &mut AVec<Self::__Storage>,
    new_len: usize,
    arena: RawArena,
  );
}

impl<T> Type for T
where
  T: Copy + for<'a> Proxied<View<'a> = T, Mut<'a> = ScalarMut<'a, T>>,
{
  type __Storage = T;

  unsafe fn __make_view<'a>(ptr: *const T) -> View<'a, Self> {
    ptr.read()
  }

  unsafe fn __make_mut<'a>(ptr: *mut T, _arena: RawArena) -> Mut<'a, Self> {
    ScalarMut::__wrap(&mut *ptr)
  }

  unsafe fn __resize(vec: &mut AVec<T>, new_len: usize, arena: RawArena) {
    vec.resize(new_len, arena)
  }
}

impl Type for Str {
  type __Storage = str::Storage;

  unsafe fn __make_view<'a>(ptr: *const str::Storage) -> View<'a, Self> {
    Str::new((*ptr).as_slice())
  }

  unsafe fn __make_mut<'a>(
    ptr: *mut str::Storage,
    arena: RawArena,
  ) -> Mut<'a, Self> {
    StrBuf::__wrap(&mut *ptr, arena)
  }

  unsafe fn __resize(
    vec: &mut AVec<str::Storage>,
    new_len: usize,
    arena: RawArena,
  ) {
    vec.resize(new_len, arena)
  }
}

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
