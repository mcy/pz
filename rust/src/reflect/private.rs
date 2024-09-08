//! Private items of reflection traits.

use std::ptr::NonNull;

use crate::arena::AVec;
use crate::arena::RawArena;
use crate::reflect::Mut;
use crate::reflect::Ref;
use crate::reflect::Views;
use crate::seal::Sealed;
use crate::tdp;
use crate::tdp::Opaque;

pub trait Type: Views {
  /// The actual underlying storage type for this type. This is used in
  /// the implementation of repeated fields.
  #[doc(hidden)]
  type __Storage<S: Sealed>;

  /// Constructs a view out of a pointer to storage for this type.
  ///
  /// # Safety
  ///
  /// The pointer must be dereferenceable for this type.
  #[doc(hidden)]
  unsafe fn __ref<'a, S: Sealed>(
    _: S,
    ptr: NonNull<Self::__Storage<S>>,
  ) -> Ref<'a, Self>;

  /// Constructs a view out of a pointer to storage for this type.
  ///
  /// # Safety
  ///
  /// The pointer must be uniquely dereferenceable for this type.
  #[doc(hidden)]
  unsafe fn __mut<'a, S: Sealed>(
    _: S,
    ptr: NonNull<Self::__Storage<S>>,
    arena: RawArena,
  ) -> Mut<'a, Self>;

  /// Resizes storage for a repeated field for this type.
  ///
  /// # Safety
  ///
  /// The arena vector must be dereferenceable and belong to the given arena.
  #[doc(hidden)]
  unsafe fn __resize<S: Sealed>(
    _: S,
    vec: &mut AVec<Self::__Storage<S>>,
    new_len: usize,
    arena: RawArena,
  ) {
    vec.resize(new_len, arena)
  }
}

pub trait Message: Views {
  /// TDP metadata for this message.
  #[doc(hidden)]
  const __TDP: tdp::Desc;

  #[doc(hidden)]
  fn __is_null(&self, _: impl Sealed) -> bool;
  #[doc(hidden)]
  fn __raw(_: impl Sealed, ptr: Ref<Self>) -> Opaque;
  #[doc(hidden)]
  fn __arena(_: impl Sealed, ptr: &mut Mut<Self>) -> RawArena;
}