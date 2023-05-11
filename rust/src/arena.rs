//! Arena support.

use std::alloc::Layout;
use std::mem;
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
  // This is intentionally the first value, so that in Hazzer we can specify
  // "clear the first eight bytes" to clear a repeated field without trashing
  // its internal pointer.
  len: usize,

  // This is a raw pointer, NOT a NonNull, because it is intended to be
  // initializeable by zeroing.
  ptr: *mut T,
  cap: usize,
}

impl<T> AVec<T> {
  pub const fn new() -> AVec<T> {
    Self {
      ptr: 0 as *mut T,
      cap: 0,
      len: 0,
    }
  }

  pub fn as_ptr(&self) -> *mut T {
    if self.ptr.is_null() {
      return mem::align_of::<T>() as *mut T;
    }

    self.ptr
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub unsafe fn set_len(&mut self, len: usize) {
    self.len = len
  }

  pub unsafe fn as_slice(&self) -> &[T] {
    slice::from_raw_parts(self.as_ptr(), self.len)
  }

  pub unsafe fn as_mut_slice(&mut self) -> &mut [T] {
    slice::from_raw_parts_mut(self.as_ptr(), self.len)
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

    let old_ptr = self.as_ptr();
    self.ptr = arena
      .alloc(Layout::array::<T>(self.cap).unwrap())
      .as_ptr()
      .cast::<T>();

    unsafe {
      self.ptr.copy_from_nonoverlapping(old_ptr, old_cap);
      self.ptr.add(old_cap).write_bytes(0, self.cap - old_cap);
    }
  }
}

impl AVec<*mut u8> {
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
        let ptr = &mut *self.ptr.add(self.len);
        if ptr.is_null() {
          *ptr = arena.alloc(layout).as_ptr();
        }
        ptr.write_bytes(0, layout.size());
      }

      self.len += 1;
    }
  }
}

pub struct Hazzer {
  // Positive indicates this is a hasbit index; negative indicates this is
  // the field number for a choice. If this is a repeated field, it should be
  // i32::MIN.
  pub hasbit_or_number: i32,
  // Offset of the field relative to the top of storage.
  pub offset: u32,
  // The number of bytes to memset to zero on init. If this is negative, instead
  // this value is behind a pointer and this is the size of the pointee (which
  // should also be memset to zero).
  pub size: i32,
}

impl Hazzer {
  pub unsafe fn has(&self, raw: *mut u8) -> bool {
    let n = self.hasbit_or_number;
    if n == i32::MIN {
      return true;
    }

    if n >= 0 {
      let word_idx = n / 32;
      let bit_idx = 1 << (n % 32);
      let word = raw.cast::<u32>().add(word_idx as usize).read();
      word & bit_idx != 0
    } else {
      raw.cast::<u32>().read() == (-n as u32)
    }
  }

  pub unsafe fn init(&self, raw: *mut u8, arena: RawArena) {
    let n = self.hasbit_or_number;
    if n == i32::MIN {
      return;
    }

    if n >= 0 {
      let word_idx = n / 32;
      let bit_idx = 1 << (n % 32);
      let word = &mut *raw.cast::<u32>().add(word_idx as usize);
      if *word & bit_idx != 0 {
        return;
      }
      *word |= bit_idx;
    } else {
      let number = -n as u32;
      let which = &mut *raw.cast::<u32>();
      if *which == number {
        return;
      }
      *which = number;
    }

    let ptr = raw.add(self.offset as usize);
    if self.size >= 0 {
      ptr.write_bytes(0, self.size as usize);
    } else {
      let ptr = &mut *ptr.cast::<*mut u8>();
      let layout = Layout::from_size_align_unchecked(-self.size as usize, 8);
      if ptr.is_null() {
        *ptr = arena.alloc(layout).as_ptr();
      }
      ptr.write_bytes(0, layout.size());
    }
  }

  pub unsafe fn clear(&self, raw: *mut u8) {
    let n = self.hasbit_or_number;
    if n == i32::MIN {
      return;
    }

    if n >= 0 {
      let word_idx = n / 32;
      let bit_idx = 1 << (n % 32);
      *raw.cast::<u32>().add(word_idx as usize) &= !bit_idx;
    } else {
      raw.cast::<u32>().write(0);
    }
  }
}
