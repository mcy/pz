//! Non-trivial pointer types.

use std::fmt;
use std::marker::PhantomData;

use crate::__z::AVec;
use crate::__z::RawArena;
use crate::str;
use crate::OptMut;
use crate::Repeated;
use crate::Slice;
use crate::Str;
use crate::StrBuf;

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

/// A type that has associated "proxy references".
///
/// These types have special reference-like types: a "view" and a "mutator".
/// These are not necessarily implemented as references, to enable various
/// layout and performance optimizations.
pub trait Proxied: 'static {
  /// The view type, analogous to a shared reference.
  type View<'a>: ViewFor<'a, Self> + Copy + Default + fmt::Debug;

  /// The mutator type, analogous to a mutable reference.
  type Mut<'a>: MutFor<'a, Self>;
}

/// Shorthand for [`Proxied::View`].
pub type View<'a, T> = <T as Proxied>::View<'a>;

/// Shorthand for [`Proxied::Mut`].
pub type Mut<'a, T> = <T as Proxied>::Mut<'a>;

/// A generic shorthand for picking `View` or `Mut` based on a type parameter.
///
/// The `Which` parameter must implement the [`select::Select`] trait, which is only
/// implemented by [`select::View`] and [`select::Mut`].
pub type Proxy<'a, T, Which> = <Which as select::Select>::__Proxy<'a, T>;

/// Selector types for [`Proxy`].
pub mod select {
  use super::*;

  /// A selector for use with the [`Proxy`] helper.
  pub trait Select {
    #[doc(hidden)]
    type __Proxy<'a, T: Proxied + ?Sized>: ViewFor<'a, T>;
  }

  /// A [`Select`] that selects a proxied type's view.
  #[derive(Copy, Clone)]
  pub struct View(());
  impl Select for View {
    type __Proxy<'a, T: Proxied + ?Sized> = T::View<'a>;
  }

  /// A [`Select`] that selects a proxied type's mutator.
  #[derive(Copy, Clone)]
  pub struct Mut(());
  impl Select for Mut {
    type __Proxy<'a, T: Proxied + ?Sized> = T::Mut<'a>;
  }
}

/// Helper for projecting a [`Type`] through to the view/mut of an optional
/// field.
pub struct Opt<T: ?Sized>(PhantomData<*mut T>);
impl<T: Type + ?Sized> Proxied for Opt<T> {
  type View<'a> = Option<View<'a, T>>;
  type Mut<'a> = OptMut<'a, T>;
}

impl<'a, T: Type + ?Sized> ViewFor<'a, Opt<T>> for Option<View<'a, T>> {
  fn as_view(&self) -> View<Opt<T>> {
    self.as_ref().map(ViewFor::as_view)
  }
}

impl<'a, T: Type + ?Sized> ViewFor<'a, Opt<T>> for OptMut<'a, T> {
  fn as_view(&self) -> View<Opt<T>> {
    if !self.has() {
      return None;
    }
    Some(self.as_view())
  }
}

impl<'a, T: Type + ?Sized> MutFor<'a, Opt<T>> for OptMut<'a, T> {
  fn into_view(self) -> View<'a, Opt<T>> {
    if !self.has() {
      return None;
    }
    Some(self.into_view())
  }

  fn as_mut(&mut self) -> Mut<Opt<T>> {
    self.reborrow()
  }
}

/// Helper for projecting a [`Type`] through to the view/mut of a repeated
/// field.
pub struct Rep<T: ?Sized>(PhantomData<*mut T>);
impl<T: Type + ?Sized> Proxied for Rep<T> {
  type View<'a> = Slice<'a, T>;
  type Mut<'a> = Repeated<'a, T>;
}

impl<'a, T: Type + ?Sized> ViewFor<'a, Rep<T>> for Slice<'a, T> {
  fn as_view(&self) -> View<Rep<T>> {
    *self
  }
}

impl<'a, T: Type + ?Sized> ViewFor<'a, Rep<T>> for Repeated<'a, T> {
  fn as_view(&self) -> View<Rep<T>> {
    self.as_view()
  }
}

impl<'a, T: Type + ?Sized> MutFor<'a, Rep<T>> for Repeated<'a, T> {
  fn into_view(self) -> View<'a, Rep<T>> {
    self.into_view()
  }

  fn as_mut(&mut self) -> Mut<Rep<T>> {
    self.reborrow()
  }
}

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
    ScalarMut { ptr: self.ptr }
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
