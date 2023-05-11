//! Iterator implementations.

use std::marker::PhantomData;

use crate::arena::RawArena;
use crate::value::Type;
use crate::Mut;
use crate::Repeated;
use crate::Slice;
use crate::SliceMut;
use crate::View;

pub struct Iter<'a, T: Type + ?Sized> {
  start: *mut T::__Storage,
  end: *mut T::__Storage,
  _ph: PhantomData<&'a T>,
}

impl<'a, T: Type + ?Sized> Iterator for Iter<'a, T> {
  type Item = View<'a, T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.start == self.end {
      return None;
    }
    let ptr = self.start;
    unsafe {
      self.start = ptr.add(1);
      Some(T::__make_view(ptr as *mut u8))
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let len = unsafe { self.end.offset_from(self.start) } as usize;
    (len, Some(len))
  }
}

pub struct IterMut<'a, T: Type + ?Sized> {
  start: *mut T::__Storage,
  end: *mut T::__Storage,
  arena: RawArena,
  _ph: PhantomData<&'a mut T>,
}

impl<'a, T: Type + ?Sized> Iterator for IterMut<'a, T> {
  type Item = Mut<'a, T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.start == self.end {
      return None;
    }
    let ptr = self.start;
    unsafe {
      self.start = ptr.add(1);
      Some(T::__make_mut(ptr as *mut u8, self.arena))
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let len = unsafe { self.end.offset_from(self.start) } as usize;
    (len, Some(len))
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a Slice<'_, T> {
  type Item = View<'a, T>;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (*self).into_iter()
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for Slice<'a, T> {
  type Item = View<'a, T>;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    Iter {
      start: self.ptr,
      end: unsafe { self.ptr.add(self.len) },
      _ph: PhantomData,
    }
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a SliceMut<'_, T> {
  type Item = View<'a, T>;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a mut SliceMut<'_, T> {
  type Item = Mut<'a, T>;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for SliceMut<'a, T> {
  type Item = Mut<'a, T>;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    IterMut {
      start: self.ptr,
      end: unsafe { self.ptr.add(self.len) },
      arena: self.arena,
      _ph: PhantomData,
    }
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a Repeated<'_, T> {
  type Item = View<'a, T>;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a mut Repeated<'_, T> {
  type Item = Mut<'a, T>;
  type IntoIter = IterMut<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}
