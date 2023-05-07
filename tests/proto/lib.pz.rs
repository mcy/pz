// ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]
extern crate pz as __rt;

/// message `pz.TestAll`
pub struct TestAll {
  ptr: __rt::rt::__z::ABox<__priv_TestAll::Storage>,
  arena: __rt::rt::__z::RawArena,
}

impl TestAll {
  pub const DEFAULT: __rt::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_TestAll::Storage = __priv_TestAll::Storage {
      __hasbits: [0; 1],
      opt_i32: 0,
      opt_i64: 0,
      opt_u32: 0,
      opt_u64: 0,
      opt_f32: 0,
      opt_f64: 0,
      opt_str: (0 as *mut u8, 0),
      opt_bool: false,
      opt_recursive: 0 as *mut u8,
      opt_nested: 0 as *mut u8,
      rep_i32: __rt::rt::__z::AVec::new(),
      rep_i64: __rt::rt::__z::AVec::new(),
      rep_u32: __rt::rt::__z::AVec::new(),
      rep_u64: __rt::rt::__z::AVec::new(),
      rep_f32: __rt::rt::__z::AVec::new(),
      rep_f64: __rt::rt::__z::AVec::new(),
      rep_str: __rt::rt::__z::AVec::new(),
      rep_bool: __rt::rt::__z::AVec::new(),
      rep_recursive: __rt::rt::__z::AVec::new(),
      rep_nested: __rt::rt::__z::AVec::new(),
    };
    __rt::rt::View::<Self> {
      ptr: __rt::rt::__z::ABox::from_ptr(&VALUE as *const __priv_TestAll::Storage as *mut __priv_TestAll::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = __rt::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: __rt::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, __rt::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), __rt::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> __rt::rt::View<Self> {
    __priv_TestAll::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::rt::Mut<Self> {
    __priv_TestAll::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { TestAll::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn opt_i32(&self) -> __rt::rt::View<'_, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(&self) -> Option<__rt::rt::View<'_, i32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().opt_i32) })
  }
  pub fn opt_i32_mut(&mut self) -> __rt::rt::Mut<'_, i32> {
    self.opt_i32_mut_or().into_mut()
  }
  pub fn opt_i32_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, i32> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_i32 as usize),
        self.arena,
        TestAll::__hazzer_opt_i32,
      )
    }
  }
  pub fn opt_i32_set(&mut self, value: i32) {
    self.opt_i32_mut().set(value);
  }

  pub fn opt_i64(&self) -> __rt::rt::View<'_, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(&self) -> Option<__rt::rt::View<'_, i64>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u64, i64>(self.ptr.as_ref().opt_i64) })
  }
  pub fn opt_i64_mut(&mut self) -> __rt::rt::Mut<'_, i64> {
    self.opt_i64_mut_or().into_mut()
  }
  pub fn opt_i64_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, i64> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_i64 as usize),
        self.arena,
        TestAll::__hazzer_opt_i64,
      )
    }
  }
  pub fn opt_i64_set(&mut self, value: i64) {
    self.opt_i64_mut().set(value);
  }

  pub fn opt_u32(&self) -> __rt::rt::View<'_, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(&self) -> Option<__rt::rt::View<'_, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().opt_u32) })
  }
  pub fn opt_u32_mut(&mut self) -> __rt::rt::Mut<'_, u32> {
    self.opt_u32_mut_or().into_mut()
  }
  pub fn opt_u32_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_u32 as usize),
        self.arena,
        TestAll::__hazzer_opt_u32,
      )
    }
  }
  pub fn opt_u32_set(&mut self, value: u32) {
    self.opt_u32_mut().set(value);
  }

  pub fn opt_u64(&self) -> __rt::rt::View<'_, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(&self) -> Option<__rt::rt::View<'_, u64>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u64, u64>(self.ptr.as_ref().opt_u64) })
  }
  pub fn opt_u64_mut(&mut self) -> __rt::rt::Mut<'_, u64> {
    self.opt_u64_mut_or().into_mut()
  }
  pub fn opt_u64_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, u64> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_u64 as usize),
        self.arena,
        TestAll::__hazzer_opt_u64,
      )
    }
  }
  pub fn opt_u64_set(&mut self, value: u64) {
    self.opt_u64_mut().set(value);
  }

  pub fn opt_f32(&self) -> __rt::rt::View<'_, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(&self) -> Option<__rt::rt::View<'_, f32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, f32>(self.ptr.as_ref().opt_f32) })
  }
  pub fn opt_f32_mut(&mut self) -> __rt::rt::Mut<'_, f32> {
    self.opt_f32_mut_or().into_mut()
  }
  pub fn opt_f32_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, f32> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_f32 as usize),
        self.arena,
        TestAll::__hazzer_opt_f32,
      )
    }
  }
  pub fn opt_f32_set(&mut self, value: f32) {
    self.opt_f32_mut().set(value);
  }

  pub fn opt_f64(&self) -> __rt::rt::View<'_, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(&self) -> Option<__rt::rt::View<'_, f64>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u64, f64>(self.ptr.as_ref().opt_f64) })
  }
  pub fn opt_f64_mut(&mut self) -> __rt::rt::Mut<'_, f64> {
    self.opt_f64_mut_or().into_mut()
  }
  pub fn opt_f64_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, f64> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_f64 as usize),
        self.arena,
        TestAll::__hazzer_opt_f64,
      )
    }
  }
  pub fn opt_f64_set(&mut self, value: f64) {
    self.opt_f64_mut().set(value);
  }

  pub fn opt_str(&self) -> __rt::rt::View<'_, __rt::rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(&self) -> Option<__rt::rt::View<'_, __rt::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 64 == 0 { return None }
    Some(unsafe {
      let (ptr, len) = self.ptr.as_ref().opt_str;
      __rt::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn opt_str_mut(&mut self) -> __rt::rt::Mut<'_, __rt::rt::Str> {
    self.opt_str_mut_or().into_mut()
  }
  pub fn opt_str_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, __rt::rt::Str> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_str as usize),
        self.arena,
        TestAll::__hazzer_opt_str,
      )
    }
  }
  pub fn opt_str_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.opt_str_mut().set(value);
  }

  pub fn opt_bool(&self) -> __rt::rt::View<'_, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(&self) -> Option<__rt::rt::View<'_, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 128 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().opt_bool) })
  }
  pub fn opt_bool_mut(&mut self) -> __rt::rt::Mut<'_, bool> {
    self.opt_bool_mut_or().into_mut()
  }
  pub fn opt_bool_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, bool> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_bool as usize),
        self.arena,
        TestAll::__hazzer_opt_bool,
      )
    }
  }
  pub fn opt_bool_set(&mut self, value: bool) {
    self.opt_bool_mut().set(value);
  }

  pub fn opt_recursive(&self) -> __rt::rt::View<'_, TestAll> {
    self.opt_recursive_or().unwrap_or(TestAll::DEFAULT)
  }
  pub fn opt_recursive_or(&self) -> Option<__rt::rt::View<'_, TestAll>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 256 == 0 { return None }
    Some(__rt::rt::View::<TestAll> {
      ptr: unsafe { __rt::rt::__z::ABox::from_ptr(self.ptr.as_ref().opt_recursive) },
      _ph: std::marker::PhantomData,
    })
  }
  pub fn opt_recursive_mut(&mut self) -> __rt::rt::Mut<'_, TestAll> {
    self.opt_recursive_mut_or().into_mut()
  }
  pub fn opt_recursive_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, TestAll> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_recursive as usize),
        self.arena,
        TestAll::__hazzer_opt_recursive,
      )
    }
  }

  pub fn opt_nested(&self) -> __rt::rt::View<'_, TestAll_Nested> {
    self.opt_nested_or().unwrap_or(TestAll_Nested::DEFAULT)
  }
  pub fn opt_nested_or(&self) -> Option<__rt::rt::View<'_, TestAll_Nested>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 512 == 0 { return None }
    Some(__rt::rt::View::<TestAll_Nested> {
      ptr: unsafe { __rt::rt::__z::ABox::from_ptr(self.ptr.as_ref().opt_nested) },
      _ph: std::marker::PhantomData,
    })
  }
  pub fn opt_nested_mut(&mut self) -> __rt::rt::Mut<'_, TestAll_Nested> {
    self.opt_nested_mut_or().into_mut()
  }
  pub fn opt_nested_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, TestAll_Nested> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_nested as usize),
        self.arena,
        TestAll::__hazzer_opt_nested,
      )
    }
  }

  pub fn rep_i32(&self) -> &'_ [i32] {
    unsafe {
      let slice = self.ptr.as_ref().rep_i32.as_slice();
      std::mem::transmute::<&'_ [u32], &'_ [i32]>(slice)
    }
  }
  pub fn rep_i32_mut(&mut self) -> &'_ mut [i32] {
    unsafe {
      let slice = self.ptr.as_mut().rep_i32.as_mut_slice();
      std::mem::transmute::<&'_ mut [u32], &'_ mut [i32]>(slice)
    }
  }
  pub fn rep_i32_set(&mut self, that: &[i32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_i32;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_i32_extend(&mut self, that: &[i32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_i32;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_i64(&self) -> &'_ [i64] {
    unsafe {
      let slice = self.ptr.as_ref().rep_i64.as_slice();
      std::mem::transmute::<&'_ [u64], &'_ [i64]>(slice)
    }
  }
  pub fn rep_i64_mut(&mut self) -> &'_ mut [i64] {
    unsafe {
      let slice = self.ptr.as_mut().rep_i64.as_mut_slice();
      std::mem::transmute::<&'_ mut [u64], &'_ mut [i64]>(slice)
    }
  }
  pub fn rep_i64_set(&mut self, that: &[i64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_i64;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_i64_extend(&mut self, that: &[i64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_i64;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_u32(&self) -> &'_ [u32] {
    unsafe {
      let slice = self.ptr.as_ref().rep_u32.as_slice();
      std::mem::transmute::<&'_ [u32], &'_ [u32]>(slice)
    }
  }
  pub fn rep_u32_mut(&mut self) -> &'_ mut [u32] {
    unsafe {
      let slice = self.ptr.as_mut().rep_u32.as_mut_slice();
      std::mem::transmute::<&'_ mut [u32], &'_ mut [u32]>(slice)
    }
  }
  pub fn rep_u32_set(&mut self, that: &[u32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_u32;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_u32_extend(&mut self, that: &[u32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_u32;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_u64(&self) -> &'_ [u64] {
    unsafe {
      let slice = self.ptr.as_ref().rep_u64.as_slice();
      std::mem::transmute::<&'_ [u64], &'_ [u64]>(slice)
    }
  }
  pub fn rep_u64_mut(&mut self) -> &'_ mut [u64] {
    unsafe {
      let slice = self.ptr.as_mut().rep_u64.as_mut_slice();
      std::mem::transmute::<&'_ mut [u64], &'_ mut [u64]>(slice)
    }
  }
  pub fn rep_u64_set(&mut self, that: &[u64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_u64;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_u64_extend(&mut self, that: &[u64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_u64;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_f32(&self) -> &'_ [f32] {
    unsafe {
      let slice = self.ptr.as_ref().rep_f32.as_slice();
      std::mem::transmute::<&'_ [u32], &'_ [f32]>(slice)
    }
  }
  pub fn rep_f32_mut(&mut self) -> &'_ mut [f32] {
    unsafe {
      let slice = self.ptr.as_mut().rep_f32.as_mut_slice();
      std::mem::transmute::<&'_ mut [u32], &'_ mut [f32]>(slice)
    }
  }
  pub fn rep_f32_set(&mut self, that: &[f32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_f32;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_f32_extend(&mut self, that: &[f32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_f32;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_f64(&self) -> &'_ [f64] {
    unsafe {
      let slice = self.ptr.as_ref().rep_f64.as_slice();
      std::mem::transmute::<&'_ [u64], &'_ [f64]>(slice)
    }
  }
  pub fn rep_f64_mut(&mut self) -> &'_ mut [f64] {
    unsafe {
      let slice = self.ptr.as_mut().rep_f64.as_mut_slice();
      std::mem::transmute::<&'_ mut [u64], &'_ mut [f64]>(slice)
    }
  }
  pub fn rep_f64_set(&mut self, that: &[f64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_f64;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_f64_extend(&mut self, that: &[f64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_f64;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_str_len(&self) -> usize {
    unsafe { self.ptr.as_ref() }.rep_str.len()
  }
  pub fn rep_str(&self, n: usize) -> Option<&'_ __rt::rt::Str> {
    unsafe { self.ptr.as_ref().rep_str.as_slice() }.get(n).map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn rep_str_iter(&self) -> impl Iterator<Item = &'_ __rt::rt::Str> + '_ {
    unsafe { self.ptr.as_ref().rep_str.as_slice() }.iter().map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn rep_str_mut(&mut self, n: usize) -> Option<__rt::rt::StrBuf<'_>> {
    unsafe { self.ptr.as_mut().rep_str.as_mut_slice() }.get_mut(n)
      .map(|data| __rt::rt::StrBuf::__wrap(data, self.arena))
  }
  pub fn rep_str_add(&mut self) -> __rt::rt::StrBuf<'_> {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_str;
      let new_len = vec.len() + 1;
      vec.resize(new_len, self.arena);
      self.rep_str_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn rep_str_resize(&mut self, n: usize) {
    unsafe {
      self.ptr.as_mut().rep_str.resize(n, self.arena);
    }
  }

  pub fn rep_bool(&self) -> &'_ [bool] {
    unsafe {
      let slice = self.ptr.as_ref().rep_bool.as_slice();
      std::mem::transmute::<&'_ [bool], &'_ [bool]>(slice)
    }
  }
  pub fn rep_bool_mut(&mut self) -> &'_ mut [bool] {
    unsafe {
      let slice = self.ptr.as_mut().rep_bool.as_mut_slice();
      std::mem::transmute::<&'_ mut [bool], &'_ mut [bool]>(slice)
    }
  }
  pub fn rep_bool_set(&mut self, that: &[bool]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_bool;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_bool_extend(&mut self, that: &[bool]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_bool;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_recursive_len(&self) -> usize {
    unsafe { self.ptr.as_ref() }.rep_recursive.len()
  }
  pub fn rep_recursive(&self, n: usize) -> Option<__rt::rt::View<'_, TestAll>> {
    unsafe { self.ptr.as_ref().rep_recursive.as_slice() }.get(n)
      .map(|&ptr| __rt::rt::View::<TestAll> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn rep_recursive_iter(&self) -> impl Iterator<Item = __rt::rt::View<'_, TestAll>> + '_ {
    unsafe { self.ptr.as_ref().rep_recursive.as_slice() }.iter()
      .map(|&ptr| __rt::rt::View::<TestAll> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn rep_recursive_mut(&mut self, n: usize) -> Option<__rt::rt::Mut<'_, TestAll>> {
    unsafe { self.ptr.as_mut().rep_recursive.as_mut_slice() }.get_mut(n)
      .map(|&mut ptr| __rt::rt::Mut::<TestAll> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
        arena: self.arena,
      })
  }
  pub fn rep_recursive_add(&mut self) -> __rt::rt::Mut<'_, TestAll> {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_recursive;
      let new_len = vec.len() + 1;
      vec.resize_msg(new_len, self.arena,
        TestAll::__LAYOUT, TestAll::__raw_init);
      self.rep_recursive_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn rep_recursive_resize(&mut self, n: usize) {
    unsafe {
      self.ptr.as_mut().rep_recursive.resize_msg(
        n, self.arena,
        TestAll::__LAYOUT, TestAll::__raw_init);
    }
  }

  pub fn rep_nested_len(&self) -> usize {
    unsafe { self.ptr.as_ref() }.rep_nested.len()
  }
  pub fn rep_nested(&self, n: usize) -> Option<__rt::rt::View<'_, TestAll_Nested>> {
    unsafe { self.ptr.as_ref().rep_nested.as_slice() }.get(n)
      .map(|&ptr| __rt::rt::View::<TestAll_Nested> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn rep_nested_iter(&self) -> impl Iterator<Item = __rt::rt::View<'_, TestAll_Nested>> + '_ {
    unsafe { self.ptr.as_ref().rep_nested.as_slice() }.iter()
      .map(|&ptr| __rt::rt::View::<TestAll_Nested> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn rep_nested_mut(&mut self, n: usize) -> Option<__rt::rt::Mut<'_, TestAll_Nested>> {
    unsafe { self.ptr.as_mut().rep_nested.as_mut_slice() }.get_mut(n)
      .map(|&mut ptr| __rt::rt::Mut::<TestAll_Nested> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
        arena: self.arena,
      })
  }
  pub fn rep_nested_add(&mut self) -> __rt::rt::Mut<'_, TestAll_Nested> {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_nested;
      let new_len = vec.len() + 1;
      vec.resize_msg(new_len, self.arena,
        TestAll_Nested::__LAYOUT, TestAll_Nested::__raw_init);
      self.rep_nested_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn rep_nested_resize(&mut self, n: usize) {
    unsafe {
      self.ptr.as_mut().rep_nested.resize_msg(
        n, self.arena,
        TestAll_Nested::__LAYOUT, TestAll_Nested::__raw_init);
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_TestAll::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_TestAll::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_TestAll::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const __rt::rt::__z::tdp::Message {
    &__priv_TestAll::TDP_INFO as *const _ as *const __rt::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_opt_i32(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll::FIELD_OFFSET_opt_i32 as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 1 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !1,
      Some(true) => {
        *word |= 1;
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_opt_i64(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll::FIELD_OFFSET_opt_i64 as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 2 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !2,
      Some(true) => {
        *word |= 2;
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_opt_u32(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll::FIELD_OFFSET_opt_u32 as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 4 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !4,
      Some(true) => {
        *word |= 4;
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_opt_u64(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll::FIELD_OFFSET_opt_u64 as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 8 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !8,
      Some(true) => {
        *word |= 8;
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_opt_f32(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll::FIELD_OFFSET_opt_f32 as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 16 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !16,
      Some(true) => {
        *word |= 16;
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_opt_f64(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll::FIELD_OFFSET_opt_f64 as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 32 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !32,
      Some(true) => {
        *word |= 32;
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_opt_str(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll::FIELD_OFFSET_opt_str as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 64 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !64,
      Some(true) => {
        *word |= 64;
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_opt_bool(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll::FIELD_OFFSET_opt_bool as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 128 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !128,
      Some(true) => {
        *word |= 128;
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_opt_recursive(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll::FIELD_OFFSET_opt_recursive as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 256 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !256,
      Some(true) => {
        *word |= 256;
        let storage = &mut *raw.cast::<__priv_TestAll::Storage>();
        if storage.opt_recursive.is_null() {
          storage.opt_recursive = self.arena.alloc(TestAll::__LAYOUT).as_ptr();
          TestAll::__raw_init(storage.opt_recursive);
        }
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_opt_nested(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll::FIELD_OFFSET_opt_nested as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 512 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !512,
      Some(true) => {
        *word |= 512;
        let storage = &mut *raw.cast::<__priv_TestAll::Storage>();
        if storage.opt_nested.is_null() {
          storage.opt_nested = self.arena.alloc(TestAll_Nested::__LAYOUT).as_ptr();
          TestAll_Nested::__raw_init(storage.opt_nested);
        }
      }
    }
    has
  }
}

impl Default for TestAll {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::rt::ptr::Proxied for TestAll {
  type View<'msg> = __priv_TestAll::View<'msg>;
  type Mut<'msg> = __priv_TestAll::Mut<'msg>;
}

impl __rt::rt::value::Type for TestAll {
  unsafe fn __make_view<'a>(ptr: *mut u8) -> __rt::rt::View<'a, Self> {
    __priv_TestAll::View {
      ptr: __rt::rt::__z::ABox::from_ptr(ptr),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: __rt::rt::__z::RawArena) -> __rt::rt::Mut<'a, Self> {
    __priv_TestAll::Mut {
      ptr: __rt::rt::__z::ABox::from_ptr(ptr),
      arena,
      _ph: std::marker::PhantomData,
    }
  }
}

impl<'msg> __priv_TestAll::View<'msg> {
  pub fn opt_i32(self) -> __rt::rt::View<'msg, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> Option<__rt::rt::View<'msg, i32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().opt_i32) })
  }

  pub fn opt_i64(self) -> __rt::rt::View<'msg, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> Option<__rt::rt::View<'msg, i64>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u64, i64>(self.ptr.as_ref().opt_i64) })
  }

  pub fn opt_u32(self) -> __rt::rt::View<'msg, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> Option<__rt::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().opt_u32) })
  }

  pub fn opt_u64(self) -> __rt::rt::View<'msg, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> Option<__rt::rt::View<'msg, u64>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u64, u64>(self.ptr.as_ref().opt_u64) })
  }

  pub fn opt_f32(self) -> __rt::rt::View<'msg, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> Option<__rt::rt::View<'msg, f32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, f32>(self.ptr.as_ref().opt_f32) })
  }

  pub fn opt_f64(self) -> __rt::rt::View<'msg, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> Option<__rt::rt::View<'msg, f64>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u64, f64>(self.ptr.as_ref().opt_f64) })
  }

  pub fn opt_str(self) -> __rt::rt::View<'msg, __rt::rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> Option<__rt::rt::View<'msg, __rt::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 64 == 0 { return None }
    Some(unsafe {
      let (ptr, len) = self.ptr.as_ref().opt_str;
      __rt::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn opt_bool(self) -> __rt::rt::View<'msg, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> Option<__rt::rt::View<'msg, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 128 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().opt_bool) })
  }

  pub fn opt_recursive(self) -> __rt::rt::View<'msg, TestAll> {
    self.opt_recursive_or().unwrap_or(TestAll::DEFAULT)
  }
  pub fn opt_recursive_or(self) -> Option<__rt::rt::View<'msg, TestAll>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 256 == 0 { return None }
    Some(__rt::rt::View::<TestAll> {
      ptr: unsafe { __rt::rt::__z::ABox::from_ptr(self.ptr.as_ref().opt_recursive) },
      _ph: std::marker::PhantomData,
    })
  }

  pub fn opt_nested(self) -> __rt::rt::View<'msg, TestAll_Nested> {
    self.opt_nested_or().unwrap_or(TestAll_Nested::DEFAULT)
  }
  pub fn opt_nested_or(self) -> Option<__rt::rt::View<'msg, TestAll_Nested>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 512 == 0 { return None }
    Some(__rt::rt::View::<TestAll_Nested> {
      ptr: unsafe { __rt::rt::__z::ABox::from_ptr(self.ptr.as_ref().opt_nested) },
      _ph: std::marker::PhantomData,
    })
  }

  pub fn rep_i32(self) -> &'msg [i32] {
    unsafe {
      let slice = self.ptr.as_ref().rep_i32.as_slice();
      std::mem::transmute::<&'msg [u32], &'msg [i32]>(slice)
    }
  }

  pub fn rep_i64(self) -> &'msg [i64] {
    unsafe {
      let slice = self.ptr.as_ref().rep_i64.as_slice();
      std::mem::transmute::<&'msg [u64], &'msg [i64]>(slice)
    }
  }

  pub fn rep_u32(self) -> &'msg [u32] {
    unsafe {
      let slice = self.ptr.as_ref().rep_u32.as_slice();
      std::mem::transmute::<&'msg [u32], &'msg [u32]>(slice)
    }
  }

  pub fn rep_u64(self) -> &'msg [u64] {
    unsafe {
      let slice = self.ptr.as_ref().rep_u64.as_slice();
      std::mem::transmute::<&'msg [u64], &'msg [u64]>(slice)
    }
  }

  pub fn rep_f32(self) -> &'msg [f32] {
    unsafe {
      let slice = self.ptr.as_ref().rep_f32.as_slice();
      std::mem::transmute::<&'msg [u32], &'msg [f32]>(slice)
    }
  }

  pub fn rep_f64(self) -> &'msg [f64] {
    unsafe {
      let slice = self.ptr.as_ref().rep_f64.as_slice();
      std::mem::transmute::<&'msg [u64], &'msg [f64]>(slice)
    }
  }

  pub fn rep_str_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.rep_str.len()
  }
  pub fn rep_str(self, n: usize) -> Option<&'msg __rt::rt::Str> {
    unsafe { self.ptr.as_ref().rep_str.as_slice() }.get(n).map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn rep_str_iter(self) -> impl Iterator<Item = &'msg __rt::rt::Str> + 'msg {
    unsafe { self.ptr.as_ref().rep_str.as_slice() }.iter().map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }

  pub fn rep_bool(self) -> &'msg [bool] {
    unsafe {
      let slice = self.ptr.as_ref().rep_bool.as_slice();
      std::mem::transmute::<&'msg [bool], &'msg [bool]>(slice)
    }
  }

  pub fn rep_recursive_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.rep_recursive.len()
  }
  pub fn rep_recursive(self, n: usize) -> Option<__rt::rt::View<'msg, TestAll>> {
    unsafe { self.ptr.as_ref().rep_recursive.as_slice() }.get(n)
      .map(|&ptr| __rt::rt::View::<TestAll> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn rep_recursive_iter(self) -> impl Iterator<Item = __rt::rt::View<'msg, TestAll>> + 'msg {
    unsafe { self.ptr.as_ref().rep_recursive.as_slice() }.iter()
      .map(|&ptr| __rt::rt::View::<TestAll> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }

  pub fn rep_nested_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.rep_nested.len()
  }
  pub fn rep_nested(self, n: usize) -> Option<__rt::rt::View<'msg, TestAll_Nested>> {
    unsafe { self.ptr.as_ref().rep_nested.as_slice() }.get(n)
      .map(|&ptr| __rt::rt::View::<TestAll_Nested> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn rep_nested_iter(self) -> impl Iterator<Item = __rt::rt::View<'msg, TestAll_Nested>> + 'msg {
    unsafe { self.ptr.as_ref().rep_nested.as_slice() }.iter()
      .map(|&ptr| __rt::rt::View::<TestAll_Nested> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __rt::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.opt_i32_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_i32")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.opt_i64_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_i64")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.opt_u32_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_u32")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.opt_u64_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_u64")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.opt_f32_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_f32")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.opt_f64_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_f64")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.opt_str_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_str")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.opt_bool_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_bool")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.opt_recursive_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_recursive")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let Some(value) = self.opt_nested_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_nested")?;
      value.__debug(debug)?;
      count += 1;
    }
    let slice = self.rep_i32();
    if !slice.is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_i32")?;
      debug.iter(slice)?;
      count += 1;
    }
    let slice = self.rep_i64();
    if !slice.is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_i64")?;
      debug.iter(slice)?;
      count += 1;
    }
    let slice = self.rep_u32();
    if !slice.is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_u32")?;
      debug.iter(slice)?;
      count += 1;
    }
    let slice = self.rep_u64();
    if !slice.is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_u64")?;
      debug.iter(slice)?;
      count += 1;
    }
    let slice = self.rep_f32();
    if !slice.is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_f32")?;
      debug.iter(slice)?;
      count += 1;
    }
    let slice = self.rep_f64();
    if !slice.is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_f64")?;
      debug.iter(slice)?;
      count += 1;
    }
    if self.rep_str_len() != 0 {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_str")?;
      debug.iter(self.rep_str_iter())?;
      count += 1;
    }
    let slice = self.rep_bool();
    if !slice.is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_bool")?;
      debug.iter(slice)?;
      count += 1;
    }
    for value in self.rep_recursive_iter() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_recursive")?;
      value.__debug(debug)?;
      count += 1;
    }
    for value in self.rep_nested_iter() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_nested")?;
      value.__debug(debug)?;
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_TestAll::View<'_> {
  fn default() -> Self {
    TestAll::DEFAULT
  }
}

