//! Implementations of comparison traits. There's a lot of them.

use crate::Repeated;
use crate::Slice;
use crate::SliceMut;
use crate::Type;
use crate::View;

// Reflexive Eq.

impl<T: Type + ?Sized> Eq for Slice<'_, T>
where
  for<'a> View<'a, T>: PartialEq,
{
  //
}

impl<T: Type + ?Sized> Eq for SliceMut<'_, T>
where
  for<'a> View<'a, T>: PartialEq,
{
  //
}

impl<T: Type + ?Sized> Eq for Repeated<'_, T>
where
  for<'a> View<'a, T>: PartialEq,
{
  //
}

// Comparing to slices.

impl<T, U> PartialEq<[U]> for Slice<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U]) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == *b)
  }
}

impl<T, U> PartialEq<[U]> for SliceMut<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U]) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == *b)
  }
}

impl<T, U> PartialEq<[U]> for Repeated<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U]) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == *b)
  }
}

// Comparing to arrays.

impl<T, U, const N: usize> PartialEq<[U; N]> for Slice<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U; N]) -> bool {
    self.eq(other.as_slice())
  }
}

impl<T, U, const N: usize> PartialEq<[U; N]> for SliceMut<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U; N]) -> bool {
    self.eq(other.as_slice())
  }
}

impl<T, U, const N: usize> PartialEq<[U; N]> for Repeated<'_, T>
where
  T: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<U>,
{
  fn eq(&self, other: &[U; N]) -> bool {
    self.eq(other.as_slice())
  }
}

// Comparing between the n^2 combinations of custom slice.

impl<T, U> PartialEq<Slice<'_, U>> for Slice<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Slice<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<SliceMut<'_, U>> for Slice<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &SliceMut<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<Repeated<'_, U>> for Slice<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Repeated<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<Slice<'_, U>> for SliceMut<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Slice<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<SliceMut<'_, U>> for SliceMut<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &SliceMut<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<Repeated<'_, U>> for SliceMut<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Repeated<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<Slice<'_, U>> for Repeated<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Slice<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<SliceMut<'_, U>> for Repeated<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &SliceMut<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}

impl<T, U> PartialEq<Repeated<'_, U>> for Repeated<'_, T>
where
  T: Type + ?Sized,
  U: Type + ?Sized,
  for<'a> View<'a, T>: PartialEq<View<'a, U>>,
{
  fn eq(&self, other: &Repeated<'_, U>) -> bool {
    if self.len() != other.len() {
      return false;
    }
    self.iter().zip(other).all(|(a, b)| a == b)
  }
}
