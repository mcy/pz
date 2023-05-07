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
      let (mut ptr, len) = self.ptr.as_ref().opt_str;
      if ptr.is_null() { ptr = 1 as *mut u8; }
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

  pub fn rep_i32(&self) -> __rt::rt::Slice<'_, i32> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_i32;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i32_at(&self, idx: usize) -> __rt::rt::View<'_, i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(&mut self) -> __rt::rt::Repeated<'_, i32> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_i32) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_i64(&self) -> __rt::rt::Slice<'_, i64> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_i64;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i64_at(&self, idx: usize) -> __rt::rt::View<'_, i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(&mut self) -> __rt::rt::Repeated<'_, i64> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_i64) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u32(&self) -> __rt::rt::Slice<'_, u32> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_u32;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u32_at(&self, idx: usize) -> __rt::rt::View<'_, u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(&mut self) -> __rt::rt::Repeated<'_, u32> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_u32) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u64(&self) -> __rt::rt::Slice<'_, u64> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_u64;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u64_at(&self, idx: usize) -> __rt::rt::View<'_, u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(&mut self) -> __rt::rt::Repeated<'_, u64> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_u64) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f32(&self) -> __rt::rt::Slice<'_, f32> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_f32;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f32_at(&self, idx: usize) -> __rt::rt::View<'_, f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(&mut self) -> __rt::rt::Repeated<'_, f32> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_f32) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f64(&self) -> __rt::rt::Slice<'_, f64> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_f64;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f64_at(&self, idx: usize) -> __rt::rt::View<'_, f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(&mut self) -> __rt::rt::Repeated<'_, f64> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_f64) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_str(&self) -> __rt::rt::Slice<'_, __rt::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_str;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_str_at(&self, idx: usize) -> __rt::rt::View<'_, __rt::rt::Str> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(&mut self) -> __rt::rt::Repeated<'_, __rt::rt::Str> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_str) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_bool(&self) -> __rt::rt::Slice<'_, bool> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_bool;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_bool_at(&self, idx: usize) -> __rt::rt::View<'_, bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(&mut self) -> __rt::rt::Repeated<'_, bool> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_bool) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_recursive(&self) -> __rt::rt::Slice<'_, TestAll> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_recursive;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_recursive_at(&self, idx: usize) -> __rt::rt::View<'_, TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(&mut self) -> __rt::rt::Repeated<'_, TestAll> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_recursive) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_nested(&self) -> __rt::rt::Slice<'_, TestAll_Nested> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_nested;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_nested_at(&self, idx: usize) -> __rt::rt::View<'_, TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(&mut self) -> __rt::rt::Repeated<'_, TestAll_Nested> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_nested) as *mut _ as *mut u8,
        self.arena,
      )
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
          storage.opt_recursive = arena.alloc(TestAll::__LAYOUT).as_ptr();
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
          storage.opt_nested = arena.alloc(TestAll_Nested::__LAYOUT).as_ptr();
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
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> __rt::rt::View<'a, Self> {
    __priv_TestAll::View {
      ptr: __rt::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: __rt::rt::__z::RawArena) -> __rt::rt::Mut<'a, Self> {
    __priv_TestAll::Mut {
      ptr: __rt::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: __rt::rt::__z::RawArena) {
    (&mut *ptr.cast::<__rt::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
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
      let (mut ptr, len) = self.ptr.as_ref().opt_str;
      if ptr.is_null() { ptr = 1 as *mut u8; }
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

  pub fn rep_i32(self) -> __rt::rt::Slice<'msg, i32> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_i32;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::rt::View<'msg, i32> {
    self.rep_i32().at(idx)
  }

  pub fn rep_i64(self) -> __rt::rt::Slice<'msg, i64> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_i64;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::rt::View<'msg, i64> {
    self.rep_i64().at(idx)
  }

  pub fn rep_u32(self) -> __rt::rt::Slice<'msg, u32> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_u32;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::rt::View<'msg, u32> {
    self.rep_u32().at(idx)
  }

  pub fn rep_u64(self) -> __rt::rt::Slice<'msg, u64> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_u64;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::rt::View<'msg, u64> {
    self.rep_u64().at(idx)
  }

  pub fn rep_f32(self) -> __rt::rt::Slice<'msg, f32> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_f32;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::rt::View<'msg, f32> {
    self.rep_f32().at(idx)
  }

  pub fn rep_f64(self) -> __rt::rt::Slice<'msg, f64> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_f64;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::rt::View<'msg, f64> {
    self.rep_f64().at(idx)
  }

  pub fn rep_str(self) -> __rt::rt::Slice<'msg, __rt::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_str;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::rt::View<'msg, __rt::rt::Str> {
    self.rep_str().at(idx)
  }

  pub fn rep_bool(self) -> __rt::rt::Slice<'msg, bool> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_bool;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::rt::View<'msg, bool> {
    self.rep_bool().at(idx)
  }

  pub fn rep_recursive(self) -> __rt::rt::Slice<'msg, TestAll> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_recursive;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::rt::View<'msg, TestAll> {
    self.rep_recursive().at(idx)
  }

  pub fn rep_nested(self) -> __rt::rt::Slice<'msg, TestAll_Nested> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_nested;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::rt::View<'msg, TestAll_Nested> {
    self.rep_nested().at(idx)
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
    if !self.rep_i32().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_i32")?;
      debug.iter(self.rep_i32())?;
      count += 1;
    }
    if !self.rep_i64().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_i64")?;
      debug.iter(self.rep_i64())?;
      count += 1;
    }
    if !self.rep_u32().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_u32")?;
      debug.iter(self.rep_u32())?;
      count += 1;
    }
    if !self.rep_u64().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_u64")?;
      debug.iter(self.rep_u64())?;
      count += 1;
    }
    if !self.rep_f32().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_f32")?;
      debug.iter(self.rep_f32())?;
      count += 1;
    }
    if !self.rep_f64().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_f64")?;
      debug.iter(self.rep_f64())?;
      count += 1;
    }
    if !self.rep_str().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_str")?;
      debug.iter(self.rep_str())?;
      count += 1;
    }
    if !self.rep_bool().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_bool")?;
      debug.iter(self.rep_bool())?;
      count += 1;
    }
    for value in self.rep_recursive() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_recursive")?;
      value.__debug(debug)?;
      count += 1;
    }
    for value in self.rep_nested() {
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
      let (mut ptr, len) = self.ptr.as_ref().opt_str;
      if ptr.is_null() { ptr = 1 as *mut u8; }
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

  pub fn rep_i32(self) -> __rt::rt::Slice<'msg, i32> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_i32;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::rt::View<'msg, i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(self) -> __rt::rt::Repeated<'msg, i32> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_i32) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_i64(self) -> __rt::rt::Slice<'msg, i64> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_i64;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::rt::View<'msg, i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(self) -> __rt::rt::Repeated<'msg, i64> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_i64) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u32(self) -> __rt::rt::Slice<'msg, u32> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_u32;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::rt::View<'msg, u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(self) -> __rt::rt::Repeated<'msg, u32> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_u32) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u64(self) -> __rt::rt::Slice<'msg, u64> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_u64;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::rt::View<'msg, u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(self) -> __rt::rt::Repeated<'msg, u64> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_u64) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f32(self) -> __rt::rt::Slice<'msg, f32> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_f32;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::rt::View<'msg, f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(self) -> __rt::rt::Repeated<'msg, f32> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_f32) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f64(self) -> __rt::rt::Slice<'msg, f64> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_f64;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::rt::View<'msg, f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(self) -> __rt::rt::Repeated<'msg, f64> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_f64) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_str(self) -> __rt::rt::Slice<'msg, __rt::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_str;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::rt::View<'msg, __rt::rt::Str> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(self) -> __rt::rt::Repeated<'msg, __rt::rt::Str> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_str) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_bool(self) -> __rt::rt::Slice<'msg, bool> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_bool;
      __rt::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::rt::View<'msg, bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(self) -> __rt::rt::Repeated<'msg, bool> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_bool) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_recursive(self) -> __rt::rt::Slice<'msg, TestAll> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_recursive;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::rt::View<'msg, TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(self) -> __rt::rt::Repeated<'msg, TestAll> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_recursive) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_nested(self) -> __rt::rt::Slice<'msg, TestAll_Nested> {
    unsafe {
      let vec = &self.ptr.as_ref().rep_nested;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::rt::View<'msg, TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(self) -> __rt::rt::Repeated<'msg, TestAll_Nested> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().rep_nested) as *mut _ as *mut u8,
        self.arena,
      )
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

  pub fn b(&self) -> __rt::rt::Slice<'_, __rt::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().b;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn b_at(&self, idx: usize) -> __rt::rt::View<'_, __rt::rt::Str> {
    self.b().at(idx)
  }
  pub fn b_mut(&mut self) -> __rt::rt::Repeated<'_, __rt::rt::Str> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().b) as *mut _ as *mut u8,
        self.arena,
      )
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
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> __rt::rt::View<'a, Self> {
    __priv_TestAll_Nested::View {
      ptr: __rt::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: __rt::rt::__z::RawArena) -> __rt::rt::Mut<'a, Self> {
    __priv_TestAll_Nested::Mut {
      ptr: __rt::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: __rt::rt::__z::RawArena) {
    (&mut *ptr.cast::<__rt::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
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

  pub fn b(self) -> __rt::rt::Slice<'msg, __rt::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().b;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn b_at(self, idx: usize) -> __rt::rt::View<'msg, __rt::rt::Str> {
    self.b().at(idx)
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
    if !self.b().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("b")?;
      debug.iter(self.b())?;
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

  pub fn b(self) -> __rt::rt::Slice<'msg, __rt::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().b;
      __rt::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn b_at(self, idx: usize) -> __rt::rt::View<'msg, __rt::rt::Str> {
    self.b().at(idx)
  }
  pub fn b_mut(self) -> __rt::rt::Repeated<'msg, __rt::rt::Str> {
    unsafe {
      __rt::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().b) as *mut _ as *mut u8,
        self.arena,
      )
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

