//! Iterator implementations.

use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::arena::RawArena;
use crate::rep::Elem;
use crate::rep::Repeated;
use crate::rep::Slice;
use crate::rep::SliceMut;
use crate::seal::Seal;
use crate::Mut;
use crate::Ref;
use crate::Type;

pub struct Iter<'a, T: Type + ?Sized> {
  start: NonNull<Elem<T>>,
  end: NonNull<Elem<T>>,
  _ph: PhantomData<&'a T>,
}

impl<'a, T: Type + ?Sized> Iterator for Iter<'a, T> {
  type Item = Ref<'a, T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.start == self.end {
      return None;
    }
    let ptr = self.start;
    unsafe {
      self.start = ptr.add(1);
      Some(T::__ref(Seal, ptr))
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let len = unsafe { self.end.offset_from(self.start) } as usize;
    (len, Some(len))
  }
}

pub struct IterMut<'a, T: Type + ?Sized> {
  start: NonNull<Elem<T>>,
  end: NonNull<Elem<T>>,
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
      Some(T::__mut(Seal, ptr, self.arena))
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let len = unsafe { self.end.offset_from(self.start) } as usize;
    (len, Some(len))
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a Slice<'_, T> {
  type Item = Ref<'a, T>;
  type IntoIter = Iter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    (*self).into_iter()
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for Slice<'a, T> {
  type Item = Ref<'a, T>;
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
  type Item = Ref<'a, T>;
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
      start: self.slice.ptr,
      end: unsafe { self.slice.ptr.add(self.slice.len) },
      arena: self.arena,
      _ph: PhantomData,
    }
  }
}

impl<'a, T: Type + ?Sized> IntoIterator for &'a Repeated<'_, T> {
  type Item = Ref<'a, T>;
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
