//! Indexing traits, a la SliceIndex.

use std::fmt;
use std::ops;

use crate::arena::RawArena;
use crate::Mut;
use crate::Slice;
use crate::SliceMut;
use crate::Type;
use crate::View;

pub trait RepIndex: Clone + fmt::Debug {
  type View<'a, T: Type + ?Sized>
  where
    T: 'a;
  type Mut<'a, T: Type + ?Sized>
  where
    T: 'a;

  #[doc(hidden)]
  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>>;

  #[doc(hidden)]
  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>>;
}

impl RepIndex for usize {
  type View<'a, T: Type + ?Sized> = View<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = Mut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    if self >= len {
      return None;
    }

    Some(T::__make_view(ptr.add(self).cast()))
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    if self >= len {
      return None;
    }

    Some(T::__make_mut(ptr.add(self).cast(), arena))
  }
}

impl RepIndex for ops::Range<usize> {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    if self.start > self.end || self.end > len {
      return None;
    }

    Some(Slice::__wrap(ptr.add(self.start), self.end - self.start))
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    if self.start > self.end || self.end > len {
      return None;
    }

    Some(SliceMut::__wrap(
      ptr.add(self.start),
      self.end - self.start,
      arena,
    ))
  }
}

impl RepIndex for ops::RangeTo<usize> {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    RepIndex::__get(0..self.end, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    RepIndex::__get_mut(0..self.end, ptr, len, arena)
  }
}

impl RepIndex for ops::RangeFrom<usize> {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    RepIndex::__get(self.start..len, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    RepIndex::__get_mut(self.start..len, ptr, len, arena)
  }
}

impl RepIndex for ops::RangeFull {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    Some(Slice::__wrap(ptr, len))
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    Some(SliceMut::__wrap(ptr, len, arena))
  }
}

impl RepIndex for ops::RangeInclusive<usize> {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    if *self.end() == usize::MAX {
      return None;
    }
    RepIndex::__get(*self.start()..*self.end() + 1, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    if *self.end() == usize::MAX {
      return None;
    }
    RepIndex::__get_mut(*self.start()..*self.end() + 1, ptr, len, arena)
  }
}

impl RepIndex for ops::RangeToInclusive<usize> {
  type View<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
  ) -> Option<Self::View<'a, T>> {
    RepIndex::__get(0..=self.end, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: *mut T::__Storage,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    RepIndex::__get_mut(0..=self.end, ptr, len, arena)
  }
}
