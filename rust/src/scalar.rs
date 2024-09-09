//! Types for scalars.

use std::ptr::NonNull;

use crate::arena::RawArena;
use crate::reflect::private;
use crate::reflect::Mut;
use crate::reflect::MutView;
use crate::reflect::Opt;
use crate::reflect::Ref;
use crate::reflect::RefView;
use crate::reflect::Set;
use crate::reflect::Type;
use crate::reflect::Views;
use crate::seal::Sealed;

/// A mutator view for a scalar type, like `i32` or an enum.
///
/// This is a custom type rather than an `&mut T` to account for layout
/// optimizations (such as packing).
pub struct ScalarMut<'a, T> {
  ptr: &'a mut T,
}

impl<'a, T: Copy> ScalarMut<'a, T> {
  #[doc(hidden)]
  pub fn __wrap(ptr: &'a mut T) -> Self {
    Self { ptr }
  }

  /// Reads out the value that this mutator view refers to.
  pub fn get(&self) -> T {
    *self.ptr
  }

  /// Sets the value this mutator view refers to.
  pub fn set(&mut self, val: T) {
    *self.ptr = val;
  }
}

impl<'a, T: Copy + Type> MutView<'a> for ScalarMut<'a, T>
where
  T: for<'b> Views<Ref<'b> = T, Mut<'b> = ScalarMut<'b, T>>,
{
  type Target = T;

  fn as_ref(&self) -> Ref<T> {
    self.get()
  }

  fn into_ref(self) -> Ref<'a, T> {
    self.get()
  }

  fn as_mut(&mut self) -> Mut<T> {
    ScalarMut { ptr: self.ptr }
  }
}

macro_rules! impl_scalar {
  ($($T:ty),*) => {$(
    impl private::Type for $T {
      type __Storage<S: Sealed> = Self;

      unsafe fn __ref<'a, S: Sealed>(_: S, ptr: NonNull<Self>) -> Ref<'a, Self> {
        ptr.read()
      }

      unsafe fn __mut<'a, S: Sealed>(
        _: S,
        mut ptr: NonNull<Self>,
        _: RawArena,
      ) -> Mut<'a, Self> {
        ScalarMut { ptr: ptr.as_mut() }
      }
    }

    impl Views for $T {
      type Ref<'a> = Self;
      type Mut<'a> = ScalarMut<'a, Self>;
    }

    impl RefView<'_> for $T {
      type Target = Self;

      fn as_ref(&self) -> Self {
        *self
      }
    }

    impl Set<$T> for $T {
      fn apply_to(self, mut m: Mut<$T>) {
        m.set(self)
      }
    }

    impl Set<Opt<$T>> for $T {
      fn apply_to(self, m: Mut<Opt<$T>>) {
        m.into_inner().set(self)
      }
    }
  )*};
}

impl_scalar!(i32, i64, u32, u64, f32, f64, bool);
