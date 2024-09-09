//! Implementations of reflection traits for tuples.

#![allow(non_snake_case)]

use crate::reflect::Message;
use crate::reflect::Mut;
use crate::reflect::MutView;
use crate::reflect::Ref;
use crate::reflect::RefView;
use crate::reflect::Selector;
use crate::reflect::Views;
use crate::seal::Seal;
use crate::seal::Sealed;

/// Bitmasks used for calculating disjointness.
#[derive(Clone, Copy)]
pub struct Mask {
  disjoint: bool,
  bits: [u128; 256],
}

impl Mask {
  /// Creates a new empty mask.
  pub(crate) const fn empty() -> Mask {
    Mask {
      disjoint: true,
      bits: [0; 256],
    }
  }

  /// Creates a new mask with a single bit set.
  pub(crate) const fn single(idx: usize) -> Mask {
    let mut new = Self::empty();
    new.bits[idx / 128] |= 1 << (idx % 128);
    new
  }

  /// Merges two masks.
  pub(crate) const fn merge(masks: &[&Mask]) -> Mask {
    if masks.is_empty() {
      return Mask::empty();
    }

    let mut new = *masks[0];
    let mut i = 1;
    while i < masks.len() {
      let next = masks[i];
      new.disjoint &= next.disjoint;

      let mut j = 0;
      while j < new.bits.len() {
        let a = new.bits[i];
        let b = next.bits[i];

        new.disjoint &= (a & b) == 0;
        new.bits[i] = a | b;

        j += 1;
      }

      i += 1;
    }

    new
  }
}

impl Views for () {
  type Ref<'a> = ();
  type Mut<'a> = ();
}
impl RefView<'_> for () {
  type Target = ();
  fn as_ref(&self) {}
}
impl MutView<'_> for () {
  type Target = ();
  fn as_ref(&self) {}
  fn into_ref(self) {}
  fn as_mut(&mut self) {}
}

impl<M: Message> Selector<M> for () {
  type Type = ();
  const DISJOINT: bool = true;
  const __M: Mask = Mask::empty();

  fn get(_: Ref<M>) -> Ref<()> {}
  unsafe fn get_mut_unchecked<'a>(
    _: impl Sealed,
    _: &mut Mut<M>,
  ) -> Mut<'a, ()> {
  }
}

macro_rules! tuples {
    ($($($T:ident),*;)*) => {$(
      impl<$($T: Views,)*> Views for ($($T,)*) {
        type Ref<'a> = ($(Ref<'a, $T>,)*);
        type Mut<'a> = ($(Mut<'a, $T>,)*);
      }
      impl<'a, $($T: RefView<'a>,)*> RefView<'a> for ($($T,)*) {
        type Target = ($($T::Target,)*);
        fn as_ref(&self) -> Ref<Self::Target> {
          let ($($T,)*) = self;
          ($($T.as_ref(),)*)
        }
      }
      impl<'a, $($T: MutView<'a>,)*> MutView<'a> for  ($($T,)*) {
        type Target = ($($T::Target,)*);
        fn as_ref(&self) -> Ref<Self::Target> {
          let ($($T,)*) = self;
          ($($T.as_ref(),)*)
        }
        fn into_ref(self) -> Ref<'a, Self::Target> {
          let ($($T,)*) = self;
          ($($T.into_ref(),)*)
        }

        fn as_mut(&mut self) -> Mut<Self::Target> {
          let ($($T,)*) = self;
          ($($T.as_mut(),)*)
        }
      }

      impl<M: Message, $($T: Selector<M>,)*> Selector<M> for ($($T,)*) {
        type Type = ($($T::Type,)*);

        const DISJOINT: bool = Self::__M.disjoint;
        const __M: Mask = Mask::merge(&[$(&$T::__M,)*]);

        fn get(message: Ref<M>) -> Ref<Self::Type> {
          ($($T::get(message),)*)
        }

        #[doc(hidden)]
        unsafe fn get_mut_unchecked<'a>(
          _: impl Sealed,
          message: &mut Mut<M>,
        ) -> Mut<'a, Self::Type> {
          ($($T::get_mut_unchecked(Seal, message),)*)
        }
      }
    )*};
}

tuples! {
  T0;
  T0, T1;
  T0, T1, T2;
  T0, T1, T2, T3;
  T0, T1, T2, T3, T4;
  T0, T1, T2, T3, T4, T5;
  T0, T1, T2, T3, T4, T5, T6;
  T0, T1, T2, T3, T4, T5, T6, T7;
  T0, T1, T2, T3, T4, T5, T6, T7, T8;
  T0, T1, T2, T3, T4, T5, T6, T7, T8, T9;
  T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10;
  T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11;
}
