//! Non-trivial pointer types.

use crate::Str;
use crate::StrBuf;

/// A type that has associated "proxy references".
///
/// These types have special reference-like types: a "view" and a "mutator".
/// These are not necessarily implemented as references, to enable various
/// layout and performance optimizations.
pub trait Proxied {
  /// The view type, analogous to a shared reference.
  type View<'a>: ViewFor<'a, Self> + Copy;

  /// The mutator type, analogous to a mutable reference.
  type Mut<'a>: MutFor<'a, Self>;
}

/// Shorthand for [`Proxied::View`].
pub type View<'a, T> = <T as Proxied>::View<'a>;

/// Shorthand for [`Proxied::Mut`].
pub type Mut<'a, T> = <T as Proxied>::Mut<'a>;

/// A view-like type, analogous to a shared reference.
pub trait ViewFor<'a, T: Proxied + ?Sized> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// In general, this function is only necessary in generic code; all view
  /// types are already covariant.
  fn as_view(&self) -> View<T>;
}

/// A mutator-like type, analogous to a mutable reference.
pub trait MutFor<'a, T: Proxied + ?Sized>: ViewFor<'a, T> {
  /// Consumes this mutator, converting it into a view.
  ///
  /// Unlike `as_view()`, calling this function preserves the lifetime of the
  /// input mutator.
  fn into_view(self) -> View<'a, T>;

  /// Shortens this mutator's lifetime, analogous to reborrowing.
  ///
  /// Use this function to create a temporary copy of this mutator, allowing
  /// for multiple mutation operations in sequence.
  fn as_mut(&mut self) -> Mut<T>;
}

/// A mutator for a scalar type, like `i32` or an enum.
pub struct ScalarMut<'a, T> {
  ptr: &'a mut T,
}

impl<'a, T: Copy> ScalarMut<'a, T> {
  #[doc(hidden)]
  pub fn __wrap(ptr: &'a mut T) -> Self {
    Self { ptr }
  }

  /// Reads the value out the mutator refers to.
  pub fn get(&self) -> T {
    *self.ptr
  }

  /// Sets the value the mutator refers to.
  pub fn set(&mut self, val: T) {
    *self.ptr = val;
  }
}

impl<'a, T> ViewFor<'a, T> for T
where
  T: Copy + for<'b> Proxied<View<'b> = T>,
{
  fn as_view(&self) -> View<T> {
    *self
  }
}

impl<'a, T> ViewFor<'a, T> for ScalarMut<'a, T>
where
  T: Copy + for<'b> Proxied<View<'b> = T>,
{
  fn as_view(&self) -> View<T> {
    self.get()
  }
}

impl<'a, T> MutFor<'a, T> for ScalarMut<'a, T>
where
  T: Copy + for<'b> Proxied<View<'b> = T, Mut<'b> = ScalarMut<'b, T>>,
{
  fn into_view(self) -> View<'a, T> {
    self.get()
  }

  fn as_mut(&mut self) -> Mut<T> {
    ScalarMut { ptr: &mut self.ptr }
  }
}

macro_rules! scalar_proxy {
  ($($Type:ty)*) => {$(
    impl Proxied for $Type {
      type View<'a> = Self;
      type Mut<'a> = ScalarMut<'a, Self>;
    }
  )*};
}

scalar_proxy!(bool i32 u32 f32 i64 u64 f64);

impl Proxied for Str {
  type View<'a> = &'a Self;
  type Mut<'a> = StrBuf<'a>;
}

impl<'a> ViewFor<'a, Str> for &'a Str {
  fn as_view(&self) -> View<Str> {
    *self
  }
}

impl<'a> ViewFor<'a, Str> for StrBuf<'a> {
  fn as_view(&self) -> View<Str> {
    self
  }
}

impl<'a> MutFor<'a, Str> for StrBuf<'a> {
  fn into_view(self) -> View<'a, Str> {
    self.into()
  }

  fn as_mut(&mut self) -> Mut<Str> {
    self.reborrow()
  }
}
