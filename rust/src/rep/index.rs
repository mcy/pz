//! Indexing traits, a la SliceIndex.

use std::fmt;
use std::ops;
use std::ptr::NonNull;

use crate::arena::RawArena;
use crate::rep::Elem;
use crate::rep::Slice;
use crate::rep::SliceMut;
use crate::seal::Seal;
use crate::Mut;
use crate::Ref;
use crate::Type;

pub trait RepIndex: Clone + fmt::Debug {
  type Ref<'a, T: Type + ?Sized>
  where
    T: 'a;
  type Mut<'a, T: Type + ?Sized>
  where
    T: 'a;

  #[doc(hidden)]
  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
  ) -> Option<Self::Ref<'a, T>>;

  #[doc(hidden)]
  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>>;
}

impl RepIndex for usize {
  type Ref<'a, T: Type + ?Sized> = Ref<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = Mut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
  ) -> Option<Self::Ref<'a, T>> {
    if self >= len {
      return None;
    }

    Some(T::__ref(Seal, ptr.add(self).cast()))
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    if self >= len {
      return None;
    }

    Some(T::__mut(Seal, ptr.add(self).cast(), arena))
  }
}

impl RepIndex for ops::Range<usize> {
  type Ref<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
  ) -> Option<Self::Ref<'a, T>> {
    if self.start > self.end || self.end > len {
      return None;
    }

    Some(Slice::from_raw_parts(
      ptr.add(self.start),
      self.end - self.start,
    ))
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    if self.start > self.end || self.end > len {
      return None;
    }

    Some(SliceMut::from_raw_parts(
      ptr.add(self.start),
      self.end - self.start,
      arena,
    ))
  }
}

impl RepIndex for ops::RangeTo<usize> {
  type Ref<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
  ) -> Option<Self::Ref<'a, T>> {
    RepIndex::__get(0..self.end, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    RepIndex::__get_mut(0..self.end, ptr, len, arena)
  }
}

impl RepIndex for ops::RangeFrom<usize> {
  type Ref<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
  ) -> Option<Self::Ref<'a, T>> {
    RepIndex::__get(self.start..len, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    RepIndex::__get_mut(self.start..len, ptr, len, arena)
  }
}

impl RepIndex for ops::RangeFull {
  type Ref<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
  ) -> Option<Self::Ref<'a, T>> {
    Some(Slice::from_raw_parts(ptr, len))
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    Some(SliceMut::from_raw_parts(ptr, len, arena))
  }
}

impl RepIndex for ops::RangeInclusive<usize> {
  type Ref<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
  ) -> Option<Self::Ref<'a, T>> {
    if *self.end() == usize::MAX {
      return None;
    }
    RepIndex::__get(*self.start()..*self.end() + 1, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
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
  type Ref<'a, T: Type + ?Sized> = Slice<'a, T> where T: 'a;
  type Mut<'a, T: Type + ?Sized> = SliceMut<'a, T> where T: 'a;

  unsafe fn __get<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
  ) -> Option<Self::Ref<'a, T>> {
    RepIndex::__get(0..=self.end, ptr, len)
  }

  unsafe fn __get_mut<'a, T: Type + ?Sized + 'a>(
    self,
    ptr: NonNull<Elem<T>>,
    len: usize,
    arena: RawArena,
  ) -> Option<Self::Mut<'a, T>> {
    RepIndex::__get_mut(0..=self.end, ptr, len, arena)
  }
}
