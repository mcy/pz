// ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]
extern crate pz as __rt;

/// message `pz.test.TestAll`
pub struct TestAll {
  ptr: __rt::__z::ABox<__priv_TestAll::Storage>,
  arena: __rt::__z::RawArena,
}

impl TestAll {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
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
      opt_choice: 0 as *mut u8,
      rep_i32: __rt::__z::AVec::new(),
      rep_i64: __rt::__z::AVec::new(),
      rep_u32: __rt::__z::AVec::new(),
      rep_u64: __rt::__z::AVec::new(),
      rep_f32: __rt::__z::AVec::new(),
      rep_f64: __rt::__z::AVec::new(),
      rep_str: __rt::__z::AVec::new(),
      rep_bool: __rt::__z::AVec::new(),
      rep_recursive: __rt::__z::AVec::new(),
      rep_nested: __rt::__z::AVec::new(),
      rep_choice: __rt::__z::AVec::new(),
    };
    __rt::View::<Self> {
      ptr: __rt::__z::ABox::from_ptr(&VALUE as *const __priv_TestAll::Storage as *mut __priv_TestAll::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = __rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: __rt::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, __rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), __rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> __rt::View<Self> {
    __priv_TestAll::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_TestAll::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { TestAll::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn opt_i32(&self) -> __rt::View<'_, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(&self) -> Option<__rt::View<'_, i32>> {
    if !unsafe { TestAll::__HAZZER_opt_i32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().opt_i32 }) })
  }
  pub fn opt_i32_mut(&mut self) -> __rt::Mut<'_, i32> {
    self.opt_i32_mut_or().into_mut()
  }
  pub fn opt_i32_mut_or(&mut self) -> __rt::value::OptMut<'_, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_i32,
      )
    }
  }
  pub fn opt_i32_set(&mut self, value: i32) {
    self.opt_i32_mut().set(value);
  }

  pub fn opt_i64(&self) -> __rt::View<'_, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(&self) -> Option<__rt::View<'_, i64>> {
    if !unsafe { TestAll::__HAZZER_opt_i64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, i64>(*unsafe { &self.ptr.as_ref().opt_i64 }) })
  }
  pub fn opt_i64_mut(&mut self) -> __rt::Mut<'_, i64> {
    self.opt_i64_mut_or().into_mut()
  }
  pub fn opt_i64_mut_or(&mut self) -> __rt::value::OptMut<'_, i64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_i64,
      )
    }
  }
  pub fn opt_i64_set(&mut self, value: i64) {
    self.opt_i64_mut().set(value);
  }

  pub fn opt_u32(&self) -> __rt::View<'_, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(&self) -> Option<__rt::View<'_, u32>> {
    if !unsafe { TestAll::__HAZZER_opt_u32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().opt_u32 }) })
  }
  pub fn opt_u32_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.opt_u32_mut_or().into_mut()
  }
  pub fn opt_u32_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_u32,
      )
    }
  }
  pub fn opt_u32_set(&mut self, value: u32) {
    self.opt_u32_mut().set(value);
  }

  pub fn opt_u64(&self) -> __rt::View<'_, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(&self) -> Option<__rt::View<'_, u64>> {
    if !unsafe { TestAll::__HAZZER_opt_u64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, u64>(*unsafe { &self.ptr.as_ref().opt_u64 }) })
  }
  pub fn opt_u64_mut(&mut self) -> __rt::Mut<'_, u64> {
    self.opt_u64_mut_or().into_mut()
  }
  pub fn opt_u64_mut_or(&mut self) -> __rt::value::OptMut<'_, u64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_u64,
      )
    }
  }
  pub fn opt_u64_set(&mut self, value: u64) {
    self.opt_u64_mut().set(value);
  }

  pub fn opt_f32(&self) -> __rt::View<'_, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(&self) -> Option<__rt::View<'_, f32>> {
    if !unsafe { TestAll::__HAZZER_opt_f32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, f32>(*unsafe { &self.ptr.as_ref().opt_f32 }) })
  }
  pub fn opt_f32_mut(&mut self) -> __rt::Mut<'_, f32> {
    self.opt_f32_mut_or().into_mut()
  }
  pub fn opt_f32_mut_or(&mut self) -> __rt::value::OptMut<'_, f32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_f32,
      )
    }
  }
  pub fn opt_f32_set(&mut self, value: f32) {
    self.opt_f32_mut().set(value);
  }

  pub fn opt_f64(&self) -> __rt::View<'_, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(&self) -> Option<__rt::View<'_, f64>> {
    if !unsafe { TestAll::__HAZZER_opt_f64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, f64>(*unsafe { &self.ptr.as_ref().opt_f64 }) })
  }
  pub fn opt_f64_mut(&mut self) -> __rt::Mut<'_, f64> {
    self.opt_f64_mut_or().into_mut()
  }
  pub fn opt_f64_mut_or(&mut self) -> __rt::value::OptMut<'_, f64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_f64,
      )
    }
  }
  pub fn opt_f64_set(&mut self, value: f64) {
    self.opt_f64_mut().set(value);
  }

  pub fn opt_str(&self) -> __rt::View<'_, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(&self) -> Option<__rt::View<'_, __rt::Str>> {
    if !unsafe { TestAll::__HAZZER_opt_str.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().opt_str };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      __rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn opt_str_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.opt_str_mut_or().into_mut()
  }
  pub fn opt_str_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_str,
      )
    }
  }
  pub fn opt_str_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.opt_str_mut().set(value);
  }

  pub fn opt_bool(&self) -> __rt::View<'_, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(&self) -> Option<__rt::View<'_, bool>> {
    if !unsafe { TestAll::__HAZZER_opt_bool.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().opt_bool }) })
  }
  pub fn opt_bool_mut(&mut self) -> __rt::Mut<'_, bool> {
    self.opt_bool_mut_or().into_mut()
  }
  pub fn opt_bool_mut_or(&mut self) -> __rt::value::OptMut<'_, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_bool,
      )
    }
  }
  pub fn opt_bool_set(&mut self, value: bool) {
    self.opt_bool_mut().set(value);
  }

  pub fn opt_recursive(&self) -> __rt::View<'_, TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(&self) -> Option<__rt::View<'_, TestAll>> {
    if !unsafe { TestAll::__HAZZER_opt_recursive.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().opt_recursive } as *mut _ as *mut u8)) }
  }
  pub fn opt_recursive_mut(&mut self) -> __rt::Mut<'_, TestAll> {
    self.opt_recursive_mut_or().into_mut()
  }
  pub fn opt_recursive_mut_or(&mut self) -> __rt::value::OptMut<'_, TestAll> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_recursive,
      )
    }
  }

  pub fn opt_nested(&self) -> __rt::View<'_, TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(&self) -> Option<__rt::View<'_, TestAll_Nested>> {
    if !unsafe { TestAll::__HAZZER_opt_nested.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll_Nested as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().opt_nested } as *mut _ as *mut u8)) }
  }
  pub fn opt_nested_mut(&mut self) -> __rt::Mut<'_, TestAll_Nested> {
    self.opt_nested_mut_or().into_mut()
  }
  pub fn opt_nested_mut_or(&mut self) -> __rt::value::OptMut<'_, TestAll_Nested> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_nested,
      )
    }
  }

  pub fn opt_choice(&self) -> __rt::View<'_, TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(&self) -> Option<__rt::View<'_, TestAll2>> {
    if !unsafe { TestAll::__HAZZER_opt_choice.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll2 as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().opt_choice } as *mut _ as *mut u8)) }
  }
  pub fn opt_choice_mut(&mut self) -> __rt::Mut<'_, TestAll2> {
    self.opt_choice_mut_or().into_mut()
  }
  pub fn opt_choice_mut_or(&mut self) -> __rt::value::OptMut<'_, TestAll2> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_choice,
      )
    }
  }

  pub fn rep_i32(&self) -> __rt::Slice<'_, i32> {
    if !unsafe { TestAll::__HAZZER_rep_i32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_i32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i32_at(&self, idx: usize) -> __rt::View<'_, i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(&mut self) -> __rt::Repeated<'_, i32> {
    unsafe {
      TestAll::__HAZZER_rep_i32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_i32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_i64(&self) -> __rt::Slice<'_, i64> {
    if !unsafe { TestAll::__HAZZER_rep_i64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_i64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i64_at(&self, idx: usize) -> __rt::View<'_, i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(&mut self) -> __rt::Repeated<'_, i64> {
    unsafe {
      TestAll::__HAZZER_rep_i64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_i64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u32(&self) -> __rt::Slice<'_, u32> {
    if !unsafe { TestAll::__HAZZER_rep_u32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_u32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u32_at(&self, idx: usize) -> __rt::View<'_, u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(&mut self) -> __rt::Repeated<'_, u32> {
    unsafe {
      TestAll::__HAZZER_rep_u32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_u32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u64(&self) -> __rt::Slice<'_, u64> {
    if !unsafe { TestAll::__HAZZER_rep_u64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_u64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u64_at(&self, idx: usize) -> __rt::View<'_, u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(&mut self) -> __rt::Repeated<'_, u64> {
    unsafe {
      TestAll::__HAZZER_rep_u64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_u64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f32(&self) -> __rt::Slice<'_, f32> {
    if !unsafe { TestAll::__HAZZER_rep_f32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_f32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f32_at(&self, idx: usize) -> __rt::View<'_, f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(&mut self) -> __rt::Repeated<'_, f32> {
    unsafe {
      TestAll::__HAZZER_rep_f32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_f32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f64(&self) -> __rt::Slice<'_, f64> {
    if !unsafe { TestAll::__HAZZER_rep_f64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_f64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f64_at(&self, idx: usize) -> __rt::View<'_, f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(&mut self) -> __rt::Repeated<'_, f64> {
    unsafe {
      TestAll::__HAZZER_rep_f64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_f64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_str(&self) -> __rt::Slice<'_, __rt::Str> {
    if !unsafe { TestAll::__HAZZER_rep_str.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_str };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_str_at(&self, idx: usize) -> __rt::View<'_, __rt::Str> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(&mut self) -> __rt::Repeated<'_, __rt::Str> {
    unsafe {
      TestAll::__HAZZER_rep_str.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_str } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_bool(&self) -> __rt::Slice<'_, bool> {
    if !unsafe { TestAll::__HAZZER_rep_bool.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_bool };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_bool_at(&self, idx: usize) -> __rt::View<'_, bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(&mut self) -> __rt::Repeated<'_, bool> {
    unsafe {
      TestAll::__HAZZER_rep_bool.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_bool } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_recursive(&self) -> __rt::Slice<'_, TestAll> {
    if !unsafe { TestAll::__HAZZER_rep_recursive.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_recursive };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_recursive_at(&self, idx: usize) -> __rt::View<'_, TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(&mut self) -> __rt::Repeated<'_, TestAll> {
    unsafe {
      TestAll::__HAZZER_rep_recursive.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_recursive } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_nested(&self) -> __rt::Slice<'_, TestAll_Nested> {
    if !unsafe { TestAll::__HAZZER_rep_nested.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_nested };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_nested_at(&self, idx: usize) -> __rt::View<'_, TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(&mut self) -> __rt::Repeated<'_, TestAll_Nested> {
    unsafe {
      TestAll::__HAZZER_rep_nested.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_nested } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_choice(&self) -> __rt::Slice<'_, TestAll2> {
    if !unsafe { TestAll::__HAZZER_rep_choice.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_choice };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_choice_at(&self, idx: usize) -> __rt::View<'_, TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(&mut self) -> __rt::Repeated<'_, TestAll2> {
    unsafe {
      TestAll::__HAZZER_rep_choice.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_choice } as *mut _ as *mut u8,
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
  pub fn __tdp_info() -> *const __rt::__z::tdp::Type {
    &__priv_TestAll::TDP_INFO as *const _ as *const __rt::__z::tdp::Type
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
  #[doc(hidden)]
  pub const __OFFSET_opt_i32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_i32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_i32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 0,
    offset: Self::__OFFSET_opt_i32,
    size: 4,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_i64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_i64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_i64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 1,
    offset: Self::__OFFSET_opt_i64,
    size: 8,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_u32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_u32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_u32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 2,
    offset: Self::__OFFSET_opt_u32,
    size: 4,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_u64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_u64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_u64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 3,
    offset: Self::__OFFSET_opt_u64,
    size: 8,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_f32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_f32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_f32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 4,
    offset: Self::__OFFSET_opt_f32,
    size: 4,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_f64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_f64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_f64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 5,
    offset: Self::__OFFSET_opt_f64,
    size: 8,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_str: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_str as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_str: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 6,
    offset: Self::__OFFSET_opt_str,
    size: (usize::BITS / 8 * 2) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_bool: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_bool as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_bool: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 7,
    offset: Self::__OFFSET_opt_bool,
    size: 1,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_recursive: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_recursive as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_recursive: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 8,
    offset: Self::__OFFSET_opt_recursive,
    size: -(TestAll::__LAYOUT.size() as i32),
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_nested: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_nested as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_nested: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 9,
    offset: Self::__OFFSET_opt_nested,
    size: -(TestAll_Nested::__LAYOUT.size() as i32),
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_choice: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().opt_choice as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_opt_choice: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 10,
    offset: Self::__OFFSET_opt_choice,
    size: -(TestAll2::__LAYOUT.size() as i32),
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_i32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_i32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_i32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_i32,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_i64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_i64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_i64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_i64,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_u32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_u32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_u32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_u32,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_u64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_u64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_u64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_u64,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_f32: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_f32 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_f32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_f32,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_f64: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_f64 as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_f64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_f64,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_str: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_str as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_str: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_str,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_bool: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_bool as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_bool: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_bool,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_recursive: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_recursive as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_recursive: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_recursive,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_nested: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_nested as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_nested: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_nested,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_choice: u32 = unsafe {
    let msg = TestAll::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().rep_choice as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_rep_choice: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_rep_choice,
    size: (usize::BITS / 8) as i32,
  };
}

impl Default for TestAll {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for TestAll {
  type View<'proto> = __priv_TestAll::View<'proto>;
  type Mut<'proto> = __priv_TestAll::Mut<'proto>;
}

impl __rt::value::Type for TestAll {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> __rt::View<'a, Self> {
    __priv_TestAll::View {
      ptr: __rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: __rt::__z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_TestAll::Mut {
      ptr: __rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: __rt::__z::RawArena) {
    (&mut *ptr.cast::<__rt::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_TestAll::View<'proto> {
  pub fn as_view(&self) -> __rt::View<TestAll> {
    __priv_TestAll::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn opt_i32(self) -> __rt::View<'proto, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> Option<__rt::View<'proto, i32>> {
    if !unsafe { TestAll::__HAZZER_opt_i32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().opt_i32 }) })
  }

  pub fn opt_i64(self) -> __rt::View<'proto, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> Option<__rt::View<'proto, i64>> {
    if !unsafe { TestAll::__HAZZER_opt_i64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, i64>(*unsafe { &self.ptr.as_ref().opt_i64 }) })
  }

  pub fn opt_u32(self) -> __rt::View<'proto, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> Option<__rt::View<'proto, u32>> {
    if !unsafe { TestAll::__HAZZER_opt_u32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().opt_u32 }) })
  }

  pub fn opt_u64(self) -> __rt::View<'proto, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> Option<__rt::View<'proto, u64>> {
    if !unsafe { TestAll::__HAZZER_opt_u64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, u64>(*unsafe { &self.ptr.as_ref().opt_u64 }) })
  }

  pub fn opt_f32(self) -> __rt::View<'proto, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> Option<__rt::View<'proto, f32>> {
    if !unsafe { TestAll::__HAZZER_opt_f32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, f32>(*unsafe { &self.ptr.as_ref().opt_f32 }) })
  }

  pub fn opt_f64(self) -> __rt::View<'proto, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> Option<__rt::View<'proto, f64>> {
    if !unsafe { TestAll::__HAZZER_opt_f64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, f64>(*unsafe { &self.ptr.as_ref().opt_f64 }) })
  }

  pub fn opt_str(self) -> __rt::View<'proto, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> Option<__rt::View<'proto, __rt::Str>> {
    if !unsafe { TestAll::__HAZZER_opt_str.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().opt_str };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      __rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn opt_bool(self) -> __rt::View<'proto, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> Option<__rt::View<'proto, bool>> {
    if !unsafe { TestAll::__HAZZER_opt_bool.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().opt_bool }) })
  }

  pub fn opt_recursive(self) -> __rt::View<'proto, TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> Option<__rt::View<'proto, TestAll>> {
    if !unsafe { TestAll::__HAZZER_opt_recursive.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().opt_recursive } as *mut _ as *mut u8)) }
  }

  pub fn opt_nested(self) -> __rt::View<'proto, TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> Option<__rt::View<'proto, TestAll_Nested>> {
    if !unsafe { TestAll::__HAZZER_opt_nested.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll_Nested as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().opt_nested } as *mut _ as *mut u8)) }
  }

  pub fn opt_choice(self) -> __rt::View<'proto, TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> Option<__rt::View<'proto, TestAll2>> {
    if !unsafe { TestAll::__HAZZER_opt_choice.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll2 as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().opt_choice } as *mut _ as *mut u8)) }
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, i32> {
    if !unsafe { TestAll::__HAZZER_rep_i32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_i32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::View<'proto, i32> {
    self.rep_i32().at(idx)
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, i64> {
    if !unsafe { TestAll::__HAZZER_rep_i64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_i64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::View<'proto, i64> {
    self.rep_i64().at(idx)
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, u32> {
    if !unsafe { TestAll::__HAZZER_rep_u32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_u32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.rep_u32().at(idx)
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, u64> {
    if !unsafe { TestAll::__HAZZER_rep_u64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_u64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::View<'proto, u64> {
    self.rep_u64().at(idx)
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, f32> {
    if !unsafe { TestAll::__HAZZER_rep_f32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_f32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::View<'proto, f32> {
    self.rep_f32().at(idx)
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, f64> {
    if !unsafe { TestAll::__HAZZER_rep_f64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_f64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::View<'proto, f64> {
    self.rep_f64().at(idx)
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::Str> {
    if !unsafe { TestAll::__HAZZER_rep_str.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_str };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.rep_str().at(idx)
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, bool> {
    if !unsafe { TestAll::__HAZZER_rep_bool.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_bool };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::View<'proto, bool> {
    self.rep_bool().at(idx)
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, TestAll> {
    if !unsafe { TestAll::__HAZZER_rep_recursive.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_recursive };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::View<'proto, TestAll> {
    self.rep_recursive().at(idx)
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, TestAll_Nested> {
    if !unsafe { TestAll::__HAZZER_rep_nested.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_nested };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::View<'proto, TestAll_Nested> {
    self.rep_nested().at(idx)
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, TestAll2> {
    if !unsafe { TestAll::__HAZZER_rep_choice.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_choice };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::View<'proto, TestAll2> {
    self.rep_choice().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __rt::__z::Debug) -> std::fmt::Result {
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
    if let Some(value) = self.opt_choice_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_choice")?;
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
    for value in self.rep_choice() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_choice")?;
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

impl<'proto> __priv_TestAll::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<TestAll> {
    __priv_TestAll::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, TestAll> {
    __priv_TestAll::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<TestAll> {
    __priv_TestAll::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { TestAll::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), __rt::Error> {
    let mut ctx = __rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, TestAll::__tdp_info())
  }

  pub fn opt_i32(self) -> __rt::View<'proto, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> Option<__rt::View<'proto, i32>> {
    if !unsafe { TestAll::__HAZZER_opt_i32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().opt_i32 }) })
  }
  pub fn opt_i32_mut(self) -> __rt::Mut<'proto, i32> {
    self.opt_i32_mut_or().into_mut()
  }
  pub fn opt_i32_mut_or(self) -> __rt::value::OptMut<'proto, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_i32,
      )
    }
  }
  pub fn opt_i32_set(self, value: i32) {
    self.opt_i32_mut().set(value);
  }

  pub fn opt_i64(self) -> __rt::View<'proto, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> Option<__rt::View<'proto, i64>> {
    if !unsafe { TestAll::__HAZZER_opt_i64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, i64>(*unsafe { &self.ptr.as_ref().opt_i64 }) })
  }
  pub fn opt_i64_mut(self) -> __rt::Mut<'proto, i64> {
    self.opt_i64_mut_or().into_mut()
  }
  pub fn opt_i64_mut_or(self) -> __rt::value::OptMut<'proto, i64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_i64,
      )
    }
  }
  pub fn opt_i64_set(self, value: i64) {
    self.opt_i64_mut().set(value);
  }

  pub fn opt_u32(self) -> __rt::View<'proto, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> Option<__rt::View<'proto, u32>> {
    if !unsafe { TestAll::__HAZZER_opt_u32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().opt_u32 }) })
  }
  pub fn opt_u32_mut(self) -> __rt::Mut<'proto, u32> {
    self.opt_u32_mut_or().into_mut()
  }
  pub fn opt_u32_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_u32,
      )
    }
  }
  pub fn opt_u32_set(self, value: u32) {
    self.opt_u32_mut().set(value);
  }

  pub fn opt_u64(self) -> __rt::View<'proto, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> Option<__rt::View<'proto, u64>> {
    if !unsafe { TestAll::__HAZZER_opt_u64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, u64>(*unsafe { &self.ptr.as_ref().opt_u64 }) })
  }
  pub fn opt_u64_mut(self) -> __rt::Mut<'proto, u64> {
    self.opt_u64_mut_or().into_mut()
  }
  pub fn opt_u64_mut_or(self) -> __rt::value::OptMut<'proto, u64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_u64,
      )
    }
  }
  pub fn opt_u64_set(self, value: u64) {
    self.opt_u64_mut().set(value);
  }

  pub fn opt_f32(self) -> __rt::View<'proto, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> Option<__rt::View<'proto, f32>> {
    if !unsafe { TestAll::__HAZZER_opt_f32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, f32>(*unsafe { &self.ptr.as_ref().opt_f32 }) })
  }
  pub fn opt_f32_mut(self) -> __rt::Mut<'proto, f32> {
    self.opt_f32_mut_or().into_mut()
  }
  pub fn opt_f32_mut_or(self) -> __rt::value::OptMut<'proto, f32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_f32,
      )
    }
  }
  pub fn opt_f32_set(self, value: f32) {
    self.opt_f32_mut().set(value);
  }

  pub fn opt_f64(self) -> __rt::View<'proto, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> Option<__rt::View<'proto, f64>> {
    if !unsafe { TestAll::__HAZZER_opt_f64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, f64>(*unsafe { &self.ptr.as_ref().opt_f64 }) })
  }
  pub fn opt_f64_mut(self) -> __rt::Mut<'proto, f64> {
    self.opt_f64_mut_or().into_mut()
  }
  pub fn opt_f64_mut_or(self) -> __rt::value::OptMut<'proto, f64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_f64,
      )
    }
  }
  pub fn opt_f64_set(self, value: f64) {
    self.opt_f64_mut().set(value);
  }

  pub fn opt_str(self) -> __rt::View<'proto, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> Option<__rt::View<'proto, __rt::Str>> {
    if !unsafe { TestAll::__HAZZER_opt_str.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().opt_str };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      __rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn opt_str_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.opt_str_mut_or().into_mut()
  }
  pub fn opt_str_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_str,
      )
    }
  }
  pub fn opt_str_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.opt_str_mut().set(value);
  }

  pub fn opt_bool(self) -> __rt::View<'proto, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> Option<__rt::View<'proto, bool>> {
    if !unsafe { TestAll::__HAZZER_opt_bool.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().opt_bool }) })
  }
  pub fn opt_bool_mut(self) -> __rt::Mut<'proto, bool> {
    self.opt_bool_mut_or().into_mut()
  }
  pub fn opt_bool_mut_or(self) -> __rt::value::OptMut<'proto, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_bool,
      )
    }
  }
  pub fn opt_bool_set(self, value: bool) {
    self.opt_bool_mut().set(value);
  }

  pub fn opt_recursive(self) -> __rt::View<'proto, TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> Option<__rt::View<'proto, TestAll>> {
    if !unsafe { TestAll::__HAZZER_opt_recursive.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().opt_recursive } as *mut _ as *mut u8)) }
  }
  pub fn opt_recursive_mut(self) -> __rt::Mut<'proto, TestAll> {
    self.opt_recursive_mut_or().into_mut()
  }
  pub fn opt_recursive_mut_or(self) -> __rt::value::OptMut<'proto, TestAll> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_recursive,
      )
    }
  }

  pub fn opt_nested(self) -> __rt::View<'proto, TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> Option<__rt::View<'proto, TestAll_Nested>> {
    if !unsafe { TestAll::__HAZZER_opt_nested.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll_Nested as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().opt_nested } as *mut _ as *mut u8)) }
  }
  pub fn opt_nested_mut(self) -> __rt::Mut<'proto, TestAll_Nested> {
    self.opt_nested_mut_or().into_mut()
  }
  pub fn opt_nested_mut_or(self) -> __rt::value::OptMut<'proto, TestAll_Nested> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_nested,
      )
    }
  }

  pub fn opt_choice(self) -> __rt::View<'proto, TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> Option<__rt::View<'proto, TestAll2>> {
    if !unsafe { TestAll::__HAZZER_opt_choice.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll2 as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().opt_choice } as *mut _ as *mut u8)) }
  }
  pub fn opt_choice_mut(self) -> __rt::Mut<'proto, TestAll2> {
    self.opt_choice_mut_or().into_mut()
  }
  pub fn opt_choice_mut_or(self) -> __rt::value::OptMut<'proto, TestAll2> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll::__HAZZER_opt_choice,
      )
    }
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, i32> {
    if !unsafe { TestAll::__HAZZER_rep_i32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_i32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::View<'proto, i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(self) -> __rt::Repeated<'proto, i32> {
    unsafe {
      TestAll::__HAZZER_rep_i32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_i32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, i64> {
    if !unsafe { TestAll::__HAZZER_rep_i64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_i64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::View<'proto, i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(self) -> __rt::Repeated<'proto, i64> {
    unsafe {
      TestAll::__HAZZER_rep_i64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_i64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, u32> {
    if !unsafe { TestAll::__HAZZER_rep_u32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_u32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(self) -> __rt::Repeated<'proto, u32> {
    unsafe {
      TestAll::__HAZZER_rep_u32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_u32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, u64> {
    if !unsafe { TestAll::__HAZZER_rep_u64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_u64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::View<'proto, u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(self) -> __rt::Repeated<'proto, u64> {
    unsafe {
      TestAll::__HAZZER_rep_u64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_u64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, f32> {
    if !unsafe { TestAll::__HAZZER_rep_f32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_f32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::View<'proto, f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(self) -> __rt::Repeated<'proto, f32> {
    unsafe {
      TestAll::__HAZZER_rep_f32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_f32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, f64> {
    if !unsafe { TestAll::__HAZZER_rep_f64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_f64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::View<'proto, f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(self) -> __rt::Repeated<'proto, f64> {
    unsafe {
      TestAll::__HAZZER_rep_f64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_f64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::Str> {
    if !unsafe { TestAll::__HAZZER_rep_str.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_str };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(self) -> __rt::Repeated<'proto, __rt::Str> {
    unsafe {
      TestAll::__HAZZER_rep_str.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_str } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, bool> {
    if !unsafe { TestAll::__HAZZER_rep_bool.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_bool };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::View<'proto, bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(self) -> __rt::Repeated<'proto, bool> {
    unsafe {
      TestAll::__HAZZER_rep_bool.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_bool } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, TestAll> {
    if !unsafe { TestAll::__HAZZER_rep_recursive.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_recursive };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::View<'proto, TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(self) -> __rt::Repeated<'proto, TestAll> {
    unsafe {
      TestAll::__HAZZER_rep_recursive.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_recursive } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, TestAll_Nested> {
    if !unsafe { TestAll::__HAZZER_rep_nested.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_nested };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::View<'proto, TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(self) -> __rt::Repeated<'proto, TestAll_Nested> {
    unsafe {
      TestAll::__HAZZER_rep_nested.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_nested } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, TestAll2> {
    if !unsafe { TestAll::__HAZZER_rep_choice.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().rep_choice };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::View<'proto, TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(self) -> __rt::Repeated<'proto, TestAll2> {
    unsafe {
      TestAll::__HAZZER_rep_choice.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().rep_choice } as *mut _ as *mut u8,
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
    fmt.write_str("pz.test.TestAll ")?;
    let mut debug = __rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_TestAll::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use __rt::ptr::ViewFor;
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
    pub(in super) opt_choice: *mut u8,
    pub (in super) rep_i32: __rt::__z::AVec<u32>,
    pub (in super) rep_i64: __rt::__z::AVec<u64>,
    pub (in super) rep_u32: __rt::__z::AVec<u32>,
    pub (in super) rep_u64: __rt::__z::AVec<u64>,
    pub (in super) rep_f32: __rt::__z::AVec<u32>,
    pub (in super) rep_f64: __rt::__z::AVec<u64>,
    pub(crate) rep_str: __rt::__z::AVec<(*mut u8, usize)>,
    pub (in super) rep_bool: __rt::__z::AVec<bool>,
    pub(in super) rep_recursive: __rt::__z::AVec<*mut u8>,
    pub(in super) rep_nested: __rt::__z::AVec<*mut u8>,
    pub(in super) rep_choice: __rt::__z::AVec<*mut u8>,
  }

  pub static TDP_INFO: __rt::__z::tdp::TypeAndFields<{22 + 1}> =
    __rt::__z::tdp::TypeAndFields::<{22 + 1}> {
      msg: __rt::__z::tdp::Type {
        size: {
          let size = TestAll::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const __rt::__z::tdp::Type] = &[
            TestAll::__tdp_info,
            TestAll_Nested::__tdp_info,
            TestAll2::__tdp_info,
          ];
          TYS.as_ptr()
        },
        kind: __rt::__z::tdp::TyKind::Msg,
      },
      fields: [
        __rt::__z::tdp::Field {
          number: 1,
          flags: (__rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_i32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 2,
          flags: (__rt::__z::tdp::Kind::I64 as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_i64,
          ty: 0,
          hasbit: 1,
        },
        __rt::__z::tdp::Field {
          number: 3,
          flags: (__rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_u32,
          ty: 0,
          hasbit: 2,
        },
        __rt::__z::tdp::Field {
          number: 4,
          flags: (__rt::__z::tdp::Kind::I64 as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_u64,
          ty: 0,
          hasbit: 3,
        },
        __rt::__z::tdp::Field {
          number: 5,
          flags: (__rt::__z::tdp::Kind::F32 as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_f32,
          ty: 0,
          hasbit: 4,
        },
        __rt::__z::tdp::Field {
          number: 6,
          flags: (__rt::__z::tdp::Kind::F64 as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_f64,
          ty: 0,
          hasbit: 5,
        },
        __rt::__z::tdp::Field {
          number: 7,
          flags: (__rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_str,
          ty: 0,
          hasbit: 6,
        },
        __rt::__z::tdp::Field {
          number: 8,
          flags: (__rt::__z::tdp::Kind::Bool as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_bool,
          ty: 0,
          hasbit: 7,
        },
        __rt::__z::tdp::Field {
          number: 10,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_recursive,
          ty: 0,
          hasbit: 8,
        },
        __rt::__z::tdp::Field {
          number: 11,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_nested,
          ty: 1,
          hasbit: 9,
        },
        __rt::__z::tdp::Field {
          number: 12,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (0 << 4),
          offset: TestAll::__OFFSET_opt_choice,
          ty: 2,
          hasbit: 10,
        },
        __rt::__z::tdp::Field {
          number: 21,
          flags: (__rt::__z::tdp::Kind::I32 as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_i32,
          ty: 0,
          hasbit: 11,
        },
        __rt::__z::tdp::Field {
          number: 22,
          flags: (__rt::__z::tdp::Kind::I64 as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_i64,
          ty: 0,
          hasbit: 11,
        },
        __rt::__z::tdp::Field {
          number: 23,
          flags: (__rt::__z::tdp::Kind::I32 as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_u32,
          ty: 0,
          hasbit: 11,
        },
        __rt::__z::tdp::Field {
          number: 24,
          flags: (__rt::__z::tdp::Kind::I64 as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_u64,
          ty: 0,
          hasbit: 11,
        },
        __rt::__z::tdp::Field {
          number: 25,
          flags: (__rt::__z::tdp::Kind::F32 as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_f32,
          ty: 0,
          hasbit: 11,
        },
        __rt::__z::tdp::Field {
          number: 26,
          flags: (__rt::__z::tdp::Kind::F64 as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_f64,
          ty: 0,
          hasbit: 11,
        },
        __rt::__z::tdp::Field {
          number: 27,
          flags: (__rt::__z::tdp::Kind::Str as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_str,
          ty: 0,
          hasbit: 11,
        },
        __rt::__z::tdp::Field {
          number: 28,
          flags: (__rt::__z::tdp::Kind::Bool as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_bool,
          ty: 0,
          hasbit: 11,
        },
        __rt::__z::tdp::Field {
          number: 30,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_recursive,
          ty: 0,
          hasbit: 11,
        },
        __rt::__z::tdp::Field {
          number: 31,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_nested,
          ty: 1,
          hasbit: 11,
        },
        __rt::__z::tdp::Field {
          number: 32,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (1 << 4),
          offset: TestAll::__OFFSET_rep_choice,
          ty: 2,
          hasbit: 11,
        },
        __rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __rt::__z::ABox<__priv_TestAll::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto TestAll>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::TestAll> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __rt::__z::ABox<__priv_TestAll::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut TestAll>,
    pub(in super) arena: __rt::__z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::TestAll> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::TestAll> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.test.TestAll.Nested`
pub struct TestAll_Nested {
  ptr: __rt::__z::ABox<__priv_TestAll_Nested::Storage>,
  arena: __rt::__z::RawArena,
}

impl TestAll_Nested {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_TestAll_Nested::Storage = __priv_TestAll_Nested::Storage {
      __hasbits: [0; 1],
      a: 0,
      b: __rt::__z::AVec::new(),
    };
    __rt::View::<Self> {
      ptr: __rt::__z::ABox::from_ptr(&VALUE as *const __priv_TestAll_Nested::Storage as *mut __priv_TestAll_Nested::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = __rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: __rt::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, __rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), __rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> __rt::View<Self> {
    __priv_TestAll_Nested::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_TestAll_Nested::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { TestAll_Nested::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn a(&self) -> __rt::View<'_, i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(&self) -> Option<__rt::View<'_, i32>> {
    if !unsafe { TestAll_Nested::__HAZZER_a.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().a }) })
  }
  pub fn a_mut(&mut self) -> __rt::Mut<'_, i32> {
    self.a_mut_or().into_mut()
  }
  pub fn a_mut_or(&mut self) -> __rt::value::OptMut<'_, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll_Nested::__HAZZER_a,
      )
    }
  }
  pub fn a_set(&mut self, value: i32) {
    self.a_mut().set(value);
  }

  pub fn b(&self) -> __rt::Slice<'_, __rt::Str> {
    if !unsafe { TestAll_Nested::__HAZZER_b.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().b };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn b_at(&self, idx: usize) -> __rt::View<'_, __rt::Str> {
    self.b().at(idx)
  }
  pub fn b_mut(&mut self) -> __rt::Repeated<'_, __rt::Str> {
    unsafe {
      TestAll_Nested::__HAZZER_b.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().b } as *mut _ as *mut u8,
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
  pub fn __tdp_info() -> *const __rt::__z::tdp::Type {
    &__priv_TestAll_Nested::TDP_INFO as *const _ as *const __rt::__z::tdp::Type
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
  #[doc(hidden)]
  pub const __OFFSET_a: u32 = unsafe {
    let msg = TestAll_Nested::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().a as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_a: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: 0,
    offset: Self::__OFFSET_a,
    size: 4,
  };
  #[doc(hidden)]
  pub const __OFFSET_b: u32 = unsafe {
    let msg = TestAll_Nested::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().b as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  #[doc(hidden)]
  pub const __HAZZER_b: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2147483648,
    offset: Self::__OFFSET_b,
    size: (usize::BITS / 8) as i32,
  };
}

impl Default for TestAll_Nested {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for TestAll_Nested {
  type View<'proto> = __priv_TestAll_Nested::View<'proto>;
  type Mut<'proto> = __priv_TestAll_Nested::Mut<'proto>;
}

impl __rt::value::Type for TestAll_Nested {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> __rt::View<'a, Self> {
    __priv_TestAll_Nested::View {
      ptr: __rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: __rt::__z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_TestAll_Nested::Mut {
      ptr: __rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: __rt::__z::RawArena) {
    (&mut *ptr.cast::<__rt::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_TestAll_Nested::View<'proto> {
  pub fn as_view(&self) -> __rt::View<TestAll_Nested> {
    __priv_TestAll_Nested::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn a(self) -> __rt::View<'proto, i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(self) -> Option<__rt::View<'proto, i32>> {
    if !unsafe { TestAll_Nested::__HAZZER_a.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().a }) })
  }

  pub fn b(self) -> __rt::Slice<'proto, __rt::Str> {
    if !unsafe { TestAll_Nested::__HAZZER_b.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().b };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn b_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.b().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __rt::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_TestAll_Nested::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<TestAll_Nested> {
    __priv_TestAll_Nested::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, TestAll_Nested> {
    __priv_TestAll_Nested::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<TestAll_Nested> {
    __priv_TestAll_Nested::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { TestAll_Nested::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), __rt::Error> {
    let mut ctx = __rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, TestAll_Nested::__tdp_info())
  }

  pub fn a(self) -> __rt::View<'proto, i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(self) -> Option<__rt::View<'proto, i32>> {
    if !unsafe { TestAll_Nested::__HAZZER_a.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().a }) })
  }
  pub fn a_mut(self) -> __rt::Mut<'proto, i32> {
    self.a_mut_or().into_mut()
  }
  pub fn a_mut_or(self) -> __rt::value::OptMut<'proto, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll_Nested::__HAZZER_a,
      )
    }
  }
  pub fn a_set(self, value: i32) {
    self.a_mut().set(value);
  }

  pub fn b(self) -> __rt::Slice<'proto, __rt::Str> {
    if !unsafe { TestAll_Nested::__HAZZER_b.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().b };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn b_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.b().at(idx)
  }
  pub fn b_mut(self) -> __rt::Repeated<'proto, __rt::Str> {
    unsafe {
      TestAll_Nested::__HAZZER_b.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().b } as *mut _ as *mut u8,
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
    fmt.write_str("pz.test.TestAll.Nested ")?;
    let mut debug = __rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_TestAll_Nested::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use __rt::ptr::ViewFor;
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
    pub(crate) b: __rt::__z::AVec<(*mut u8, usize)>,
  }

  pub static TDP_INFO: __rt::__z::tdp::TypeAndFields<{2 + 1}> =
    __rt::__z::tdp::TypeAndFields::<{2 + 1}> {
      msg: __rt::__z::tdp::Type {
        size: {
          let size = TestAll_Nested::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const __rt::__z::tdp::Type] = &[
          ];
          TYS.as_ptr()
        },
        kind: __rt::__z::tdp::TyKind::Msg,
      },
      fields: [
        __rt::__z::tdp::Field {
          number: 1,
          flags: (__rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: TestAll_Nested::__OFFSET_a,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 2,
          flags: (__rt::__z::tdp::Kind::Str as u8 as u32) | (1 << 4),
          offset: TestAll_Nested::__OFFSET_b,
          ty: 0,
          hasbit: 1,
        },
        __rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __rt::__z::ABox<__priv_TestAll_Nested::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto TestAll_Nested>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::TestAll_Nested> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __rt::__z::ABox<__priv_TestAll_Nested::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut TestAll_Nested>,
    pub(in super) arena: __rt::__z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::TestAll_Nested> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::TestAll_Nested> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// choice `pz.test.TestAll2`
pub struct TestAll2 {
  ptr: __rt::__z::ABox<__priv_TestAll2::Storage>,
  arena: __rt::__z::RawArena,
}

impl TestAll2 {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_TestAll2::Storage = __priv_TestAll2::Storage {
      which: 0,
      union: __priv_TestAll2::Union { __unset: () },
    };
    __rt::View::<Self> {
      ptr: __rt::__z::ABox::from_ptr(&VALUE as *const __priv_TestAll2::Storage as *mut __priv_TestAll2::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = __rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: __rt::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, __rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), __rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> __rt::View<Self> {
    __priv_TestAll2::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_TestAll2::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(&self) -> TestAll2Cases<'_, __rt::ptr::select::View> {
    self.as_view().cases()
  }

  pub fn cases_mut(&mut self) -> TestAll2Cases<'_, __rt::ptr::select::Mut> {
    self.as_mut().cases_mut()
  }

  pub fn clear(&mut self) {
    unsafe { TestAll2::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn opt_i32(&self) -> __rt::View<'_, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(&self) -> Option<__rt::View<'_, i32>> {
    if !unsafe { TestAll2::__HAZZER_opt_i32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().union.opt_i32 }) })
  }
  pub fn opt_i32_mut(&mut self) -> __rt::Mut<'_, i32> {
    self.opt_i32_mut_or().into_mut()
  }
  pub fn opt_i32_mut_or(&mut self) -> __rt::value::OptMut<'_, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_i32,
      )
    }
  }
  pub fn opt_i32_set(&mut self, value: i32) {
    self.opt_i32_mut().set(value);
  }

  pub fn opt_i64(&self) -> __rt::View<'_, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(&self) -> Option<__rt::View<'_, i64>> {
    if !unsafe { TestAll2::__HAZZER_opt_i64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, i64>(*unsafe { &self.ptr.as_ref().union.opt_i64 }) })
  }
  pub fn opt_i64_mut(&mut self) -> __rt::Mut<'_, i64> {
    self.opt_i64_mut_or().into_mut()
  }
  pub fn opt_i64_mut_or(&mut self) -> __rt::value::OptMut<'_, i64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_i64,
      )
    }
  }
  pub fn opt_i64_set(&mut self, value: i64) {
    self.opt_i64_mut().set(value);
  }

  pub fn opt_u32(&self) -> __rt::View<'_, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(&self) -> Option<__rt::View<'_, u32>> {
    if !unsafe { TestAll2::__HAZZER_opt_u32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().union.opt_u32 }) })
  }
  pub fn opt_u32_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.opt_u32_mut_or().into_mut()
  }
  pub fn opt_u32_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_u32,
      )
    }
  }
  pub fn opt_u32_set(&mut self, value: u32) {
    self.opt_u32_mut().set(value);
  }

  pub fn opt_u64(&self) -> __rt::View<'_, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(&self) -> Option<__rt::View<'_, u64>> {
    if !unsafe { TestAll2::__HAZZER_opt_u64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, u64>(*unsafe { &self.ptr.as_ref().union.opt_u64 }) })
  }
  pub fn opt_u64_mut(&mut self) -> __rt::Mut<'_, u64> {
    self.opt_u64_mut_or().into_mut()
  }
  pub fn opt_u64_mut_or(&mut self) -> __rt::value::OptMut<'_, u64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_u64,
      )
    }
  }
  pub fn opt_u64_set(&mut self, value: u64) {
    self.opt_u64_mut().set(value);
  }

  pub fn opt_f32(&self) -> __rt::View<'_, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(&self) -> Option<__rt::View<'_, f32>> {
    if !unsafe { TestAll2::__HAZZER_opt_f32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, f32>(*unsafe { &self.ptr.as_ref().union.opt_f32 }) })
  }
  pub fn opt_f32_mut(&mut self) -> __rt::Mut<'_, f32> {
    self.opt_f32_mut_or().into_mut()
  }
  pub fn opt_f32_mut_or(&mut self) -> __rt::value::OptMut<'_, f32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_f32,
      )
    }
  }
  pub fn opt_f32_set(&mut self, value: f32) {
    self.opt_f32_mut().set(value);
  }

  pub fn opt_f64(&self) -> __rt::View<'_, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(&self) -> Option<__rt::View<'_, f64>> {
    if !unsafe { TestAll2::__HAZZER_opt_f64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, f64>(*unsafe { &self.ptr.as_ref().union.opt_f64 }) })
  }
  pub fn opt_f64_mut(&mut self) -> __rt::Mut<'_, f64> {
    self.opt_f64_mut_or().into_mut()
  }
  pub fn opt_f64_mut_or(&mut self) -> __rt::value::OptMut<'_, f64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_f64,
      )
    }
  }
  pub fn opt_f64_set(&mut self, value: f64) {
    self.opt_f64_mut().set(value);
  }

  pub fn opt_str(&self) -> __rt::View<'_, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(&self) -> Option<__rt::View<'_, __rt::Str>> {
    if !unsafe { TestAll2::__HAZZER_opt_str.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().union.opt_str };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      __rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn opt_str_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.opt_str_mut_or().into_mut()
  }
  pub fn opt_str_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_str,
      )
    }
  }
  pub fn opt_str_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.opt_str_mut().set(value);
  }

  pub fn opt_bool(&self) -> __rt::View<'_, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(&self) -> Option<__rt::View<'_, bool>> {
    if !unsafe { TestAll2::__HAZZER_opt_bool.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().union.opt_bool }) })
  }
  pub fn opt_bool_mut(&mut self) -> __rt::Mut<'_, bool> {
    self.opt_bool_mut_or().into_mut()
  }
  pub fn opt_bool_mut_or(&mut self) -> __rt::value::OptMut<'_, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_bool,
      )
    }
  }
  pub fn opt_bool_set(&mut self, value: bool) {
    self.opt_bool_mut().set(value);
  }

  pub fn opt_recursive(&self) -> __rt::View<'_, TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(&self) -> Option<__rt::View<'_, TestAll>> {
    if !unsafe { TestAll2::__HAZZER_opt_recursive.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.opt_recursive } as *mut _ as *mut u8)) }
  }
  pub fn opt_recursive_mut(&mut self) -> __rt::Mut<'_, TestAll> {
    self.opt_recursive_mut_or().into_mut()
  }
  pub fn opt_recursive_mut_or(&mut self) -> __rt::value::OptMut<'_, TestAll> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_recursive,
      )
    }
  }

  pub fn opt_nested(&self) -> __rt::View<'_, TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(&self) -> Option<__rt::View<'_, TestAll_Nested>> {
    if !unsafe { TestAll2::__HAZZER_opt_nested.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll_Nested as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.opt_nested } as *mut _ as *mut u8)) }
  }
  pub fn opt_nested_mut(&mut self) -> __rt::Mut<'_, TestAll_Nested> {
    self.opt_nested_mut_or().into_mut()
  }
  pub fn opt_nested_mut_or(&mut self) -> __rt::value::OptMut<'_, TestAll_Nested> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_nested,
      )
    }
  }

  pub fn opt_choice(&self) -> __rt::View<'_, TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(&self) -> Option<__rt::View<'_, TestAll2>> {
    if !unsafe { TestAll2::__HAZZER_opt_choice.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll2 as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.opt_choice } as *mut _ as *mut u8)) }
  }
  pub fn opt_choice_mut(&mut self) -> __rt::Mut<'_, TestAll2> {
    self.opt_choice_mut_or().into_mut()
  }
  pub fn opt_choice_mut_or(&mut self) -> __rt::value::OptMut<'_, TestAll2> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_choice,
      )
    }
  }

  pub fn rep_i32(&self) -> __rt::Slice<'_, i32> {
    if !unsafe { TestAll2::__HAZZER_rep_i32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_i32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i32_at(&self, idx: usize) -> __rt::View<'_, i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(&mut self) -> __rt::Repeated<'_, i32> {
    unsafe {
      TestAll2::__HAZZER_rep_i32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_i32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_i64(&self) -> __rt::Slice<'_, i64> {
    if !unsafe { TestAll2::__HAZZER_rep_i64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_i64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i64_at(&self, idx: usize) -> __rt::View<'_, i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(&mut self) -> __rt::Repeated<'_, i64> {
    unsafe {
      TestAll2::__HAZZER_rep_i64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_i64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u32(&self) -> __rt::Slice<'_, u32> {
    if !unsafe { TestAll2::__HAZZER_rep_u32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_u32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u32_at(&self, idx: usize) -> __rt::View<'_, u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(&mut self) -> __rt::Repeated<'_, u32> {
    unsafe {
      TestAll2::__HAZZER_rep_u32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_u32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u64(&self) -> __rt::Slice<'_, u64> {
    if !unsafe { TestAll2::__HAZZER_rep_u64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_u64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u64_at(&self, idx: usize) -> __rt::View<'_, u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(&mut self) -> __rt::Repeated<'_, u64> {
    unsafe {
      TestAll2::__HAZZER_rep_u64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_u64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f32(&self) -> __rt::Slice<'_, f32> {
    if !unsafe { TestAll2::__HAZZER_rep_f32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_f32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f32_at(&self, idx: usize) -> __rt::View<'_, f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(&mut self) -> __rt::Repeated<'_, f32> {
    unsafe {
      TestAll2::__HAZZER_rep_f32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_f32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f64(&self) -> __rt::Slice<'_, f64> {
    if !unsafe { TestAll2::__HAZZER_rep_f64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_f64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f64_at(&self, idx: usize) -> __rt::View<'_, f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(&mut self) -> __rt::Repeated<'_, f64> {
    unsafe {
      TestAll2::__HAZZER_rep_f64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_f64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_str(&self) -> __rt::Slice<'_, __rt::Str> {
    if !unsafe { TestAll2::__HAZZER_rep_str.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_str };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_str_at(&self, idx: usize) -> __rt::View<'_, __rt::Str> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(&mut self) -> __rt::Repeated<'_, __rt::Str> {
    unsafe {
      TestAll2::__HAZZER_rep_str.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_str } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_bool(&self) -> __rt::Slice<'_, bool> {
    if !unsafe { TestAll2::__HAZZER_rep_bool.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_bool };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_bool_at(&self, idx: usize) -> __rt::View<'_, bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(&mut self) -> __rt::Repeated<'_, bool> {
    unsafe {
      TestAll2::__HAZZER_rep_bool.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_bool } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_recursive(&self) -> __rt::Slice<'_, TestAll> {
    if !unsafe { TestAll2::__HAZZER_rep_recursive.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_recursive };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_recursive_at(&self, idx: usize) -> __rt::View<'_, TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(&mut self) -> __rt::Repeated<'_, TestAll> {
    unsafe {
      TestAll2::__HAZZER_rep_recursive.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_recursive } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_nested(&self) -> __rt::Slice<'_, TestAll_Nested> {
    if !unsafe { TestAll2::__HAZZER_rep_nested.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_nested };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_nested_at(&self, idx: usize) -> __rt::View<'_, TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(&mut self) -> __rt::Repeated<'_, TestAll_Nested> {
    unsafe {
      TestAll2::__HAZZER_rep_nested.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_nested } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_choice(&self) -> __rt::Slice<'_, TestAll2> {
    if !unsafe { TestAll2::__HAZZER_rep_choice.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_choice };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_choice_at(&self, idx: usize) -> __rt::View<'_, TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(&mut self) -> __rt::Repeated<'_, TestAll2> {
    unsafe {
      TestAll2::__HAZZER_rep_choice.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_choice } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_TestAll2::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_TestAll2::Storage>()).which = 0;
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const __rt::__z::tdp::Type {
    &__priv_TestAll2::TDP_INFO as *const _ as *const __rt::__z::tdp::Type
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
  #[doc(hidden)]
  pub const __OFFSET_opt_i32: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_i32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -1,
    offset: Self::__OFFSET_opt_i32,
    size: 4,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_i64: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_i64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -2,
    offset: Self::__OFFSET_opt_i64,
    size: 8,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_u32: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_u32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -3,
    offset: Self::__OFFSET_opt_u32,
    size: 4,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_u64: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_u64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -4,
    offset: Self::__OFFSET_opt_u64,
    size: 8,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_f32: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_f32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -5,
    offset: Self::__OFFSET_opt_f32,
    size: 4,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_f64: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_f64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -6,
    offset: Self::__OFFSET_opt_f64,
    size: 8,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_str: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_str: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -7,
    offset: Self::__OFFSET_opt_str,
    size: (usize::BITS / 8 * 2) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_bool: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_bool: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -8,
    offset: Self::__OFFSET_opt_bool,
    size: 1,
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_recursive: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_recursive: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -10,
    offset: Self::__OFFSET_opt_recursive,
    size: -(TestAll::__LAYOUT.size() as i32),
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_nested: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_nested: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -11,
    offset: Self::__OFFSET_opt_nested,
    size: -(TestAll_Nested::__LAYOUT.size() as i32),
  };
  #[doc(hidden)]
  pub const __OFFSET_opt_choice: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_opt_choice: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -12,
    offset: Self::__OFFSET_opt_choice,
    size: -(TestAll2::__LAYOUT.size() as i32),
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_i32: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_i32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -21,
    offset: Self::__OFFSET_rep_i32,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_i64: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_i64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -22,
    offset: Self::__OFFSET_rep_i64,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_u32: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_u32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -23,
    offset: Self::__OFFSET_rep_u32,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_u64: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_u64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -24,
    offset: Self::__OFFSET_rep_u64,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_f32: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_f32: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -25,
    offset: Self::__OFFSET_rep_f32,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_f64: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_f64: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -26,
    offset: Self::__OFFSET_rep_f64,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_str: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_str: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -27,
    offset: Self::__OFFSET_rep_str,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_bool: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_bool: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -28,
    offset: Self::__OFFSET_rep_bool,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_recursive: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_recursive: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -30,
    offset: Self::__OFFSET_rep_recursive,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_nested: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_nested: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -31,
    offset: Self::__OFFSET_rep_nested,
    size: (usize::BITS / 8) as i32,
  };
  #[doc(hidden)]
  pub const __OFFSET_rep_choice: u32 = __priv_TestAll2::UNION_OFFSET as u32;
  #[doc(hidden)]
  pub const __HAZZER_rep_choice: &__rt::__z::Hazzer = &__rt::__z::Hazzer {
    hasbit_or_number: -32,
    offset: Self::__OFFSET_rep_choice,
    size: (usize::BITS / 8) as i32,
  };
}

pub enum TestAll2Cases<'proto, Which: __rt::ptr::select::Select> {
  Unset(std::marker::PhantomData<&'proto Which>),
  OptI32(__rt::ptr::Proxy<'proto, i32, Which>),
  OptI64(__rt::ptr::Proxy<'proto, i64, Which>),
  OptU32(__rt::ptr::Proxy<'proto, u32, Which>),
  OptU64(__rt::ptr::Proxy<'proto, u64, Which>),
  OptF32(__rt::ptr::Proxy<'proto, f32, Which>),
  OptF64(__rt::ptr::Proxy<'proto, f64, Which>),
  OptStr(__rt::ptr::Proxy<'proto, __rt::Str, Which>),
  OptBool(__rt::ptr::Proxy<'proto, bool, Which>),
  OptRecursive(__rt::ptr::Proxy<'proto, TestAll, Which>),
  OptNested(__rt::ptr::Proxy<'proto, TestAll_Nested, Which>),
  OptChoice(__rt::ptr::Proxy<'proto, TestAll2, Which>),
  RepI32(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<i32>, Which>),
  RepI64(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<i64>, Which>),
  RepU32(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<u32>, Which>),
  RepU64(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<u64>, Which>),
  RepF32(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<f32>, Which>),
  RepF64(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<f64>, Which>),
  RepStr(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<__rt::Str>, Which>),
  RepBool(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<bool>, Which>),
  RepRecursive(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<TestAll>, Which>),
  RepNested(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<TestAll_Nested>, Which>),
  RepChoice(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<TestAll2>, Which>),
}

impl Default for TestAll2 {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for TestAll2 {
  type View<'proto> = __priv_TestAll2::View<'proto>;
  type Mut<'proto> = __priv_TestAll2::Mut<'proto>;
}

impl<'proto> __priv_TestAll2::View<'proto> {
  pub fn as_view(&self) -> __rt::View<TestAll2> {
    __priv_TestAll2::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn cases(self) -> TestAll2Cases<'proto, __rt::ptr::select::View> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      let raw = self.ptr.as_ptr().add(__priv_TestAll2::UNION_OFFSET);
      match number {
        0 => TestAll2Cases::Unset(std::marker::PhantomData),
        1 => TestAll2Cases::OptI32(<i32 as __rt::value::Type>::__make_view(raw)),
        2 => TestAll2Cases::OptI64(<i64 as __rt::value::Type>::__make_view(raw)),
        3 => TestAll2Cases::OptU32(<u32 as __rt::value::Type>::__make_view(raw)),
        4 => TestAll2Cases::OptU64(<u64 as __rt::value::Type>::__make_view(raw)),
        5 => TestAll2Cases::OptF32(<f32 as __rt::value::Type>::__make_view(raw)),
        6 => TestAll2Cases::OptF64(<f64 as __rt::value::Type>::__make_view(raw)),
        7 => TestAll2Cases::OptStr(<__rt::Str as __rt::value::Type>::__make_view(raw)),
        8 => TestAll2Cases::OptBool(<bool as __rt::value::Type>::__make_view(raw)),
        10 => TestAll2Cases::OptRecursive(<TestAll as __rt::value::Type>::__make_view(raw)),
        11 => TestAll2Cases::OptNested(<TestAll_Nested as __rt::value::Type>::__make_view(raw)),
        12 => TestAll2Cases::OptChoice(<TestAll2 as __rt::value::Type>::__make_view(raw)),
        21 => TestAll2Cases::RepI32(__rt::Repeated::<i32>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        22 => TestAll2Cases::RepI64(__rt::Repeated::<i64>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        23 => TestAll2Cases::RepU32(__rt::Repeated::<u32>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        24 => TestAll2Cases::RepU64(__rt::Repeated::<u64>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        25 => TestAll2Cases::RepF32(__rt::Repeated::<f32>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        26 => TestAll2Cases::RepF64(__rt::Repeated::<f64>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        27 => TestAll2Cases::RepStr(__rt::Repeated::<__rt::Str>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        28 => TestAll2Cases::RepBool(__rt::Repeated::<bool>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        30 => TestAll2Cases::RepRecursive(__rt::Repeated::<TestAll>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        31 => TestAll2Cases::RepNested(__rt::Repeated::<TestAll_Nested>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        32 => TestAll2Cases::RepChoice(__rt::Repeated::<TestAll2>::__wrap(raw, __rt::__z::RawArena::null()).into_view()),
        _ => unreachable!(),
      }
    }
  }

  pub fn opt_i32(self) -> __rt::View<'proto, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> Option<__rt::View<'proto, i32>> {
    if !unsafe { TestAll2::__HAZZER_opt_i32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().union.opt_i32 }) })
  }

  pub fn opt_i64(self) -> __rt::View<'proto, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> Option<__rt::View<'proto, i64>> {
    if !unsafe { TestAll2::__HAZZER_opt_i64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, i64>(*unsafe { &self.ptr.as_ref().union.opt_i64 }) })
  }

  pub fn opt_u32(self) -> __rt::View<'proto, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> Option<__rt::View<'proto, u32>> {
    if !unsafe { TestAll2::__HAZZER_opt_u32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().union.opt_u32 }) })
  }

  pub fn opt_u64(self) -> __rt::View<'proto, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> Option<__rt::View<'proto, u64>> {
    if !unsafe { TestAll2::__HAZZER_opt_u64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, u64>(*unsafe { &self.ptr.as_ref().union.opt_u64 }) })
  }

  pub fn opt_f32(self) -> __rt::View<'proto, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> Option<__rt::View<'proto, f32>> {
    if !unsafe { TestAll2::__HAZZER_opt_f32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, f32>(*unsafe { &self.ptr.as_ref().union.opt_f32 }) })
  }

  pub fn opt_f64(self) -> __rt::View<'proto, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> Option<__rt::View<'proto, f64>> {
    if !unsafe { TestAll2::__HAZZER_opt_f64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, f64>(*unsafe { &self.ptr.as_ref().union.opt_f64 }) })
  }

  pub fn opt_str(self) -> __rt::View<'proto, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> Option<__rt::View<'proto, __rt::Str>> {
    if !unsafe { TestAll2::__HAZZER_opt_str.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().union.opt_str };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      __rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn opt_bool(self) -> __rt::View<'proto, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> Option<__rt::View<'proto, bool>> {
    if !unsafe { TestAll2::__HAZZER_opt_bool.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().union.opt_bool }) })
  }

  pub fn opt_recursive(self) -> __rt::View<'proto, TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> Option<__rt::View<'proto, TestAll>> {
    if !unsafe { TestAll2::__HAZZER_opt_recursive.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.opt_recursive } as *mut _ as *mut u8)) }
  }

  pub fn opt_nested(self) -> __rt::View<'proto, TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> Option<__rt::View<'proto, TestAll_Nested>> {
    if !unsafe { TestAll2::__HAZZER_opt_nested.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll_Nested as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.opt_nested } as *mut _ as *mut u8)) }
  }

  pub fn opt_choice(self) -> __rt::View<'proto, TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> Option<__rt::View<'proto, TestAll2>> {
    if !unsafe { TestAll2::__HAZZER_opt_choice.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll2 as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.opt_choice } as *mut _ as *mut u8)) }
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, i32> {
    if !unsafe { TestAll2::__HAZZER_rep_i32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_i32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::View<'proto, i32> {
    self.rep_i32().at(idx)
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, i64> {
    if !unsafe { TestAll2::__HAZZER_rep_i64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_i64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::View<'proto, i64> {
    self.rep_i64().at(idx)
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, u32> {
    if !unsafe { TestAll2::__HAZZER_rep_u32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_u32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.rep_u32().at(idx)
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, u64> {
    if !unsafe { TestAll2::__HAZZER_rep_u64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_u64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::View<'proto, u64> {
    self.rep_u64().at(idx)
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, f32> {
    if !unsafe { TestAll2::__HAZZER_rep_f32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_f32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::View<'proto, f32> {
    self.rep_f32().at(idx)
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, f64> {
    if !unsafe { TestAll2::__HAZZER_rep_f64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_f64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::View<'proto, f64> {
    self.rep_f64().at(idx)
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::Str> {
    if !unsafe { TestAll2::__HAZZER_rep_str.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_str };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.rep_str().at(idx)
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, bool> {
    if !unsafe { TestAll2::__HAZZER_rep_bool.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_bool };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::View<'proto, bool> {
    self.rep_bool().at(idx)
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, TestAll> {
    if !unsafe { TestAll2::__HAZZER_rep_recursive.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_recursive };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::View<'proto, TestAll> {
    self.rep_recursive().at(idx)
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, TestAll_Nested> {
    if !unsafe { TestAll2::__HAZZER_rep_nested.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_nested };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::View<'proto, TestAll_Nested> {
    self.rep_nested().at(idx)
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, TestAll2> {
    if !unsafe { TestAll2::__HAZZER_rep_choice.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_choice };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::View<'proto, TestAll2> {
    self.rep_choice().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __rt::__z::Debug) -> std::fmt::Result {
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
    if let Some(value) = self.opt_choice_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_choice")?;
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
    for value in self.rep_choice() {
      if count != 0 { debug.comma(false)?; }
      debug.field("rep_choice")?;
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

impl Default for __priv_TestAll2::View<'_> {
  fn default() -> Self {
    TestAll2::DEFAULT
  }
}

impl<'proto> __priv_TestAll2::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<TestAll2> {
    __priv_TestAll2::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, TestAll2> {
    __priv_TestAll2::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<TestAll2> {
    __priv_TestAll2::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(self) -> TestAll2Cases<'proto, __rt::ptr::select::View> {
    self.into_view().cases()
  }

  pub fn cases_mut(self) -> TestAll2Cases<'proto, __rt::ptr::select::Mut> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      let raw = self.ptr.as_ptr().add(__priv_TestAll2::UNION_OFFSET);
      match number {
        0 => TestAll2Cases::Unset(std::marker::PhantomData),
        1 => TestAll2Cases::OptI32(<i32 as __rt::value::Type>::__make_mut(raw, self.arena)),
        2 => TestAll2Cases::OptI64(<i64 as __rt::value::Type>::__make_mut(raw, self.arena)),
        3 => TestAll2Cases::OptU32(<u32 as __rt::value::Type>::__make_mut(raw, self.arena)),
        4 => TestAll2Cases::OptU64(<u64 as __rt::value::Type>::__make_mut(raw, self.arena)),
        5 => TestAll2Cases::OptF32(<f32 as __rt::value::Type>::__make_mut(raw, self.arena)),
        6 => TestAll2Cases::OptF64(<f64 as __rt::value::Type>::__make_mut(raw, self.arena)),
        7 => TestAll2Cases::OptStr(<__rt::Str as __rt::value::Type>::__make_mut(raw, self.arena)),
        8 => TestAll2Cases::OptBool(<bool as __rt::value::Type>::__make_mut(raw, self.arena)),
        10 => TestAll2Cases::OptRecursive(<TestAll as __rt::value::Type>::__make_mut(raw, self.arena)),
        11 => TestAll2Cases::OptNested(<TestAll_Nested as __rt::value::Type>::__make_mut(raw, self.arena)),
        12 => TestAll2Cases::OptChoice(<TestAll2 as __rt::value::Type>::__make_mut(raw, self.arena)),
        21 => TestAll2Cases::RepI32(__rt::Repeated::<i32>::__wrap(raw, self.arena)),
        22 => TestAll2Cases::RepI64(__rt::Repeated::<i64>::__wrap(raw, self.arena)),
        23 => TestAll2Cases::RepU32(__rt::Repeated::<u32>::__wrap(raw, self.arena)),
        24 => TestAll2Cases::RepU64(__rt::Repeated::<u64>::__wrap(raw, self.arena)),
        25 => TestAll2Cases::RepF32(__rt::Repeated::<f32>::__wrap(raw, self.arena)),
        26 => TestAll2Cases::RepF64(__rt::Repeated::<f64>::__wrap(raw, self.arena)),
        27 => TestAll2Cases::RepStr(__rt::Repeated::<__rt::Str>::__wrap(raw, self.arena)),
        28 => TestAll2Cases::RepBool(__rt::Repeated::<bool>::__wrap(raw, self.arena)),
        30 => TestAll2Cases::RepRecursive(__rt::Repeated::<TestAll>::__wrap(raw, self.arena)),
        31 => TestAll2Cases::RepNested(__rt::Repeated::<TestAll_Nested>::__wrap(raw, self.arena)),
        32 => TestAll2Cases::RepChoice(__rt::Repeated::<TestAll2>::__wrap(raw, self.arena)),
        _ => unreachable!(),
      }
    }
  }

  pub fn clear(self) {
    unsafe { TestAll2::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), __rt::Error> {
    let mut ctx = __rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, TestAll2::__tdp_info())
  }

  pub fn opt_i32(self) -> __rt::View<'proto, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> Option<__rt::View<'proto, i32>> {
    if !unsafe { TestAll2::__HAZZER_opt_i32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().union.opt_i32 }) })
  }
  pub fn opt_i32_mut(self) -> __rt::Mut<'proto, i32> {
    self.opt_i32_mut_or().into_mut()
  }
  pub fn opt_i32_mut_or(self) -> __rt::value::OptMut<'proto, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_i32,
      )
    }
  }
  pub fn opt_i32_set(self, value: i32) {
    self.opt_i32_mut().set(value);
  }

  pub fn opt_i64(self) -> __rt::View<'proto, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> Option<__rt::View<'proto, i64>> {
    if !unsafe { TestAll2::__HAZZER_opt_i64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, i64>(*unsafe { &self.ptr.as_ref().union.opt_i64 }) })
  }
  pub fn opt_i64_mut(self) -> __rt::Mut<'proto, i64> {
    self.opt_i64_mut_or().into_mut()
  }
  pub fn opt_i64_mut_or(self) -> __rt::value::OptMut<'proto, i64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_i64,
      )
    }
  }
  pub fn opt_i64_set(self, value: i64) {
    self.opt_i64_mut().set(value);
  }

  pub fn opt_u32(self) -> __rt::View<'proto, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> Option<__rt::View<'proto, u32>> {
    if !unsafe { TestAll2::__HAZZER_opt_u32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().union.opt_u32 }) })
  }
  pub fn opt_u32_mut(self) -> __rt::Mut<'proto, u32> {
    self.opt_u32_mut_or().into_mut()
  }
  pub fn opt_u32_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_u32,
      )
    }
  }
  pub fn opt_u32_set(self, value: u32) {
    self.opt_u32_mut().set(value);
  }

  pub fn opt_u64(self) -> __rt::View<'proto, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> Option<__rt::View<'proto, u64>> {
    if !unsafe { TestAll2::__HAZZER_opt_u64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, u64>(*unsafe { &self.ptr.as_ref().union.opt_u64 }) })
  }
  pub fn opt_u64_mut(self) -> __rt::Mut<'proto, u64> {
    self.opt_u64_mut_or().into_mut()
  }
  pub fn opt_u64_mut_or(self) -> __rt::value::OptMut<'proto, u64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_u64,
      )
    }
  }
  pub fn opt_u64_set(self, value: u64) {
    self.opt_u64_mut().set(value);
  }

  pub fn opt_f32(self) -> __rt::View<'proto, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> Option<__rt::View<'proto, f32>> {
    if !unsafe { TestAll2::__HAZZER_opt_f32.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, f32>(*unsafe { &self.ptr.as_ref().union.opt_f32 }) })
  }
  pub fn opt_f32_mut(self) -> __rt::Mut<'proto, f32> {
    self.opt_f32_mut_or().into_mut()
  }
  pub fn opt_f32_mut_or(self) -> __rt::value::OptMut<'proto, f32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_f32,
      )
    }
  }
  pub fn opt_f32_set(self, value: f32) {
    self.opt_f32_mut().set(value);
  }

  pub fn opt_f64(self) -> __rt::View<'proto, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> Option<__rt::View<'proto, f64>> {
    if !unsafe { TestAll2::__HAZZER_opt_f64.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u64, f64>(*unsafe { &self.ptr.as_ref().union.opt_f64 }) })
  }
  pub fn opt_f64_mut(self) -> __rt::Mut<'proto, f64> {
    self.opt_f64_mut_or().into_mut()
  }
  pub fn opt_f64_mut_or(self) -> __rt::value::OptMut<'proto, f64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_f64,
      )
    }
  }
  pub fn opt_f64_set(self, value: f64) {
    self.opt_f64_mut().set(value);
  }

  pub fn opt_str(self) -> __rt::View<'proto, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> Option<__rt::View<'proto, __rt::Str>> {
    if !unsafe { TestAll2::__HAZZER_opt_str.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().union.opt_str };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      __rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn opt_str_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.opt_str_mut_or().into_mut()
  }
  pub fn opt_str_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_str,
      )
    }
  }
  pub fn opt_str_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.opt_str_mut().set(value);
  }

  pub fn opt_bool(self) -> __rt::View<'proto, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> Option<__rt::View<'proto, bool>> {
    if !unsafe { TestAll2::__HAZZER_opt_bool.has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().union.opt_bool }) })
  }
  pub fn opt_bool_mut(self) -> __rt::Mut<'proto, bool> {
    self.opt_bool_mut_or().into_mut()
  }
  pub fn opt_bool_mut_or(self) -> __rt::value::OptMut<'proto, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_bool,
      )
    }
  }
  pub fn opt_bool_set(self, value: bool) {
    self.opt_bool_mut().set(value);
  }

  pub fn opt_recursive(self) -> __rt::View<'proto, TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> Option<__rt::View<'proto, TestAll>> {
    if !unsafe { TestAll2::__HAZZER_opt_recursive.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.opt_recursive } as *mut _ as *mut u8)) }
  }
  pub fn opt_recursive_mut(self) -> __rt::Mut<'proto, TestAll> {
    self.opt_recursive_mut_or().into_mut()
  }
  pub fn opt_recursive_mut_or(self) -> __rt::value::OptMut<'proto, TestAll> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_recursive,
      )
    }
  }

  pub fn opt_nested(self) -> __rt::View<'proto, TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> Option<__rt::View<'proto, TestAll_Nested>> {
    if !unsafe { TestAll2::__HAZZER_opt_nested.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll_Nested as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.opt_nested } as *mut _ as *mut u8)) }
  }
  pub fn opt_nested_mut(self) -> __rt::Mut<'proto, TestAll_Nested> {
    self.opt_nested_mut_or().into_mut()
  }
  pub fn opt_nested_mut_or(self) -> __rt::value::OptMut<'proto, TestAll_Nested> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_nested,
      )
    }
  }

  pub fn opt_choice(self) -> __rt::View<'proto, TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> Option<__rt::View<'proto, TestAll2>> {
    if !unsafe { TestAll2::__HAZZER_opt_choice.has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<TestAll2 as __rt::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.opt_choice } as *mut _ as *mut u8)) }
  }
  pub fn opt_choice_mut(self) -> __rt::Mut<'proto, TestAll2> {
    self.opt_choice_mut_or().into_mut()
  }
  pub fn opt_choice_mut_or(self) -> __rt::value::OptMut<'proto, TestAll2> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        TestAll2::__HAZZER_opt_choice,
      )
    }
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, i32> {
    if !unsafe { TestAll2::__HAZZER_rep_i32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_i32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::View<'proto, i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(self) -> __rt::Repeated<'proto, i32> {
    unsafe {
      TestAll2::__HAZZER_rep_i32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_i32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, i64> {
    if !unsafe { TestAll2::__HAZZER_rep_i64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_i64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::View<'proto, i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(self) -> __rt::Repeated<'proto, i64> {
    unsafe {
      TestAll2::__HAZZER_rep_i64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_i64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, u32> {
    if !unsafe { TestAll2::__HAZZER_rep_u32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_u32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(self) -> __rt::Repeated<'proto, u32> {
    unsafe {
      TestAll2::__HAZZER_rep_u32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_u32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, u64> {
    if !unsafe { TestAll2::__HAZZER_rep_u64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_u64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::View<'proto, u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(self) -> __rt::Repeated<'proto, u64> {
    unsafe {
      TestAll2::__HAZZER_rep_u64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_u64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, f32> {
    if !unsafe { TestAll2::__HAZZER_rep_f32.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_f32 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::View<'proto, f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(self) -> __rt::Repeated<'proto, f32> {
    unsafe {
      TestAll2::__HAZZER_rep_f32.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_f32 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, f64> {
    if !unsafe { TestAll2::__HAZZER_rep_f64.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_f64 };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::View<'proto, f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(self) -> __rt::Repeated<'proto, f64> {
    unsafe {
      TestAll2::__HAZZER_rep_f64.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_f64 } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::Str> {
    if !unsafe { TestAll2::__HAZZER_rep_str.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_str };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(self) -> __rt::Repeated<'proto, __rt::Str> {
    unsafe {
      TestAll2::__HAZZER_rep_str.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_str } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, bool> {
    if !unsafe { TestAll2::__HAZZER_rep_bool.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_bool };
      __rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::View<'proto, bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(self) -> __rt::Repeated<'proto, bool> {
    unsafe {
      TestAll2::__HAZZER_rep_bool.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_bool } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, TestAll> {
    if !unsafe { TestAll2::__HAZZER_rep_recursive.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_recursive };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::View<'proto, TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(self) -> __rt::Repeated<'proto, TestAll> {
    unsafe {
      TestAll2::__HAZZER_rep_recursive.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_recursive } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, TestAll_Nested> {
    if !unsafe { TestAll2::__HAZZER_rep_nested.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_nested };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::View<'proto, TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(self) -> __rt::Repeated<'proto, TestAll_Nested> {
    unsafe {
      TestAll2::__HAZZER_rep_nested.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_nested } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, TestAll2> {
    if !unsafe { TestAll2::__HAZZER_rep_choice.has(self.ptr.as_ptr()) } { return __rt::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().union.rep_choice };
      __rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::View<'proto, TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(self) -> __rt::Repeated<'proto, TestAll2> {
    unsafe {
      TestAll2::__HAZZER_rep_choice.init(self.ptr.as_ptr(), self.arena);
      __rt::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().union.rep_choice } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

}

impl Drop for TestAll2 {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_TestAll2::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.test.TestAll2 ")?;
    let mut debug = __rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_TestAll2::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use __rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for TestAll2 {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __rt::value::Type for TestAll2 {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> __rt::View<'a, Self> {
    __priv_TestAll2::View {
      ptr: __rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: __rt::__z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_TestAll2::Mut {
      ptr: __rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: __rt::__z::RawArena) {
    (&mut *ptr.cast::<__rt::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

mod __priv_TestAll2 {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(super) which: u32,
    pub(super) union: Union,
  }

  #[repr(C)]
  pub union Union {
    pub(super) __unset: (),
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
    pub(in super) opt_choice: *mut u8,
    pub (in super) rep_i32: __rt::__z::AVec<u32>,
    pub (in super) rep_i64: __rt::__z::AVec<u64>,
    pub (in super) rep_u32: __rt::__z::AVec<u32>,
    pub (in super) rep_u64: __rt::__z::AVec<u64>,
    pub (in super) rep_f32: __rt::__z::AVec<u32>,
    pub (in super) rep_f64: __rt::__z::AVec<u64>,
    pub(crate) rep_str: __rt::__z::AVec<(*mut u8, usize)>,
    pub (in super) rep_bool: __rt::__z::AVec<bool>,
    pub(in super) rep_recursive: __rt::__z::AVec<*mut u8>,
    pub(in super) rep_nested: __rt::__z::AVec<*mut u8>,
    pub(in super) rep_choice: __rt::__z::AVec<*mut u8>,
  }

  pub const UNION_OFFSET: usize = {
    let align = std::mem::align_of::<__priv_TestAll2::Union>();
    if align < 4 { 4 } else { align }
  };

  pub static TDP_INFO: __rt::__z::tdp::TypeAndFields<{22 + 1}> =
    __rt::__z::tdp::TypeAndFields::<{22 + 1}> {
      msg: __rt::__z::tdp::Type {
        size: {
          let size = TestAll2::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const __rt::__z::tdp::Type] = &[
            TestAll::__tdp_info,
            TestAll_Nested::__tdp_info,
            TestAll2::__tdp_info,
          ];
          TYS.as_ptr()
        },
        kind: __rt::__z::tdp::TyKind::Choice,
      },
      fields: [
        __rt::__z::tdp::Field {
          number: 1,
          flags: (__rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 2,
          flags: (__rt::__z::tdp::Kind::I64 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 3,
          flags: (__rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 4,
          flags: (__rt::__z::tdp::Kind::I64 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 5,
          flags: (__rt::__z::tdp::Kind::F32 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 6,
          flags: (__rt::__z::tdp::Kind::F64 as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 7,
          flags: (__rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 8,
          flags: (__rt::__z::tdp::Kind::Bool as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 10,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 11,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 1,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 12,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (0 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 2,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 21,
          flags: (__rt::__z::tdp::Kind::I32 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 22,
          flags: (__rt::__z::tdp::Kind::I64 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 23,
          flags: (__rt::__z::tdp::Kind::I32 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 24,
          flags: (__rt::__z::tdp::Kind::I64 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 25,
          flags: (__rt::__z::tdp::Kind::F32 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 26,
          flags: (__rt::__z::tdp::Kind::F64 as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 27,
          flags: (__rt::__z::tdp::Kind::Str as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 28,
          flags: (__rt::__z::tdp::Kind::Bool as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 30,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 0,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 31,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 1,
          hasbit: 0,
        },
        __rt::__z::tdp::Field {
          number: 32,
          flags: (__rt::__z::tdp::Kind::Type as u8 as u32) | (1 << 4),
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          ty: 2,
          hasbit: 0,
        },
        __rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __rt::__z::ABox<__priv_TestAll2::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto TestAll2>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::TestAll2> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __rt::__z::ABox<__priv_TestAll2::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut TestAll2>,
    pub(in super) arena: __rt::__z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::TestAll2> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::TestAll2> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

