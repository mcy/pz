use std::fmt;

use crate::seal::Seal;
use crate::seal::Sealed;

/// A type that has generalized view types.
///
/// A view is like a reference, but does not carry all of the semantics of a
/// reference. For example, `View<i32>` is just an `i32`, not a reference.
///
/// Views can also carry additional information for a given type. For example,
/// mutable views for message types carry around an arena, for allocating new
/// submessages.
pub trait Views: 'static + Sized {
  /// The shared view type, analogous to a shared reference.
  type Ref<'a>: RefView<'a, Target = Self> + fmt::Debug;

  /// The mutable view type, analogous to a mutable reference.
  type Mut<'a>: MutView<'a, Target = Self>;
}

/// Selects [`Ref<T>`] or [`Mut<T>`] based on a type parameter.
///
/// The `Which` parameter must implement the [`Select`] trait, which is only
/// implemented by [`SelectRef`] and [`SelectMut`].
pub type View<'a, T, Which> = <Which as Select>::View<'a, T, Seal>;

/// Shorthand for [`Views::Ref`].
pub type Ref<'a, T> = <T as Views>::Ref<'a>;

/// Shorthand for [`Views::Mut`].
pub type Mut<'a, T> = <T as Views>::Mut<'a>;

/// A view-like type, analogous to a shared reference.
pub trait RefView<'a>: Copy + Default {
  type Target: Views + ?Sized;

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// In general, this function is only necessary in generic code; all view
  /// types are already covariant.
  fn as_ref(&self) -> Ref<Self::Target>;
}

/// A mutator-like type, analogous to a mutable reference.
pub trait MutView<'a> {
  type Target: Views + ?Sized;

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// In general, this function is only necessary in generic code; all view
  /// types are already covariant.
  fn as_ref(&self) -> Ref<Self::Target>;

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// Unlike `as_view()`, calling this function preserves the lifetime of the
  /// input mutator.
  fn into_ref(self) -> Ref<'a, Self::Target>;

  /// Shortens this mutator's lifetime, analogous to reborrowing.
  ///
  /// Use this function to create a temporary copy of this mutator, allowing
  /// for multiple mutation operations in sequence.
  fn as_mut(&mut self) -> Mut<Self::Target>;
}

/// A selector for use with [`View`].
pub trait Select: Sealed {
  #[doc(hidden)]
  type View<'a, T: Views, S: Sealed>;
}

/// A [`Select`] that selects a proxied type's view.
#[derive(Copy, Clone)]
pub struct SelectRef(());
impl Sealed for SelectRef {}
impl Select for SelectRef {
  type View<'a, T: Views, S: Sealed> = T::Ref<'a>;
}

/// A [`Select`] that selects a proxied type's mutator.
#[derive(Copy, Clone)]
pub struct SelectMut(());
impl Sealed for SelectMut {}
impl Select for SelectMut {
  type View<'a, T: Views, S: Sealed> = T::Mut<'a>;
}
