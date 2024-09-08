//! Wrappers for field values.

use std::marker::PhantomData;

use crate::arena::RawArena;
use crate::reflect::Mut;
use crate::reflect::MutView;
use crate::reflect::Opt;
use crate::reflect::Ref;
use crate::reflect::Set;
use crate::reflect::Type;
use crate::seal::Seal;
use crate::tdp;
use crate::tdp::Opaque;

/// A mutator for optional (singular) fields.
///
/// An `OptMut<T>` is essentially an `&mut Option<Mut<T>>`, but with a more
/// flexible layout. For example, the "has bits" of all of the fields in a
/// `message` are stacked into single bits at the top of the message; the
/// "which word" for a `choice` determines which variant is engaged. In both
/// cases, the layout of the difference between `Some` and `None` is
/// specialized, so instead of an `&mut Option`, we must provide a wrapper.
pub struct OptMut<'a, T: Type> {
  ptr: Opaque,
  arena: RawArena,
  field: tdp::Field,
  _ph: PhantomData<&'a mut T>,
}

impl<'a, T: Type> OptMut<'a, T> {
  /// Returns whether this value is unset.
  pub fn is_none(&self) -> bool {
    !self.is_some()
  }

  /// Returns whether this value is present.
  pub fn is_some(&self) -> bool {
    unsafe { self.field.has(self.ptr) }
  }

  /// Clears the field.
  #[inline(always)]
  pub fn clear(&mut self) {
    unsafe { self.field.clear(self.ptr) }
  }

  /// Converts this mutator into a view, returning a view of the default value
  /// if the view it isn't present.
  #[track_caller]
  pub fn unwrap(&self) -> Ref<T> {
    // TODO: default?
    self.as_ref().unwrap()
  }

  /// Converts this mutator into a view, returning a view of the default value
  /// if the view it isn't present.
  ///
  /// This version consumes the mutator to make the returned view's lifetime as
  /// long as possible.
  #[track_caller]
  pub fn into_unwrap(self) -> Ref<'a, T> {
    self.into_ref().unwrap()
  }

  /// Converts this mutator into a mutator of the underlying type, initializing
  /// the field if it isn't already
  pub fn as_inner(&mut self) -> Mut<T> {
    self.as_mut().into_inner()
  }

  /// Converts this mutator into a mutator of the underlying type, initializing
  /// the field if it isn't already.
  ///
  /// This version consumes the mutator to make the returned mutator's lifetime
  /// as long as possible.
  pub fn into_inner(mut self) -> Mut<'a, T> {
    unsafe {
      self.field.init(self.ptr, self.arena);
      self.__mut()
    }
  }

  /// Converts this mutator into a mutator of the underlying type, or returns
  /// `None` if it isn't present.
  pub fn as_option(&mut self) -> Option<Mut<T>> {
    self.as_mut().into_option()
  }

  /// Converts this mutator into a mutator of the underlying type, or returns
  /// `None` if it isn't present.
  ///
  /// This version consumes the mutator to make the returned mutator's lifetime
  /// as long as possible.
  pub fn into_option(mut self) -> Option<Mut<'a, T>> {
    if !self.is_some() {
      return None;
    }
    unsafe { Some(self.__mut()) }
  }
}

impl<'a, T: Type> MutView<'a> for OptMut<'a, T> {
  type Target = Opt<T>;

  fn as_ref(&self) -> Ref<Self::Target> {
    if self.is_none() {
      return None;
    }
    Some(unsafe { self.__ref() })
  }

  fn into_ref(self) -> Ref<'a, Self::Target> {
    if self.is_none() {
      return None;
    }
    Some(unsafe { self.__ref() })
  }

  fn as_mut(&mut self) -> Mut<Self::Target> {
    OptMut {
      ptr: self.ptr,
      arena: self.arena,
      field: self.field,
      _ph: PhantomData,
    }
  }
}

impl<T: Type> OptMut<'_, T> {
  /// Wraps a pointer to a message type, its arena, and a field to make an
  /// `OptMut`.
  pub(crate) unsafe fn new(
    ptr: Opaque,
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
  unsafe fn __ref<'b>(&self) -> Ref<'b, T> {
    unsafe { T::__ref(Seal, self.field.cast(self.ptr)) }
  }

  #[inline(always)]
  unsafe fn __mut<'b>(&mut self) -> Mut<'b, T> {
    unsafe { T::__mut(Seal, self.field.cast(self.ptr), self.arena) }
  }
}

impl<T: Type, U: Set<T>> Set<Opt<T>> for Option<U> {
  fn apply_to(self, mut m: Mut<Opt<T>>) {
    let Some(value) = self else {
      m.clear();
      return
    };

    value.apply_to(m.into_inner())
  }
}
