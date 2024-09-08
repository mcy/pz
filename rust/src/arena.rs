//! Arena support.

#![allow(clippy::missing_safety_doc)]

use std::alloc::Layout;
use std::ptr;
use std::ptr::NonNull;
use std::slice;

use crate::tdp::Opaque;

#[derive(Copy, Clone)]
pub struct RawArena {
  bump: NonNull<bumpalo::Bump>,
}

impl RawArena {
  #[allow(clippy::new_without_default)]
  pub fn new() -> Self {
    unsafe {
      Self {
        bump: NonNull::new_unchecked(Box::leak(Box::new(bumpalo::Bump::new()))),
      }
    }
  }

  pub fn alloc(&self, layout: Layout) -> NonNull<u8> {
    unsafe { self.bump.as_ref() }.alloc_layout(layout)
  }

  pub unsafe fn destroy(&self) {
    let _ = Box::from_raw(self.bump.as_ptr());
  }
}

#[repr(C)]
pub struct AVec<T> {
  // This is intentionally the first value, so that in Hazzer we can specify
  // "clear the first eight bytes" to clear a repeated field without trashing
  // its internal pointer.
  len: usize,

  // Option so that all-zeros is a valid representation.
  ptr: Option<NonNull<T>>,
  cap: usize,
}

// This type is Copy so we can put it inside of a union; it has no destructor.
impl<T> Copy for AVec<T> {}
impl<T> Clone for AVec<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T> Default for AVec<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T> AVec<T> {
  pub const fn new() -> AVec<T> {
    Self {
      ptr: None,
      cap: 0,
      len: 0,
    }
  }

  pub fn as_ptr(&self) -> Option<NonNull<T>> {
    self.ptr
  }

  pub fn as_raw_ptr(&self) -> *mut T {
    self.ptr.map(NonNull::as_ptr).unwrap_or(ptr::null_mut())
  }

  #[allow(clippy::len_without_is_empty)]
  pub fn len(&self) -> usize {
    self.len
  }

  pub fn cap(&self) -> usize {
    self.cap
  }

  pub unsafe fn set_len(&mut self, len: usize) {
    self.len = len
  }

  pub unsafe fn as_slice(&self) -> &[T] {
    slice::from_raw_parts(
      self.as_ptr().unwrap_or(NonNull::dangling()).as_ptr(),
      self.len,
    )
  }

  pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
    slice::from_raw_parts_mut(
      self.as_ptr().unwrap_or(NonNull::dangling()).as_ptr(),
      self.len,
    )
  }

  pub fn add(&mut self, arena: RawArena) -> &mut T {
    unsafe {
      self.resize(self.len() + 1, arena);
      self.ptr.unwrap_unchecked().add(self.len() - 1).as_mut()
    }
  }

  pub fn push(&mut self, value: T, arena: RawArena) {
    *self.add(arena) = value;
  }

  pub fn resize(&mut self, new_len: usize, arena: RawArena) {
    if new_len > self.cap {
      let cap = if new_len.is_power_of_two() {
        new_len
      } else {
        new_len.checked_next_power_of_two().unwrap()
      };

      self.grow(Some(cap), arena);
    }
    self.len = new_len;
  }

  pub fn grow(&mut self, new_cap: Option<usize>, arena: RawArena) {
    let old_cap = self.cap;
    self.cap = new_cap.unwrap_or(self.cap * 2);
    if self.cap < 16 {
      self.cap = 16;
    }

    let new_ptr = arena
      .alloc(Layout::array::<T>(self.cap).unwrap())
      .cast::<T>();

    unsafe {
      if let Some(old_ptr) = self.as_ptr() {
        new_ptr
          .as_ptr()
          .copy_from_nonoverlapping(old_ptr.as_ptr(), old_cap);
        new_ptr
          .as_ptr()
          .add(old_cap)
          .write_bytes(0, self.cap - old_cap);
      }
    }

    self.ptr = Some(new_ptr)
  }
}

impl AVec<Option<Opaque>> {
  pub fn resize_msg(
    &mut self,
    new_len: usize,
    arena: RawArena,
    layout: Layout,
  ) {
    if new_len > self.cap {
      let cap = if new_len.is_power_of_two() {
        new_len
      } else {
        new_len.checked_next_power_of_two().unwrap()
      };

      self.grow(Some(cap), arena);
    }

    while self.len < new_len {
      unsafe {
        let ptr = &mut *self.as_raw_ptr().add(self.len);
        if ptr.is_none() {
          *ptr = Some(arena.alloc(layout));
        }
        ptr.unwrap_unchecked().write_bytes(0, layout.size());
      }

      self.len += 1;
    }
  }
}