impl<'msg> __priv_TestAll::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { TestAll::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), __rt::rt::Error> {
    let mut ctx = __rt::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, TestAll::__tdp_info())
  }

  pub fn opt_i32(self) -> __rt::rt::View<'msg, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> Option<__rt::rt::View<'msg, i32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().opt_i32) })
  }
  pub fn opt_i32_mut(self) -> __rt::rt::Mut<'msg, i32> {
    self.opt_i32_mut_or().into_mut()
  }
  pub fn opt_i32_mut_or(self) -> __rt::rt::value::OptMut<'msg, i32> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_i32 as usize),
        self.arena,
        TestAll::__hazzer_opt_i32,
      )
    }
  }
  pub fn opt_i32_set(self, value: i32) {
    self.opt_i32_mut().set(value);
  }

  pub fn opt_i64(self) -> __rt::rt::View<'msg, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> Option<__rt::rt::View<'msg, i64>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u64, i64>(self.ptr.as_ref().opt_i64) })
  }
  pub fn opt_i64_mut(self) -> __rt::rt::Mut<'msg, i64> {
    self.opt_i64_mut_or().into_mut()
  }
  pub fn opt_i64_mut_or(self) -> __rt::rt::value::OptMut<'msg, i64> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_i64 as usize),
        self.arena,
        TestAll::__hazzer_opt_i64,
      )
    }
  }
  pub fn opt_i64_set(self, value: i64) {
    self.opt_i64_mut().set(value);
  }

  pub fn opt_u32(self) -> __rt::rt::View<'msg, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> Option<__rt::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().opt_u32) })
  }
  pub fn opt_u32_mut(self) -> __rt::rt::Mut<'msg, u32> {
    self.opt_u32_mut_or().into_mut()
  }
  pub fn opt_u32_mut_or(self) -> __rt::rt::value::OptMut<'msg, u32> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_u32 as usize),
        self.arena,
        TestAll::__hazzer_opt_u32,
      )
    }
  }
  pub fn opt_u32_set(self, value: u32) {
    self.opt_u32_mut().set(value);
  }

  pub fn opt_u64(self) -> __rt::rt::View<'msg, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> Option<__rt::rt::View<'msg, u64>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u64, u64>(self.ptr.as_ref().opt_u64) })
  }
  pub fn opt_u64_mut(self) -> __rt::rt::Mut<'msg, u64> {
    self.opt_u64_mut_or().into_mut()
  }
  pub fn opt_u64_mut_or(self) -> __rt::rt::value::OptMut<'msg, u64> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_u64 as usize),
        self.arena,
        TestAll::__hazzer_opt_u64,
      )
    }
  }
  pub fn opt_u64_set(self, value: u64) {
    self.opt_u64_mut().set(value);
  }

  pub fn opt_f32(self) -> __rt::rt::View<'msg, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> Option<__rt::rt::View<'msg, f32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, f32>(self.ptr.as_ref().opt_f32) })
  }
  pub fn opt_f32_mut(self) -> __rt::rt::Mut<'msg, f32> {
    self.opt_f32_mut_or().into_mut()
  }
  pub fn opt_f32_mut_or(self) -> __rt::rt::value::OptMut<'msg, f32> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_f32 as usize),
        self.arena,
        TestAll::__hazzer_opt_f32,
      )
    }
  }
  pub fn opt_f32_set(self, value: f32) {
    self.opt_f32_mut().set(value);
  }

  pub fn opt_f64(self) -> __rt::rt::View<'msg, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> Option<__rt::rt::View<'msg, f64>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u64, f64>(self.ptr.as_ref().opt_f64) })
  }
  pub fn opt_f64_mut(self) -> __rt::rt::Mut<'msg, f64> {
    self.opt_f64_mut_or().into_mut()
  }
  pub fn opt_f64_mut_or(self) -> __rt::rt::value::OptMut<'msg, f64> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_f64 as usize),
        self.arena,
        TestAll::__hazzer_opt_f64,
      )
    }
  }
  pub fn opt_f64_set(self, value: f64) {
    self.opt_f64_mut().set(value);
  }

  pub fn opt_str(self) -> __rt::rt::View<'msg, __rt::rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> Option<__rt::rt::View<'msg, __rt::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 64 == 0 { return None }
    Some(unsafe {
      let (ptr, len) = self.ptr.as_ref().opt_str;
      __rt::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn opt_str_mut(self) -> __rt::rt::Mut<'msg, __rt::rt::Str> {
    self.opt_str_mut_or().into_mut()
  }
  pub fn opt_str_mut_or(self) -> __rt::rt::value::OptMut<'msg, __rt::rt::Str> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_str as usize),
        self.arena,
        TestAll::__hazzer_opt_str,
      )
    }
  }
  pub fn opt_str_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.opt_str_mut().set(value);
  }

  pub fn opt_bool(self) -> __rt::rt::View<'msg, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> Option<__rt::rt::View<'msg, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 128 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().opt_bool) })
  }
  pub fn opt_bool_mut(self) -> __rt::rt::Mut<'msg, bool> {
    self.opt_bool_mut_or().into_mut()
  }
  pub fn opt_bool_mut_or(self) -> __rt::rt::value::OptMut<'msg, bool> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_bool as usize),
        self.arena,
        TestAll::__hazzer_opt_bool,
      )
    }
  }
  pub fn opt_bool_set(self, value: bool) {
    self.opt_bool_mut().set(value);
  }

  pub fn opt_recursive(self) -> __rt::rt::View<'msg, TestAll> {
    self.opt_recursive_or().unwrap_or(TestAll::DEFAULT)
  }
  pub fn opt_recursive_or(self) -> Option<__rt::rt::View<'msg, TestAll>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 256 == 0 { return None }
    Some(__rt::rt::View::<TestAll> {
      ptr: unsafe { __rt::rt::__z::ABox::from_ptr(self.ptr.as_ref().opt_recursive) },
      _ph: std::marker::PhantomData,
    })
  }
  pub fn opt_recursive_mut(self) -> __rt::rt::Mut<'msg, TestAll> {
    self.opt_recursive_mut_or().into_mut()
  }
  pub fn opt_recursive_mut_or(self) -> __rt::rt::value::OptMut<'msg, TestAll> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_recursive as usize),
        self.arena,
        TestAll::__hazzer_opt_recursive,
      )
    }
  }

  pub fn opt_nested(self) -> __rt::rt::View<'msg, TestAll_Nested> {
    self.opt_nested_or().unwrap_or(TestAll_Nested::DEFAULT)
  }
  pub fn opt_nested_or(self) -> Option<__rt::rt::View<'msg, TestAll_Nested>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 512 == 0 { return None }
    Some(__rt::rt::View::<TestAll_Nested> {
      ptr: unsafe { __rt::rt::__z::ABox::from_ptr(self.ptr.as_ref().opt_nested) },
      _ph: std::marker::PhantomData,
    })
  }
  pub fn opt_nested_mut(self) -> __rt::rt::Mut<'msg, TestAll_Nested> {
    self.opt_nested_mut_or().into_mut()
  }
  pub fn opt_nested_mut_or(self) -> __rt::rt::value::OptMut<'msg, TestAll_Nested> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll::FIELD_OFFSET_opt_nested as usize),
        self.arena,
        TestAll::__hazzer_opt_nested,
      )
    }
  }

  pub fn rep_i32(self) -> &'msg [i32] {
    unsafe {
      let slice = self.ptr.as_ref().rep_i32.as_slice();
      std::mem::transmute::<&'msg [u32], &'msg [i32]>(slice)
    }
  }
  pub fn rep_i32_mut(self) -> &'msg mut [i32] {
    unsafe {
      let slice = self.ptr.as_mut().rep_i32.as_mut_slice();
      std::mem::transmute::<&'msg mut [u32], &'msg mut [i32]>(slice)
    }
  }
  pub fn rep_i32_set(self, that: &[i32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_i32;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_i32_extend(self, that: &[i32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_i32;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_i64(self) -> &'msg [i64] {
    unsafe {
      let slice = self.ptr.as_ref().rep_i64.as_slice();
      std::mem::transmute::<&'msg [u64], &'msg [i64]>(slice)
    }
  }
  pub fn rep_i64_mut(self) -> &'msg mut [i64] {
    unsafe {
      let slice = self.ptr.as_mut().rep_i64.as_mut_slice();
      std::mem::transmute::<&'msg mut [u64], &'msg mut [i64]>(slice)
    }
  }
  pub fn rep_i64_set(self, that: &[i64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_i64;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_i64_extend(self, that: &[i64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_i64;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_u32(self) -> &'msg [u32] {
    unsafe {
      let slice = self.ptr.as_ref().rep_u32.as_slice();
      std::mem::transmute::<&'msg [u32], &'msg [u32]>(slice)
    }
  }
  pub fn rep_u32_mut(self) -> &'msg mut [u32] {
    unsafe {
      let slice = self.ptr.as_mut().rep_u32.as_mut_slice();
      std::mem::transmute::<&'msg mut [u32], &'msg mut [u32]>(slice)
    }
  }
  pub fn rep_u32_set(self, that: &[u32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_u32;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_u32_extend(self, that: &[u32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_u32;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_u64(self) -> &'msg [u64] {
    unsafe {
      let slice = self.ptr.as_ref().rep_u64.as_slice();
      std::mem::transmute::<&'msg [u64], &'msg [u64]>(slice)
    }
  }
  pub fn rep_u64_mut(self) -> &'msg mut [u64] {
    unsafe {
      let slice = self.ptr.as_mut().rep_u64.as_mut_slice();
      std::mem::transmute::<&'msg mut [u64], &'msg mut [u64]>(slice)
    }
  }
  pub fn rep_u64_set(self, that: &[u64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_u64;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_u64_extend(self, that: &[u64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_u64;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_f32(self) -> &'msg [f32] {
    unsafe {
      let slice = self.ptr.as_ref().rep_f32.as_slice();
      std::mem::transmute::<&'msg [u32], &'msg [f32]>(slice)
    }
  }
  pub fn rep_f32_mut(self) -> &'msg mut [f32] {
    unsafe {
      let slice = self.ptr.as_mut().rep_f32.as_mut_slice();
      std::mem::transmute::<&'msg mut [u32], &'msg mut [f32]>(slice)
    }
  }
  pub fn rep_f32_set(self, that: &[f32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_f32;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_f32_extend(self, that: &[f32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_f32;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_f64(self) -> &'msg [f64] {
    unsafe {
      let slice = self.ptr.as_ref().rep_f64.as_slice();
      std::mem::transmute::<&'msg [u64], &'msg [f64]>(slice)
    }
  }
  pub fn rep_f64_mut(self) -> &'msg mut [f64] {
    unsafe {
      let slice = self.ptr.as_mut().rep_f64.as_mut_slice();
      std::mem::transmute::<&'msg mut [u64], &'msg mut [f64]>(slice)
    }
  }
  pub fn rep_f64_set(self, that: &[f64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_f64;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_f64_extend(self, that: &[f64]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_f64;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_str_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.rep_str.len()
  }
  pub fn rep_str(self, n: usize) -> Option<&'msg __rt::rt::Str> {
    unsafe { self.ptr.as_ref().rep_str.as_slice() }.get(n).map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn rep_str_iter(self) -> impl Iterator<Item = &'msg __rt::rt::Str> + 'msg {
    unsafe { self.ptr.as_ref().rep_str.as_slice() }.iter().map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn rep_str_mut(self, n: usize) -> Option<__rt::rt::StrBuf<'msg>> {
    unsafe { self.ptr.as_mut().rep_str.as_mut_slice() }.get_mut(n)
      .map(|data| __rt::rt::StrBuf::__wrap(data, self.arena))
  }
  pub fn rep_str_add(self) -> __rt::rt::StrBuf<'msg> {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_str;
      let new_len = vec.len() + 1;
      vec.resize(new_len, self.arena);
      self.rep_str_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn rep_str_resize(self, n: usize) {
    unsafe {
      self.ptr.as_mut().rep_str.resize(n, self.arena);
    }
  }

  pub fn rep_bool(self) -> &'msg [bool] {
    unsafe {
      let slice = self.ptr.as_ref().rep_bool.as_slice();
      std::mem::transmute::<&'msg [bool], &'msg [bool]>(slice)
    }
  }
  pub fn rep_bool_mut(self) -> &'msg mut [bool] {
    unsafe {
      let slice = self.ptr.as_mut().rep_bool.as_mut_slice();
      std::mem::transmute::<&'msg mut [bool], &'msg mut [bool]>(slice)
    }
  }
  pub fn rep_bool_set(self, that: &[bool]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_bool;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn rep_bool_extend(self, that: &[bool]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_bool;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn rep_recursive_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.rep_recursive.len()
  }
  pub fn rep_recursive(self, n: usize) -> Option<__rt::rt::View<'msg, TestAll>> {
    unsafe { self.ptr.as_ref().rep_recursive.as_slice() }.get(n)
      .map(|&ptr| __rt::rt::View::<TestAll> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn rep_recursive_iter(self) -> impl Iterator<Item = __rt::rt::View<'msg, TestAll>> + 'msg {
    unsafe { self.ptr.as_ref().rep_recursive.as_slice() }.iter()
      .map(|&ptr| __rt::rt::View::<TestAll> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn rep_recursive_mut(self, n: usize) -> Option<__rt::rt::Mut<'msg, TestAll>> {
    unsafe { self.ptr.as_mut().rep_recursive.as_mut_slice() }.get_mut(n)
      .map(|&mut ptr| __rt::rt::Mut::<TestAll> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
        arena: self.arena,
      })
  }
  pub fn rep_recursive_add(self) -> __rt::rt::Mut<'msg, TestAll> {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_recursive;
      let new_len = vec.len() + 1;
      vec.resize_msg(new_len, self.arena,
        TestAll::__LAYOUT, TestAll::__raw_init);
      self.rep_recursive_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn rep_recursive_resize(self, n: usize) {
    unsafe {
      self.ptr.as_mut().rep_recursive.resize_msg(
        n, self.arena,
        TestAll::__LAYOUT, TestAll::__raw_init);
    }
  }

  pub fn rep_nested_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.rep_nested.len()
  }
  pub fn rep_nested(self, n: usize) -> Option<__rt::rt::View<'msg, TestAll_Nested>> {
    unsafe { self.ptr.as_ref().rep_nested.as_slice() }.get(n)
      .map(|&ptr| __rt::rt::View::<TestAll_Nested> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn rep_nested_iter(self) -> impl Iterator<Item = __rt::rt::View<'msg, TestAll_Nested>> + 'msg {
    unsafe { self.ptr.as_ref().rep_nested.as_slice() }.iter()
      .map(|&ptr| __rt::rt::View::<TestAll_Nested> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn rep_nested_mut(self, n: usize) -> Option<__rt::rt::Mut<'msg, TestAll_Nested>> {
    unsafe { self.ptr.as_mut().rep_nested.as_mut_slice() }.get_mut(n)
      .map(|&mut ptr| __rt::rt::Mut::<TestAll_Nested> {
        ptr: unsafe { __rt::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
        arena: self.arena,
      })
  }
  pub fn rep_nested_add(self) -> __rt::rt::Mut<'msg, TestAll_Nested> {
    unsafe {
      let vec = &mut self.ptr.as_mut().rep_nested;
      let new_len = vec.len() + 1;
      vec.resize_msg(new_len, self.arena,
        TestAll_Nested::__LAYOUT, TestAll_Nested::__raw_init);
      self.rep_nested_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn rep_nested_resize(self, n: usize) {
    unsafe {
      self.ptr.as_mut().rep_nested.resize_msg(
        n, self.arena,
        TestAll_Nested::__LAYOUT, TestAll_Nested::__raw_init);
    }
  }

}

impl Drop for TestAll {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_TestAll::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.TestAll ")?;
    let mut debug = __rt::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_TestAll::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use __rt::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for TestAll {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_TestAll {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) opt_i32: u32,
    pub(in super) opt_i64: u64,
    pub(in super) opt_u32: u32,
    pub(in super) opt_u64: u64,
    pub(in super) opt_f32: u32,
    pub(in super) opt_f64: u64,
    pub(in super) opt_str: (*mut u8, usize),
    pub(in super) opt_bool: bool,
    pub(in super) opt_recursive: *mut u8,
    pub(in super) opt_nested: *mut u8,
    pub (in super) rep_i32: __rt::rt::__z::AVec<u32>,
    pub (in super) rep_i64: __rt::rt::__z::AVec<u64>,
    pub (in super) rep_u32: __rt::rt::__z::AVec<u32>,
    pub (in super) rep_u64: __rt::rt::__z::AVec<u64>,
    pub (in super) rep_f32: __rt::rt::__z::AVec<u32>,
    pub (in super) rep_f64: __rt::rt::__z::AVec<u64>,
    pub(crate) rep_str: __rt::rt::__z::AVec<(*mut u8, usize)>,
    pub (in super) rep_bool: __rt::rt::__z::AVec<bool>,
    pub(in super) rep_recursive: __rt::rt::__z::AVec<*mut u8>,
    pub(in super) rep_nested: __rt::rt::__z::AVec<*mut u8>,
  }

  pub const FIELD_OFFSET_opt_i32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_i32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_opt_i64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_i64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_opt_u32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_u32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_opt_u64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_u64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_opt_f32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_f32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_opt_f64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_f64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_opt_str: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_str as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_opt_bool: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_bool as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_opt_recursive: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_recursive as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_opt_nested: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_nested as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_rep_i32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_i32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_rep_i64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_i64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_rep_u32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_u32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_rep_u64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_u64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_rep_f32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_f32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_rep_f64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_f64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_rep_str: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_str as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_rep_bool: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_bool as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_rep_recursive: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_recursive as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_rep_nested: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_nested as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: __rt::rt::__z::tdp::MessageAndFields<{20 + 1}> =
    __rt::rt::__z::tdp::MessageAndFields::<{20 + 1}> {
      msg: __rt::rt::__z::tdp::Message {
        size: {
          let size = TestAll::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const __rt::rt::__z::tdp::Message] = &[
            TestAll::__tdp_info,
            TestAll_Nested::__tdp_info,
          ];
          TYS.as_ptr()
        },
        raw_init: TestAll::__raw_init,
      },
      fields: [
        __rt::rt::__z::tdp::Field {
          number: 1,
          flags: (__rt::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_opt_i32,
          ty: 0,
          hasbit: 0,
        },
        __rt::rt::__z::tdp::Field {
          number: 2,
          flags: (__rt::rt::__z::tdp::Kind::I64 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_opt_i64,
          ty: 0,
          hasbit: 1,
        },
        __rt::rt::__z::tdp::Field {
          number: 3,
          flags: (__rt::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_opt_u32,
          ty: 0,
          hasbit: 2,
        },
        __rt::rt::__z::tdp::Field {
          number: 4,
          flags: (__rt::rt::__z::tdp::Kind::I64 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_opt_u64,
          ty: 0,
          hasbit: 3,
        },
        __rt::rt::__z::tdp::Field {
          number: 5,
          flags: (__rt::rt::__z::tdp::Kind::F32 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_opt_f32,
          ty: 0,
          hasbit: 4,
        },
        __rt::rt::__z::tdp::Field {
          number: 6,
          flags: (__rt::rt::__z::tdp::Kind::F64 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_opt_f64,
          ty: 0,
          hasbit: 5,
        },
        __rt::rt::__z::tdp::Field {
          number: 7,
          flags: (__rt::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_opt_str,
          ty: 0,
          hasbit: 6,
        },
        __rt::rt::__z::tdp::Field {
          number: 8,
          flags: (__rt::rt::__z::tdp::Kind::Bool as u8 as u32) | (0 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_opt_bool,
          ty: 0,
          hasbit: 7,
        },
        __rt::rt::__z::tdp::Field {
          number: 10,
          flags: (__rt::rt::__z::tdp::Kind::Msg as u8 as u32) | (0 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_opt_recursive,
          ty: 0,
          hasbit: 8,
        },
        __rt::rt::__z::tdp::Field {
          number: 11,
          flags: (__rt::rt::__z::tdp::Kind::Msg as u8 as u32) | (0 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_opt_nested,
          ty: 1,
          hasbit: 9,
        },
        __rt::rt::__z::tdp::Field {
          number: 21,
          flags: (__rt::rt::__z::tdp::Kind::I32 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_rep_i32,
          ty: 0,
          hasbit: 10,
        },
        __rt::rt::__z::tdp::Field {
          number: 22,
          flags: (__rt::rt::__z::tdp::Kind::I64 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_rep_i64,
          ty: 0,
          hasbit: 10,
        },
        __rt::rt::__z::tdp::Field {
          number: 23,
          flags: (__rt::rt::__z::tdp::Kind::I32 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_rep_u32,
          ty: 0,
          hasbit: 10,
        },
        __rt::rt::__z::tdp::Field {
          number: 24,
          flags: (__rt::rt::__z::tdp::Kind::I64 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_rep_u64,
          ty: 0,
          hasbit: 10,
        },
        __rt::rt::__z::tdp::Field {
          number: 25,
          flags: (__rt::rt::__z::tdp::Kind::F32 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_rep_f32,
          ty: 0,
          hasbit: 10,
        },
        __rt::rt::__z::tdp::Field {
          number: 26,
          flags: (__rt::rt::__z::tdp::Kind::F64 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_rep_f64,
          ty: 0,
          hasbit: 10,
        },
        __rt::rt::__z::tdp::Field {
          number: 27,
          flags: (__rt::rt::__z::tdp::Kind::Str as u8 as u32) | (1 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_rep_str,
          ty: 0,
          hasbit: 10,
        },
        __rt::rt::__z::tdp::Field {
          number: 28,
          flags: (__rt::rt::__z::tdp::Kind::Bool as u8 as u32) | (1 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_rep_bool,
          ty: 0,
          hasbit: 10,
        },
        __rt::rt::__z::tdp::Field {
          number: 30,
          flags: (__rt::rt::__z::tdp::Kind::Msg as u8 as u32) | (1 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_rep_recursive,
          ty: 0,
          hasbit: 10,
        },
        __rt::rt::__z::tdp::Field {
          number: 31,
          flags: (__rt::rt::__z::tdp::Kind::Msg as u8 as u32) | (1 << 4),
          offset: __priv_TestAll::FIELD_OFFSET_rep_nested,
          ty: 1,
          hasbit: 10,
        },
        __rt::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: __rt::rt::__z::ABox<__priv_TestAll::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg TestAll>,
  }

  impl<'msg> __rt::rt::ptr::ViewFor<'msg, super::TestAll> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: __rt::rt::__z::ABox<__priv_TestAll::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut TestAll>,
    pub(in super) arena: __rt::rt::__z::RawArena,
  }

  impl<'msg> __rt::rt::ptr::ViewFor<'msg, super::TestAll> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> __rt::rt::ptr::MutFor<'msg, super::TestAll> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.TestAll.Nested`
pub struct TestAll_Nested {
  ptr: __rt::rt::__z::ABox<__priv_TestAll_Nested::Storage>,
  arena: __rt::rt::__z::RawArena,
}

impl TestAll_Nested {
  pub const DEFAULT: __rt::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_TestAll_Nested::Storage = __priv_TestAll_Nested::Storage {
      __hasbits: [0; 1],
      a: 0,
      b: __rt::rt::__z::AVec::new(),
    };
    __rt::rt::View::<Self> {
      ptr: __rt::rt::__z::ABox::from_ptr(&VALUE as *const __priv_TestAll_Nested::Storage as *mut __priv_TestAll_Nested::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = __rt::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: __rt::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, __rt::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), __rt::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> __rt::rt::View<Self> {
    __priv_TestAll_Nested::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::rt::Mut<Self> {
    __priv_TestAll_Nested::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { TestAll_Nested::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn a(&self) -> __rt::rt::View<'_, i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(&self) -> Option<__rt::rt::View<'_, i32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().a) })
  }
  pub fn a_mut(&mut self) -> __rt::rt::Mut<'_, i32> {
    self.a_mut_or().into_mut()
  }
  pub fn a_mut_or(&mut self) -> __rt::rt::value::OptMut<'_, i32> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll_Nested::FIELD_OFFSET_a as usize),
        self.arena,
        TestAll_Nested::__hazzer_a,
      )
    }
  }
  pub fn a_set(&mut self, value: i32) {
    self.a_mut().set(value);
  }

  pub fn b_len(&self) -> usize {
    unsafe { self.ptr.as_ref() }.b.len()
  }
  pub fn b(&self, n: usize) -> Option<&'_ __rt::rt::Str> {
    unsafe { self.ptr.as_ref().b.as_slice() }.get(n).map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn b_iter(&self) -> impl Iterator<Item = &'_ __rt::rt::Str> + '_ {
    unsafe { self.ptr.as_ref().b.as_slice() }.iter().map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn b_mut(&mut self, n: usize) -> Option<__rt::rt::StrBuf<'_>> {
    unsafe { self.ptr.as_mut().b.as_mut_slice() }.get_mut(n)
      .map(|data| __rt::rt::StrBuf::__wrap(data, self.arena))
  }
  pub fn b_add(&mut self) -> __rt::rt::StrBuf<'_> {
    unsafe {
      let vec = &mut self.ptr.as_mut().b;
      let new_len = vec.len() + 1;
      vec.resize(new_len, self.arena);
      self.b_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn b_resize(&mut self, n: usize) {
    unsafe {
      self.ptr.as_mut().b.resize(n, self.arena);
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_TestAll_Nested::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_TestAll_Nested::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_TestAll_Nested::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const __rt::rt::__z::tdp::Message {
    &__priv_TestAll_Nested::TDP_INFO as *const _ as *const __rt::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_a(
    raw: *mut u8,
    arena: __rt::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_TestAll_Nested::FIELD_OFFSET_a as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 1 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !1,
      Some(true) => {
        *word |= 1;
      }
    }
    has
  }
}

impl Default for TestAll_Nested {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::rt::ptr::Proxied for TestAll_Nested {
  type View<'msg> = __priv_TestAll_Nested::View<'msg>;
  type Mut<'msg> = __priv_TestAll_Nested::Mut<'msg>;
}

impl __rt::rt::value::Type for TestAll_Nested {
  unsafe fn __make_view<'a>(ptr: *mut u8) -> __rt::rt::View<'a, Self> {
    __priv_TestAll_Nested::View {
      ptr: __rt::rt::__z::ABox::from_ptr(ptr),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: __rt::rt::__z::RawArena) -> __rt::rt::Mut<'a, Self> {
    __priv_TestAll_Nested::Mut {
      ptr: __rt::rt::__z::ABox::from_ptr(ptr),
      arena,
      _ph: std::marker::PhantomData,
    }
  }
}

impl<'msg> __priv_TestAll_Nested::View<'msg> {
  pub fn a(self) -> __rt::rt::View<'msg, i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(self) -> Option<__rt::rt::View<'msg, i32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().a) })
  }

  pub fn b_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.b.len()
  }
  pub fn b(self, n: usize) -> Option<&'msg __rt::rt::Str> {
    unsafe { self.ptr.as_ref().b.as_slice() }.get(n).map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn b_iter(self) -> impl Iterator<Item = &'msg __rt::rt::Str> + 'msg {
    unsafe { self.ptr.as_ref().b.as_slice() }.iter().map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __rt::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.a_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("a")?;
      debug.write_debug(value);
      count += 1;
    }
    if self.b_len() != 0 {
      if count != 0 { debug.comma(false)?; }
      debug.field("b")?;
      debug.iter(self.b_iter())?;
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_TestAll_Nested::View<'_> {
  fn default() -> Self {
    TestAll_Nested::DEFAULT
  }
}

impl<'msg> __priv_TestAll_Nested::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { TestAll_Nested::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), __rt::rt::Error> {
    let mut ctx = __rt::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, TestAll_Nested::__tdp_info())
  }

  pub fn a(self) -> __rt::rt::View<'msg, i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(self) -> Option<__rt::rt::View<'msg, i32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().a) })
  }
  pub fn a_mut(self) -> __rt::rt::Mut<'msg, i32> {
    self.a_mut_or().into_mut()
  }
  pub fn a_mut_or(self) -> __rt::rt::value::OptMut<'msg, i32> {
    unsafe {
      __rt::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_TestAll_Nested::FIELD_OFFSET_a as usize),
        self.arena,
        TestAll_Nested::__hazzer_a,
      )
    }
  }
  pub fn a_set(self, value: i32) {
    self.a_mut().set(value);
  }

  pub fn b_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.b.len()
  }
  pub fn b(self, n: usize) -> Option<&'msg __rt::rt::Str> {
    unsafe { self.ptr.as_ref().b.as_slice() }.get(n).map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn b_iter(self) -> impl Iterator<Item = &'msg __rt::rt::Str> + 'msg {
    unsafe { self.ptr.as_ref().b.as_slice() }.iter().map(|&(p, n)| unsafe {
      __rt::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn b_mut(self, n: usize) -> Option<__rt::rt::StrBuf<'msg>> {
    unsafe { self.ptr.as_mut().b.as_mut_slice() }.get_mut(n)
      .map(|data| __rt::rt::StrBuf::__wrap(data, self.arena))
  }
  pub fn b_add(self) -> __rt::rt::StrBuf<'msg> {
    unsafe {
      let vec = &mut self.ptr.as_mut().b;
      let new_len = vec.len() + 1;
      vec.resize(new_len, self.arena);
      self.b_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn b_resize(self, n: usize) {
    unsafe {
      self.ptr.as_mut().b.resize(n, self.arena);
    }
  }

}

impl Drop for TestAll_Nested {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_TestAll_Nested::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.TestAll.Nested ")?;
    let mut debug = __rt::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_TestAll_Nested::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use __rt::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for TestAll_Nested {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_TestAll_Nested {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) a: u32,
    pub(crate) b: __rt::rt::__z::AVec<(*mut u8, usize)>,
  }

  pub const FIELD_OFFSET_a: u32 = unsafe {
    let msg = TestAll_Nested::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().a as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_b: u32 = unsafe {
    let msg = TestAll_Nested::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().b as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: __rt::rt::__z::tdp::MessageAndFields<{2 + 1}> =
    __rt::rt::__z::tdp::MessageAndFields::<{2 + 1}> {
      msg: __rt::rt::__z::tdp::Message {
        size: {
          let size = TestAll_Nested::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const __rt::rt::__z::tdp::Message] = &[
          ];
          TYS.as_ptr()
        },
        raw_init: TestAll_Nested::__raw_init,
      },
      fields: [
        __rt::rt::__z::tdp::Field {
          number: 1,
          flags: (__rt::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll_Nested::FIELD_OFFSET_a,
          ty: 0,
          hasbit: 0,
        },
        __rt::rt::__z::tdp::Field {
          number: 2,
          flags: (__rt::rt::__z::tdp::Kind::Str as u8 as u32) | (1 << 4),
          offset: __priv_TestAll_Nested::FIELD_OFFSET_b,
          ty: 0,
          hasbit: 1,
        },
        __rt::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: __rt::rt::__z::ABox<__priv_TestAll_Nested::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg TestAll_Nested>,
  }

  impl<'msg> __rt::rt::ptr::ViewFor<'msg, super::TestAll_Nested> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: __rt::rt::__z::ABox<__priv_TestAll_Nested::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut TestAll_Nested>,
    pub(in super) arena: __rt::rt::__z::RawArena,
  }

  impl<'msg> __rt::rt::ptr::ViewFor<'msg, super::TestAll_Nested> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> __rt::rt::ptr::MutFor<'msg, super::TestAll_Nested> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

