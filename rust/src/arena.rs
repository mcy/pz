//! Arena support.

use std::alloc::Layout;
use std::ptr::NonNull;
use std::slice;

#[derive(Copy, Clone)]
pub struct RawArena {
  bump: *mut bumpalo::Bump,
}

impl RawArena {
  pub fn new() -> Self {
    Self {
      bump: Box::leak(Box::new(bumpalo::Bump::new())),
    }
  }

  pub fn alloc(&self, layout: Layout) -> NonNull<u8> {
    unsafe { &*self.bump }.alloc_layout(layout)
  }

  pub unsafe fn destroy(&self) {
    let _ = Box::from_raw(self.bump);
  }
}

pub struct ABox<T>(NonNull<T>);
impl<T> ABox<T> {
  pub const unsafe fn from_ptr(ptr: *mut u8) -> Self {
    Self(NonNull::new_unchecked(ptr.cast()))
  }

  pub const fn as_ptr(self) -> *mut u8 {
    self.0.as_ptr().cast()
  }

  pub const unsafe fn as_ref<'a>(self) -> &'a T {
    &*(self.0.as_ptr() as *const _)
  }

  pub unsafe fn as_mut<'a>(self) -> &'a mut T {
    &mut *self.0.as_ptr()
  }
}

impl<T> Clone for ABox<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T> Copy for ABox<T> {}

#[repr(C)]
pub struct AVec<T> {
  ptr: NonNull<T>,
  cap: usize,
  len: usize,
}

impl<T> AVec<T> {
  pub const fn new() -> AVec<T> {
    Self {
      ptr: NonNull::dangling(),
      cap: 0,
      len: 0,
    }
  }

  pub fn as_ptr(&self) -> *mut T {
    self.ptr.as_ptr()
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub unsafe fn set_len(&mut self, len: usize) {
    self.len = len
  }

  pub unsafe fn as_slice(&self) -> &[T] {
    slice::from_raw_parts(self.ptr.as_ptr(), self.len)
  }

  pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
    slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
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

  fn grow(&mut self, new_cap: Option<usize>, arena: RawArena) {
    let old_cap = self.cap;
    self.cap = new_cap.unwrap_or(self.cap * 2);
    if self.cap < 16 {
      self.cap = 16;
    }

    let old_ptr = self.ptr.as_ptr();
    self.ptr = arena.alloc(Layout::array::<T>(self.cap).unwrap()).cast();

    unsafe {
      self.ptr.as_ptr().copy_from_nonoverlapping(old_ptr, old_cap);
      self
        .ptr
        .as_ptr()
        .add(old_cap)
        .write_bytes(0, self.cap - old_cap);
    }
  }
}

impl AVec<*mut u8> {
  pub fn resize_msg(
    &mut self,
    new_len: usize,
    arena: RawArena,
    layout: Layout,
    clear: unsafe fn(*mut u8),
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
        let ptr = &mut *self.ptr.as_ptr().add(self.len);
        if ptr.is_null() {
          *ptr = arena.alloc(layout).as_ptr();
        }
        clear(*ptr);
      }

      self.len += 1;
    }
  }
}
