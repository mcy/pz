// ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]

#![allow(clippy::derivable_impls)]
#![allow(clippy::identity_op)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::transmute_float_to_int)]
#![allow(clippy::transmute_int_to_float)]
#![allow(clippy::useless_transmute)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::wrong_self_convention)]

#![no_implicit_prelude]

extern crate pz as __rt;
use __rt::__z;
use __z::std as __s;

use __s::default::Default as _;

pub mod test {
use super::{__root, __rt, __z, __s};
use __s::default::Default as _;
/// message `pz.test.TestAll`
pub struct TestAll {
  ptr: __z::ABox<__priv_TestAll::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_TestAll::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::test::TestAll {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_TestAll::Storage = __priv_TestAll::Storage {
      __hasbits: [0; 1],
      opt_i32: 0,
      opt_i64: 0,
      opt_u32: 0,
      opt_u64: 0,
      opt_f32: 0,
      opt_f64: 0,
      opt_str: __z::RawStr::new(),
      opt_bool: false,
      opt_recursive: 0 as *mut u8,
      opt_nested: 0 as *mut u8,
      opt_choice: 0 as *mut u8,
      rep_i32: __z::AVec::new(),
      rep_i64: __z::AVec::new(),
      rep_u32: __z::AVec::new(),
      rep_u64: __z::AVec::new(),
      rep_f32: __z::AVec::new(),
      rep_f64: __z::AVec::new(),
      rep_str: __z::AVec::new(),
      rep_bool: __z::AVec::new(),
      rep_recursive: __z::AVec::new(),
      rep_nested: __z::AVec::new(),
      rep_choice: __z::AVec::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_TestAll::Storage as *mut __priv_TestAll::Storage as *mut u8),
      _ph: __s::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = __z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: __z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    __s::result::Result::Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> __rt::View<Self> {
    __priv_TestAll::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_TestAll::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::test::TestAll::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn opt_i32(&self) -> __rt::View<'_, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(&self) -> __s::option::Option<__rt::View<'_, i32>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_i32_mut(&mut self) -> __rt::Mut<'_, i32> {
    self.opt_i32_mut_or().into_mut()
  }
  pub fn opt_i32_mut_or(&mut self) -> __rt::value::OptMut<'_, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(0),
      )
    }
  }
  pub fn opt_i32_set(&mut self, value: i32) {
    self.opt_i32_mut().set(value);
  }

  pub fn opt_i64(&self) -> __rt::View<'_, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(&self) -> __s::option::Option<__rt::View<'_, i64>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_i64_mut(&mut self) -> __rt::Mut<'_, i64> {
    self.opt_i64_mut_or().into_mut()
  }
  pub fn opt_i64_mut_or(&mut self) -> __rt::value::OptMut<'_, i64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(1),
      )
    }
  }
  pub fn opt_i64_set(&mut self, value: i64) {
    self.opt_i64_mut().set(value);
  }

  pub fn opt_u32(&self) -> __rt::View<'_, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(&self) -> __s::option::Option<__rt::View<'_, u32>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_u32_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.opt_u32_mut_or().into_mut()
  }
  pub fn opt_u32_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(2),
      )
    }
  }
  pub fn opt_u32_set(&mut self, value: u32) {
    self.opt_u32_mut().set(value);
  }

  pub fn opt_u64(&self) -> __rt::View<'_, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(&self) -> __s::option::Option<__rt::View<'_, u64>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_u64_mut(&mut self) -> __rt::Mut<'_, u64> {
    self.opt_u64_mut_or().into_mut()
  }
  pub fn opt_u64_mut_or(&mut self) -> __rt::value::OptMut<'_, u64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(3),
      )
    }
  }
  pub fn opt_u64_set(&mut self, value: u64) {
    self.opt_u64_mut().set(value);
  }

  pub fn opt_f32(&self) -> __rt::View<'_, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(&self) -> __s::option::Option<__rt::View<'_, f32>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_f32_mut(&mut self) -> __rt::Mut<'_, f32> {
    self.opt_f32_mut_or().into_mut()
  }
  pub fn opt_f32_mut_or(&mut self) -> __rt::value::OptMut<'_, f32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(4),
      )
    }
  }
  pub fn opt_f32_set(&mut self, value: f32) {
    self.opt_f32_mut().set(value);
  }

  pub fn opt_f64(&self) -> __rt::View<'_, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(&self) -> __s::option::Option<__rt::View<'_, f64>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_f64_mut(&mut self) -> __rt::Mut<'_, f64> {
    self.opt_f64_mut_or().into_mut()
  }
  pub fn opt_f64_mut_or(&mut self) -> __rt::value::OptMut<'_, f64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(5),
      )
    }
  }
  pub fn opt_f64_set(&mut self, value: f64) {
    self.opt_f64_mut().set(value);
  }

  pub fn opt_str(&self) -> __rt::View<'_, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_str_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.opt_str_mut_or().into_mut()
  }
  pub fn opt_str_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(6),
      )
    }
  }
  pub fn opt_str_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.opt_str_mut().set(value);
  }

  pub fn opt_bool(&self) -> __rt::View<'_, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(&self) -> __s::option::Option<__rt::View<'_, bool>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(7);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_bool_mut(&mut self) -> __rt::Mut<'_, bool> {
    self.opt_bool_mut_or().into_mut()
  }
  pub fn opt_bool_mut_or(&mut self) -> __rt::value::OptMut<'_, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(7),
      )
    }
  }
  pub fn opt_bool_set(&mut self, value: bool) {
    self.opt_bool_mut().set(value);
  }

  pub fn opt_recursive(&self) -> __rt::View<'_, __root::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::test::TestAll>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(8);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_recursive_mut(&mut self) -> __rt::Mut<'_, __root::pz::test::TestAll> {
    self.opt_recursive_mut_or().into_mut()
  }
  pub fn opt_recursive_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::test::TestAll> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(8),
      )
    }
  }

  pub fn opt_nested(&self) -> __rt::View<'_, __root::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::test::TestAll_Nested>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(9);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_nested_mut(&mut self) -> __rt::Mut<'_, __root::pz::test::TestAll_Nested> {
    self.opt_nested_mut_or().into_mut()
  }
  pub fn opt_nested_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::test::TestAll_Nested> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(9),
      )
    }
  }

  pub fn opt_choice(&self) -> __rt::View<'_, __root::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::test::TestAll2>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(10);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll2>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_choice_mut(&mut self) -> __rt::Mut<'_, __root::pz::test::TestAll2> {
    self.opt_choice_mut_or().into_mut()
  }
  pub fn opt_choice_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::test::TestAll2> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(10),
      )
    }
  }

  pub fn rep_i32(&self) -> __rt::Slice<'_, i32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(11);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i32_at(&self, idx: usize) -> __rt::View<'_, i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(&mut self) -> __rt::Repeated<'_, i32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(11);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<i32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_i64(&self) -> __rt::Slice<'_, i64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(12);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i64_at(&self, idx: usize) -> __rt::View<'_, i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(&mut self) -> __rt::Repeated<'_, i64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(12);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<i64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_u32(&self) -> __rt::Slice<'_, u32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(13);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u32_at(&self, idx: usize) -> __rt::View<'_, u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(&mut self) -> __rt::Repeated<'_, u32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(13);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_u64(&self) -> __rt::Slice<'_, u64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(14);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u64_at(&self, idx: usize) -> __rt::View<'_, u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(&mut self) -> __rt::Repeated<'_, u64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(14);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_f32(&self) -> __rt::Slice<'_, f32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(15);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f32_at(&self, idx: usize) -> __rt::View<'_, f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(&mut self) -> __rt::Repeated<'_, f32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(15);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<f32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_f64(&self) -> __rt::Slice<'_, f64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(16);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f64_at(&self, idx: usize) -> __rt::View<'_, f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(&mut self) -> __rt::Repeated<'_, f64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(16);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<f64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_str(&self) -> __rt::Slice<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(17);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn rep_str_at(&self, idx: usize) -> __rt::View<'_, __rt::Str> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(&mut self) -> __rt::Repeated<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(17);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_bool(&self) -> __rt::Slice<'_, bool> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(18);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<bool>(self.ptr.as_ptr())
    }
  }
  pub fn rep_bool_at(&self, idx: usize) -> __rt::View<'_, bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(&mut self) -> __rt::Repeated<'_, bool> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(18);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<bool>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_recursive(&self) -> __rt::Slice<'_, __root::pz::test::TestAll> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(19);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll>(self.ptr.as_ptr())
    }
  }
  pub fn rep_recursive_at(&self, idx: usize) -> __rt::View<'_, __root::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(&mut self) -> __rt::Repeated<'_, __root::pz::test::TestAll> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(19);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_nested(&self) -> __rt::Slice<'_, __root::pz::test::TestAll_Nested> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(20);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr())
    }
  }
  pub fn rep_nested_at(&self, idx: usize) -> __rt::View<'_, __root::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(&mut self) -> __rt::Repeated<'_, __root::pz::test::TestAll_Nested> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(20);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_choice(&self) -> __rt::Slice<'_, __root::pz::test::TestAll2> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(21);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll2>(self.ptr.as_ptr())
    }
  }
  pub fn rep_choice_at(&self, idx: usize) -> __rt::View<'_, __root::pz::test::TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(&mut self) -> __rt::Repeated<'_, __root::pz::test::TestAll2> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(21);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll2>(self.ptr.as_ptr(), self.arena)
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_TestAll::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_TestAll::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_TestAll::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::test::TestAll {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::test::TestAll {
  type View<'proto> = __priv_TestAll::View<'proto>;
  type Mut<'proto> = __priv_TestAll::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::test::TestAll {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_TestAll::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_TestAll::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_TestAll::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::test::TestAll> {
    __priv_TestAll::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn opt_i32(self) -> __rt::View<'proto, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> __s::option::Option<__rt::View<'proto, i32>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_i64(self) -> __rt::View<'proto, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> __s::option::Option<__rt::View<'proto, i64>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i64>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_u32(self) -> __rt::View<'proto, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_u64(self) -> __rt::View<'proto, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> __s::option::Option<__rt::View<'proto, u64>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u64>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_f32(self) -> __rt::View<'proto, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> __s::option::Option<__rt::View<'proto, f32>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f32>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_f64(self) -> __rt::View<'proto, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> __s::option::Option<__rt::View<'proto, f64>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f64>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_str(self) -> __rt::View<'proto, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_bool(self) -> __rt::View<'proto, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> __s::option::Option<__rt::View<'proto, bool>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(7);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_recursive(self) -> __rt::View<'proto, __root::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(8);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_nested(self) -> __rt::View<'proto, __root::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll_Nested>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(9);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_choice(self) -> __rt::View<'proto, __root::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll2>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(10);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll2>(self.ptr.as_ptr()))
    }
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, i32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(11);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::View<'proto, i32> {
    self.rep_i32().at(idx)
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, i64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(12);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::View<'proto, i64> {
    self.rep_i64().at(idx)
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, u32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(13);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.rep_u32().at(idx)
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, u64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(14);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::View<'proto, u64> {
    self.rep_u64().at(idx)
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, f32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(15);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::View<'proto, f32> {
    self.rep_f32().at(idx)
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, f64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(16);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::View<'proto, f64> {
    self.rep_f64().at(idx)
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(17);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.rep_str().at(idx)
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, bool> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(18);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<bool>(self.ptr.as_ptr())
    }
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::View<'proto, bool> {
    self.rep_bool().at(idx)
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, __root::pz::test::TestAll> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(19);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll>(self.ptr.as_ptr())
    }
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, __root::pz::test::TestAll_Nested> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(20);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr())
    }
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, __root::pz::test::TestAll2> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(21);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll2>(self.ptr.as_ptr())
    }
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll2> {
    self.rep_choice().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.opt_i32_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_i32")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_i64_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_i64")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_u32_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_u32")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_u64_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_u64")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_f32_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_f32")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_f64_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_f64")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_str_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_str")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_bool_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_bool")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_recursive_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_recursive")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_nested_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_nested")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_choice_or() {
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
    __s::result::Result::Ok(())
  }
}

impl __s::default::Default for __priv_TestAll::View<'_> {
  fn default() -> Self {
    __root::pz::test::TestAll::DEFAULT
  }
}

impl<'proto> __priv_TestAll::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::test::TestAll> {
    __priv_TestAll::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::test::TestAll> {
    __priv_TestAll::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::test::TestAll> {
    __priv_TestAll::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::test::TestAll::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::test::TestAll::__tdp_info())
  }

  pub fn opt_i32(self) -> __rt::View<'proto, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> __s::option::Option<__rt::View<'proto, i32>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_i32_mut(self) -> __rt::Mut<'proto, i32> {
    self.opt_i32_mut_or().into_mut()
  }
  pub fn opt_i32_mut_or(self) -> __rt::value::OptMut<'proto, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(0),
      )
    }
  }
  pub fn opt_i32_set(self, value: i32) {
    self.opt_i32_mut().set(value);
  }

  pub fn opt_i64(self) -> __rt::View<'proto, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> __s::option::Option<__rt::View<'proto, i64>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_i64_mut(self) -> __rt::Mut<'proto, i64> {
    self.opt_i64_mut_or().into_mut()
  }
  pub fn opt_i64_mut_or(self) -> __rt::value::OptMut<'proto, i64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(1),
      )
    }
  }
  pub fn opt_i64_set(self, value: i64) {
    self.opt_i64_mut().set(value);
  }

  pub fn opt_u32(self) -> __rt::View<'proto, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_u32_mut(self) -> __rt::Mut<'proto, u32> {
    self.opt_u32_mut_or().into_mut()
  }
  pub fn opt_u32_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(2),
      )
    }
  }
  pub fn opt_u32_set(self, value: u32) {
    self.opt_u32_mut().set(value);
  }

  pub fn opt_u64(self) -> __rt::View<'proto, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> __s::option::Option<__rt::View<'proto, u64>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_u64_mut(self) -> __rt::Mut<'proto, u64> {
    self.opt_u64_mut_or().into_mut()
  }
  pub fn opt_u64_mut_or(self) -> __rt::value::OptMut<'proto, u64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(3),
      )
    }
  }
  pub fn opt_u64_set(self, value: u64) {
    self.opt_u64_mut().set(value);
  }

  pub fn opt_f32(self) -> __rt::View<'proto, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> __s::option::Option<__rt::View<'proto, f32>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_f32_mut(self) -> __rt::Mut<'proto, f32> {
    self.opt_f32_mut_or().into_mut()
  }
  pub fn opt_f32_mut_or(self) -> __rt::value::OptMut<'proto, f32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(4),
      )
    }
  }
  pub fn opt_f32_set(self, value: f32) {
    self.opt_f32_mut().set(value);
  }

  pub fn opt_f64(self) -> __rt::View<'proto, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> __s::option::Option<__rt::View<'proto, f64>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_f64_mut(self) -> __rt::Mut<'proto, f64> {
    self.opt_f64_mut_or().into_mut()
  }
  pub fn opt_f64_mut_or(self) -> __rt::value::OptMut<'proto, f64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(5),
      )
    }
  }
  pub fn opt_f64_set(self, value: f64) {
    self.opt_f64_mut().set(value);
  }

  pub fn opt_str(self) -> __rt::View<'proto, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_str_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.opt_str_mut_or().into_mut()
  }
  pub fn opt_str_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(6),
      )
    }
  }
  pub fn opt_str_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.opt_str_mut().set(value);
  }

  pub fn opt_bool(self) -> __rt::View<'proto, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> __s::option::Option<__rt::View<'proto, bool>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(7);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_bool_mut(self) -> __rt::Mut<'proto, bool> {
    self.opt_bool_mut_or().into_mut()
  }
  pub fn opt_bool_mut_or(self) -> __rt::value::OptMut<'proto, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(7),
      )
    }
  }
  pub fn opt_bool_set(self, value: bool) {
    self.opt_bool_mut().set(value);
  }

  pub fn opt_recursive(self) -> __rt::View<'proto, __root::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(8);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_recursive_mut(self) -> __rt::Mut<'proto, __root::pz::test::TestAll> {
    self.opt_recursive_mut_or().into_mut()
  }
  pub fn opt_recursive_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::test::TestAll> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(8),
      )
    }
  }

  pub fn opt_nested(self) -> __rt::View<'proto, __root::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll_Nested>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(9);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_nested_mut(self) -> __rt::Mut<'proto, __root::pz::test::TestAll_Nested> {
    self.opt_nested_mut_or().into_mut()
  }
  pub fn opt_nested_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::test::TestAll_Nested> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(9),
      )
    }
  }

  pub fn opt_choice(self) -> __rt::View<'proto, __root::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll2>> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(10);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll2>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_choice_mut(self) -> __rt::Mut<'proto, __root::pz::test::TestAll2> {
    self.opt_choice_mut_or().into_mut()
  }
  pub fn opt_choice_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::test::TestAll2> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll::__tdp_info().field(10),
      )
    }
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, i32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(11);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::View<'proto, i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(self) -> __rt::Repeated<'proto, i32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(11);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<i32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, i64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(12);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::View<'proto, i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(self) -> __rt::Repeated<'proto, i64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(12);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<i64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, u32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(13);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(self) -> __rt::Repeated<'proto, u32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(13);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, u64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(14);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::View<'proto, u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(self) -> __rt::Repeated<'proto, u64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(14);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, f32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(15);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::View<'proto, f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(self) -> __rt::Repeated<'proto, f32> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(15);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<f32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, f64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(16);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::View<'proto, f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(self) -> __rt::Repeated<'proto, f64> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(16);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<f64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(17);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(self) -> __rt::Repeated<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(17);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, bool> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(18);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<bool>(self.ptr.as_ptr())
    }
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::View<'proto, bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(self) -> __rt::Repeated<'proto, bool> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(18);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<bool>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, __root::pz::test::TestAll> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(19);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll>(self.ptr.as_ptr())
    }
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(self) -> __rt::Repeated<'proto, __root::pz::test::TestAll> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(19);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, __root::pz::test::TestAll_Nested> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(20);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr())
    }
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(self) -> __rt::Repeated<'proto, __root::pz::test::TestAll_Nested> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(20);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, __root::pz::test::TestAll2> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(21);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll2>(self.ptr.as_ptr())
    }
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(self) -> __rt::Repeated<'proto, __root::pz::test::TestAll2> {
    unsafe {
      let field = __root::pz::test::TestAll::__tdp_info().field(21);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll2>(self.ptr.as_ptr(), self.arena)
    }
  }

}

impl __s::ops::Drop for __root::pz::test::TestAll {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_TestAll::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.test.TestAll ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_TestAll::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::test::TestAll {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
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
    pub(in super) opt_str: __z::RawStr,
    pub(in super) opt_bool: bool,
    pub(in super) opt_recursive: *mut u8,
    pub(in super) opt_nested: *mut u8,
    pub(in super) opt_choice: *mut u8,
    pub(in super) rep_i32: __z::AVec<u32>,
    pub(in super) rep_i64: __z::AVec<u64>,
    pub(in super) rep_u32: __z::AVec<u32>,
    pub(in super) rep_u64: __z::AVec<u64>,
    pub(in super) rep_f32: __z::AVec<u32>,
    pub(in super) rep_f64: __z::AVec<u64>,
    pub(crate) rep_str: __z::AVec<(*mut u8, usize)>,
    pub(in super) rep_bool: __z::AVec<bool>,
    pub(in super) rep_recursive: __z::AVec<*mut u8>,
    pub(in super) rep_nested: __z::AVec<*mut u8>,
    pub(in super) rep_choice: __z::AVec<*mut u8>,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{22 + 1}> =
    __z::tdp::DescStorage::<{22 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::test::TestAll::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::test::TestAll::__tdp_info,
            __root::pz::test::TestAll_Nested::__tdp_info,
            __root::pz::test::TestAll2::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: __z::tdp::DescKind::Message,
      },
      fields: [
        __z::tdp::FieldStorage {
          number: 1,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_i32 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::I64.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_i64 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        __z::tdp::FieldStorage {
          number: 3,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_u32 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
        },
        __z::tdp::FieldStorage {
          number: 4,
          flags:
            __z::tdp::Kind::I64.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_u64 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 3,
        },
        __z::tdp::FieldStorage {
          number: 5,
          flags:
            __z::tdp::Kind::F32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_f32 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 4,
        },
        __z::tdp::FieldStorage {
          number: 6,
          flags:
            __z::tdp::Kind::F64.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_f64 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 5,
        },
        __z::tdp::FieldStorage {
          number: 7,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_str as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 6,
        },
        __z::tdp::FieldStorage {
          number: 8,
          flags:
            __z::tdp::Kind::Bool.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_bool as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 7,
        },
        __z::tdp::FieldStorage {
          number: 10,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_recursive as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 8,
        },
        __z::tdp::FieldStorage {
          number: 11,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_nested as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 1,
          hasbit: 9,
        },
        __z::tdp::FieldStorage {
          number: 12,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().opt_choice as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 2,
          hasbit: 10,
        },
        __z::tdp::FieldStorage {
          number: 21,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_i32 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 11,
        },
        __z::tdp::FieldStorage {
          number: 22,
          flags:
            __z::tdp::Kind::I64.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_i64 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 11,
        },
        __z::tdp::FieldStorage {
          number: 23,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_u32 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 11,
        },
        __z::tdp::FieldStorage {
          number: 24,
          flags:
            __z::tdp::Kind::I64.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_u64 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 11,
        },
        __z::tdp::FieldStorage {
          number: 25,
          flags:
            __z::tdp::Kind::F32.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_f32 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 11,
        },
        __z::tdp::FieldStorage {
          number: 26,
          flags:
            __z::tdp::Kind::F64.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_f64 as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 11,
        },
        __z::tdp::FieldStorage {
          number: 27,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_str as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 11,
        },
        __z::tdp::FieldStorage {
          number: 28,
          flags:
            __z::tdp::Kind::Bool.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_bool as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 11,
        },
        __z::tdp::FieldStorage {
          number: 30,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_recursive as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 11,
        },
        __z::tdp::FieldStorage {
          number: 31,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_nested as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 1,
          hasbit: 11,
        },
        __z::tdp::FieldStorage {
          number: 32,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().rep_choice as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 2,
          hasbit: 11,
        },
        __z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_TestAll::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::test::TestAll>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::test::TestAll> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_TestAll::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::test::TestAll>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::test::TestAll> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::test::TestAll> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.test.TestAll.Nested`
pub struct TestAll_Nested {
  ptr: __z::ABox<__priv_TestAll_Nested::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_TestAll_Nested::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::test::TestAll_Nested {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_TestAll_Nested::Storage = __priv_TestAll_Nested::Storage {
      __hasbits: [0; 1],
      a: 0,
      b: __z::AVec::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_TestAll_Nested::Storage as *mut __priv_TestAll_Nested::Storage as *mut u8),
      _ph: __s::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = __z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: __z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    __s::result::Result::Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> __rt::View<Self> {
    __priv_TestAll_Nested::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_TestAll_Nested::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::test::TestAll_Nested::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn a(&self) -> __rt::View<'_, i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(&self) -> __s::option::Option<__rt::View<'_, i32>> {
    unsafe {
      let field = __root::pz::test::TestAll_Nested::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }
  pub fn a_mut(&mut self) -> __rt::Mut<'_, i32> {
    self.a_mut_or().into_mut()
  }
  pub fn a_mut_or(&mut self) -> __rt::value::OptMut<'_, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll_Nested::__tdp_info().field(0),
      )
    }
  }
  pub fn a_set(&mut self, value: i32) {
    self.a_mut().set(value);
  }

  pub fn b(&self) -> __rt::Slice<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll_Nested::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn b_at(&self, idx: usize) -> __rt::View<'_, __rt::Str> {
    self.b().at(idx)
  }
  pub fn b_mut(&mut self) -> __rt::Repeated<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll_Nested::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_TestAll_Nested::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_TestAll_Nested::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_TestAll_Nested::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::test::TestAll_Nested {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::test::TestAll_Nested {
  type View<'proto> = __priv_TestAll_Nested::View<'proto>;
  type Mut<'proto> = __priv_TestAll_Nested::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::test::TestAll_Nested {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_TestAll_Nested::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_TestAll_Nested::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_TestAll_Nested::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::test::TestAll_Nested> {
    __priv_TestAll_Nested::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn a(self) -> __rt::View<'proto, i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(self) -> __s::option::Option<__rt::View<'proto, i32>> {
    unsafe {
      let field = __root::pz::test::TestAll_Nested::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }

  pub fn b(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll_Nested::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn b_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.b().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.a_or() {
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
    __s::result::Result::Ok(())
  }
}

impl __s::default::Default for __priv_TestAll_Nested::View<'_> {
  fn default() -> Self {
    __root::pz::test::TestAll_Nested::DEFAULT
  }
}

impl<'proto> __priv_TestAll_Nested::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::test::TestAll_Nested> {
    __priv_TestAll_Nested::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::test::TestAll_Nested> {
    __priv_TestAll_Nested::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::test::TestAll_Nested> {
    __priv_TestAll_Nested::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::test::TestAll_Nested::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::test::TestAll_Nested::__tdp_info())
  }

  pub fn a(self) -> __rt::View<'proto, i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(self) -> __s::option::Option<__rt::View<'proto, i32>> {
    unsafe {
      let field = __root::pz::test::TestAll_Nested::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }
  pub fn a_mut(self) -> __rt::Mut<'proto, i32> {
    self.a_mut_or().into_mut()
  }
  pub fn a_mut_or(self) -> __rt::value::OptMut<'proto, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll_Nested::__tdp_info().field(0),
      )
    }
  }
  pub fn a_set(self, value: i32) {
    self.a_mut().set(value);
  }

  pub fn b(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll_Nested::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn b_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.b().at(idx)
  }
  pub fn b_mut(self) -> __rt::Repeated<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll_Nested::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

}

impl __s::ops::Drop for __root::pz::test::TestAll_Nested {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_TestAll_Nested::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.test.TestAll.Nested ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_TestAll_Nested::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::test::TestAll_Nested {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_TestAll_Nested {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) a: u32,
    pub(crate) b: __z::AVec<(*mut u8, usize)>,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{2 + 1}> =
    __z::tdp::DescStorage::<{2 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::test::TestAll_Nested::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: __z::tdp::DescKind::Message,
      },
      fields: [
        __z::tdp::FieldStorage {
          number: 1,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll_Nested::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().a as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __root::pz::test::TestAll_Nested::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().b as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        __z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_TestAll_Nested::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::test::TestAll_Nested>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::test::TestAll_Nested> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_TestAll_Nested::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::test::TestAll_Nested>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::test::TestAll_Nested> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::test::TestAll_Nested> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// choice `pz.test.TestAll2`
pub struct TestAll2 {
  ptr: __z::ABox<__priv_TestAll2::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_TestAll2::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::test::TestAll2 {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_TestAll2::Storage = __priv_TestAll2::Storage {
      which: 0,
      union: __priv_TestAll2::Union { __unset: () },
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_TestAll2::Storage as *mut __priv_TestAll2::Storage as *mut u8),
      _ph: __s::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = __z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: __z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    __s::result::Result::Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> __rt::View<Self> {
    __priv_TestAll2::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_TestAll2::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(&self) -> __root::pz::test::TestAll2Cases<'_, __rt::ptr::select::View> {
    self.as_view().cases()
  }

  pub fn cases_mut(&mut self) -> __root::pz::test::TestAll2Cases<'_, __rt::ptr::select::Mut> {
    self.as_mut().cases_mut()
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::test::TestAll2::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn opt_i32(&self) -> __rt::View<'_, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(&self) -> __s::option::Option<__rt::View<'_, i32>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_i32_mut(&mut self) -> __rt::Mut<'_, i32> {
    self.opt_i32_mut_or().into_mut()
  }
  pub fn opt_i32_mut_or(&mut self) -> __rt::value::OptMut<'_, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(0),
      )
    }
  }
  pub fn opt_i32_set(&mut self, value: i32) {
    self.opt_i32_mut().set(value);
  }

  pub fn opt_i64(&self) -> __rt::View<'_, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(&self) -> __s::option::Option<__rt::View<'_, i64>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_i64_mut(&mut self) -> __rt::Mut<'_, i64> {
    self.opt_i64_mut_or().into_mut()
  }
  pub fn opt_i64_mut_or(&mut self) -> __rt::value::OptMut<'_, i64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(1),
      )
    }
  }
  pub fn opt_i64_set(&mut self, value: i64) {
    self.opt_i64_mut().set(value);
  }

  pub fn opt_u32(&self) -> __rt::View<'_, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(&self) -> __s::option::Option<__rt::View<'_, u32>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_u32_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.opt_u32_mut_or().into_mut()
  }
  pub fn opt_u32_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(2),
      )
    }
  }
  pub fn opt_u32_set(&mut self, value: u32) {
    self.opt_u32_mut().set(value);
  }

  pub fn opt_u64(&self) -> __rt::View<'_, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(&self) -> __s::option::Option<__rt::View<'_, u64>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_u64_mut(&mut self) -> __rt::Mut<'_, u64> {
    self.opt_u64_mut_or().into_mut()
  }
  pub fn opt_u64_mut_or(&mut self) -> __rt::value::OptMut<'_, u64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(3),
      )
    }
  }
  pub fn opt_u64_set(&mut self, value: u64) {
    self.opt_u64_mut().set(value);
  }

  pub fn opt_f32(&self) -> __rt::View<'_, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(&self) -> __s::option::Option<__rt::View<'_, f32>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_f32_mut(&mut self) -> __rt::Mut<'_, f32> {
    self.opt_f32_mut_or().into_mut()
  }
  pub fn opt_f32_mut_or(&mut self) -> __rt::value::OptMut<'_, f32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(4),
      )
    }
  }
  pub fn opt_f32_set(&mut self, value: f32) {
    self.opt_f32_mut().set(value);
  }

  pub fn opt_f64(&self) -> __rt::View<'_, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(&self) -> __s::option::Option<__rt::View<'_, f64>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_f64_mut(&mut self) -> __rt::Mut<'_, f64> {
    self.opt_f64_mut_or().into_mut()
  }
  pub fn opt_f64_mut_or(&mut self) -> __rt::value::OptMut<'_, f64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(5),
      )
    }
  }
  pub fn opt_f64_set(&mut self, value: f64) {
    self.opt_f64_mut().set(value);
  }

  pub fn opt_str(&self) -> __rt::View<'_, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_str_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.opt_str_mut_or().into_mut()
  }
  pub fn opt_str_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(6),
      )
    }
  }
  pub fn opt_str_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.opt_str_mut().set(value);
  }

  pub fn opt_bool(&self) -> __rt::View<'_, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(&self) -> __s::option::Option<__rt::View<'_, bool>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(7);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_bool_mut(&mut self) -> __rt::Mut<'_, bool> {
    self.opt_bool_mut_or().into_mut()
  }
  pub fn opt_bool_mut_or(&mut self) -> __rt::value::OptMut<'_, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(7),
      )
    }
  }
  pub fn opt_bool_set(&mut self, value: bool) {
    self.opt_bool_mut().set(value);
  }

  pub fn opt_recursive(&self) -> __rt::View<'_, __root::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::test::TestAll>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(8);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_recursive_mut(&mut self) -> __rt::Mut<'_, __root::pz::test::TestAll> {
    self.opt_recursive_mut_or().into_mut()
  }
  pub fn opt_recursive_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::test::TestAll> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(8),
      )
    }
  }

  pub fn opt_nested(&self) -> __rt::View<'_, __root::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::test::TestAll_Nested>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(9);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_nested_mut(&mut self) -> __rt::Mut<'_, __root::pz::test::TestAll_Nested> {
    self.opt_nested_mut_or().into_mut()
  }
  pub fn opt_nested_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::test::TestAll_Nested> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(9),
      )
    }
  }

  pub fn opt_choice(&self) -> __rt::View<'_, __root::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::test::TestAll2>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(10);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll2>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_choice_mut(&mut self) -> __rt::Mut<'_, __root::pz::test::TestAll2> {
    self.opt_choice_mut_or().into_mut()
  }
  pub fn opt_choice_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::test::TestAll2> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(10),
      )
    }
  }

  pub fn rep_i32(&self) -> __rt::Slice<'_, i32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(11);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i32_at(&self, idx: usize) -> __rt::View<'_, i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(&mut self) -> __rt::Repeated<'_, i32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(11);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<i32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_i64(&self) -> __rt::Slice<'_, i64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(12);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i64_at(&self, idx: usize) -> __rt::View<'_, i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(&mut self) -> __rt::Repeated<'_, i64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(12);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<i64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_u32(&self) -> __rt::Slice<'_, u32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(13);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u32_at(&self, idx: usize) -> __rt::View<'_, u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(&mut self) -> __rt::Repeated<'_, u32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(13);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_u64(&self) -> __rt::Slice<'_, u64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(14);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u64_at(&self, idx: usize) -> __rt::View<'_, u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(&mut self) -> __rt::Repeated<'_, u64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(14);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_f32(&self) -> __rt::Slice<'_, f32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(15);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f32_at(&self, idx: usize) -> __rt::View<'_, f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(&mut self) -> __rt::Repeated<'_, f32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(15);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<f32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_f64(&self) -> __rt::Slice<'_, f64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(16);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f64_at(&self, idx: usize) -> __rt::View<'_, f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(&mut self) -> __rt::Repeated<'_, f64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(16);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<f64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_str(&self) -> __rt::Slice<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(17);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn rep_str_at(&self, idx: usize) -> __rt::View<'_, __rt::Str> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(&mut self) -> __rt::Repeated<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(17);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_bool(&self) -> __rt::Slice<'_, bool> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(18);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<bool>(self.ptr.as_ptr())
    }
  }
  pub fn rep_bool_at(&self, idx: usize) -> __rt::View<'_, bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(&mut self) -> __rt::Repeated<'_, bool> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(18);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<bool>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_recursive(&self) -> __rt::Slice<'_, __root::pz::test::TestAll> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(19);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll>(self.ptr.as_ptr())
    }
  }
  pub fn rep_recursive_at(&self, idx: usize) -> __rt::View<'_, __root::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(&mut self) -> __rt::Repeated<'_, __root::pz::test::TestAll> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(19);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_nested(&self) -> __rt::Slice<'_, __root::pz::test::TestAll_Nested> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(20);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr())
    }
  }
  pub fn rep_nested_at(&self, idx: usize) -> __rt::View<'_, __root::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(&mut self) -> __rt::Repeated<'_, __root::pz::test::TestAll_Nested> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(20);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_choice(&self) -> __rt::Slice<'_, __root::pz::test::TestAll2> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(21);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll2>(self.ptr.as_ptr())
    }
  }
  pub fn rep_choice_at(&self, idx: usize) -> __rt::View<'_, __root::pz::test::TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(&mut self) -> __rt::Repeated<'_, __root::pz::test::TestAll2> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(21);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll2>(self.ptr.as_ptr(), self.arena)
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_TestAll2::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_TestAll2::Storage>()).which = 0;
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_TestAll2::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

pub enum TestAll2Cases<'proto, Which: __rt::ptr::select::Select> {
  Unset(__s::marker::PhantomData<&'proto Which>),
  OptI32(__rt::ptr::Proxy<'proto, i32, Which>),
  OptI64(__rt::ptr::Proxy<'proto, i64, Which>),
  OptU32(__rt::ptr::Proxy<'proto, u32, Which>),
  OptU64(__rt::ptr::Proxy<'proto, u64, Which>),
  OptF32(__rt::ptr::Proxy<'proto, f32, Which>),
  OptF64(__rt::ptr::Proxy<'proto, f64, Which>),
  OptStr(__rt::ptr::Proxy<'proto, __rt::Str, Which>),
  OptBool(__rt::ptr::Proxy<'proto, bool, Which>),
  OptRecursive(__rt::ptr::Proxy<'proto, __root::pz::test::TestAll, Which>),
  OptNested(__rt::ptr::Proxy<'proto, __root::pz::test::TestAll_Nested, Which>),
  OptChoice(__rt::ptr::Proxy<'proto, __root::pz::test::TestAll2, Which>),
  RepI32(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<i32>, Which>),
  RepI64(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<i64>, Which>),
  RepU32(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<u32>, Which>),
  RepU64(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<u64>, Which>),
  RepF32(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<f32>, Which>),
  RepF64(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<f64>, Which>),
  RepStr(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<__rt::Str>, Which>),
  RepBool(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<bool>, Which>),
  RepRecursive(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<__root::pz::test::TestAll>, Which>),
  RepNested(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<__root::pz::test::TestAll_Nested>, Which>),
  RepChoice(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<__root::pz::test::TestAll2>, Which>),
}

impl __s::default::Default for __root::pz::test::TestAll2 {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::test::TestAll2 {
  type View<'proto> = __priv_TestAll2::View<'proto>;
  type Mut<'proto> = __priv_TestAll2::Mut<'proto>;
}

impl<'proto> __priv_TestAll2::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::test::TestAll2> {
    __priv_TestAll2::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn cases(self) -> __root::pz::test::TestAll2Cases<'proto, __rt::ptr::select::View> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      match number {
        0 => __root::pz::test::TestAll2Cases::Unset(__s::marker::PhantomData),
        1 => __root::pz::test::TestAll2Cases::OptI32(__root::pz::test::TestAll2::__tdp_info().field(0).make_view::<i32>(self.ptr.as_ptr())),
        2 => __root::pz::test::TestAll2Cases::OptI64(__root::pz::test::TestAll2::__tdp_info().field(1).make_view::<i64>(self.ptr.as_ptr())),
        3 => __root::pz::test::TestAll2Cases::OptU32(__root::pz::test::TestAll2::__tdp_info().field(2).make_view::<u32>(self.ptr.as_ptr())),
        4 => __root::pz::test::TestAll2Cases::OptU64(__root::pz::test::TestAll2::__tdp_info().field(3).make_view::<u64>(self.ptr.as_ptr())),
        5 => __root::pz::test::TestAll2Cases::OptF32(__root::pz::test::TestAll2::__tdp_info().field(4).make_view::<f32>(self.ptr.as_ptr())),
        6 => __root::pz::test::TestAll2Cases::OptF64(__root::pz::test::TestAll2::__tdp_info().field(5).make_view::<f64>(self.ptr.as_ptr())),
        7 => __root::pz::test::TestAll2Cases::OptStr(__root::pz::test::TestAll2::__tdp_info().field(6).make_view::<__rt::Str>(self.ptr.as_ptr())),
        8 => __root::pz::test::TestAll2Cases::OptBool(__root::pz::test::TestAll2::__tdp_info().field(7).make_view::<bool>(self.ptr.as_ptr())),
        10 => __root::pz::test::TestAll2Cases::OptRecursive(__root::pz::test::TestAll2::__tdp_info().field(8).make_view::<__root::pz::test::TestAll>(self.ptr.as_ptr())),
        11 => __root::pz::test::TestAll2Cases::OptNested(__root::pz::test::TestAll2::__tdp_info().field(9).make_view::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr())),
        12 => __root::pz::test::TestAll2Cases::OptChoice(__root::pz::test::TestAll2::__tdp_info().field(10).make_view::<__root::pz::test::TestAll2>(self.ptr.as_ptr())),
        21 => __root::pz::test::TestAll2Cases::RepI32(__root::pz::test::TestAll2::__tdp_info().field(11).make_slice::<i32>(self.ptr.as_ptr())),
        22 => __root::pz::test::TestAll2Cases::RepI64(__root::pz::test::TestAll2::__tdp_info().field(12).make_slice::<i64>(self.ptr.as_ptr())),
        23 => __root::pz::test::TestAll2Cases::RepU32(__root::pz::test::TestAll2::__tdp_info().field(13).make_slice::<u32>(self.ptr.as_ptr())),
        24 => __root::pz::test::TestAll2Cases::RepU64(__root::pz::test::TestAll2::__tdp_info().field(14).make_slice::<u64>(self.ptr.as_ptr())),
        25 => __root::pz::test::TestAll2Cases::RepF32(__root::pz::test::TestAll2::__tdp_info().field(15).make_slice::<f32>(self.ptr.as_ptr())),
        26 => __root::pz::test::TestAll2Cases::RepF64(__root::pz::test::TestAll2::__tdp_info().field(16).make_slice::<f64>(self.ptr.as_ptr())),
        27 => __root::pz::test::TestAll2Cases::RepStr(__root::pz::test::TestAll2::__tdp_info().field(17).make_slice::<__rt::Str>(self.ptr.as_ptr())),
        28 => __root::pz::test::TestAll2Cases::RepBool(__root::pz::test::TestAll2::__tdp_info().field(18).make_slice::<bool>(self.ptr.as_ptr())),
        30 => __root::pz::test::TestAll2Cases::RepRecursive(__root::pz::test::TestAll2::__tdp_info().field(19).make_slice::<__root::pz::test::TestAll>(self.ptr.as_ptr())),
        31 => __root::pz::test::TestAll2Cases::RepNested(__root::pz::test::TestAll2::__tdp_info().field(20).make_slice::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr())),
        32 => __root::pz::test::TestAll2Cases::RepChoice(__root::pz::test::TestAll2::__tdp_info().field(21).make_slice::<__root::pz::test::TestAll2>(self.ptr.as_ptr())),
        _ => __s::unreachable!(),
      }
    }
  }

  pub fn opt_i32(self) -> __rt::View<'proto, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> __s::option::Option<__rt::View<'proto, i32>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_i64(self) -> __rt::View<'proto, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> __s::option::Option<__rt::View<'proto, i64>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i64>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_u32(self) -> __rt::View<'proto, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_u64(self) -> __rt::View<'proto, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> __s::option::Option<__rt::View<'proto, u64>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u64>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_f32(self) -> __rt::View<'proto, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> __s::option::Option<__rt::View<'proto, f32>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f32>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_f64(self) -> __rt::View<'proto, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> __s::option::Option<__rt::View<'proto, f64>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f64>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_str(self) -> __rt::View<'proto, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_bool(self) -> __rt::View<'proto, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> __s::option::Option<__rt::View<'proto, bool>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(7);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_recursive(self) -> __rt::View<'proto, __root::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(8);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_nested(self) -> __rt::View<'proto, __root::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll_Nested>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(9);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr()))
    }
  }

  pub fn opt_choice(self) -> __rt::View<'proto, __root::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll2>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(10);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll2>(self.ptr.as_ptr()))
    }
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, i32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(11);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::View<'proto, i32> {
    self.rep_i32().at(idx)
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, i64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(12);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::View<'proto, i64> {
    self.rep_i64().at(idx)
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, u32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(13);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.rep_u32().at(idx)
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, u64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(14);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::View<'proto, u64> {
    self.rep_u64().at(idx)
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, f32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(15);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::View<'proto, f32> {
    self.rep_f32().at(idx)
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, f64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(16);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::View<'proto, f64> {
    self.rep_f64().at(idx)
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(17);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.rep_str().at(idx)
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, bool> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(18);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<bool>(self.ptr.as_ptr())
    }
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::View<'proto, bool> {
    self.rep_bool().at(idx)
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, __root::pz::test::TestAll> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(19);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll>(self.ptr.as_ptr())
    }
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, __root::pz::test::TestAll_Nested> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(20);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr())
    }
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, __root::pz::test::TestAll2> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(21);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll2>(self.ptr.as_ptr())
    }
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll2> {
    self.rep_choice().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.opt_i32_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_i32")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_i64_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_i64")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_u32_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_u32")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_u64_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_u64")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_f32_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_f32")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_f64_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_f64")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_str_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_str")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_bool_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_bool")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_recursive_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_recursive")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_nested_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("opt_nested")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.opt_choice_or() {
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
    __s::result::Result::Ok(())
  }
}

impl __s::default::Default for __priv_TestAll2::View<'_> {
  fn default() -> Self {
    __root::pz::test::TestAll2::DEFAULT
  }
}

impl<'proto> __priv_TestAll2::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::test::TestAll2> {
    __priv_TestAll2::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::test::TestAll2> {
    __priv_TestAll2::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::test::TestAll2> {
    __priv_TestAll2::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(self) -> __root::pz::test::TestAll2Cases<'proto, __rt::ptr::select::View> {
    self.into_view().cases()
  }

  pub fn cases_mut(self) -> __root::pz::test::TestAll2Cases<'proto, __rt::ptr::select::Mut> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      match number {
        0 => __root::pz::test::TestAll2Cases::Unset(__s::marker::PhantomData),
        1 => __root::pz::test::TestAll2Cases::OptI32(__root::pz::test::TestAll2::__tdp_info().field(0).make_mut::<i32>(self.ptr.as_ptr(), self.arena)),
        2 => __root::pz::test::TestAll2Cases::OptI64(__root::pz::test::TestAll2::__tdp_info().field(1).make_mut::<i64>(self.ptr.as_ptr(), self.arena)),
        3 => __root::pz::test::TestAll2Cases::OptU32(__root::pz::test::TestAll2::__tdp_info().field(2).make_mut::<u32>(self.ptr.as_ptr(), self.arena)),
        4 => __root::pz::test::TestAll2Cases::OptU64(__root::pz::test::TestAll2::__tdp_info().field(3).make_mut::<u64>(self.ptr.as_ptr(), self.arena)),
        5 => __root::pz::test::TestAll2Cases::OptF32(__root::pz::test::TestAll2::__tdp_info().field(4).make_mut::<f32>(self.ptr.as_ptr(), self.arena)),
        6 => __root::pz::test::TestAll2Cases::OptF64(__root::pz::test::TestAll2::__tdp_info().field(5).make_mut::<f64>(self.ptr.as_ptr(), self.arena)),
        7 => __root::pz::test::TestAll2Cases::OptStr(__root::pz::test::TestAll2::__tdp_info().field(6).make_mut::<__rt::Str>(self.ptr.as_ptr(), self.arena)),
        8 => __root::pz::test::TestAll2Cases::OptBool(__root::pz::test::TestAll2::__tdp_info().field(7).make_mut::<bool>(self.ptr.as_ptr(), self.arena)),
        10 => __root::pz::test::TestAll2Cases::OptRecursive(__root::pz::test::TestAll2::__tdp_info().field(8).make_mut::<__root::pz::test::TestAll>(self.ptr.as_ptr(), self.arena)),
        11 => __root::pz::test::TestAll2Cases::OptNested(__root::pz::test::TestAll2::__tdp_info().field(9).make_mut::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr(), self.arena)),
        12 => __root::pz::test::TestAll2Cases::OptChoice(__root::pz::test::TestAll2::__tdp_info().field(10).make_mut::<__root::pz::test::TestAll2>(self.ptr.as_ptr(), self.arena)),
        21 => __root::pz::test::TestAll2Cases::RepI32(__root::pz::test::TestAll2::__tdp_info().field(11).make_rep::<i32>(self.ptr.as_ptr(), self.arena)),
        22 => __root::pz::test::TestAll2Cases::RepI64(__root::pz::test::TestAll2::__tdp_info().field(12).make_rep::<i64>(self.ptr.as_ptr(), self.arena)),
        23 => __root::pz::test::TestAll2Cases::RepU32(__root::pz::test::TestAll2::__tdp_info().field(13).make_rep::<u32>(self.ptr.as_ptr(), self.arena)),
        24 => __root::pz::test::TestAll2Cases::RepU64(__root::pz::test::TestAll2::__tdp_info().field(14).make_rep::<u64>(self.ptr.as_ptr(), self.arena)),
        25 => __root::pz::test::TestAll2Cases::RepF32(__root::pz::test::TestAll2::__tdp_info().field(15).make_rep::<f32>(self.ptr.as_ptr(), self.arena)),
        26 => __root::pz::test::TestAll2Cases::RepF64(__root::pz::test::TestAll2::__tdp_info().field(16).make_rep::<f64>(self.ptr.as_ptr(), self.arena)),
        27 => __root::pz::test::TestAll2Cases::RepStr(__root::pz::test::TestAll2::__tdp_info().field(17).make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)),
        28 => __root::pz::test::TestAll2Cases::RepBool(__root::pz::test::TestAll2::__tdp_info().field(18).make_rep::<bool>(self.ptr.as_ptr(), self.arena)),
        30 => __root::pz::test::TestAll2Cases::RepRecursive(__root::pz::test::TestAll2::__tdp_info().field(19).make_rep::<__root::pz::test::TestAll>(self.ptr.as_ptr(), self.arena)),
        31 => __root::pz::test::TestAll2Cases::RepNested(__root::pz::test::TestAll2::__tdp_info().field(20).make_rep::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr(), self.arena)),
        32 => __root::pz::test::TestAll2Cases::RepChoice(__root::pz::test::TestAll2::__tdp_info().field(21).make_rep::<__root::pz::test::TestAll2>(self.ptr.as_ptr(), self.arena)),
        _ => __s::unreachable!(),
      }
    }
  }

  pub fn clear(self) {
    unsafe { __root::pz::test::TestAll2::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::test::TestAll2::__tdp_info())
  }

  pub fn opt_i32(self) -> __rt::View<'proto, i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> __s::option::Option<__rt::View<'proto, i32>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_i32_mut(self) -> __rt::Mut<'proto, i32> {
    self.opt_i32_mut_or().into_mut()
  }
  pub fn opt_i32_mut_or(self) -> __rt::value::OptMut<'proto, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(0),
      )
    }
  }
  pub fn opt_i32_set(self, value: i32) {
    self.opt_i32_mut().set(value);
  }

  pub fn opt_i64(self) -> __rt::View<'proto, i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> __s::option::Option<__rt::View<'proto, i64>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_i64_mut(self) -> __rt::Mut<'proto, i64> {
    self.opt_i64_mut_or().into_mut()
  }
  pub fn opt_i64_mut_or(self) -> __rt::value::OptMut<'proto, i64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(1),
      )
    }
  }
  pub fn opt_i64_set(self, value: i64) {
    self.opt_i64_mut().set(value);
  }

  pub fn opt_u32(self) -> __rt::View<'proto, u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_u32_mut(self) -> __rt::Mut<'proto, u32> {
    self.opt_u32_mut_or().into_mut()
  }
  pub fn opt_u32_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(2),
      )
    }
  }
  pub fn opt_u32_set(self, value: u32) {
    self.opt_u32_mut().set(value);
  }

  pub fn opt_u64(self) -> __rt::View<'proto, u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> __s::option::Option<__rt::View<'proto, u64>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_u64_mut(self) -> __rt::Mut<'proto, u64> {
    self.opt_u64_mut_or().into_mut()
  }
  pub fn opt_u64_mut_or(self) -> __rt::value::OptMut<'proto, u64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(3),
      )
    }
  }
  pub fn opt_u64_set(self, value: u64) {
    self.opt_u64_mut().set(value);
  }

  pub fn opt_f32(self) -> __rt::View<'proto, f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> __s::option::Option<__rt::View<'proto, f32>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f32>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_f32_mut(self) -> __rt::Mut<'proto, f32> {
    self.opt_f32_mut_or().into_mut()
  }
  pub fn opt_f32_mut_or(self) -> __rt::value::OptMut<'proto, f32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(4),
      )
    }
  }
  pub fn opt_f32_set(self, value: f32) {
    self.opt_f32_mut().set(value);
  }

  pub fn opt_f64(self) -> __rt::View<'proto, f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> __s::option::Option<__rt::View<'proto, f64>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<f64>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_f64_mut(self) -> __rt::Mut<'proto, f64> {
    self.opt_f64_mut_or().into_mut()
  }
  pub fn opt_f64_mut_or(self) -> __rt::value::OptMut<'proto, f64> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(5),
      )
    }
  }
  pub fn opt_f64_set(self, value: f64) {
    self.opt_f64_mut().set(value);
  }

  pub fn opt_str(self) -> __rt::View<'proto, __rt::Str> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_str_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.opt_str_mut_or().into_mut()
  }
  pub fn opt_str_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(6),
      )
    }
  }
  pub fn opt_str_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.opt_str_mut().set(value);
  }

  pub fn opt_bool(self) -> __rt::View<'proto, bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> __s::option::Option<__rt::View<'proto, bool>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(7);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_bool_mut(self) -> __rt::Mut<'proto, bool> {
    self.opt_bool_mut_or().into_mut()
  }
  pub fn opt_bool_mut_or(self) -> __rt::value::OptMut<'proto, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(7),
      )
    }
  }
  pub fn opt_bool_set(self, value: bool) {
    self.opt_bool_mut().set(value);
  }

  pub fn opt_recursive(self) -> __rt::View<'proto, __root::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(8);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_recursive_mut(self) -> __rt::Mut<'proto, __root::pz::test::TestAll> {
    self.opt_recursive_mut_or().into_mut()
  }
  pub fn opt_recursive_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::test::TestAll> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(8),
      )
    }
  }

  pub fn opt_nested(self) -> __rt::View<'proto, __root::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll_Nested>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(9);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_nested_mut(self) -> __rt::Mut<'proto, __root::pz::test::TestAll_Nested> {
    self.opt_nested_mut_or().into_mut()
  }
  pub fn opt_nested_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::test::TestAll_Nested> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(9),
      )
    }
  }

  pub fn opt_choice(self) -> __rt::View<'proto, __root::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::test::TestAll2>> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(10);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::test::TestAll2>(self.ptr.as_ptr()))
    }
  }
  pub fn opt_choice_mut(self) -> __rt::Mut<'proto, __root::pz::test::TestAll2> {
    self.opt_choice_mut_or().into_mut()
  }
  pub fn opt_choice_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::test::TestAll2> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::test::TestAll2::__tdp_info().field(10),
      )
    }
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, i32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(11);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::View<'proto, i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(self) -> __rt::Repeated<'proto, i32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(11);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<i32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, i64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(12);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<i64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::View<'proto, i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(self) -> __rt::Repeated<'proto, i64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(12);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<i64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, u32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(13);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(self) -> __rt::Repeated<'proto, u32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(13);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, u64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(14);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::View<'proto, u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(self) -> __rt::Repeated<'proto, u64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(14);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, f32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(15);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f32>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::View<'proto, f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(self) -> __rt::Repeated<'proto, f32> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(15);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<f32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, f64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(16);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<f64>(self.ptr.as_ptr())
    }
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::View<'proto, f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(self) -> __rt::Repeated<'proto, f64> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(16);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<f64>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(17);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(self) -> __rt::Repeated<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(17);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, bool> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(18);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<bool>(self.ptr.as_ptr())
    }
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::View<'proto, bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(self) -> __rt::Repeated<'proto, bool> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(18);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<bool>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, __root::pz::test::TestAll> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(19);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll>(self.ptr.as_ptr())
    }
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(self) -> __rt::Repeated<'proto, __root::pz::test::TestAll> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(19);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, __root::pz::test::TestAll_Nested> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(20);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr())
    }
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(self) -> __rt::Repeated<'proto, __root::pz::test::TestAll_Nested> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(20);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll_Nested>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, __root::pz::test::TestAll2> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(21);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::test::TestAll2>(self.ptr.as_ptr())
    }
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::View<'proto, __root::pz::test::TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(self) -> __rt::Repeated<'proto, __root::pz::test::TestAll2> {
    unsafe {
      let field = __root::pz::test::TestAll2::__tdp_info().field(21);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::test::TestAll2>(self.ptr.as_ptr(), self.arena)
    }
  }

}

impl __s::ops::Drop for __root::pz::test::TestAll2 {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_TestAll2::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.test.TestAll2 ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_TestAll2::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::test::TestAll2 {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __rt::value::Type for __root::pz::test::TestAll2 {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_TestAll2::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_TestAll2::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
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
    pub(in super) opt_str: __z::RawStr,
    pub(in super) opt_bool: bool,
    pub(in super) opt_recursive: *mut u8,
    pub(in super) opt_nested: *mut u8,
    pub(in super) opt_choice: *mut u8,
    pub(in super) rep_i32: __z::AVec<u32>,
    pub(in super) rep_i64: __z::AVec<u64>,
    pub(in super) rep_u32: __z::AVec<u32>,
    pub(in super) rep_u64: __z::AVec<u64>,
    pub(in super) rep_f32: __z::AVec<u32>,
    pub(in super) rep_f64: __z::AVec<u64>,
    pub(crate) rep_str: __z::AVec<(*mut u8, usize)>,
    pub(in super) rep_bool: __z::AVec<bool>,
    pub(in super) rep_recursive: __z::AVec<*mut u8>,
    pub(in super) rep_nested: __z::AVec<*mut u8>,
    pub(in super) rep_choice: __z::AVec<*mut u8>,
  }

  pub const UNION_OFFSET: usize = {
    let align = __s::mem::align_of::<__priv_TestAll2::Union>();
    if align < 4 { 4 } else { align }
  };

  pub static TDP_INFO: __z::tdp::DescStorage<{22 + 1}> =
    __z::tdp::DescStorage::<{22 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::test::TestAll2::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::test::TestAll::__tdp_info,
            __root::pz::test::TestAll_Nested::__tdp_info,
            __root::pz::test::TestAll2::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 0,
        kind: __z::tdp::DescKind::Choice,
      },
      fields: [
        __z::tdp::FieldStorage {
          number: 1,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::I64.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 3,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 4,
          flags:
            __z::tdp::Kind::I64.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 5,
          flags:
            __z::tdp::Kind::F32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 6,
          flags:
            __z::tdp::Kind::F64.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 7,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 8,
          flags:
            __z::tdp::Kind::Bool.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 10,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 11,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 1,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 12,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 2,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 21,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 22,
          flags:
            __z::tdp::Kind::I64.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 23,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 24,
          flags:
            __z::tdp::Kind::I64.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 25,
          flags:
            __z::tdp::Kind::F32.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 26,
          flags:
            __z::tdp::Kind::F64.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 27,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 28,
          flags:
            __z::tdp::Kind::Bool.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 30,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 31,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 1,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 32,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_TestAll2::UNION_OFFSET as u32,
          desc: 2,
          hasbit: 0,
        },
        __z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_TestAll2::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::test::TestAll2>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::test::TestAll2> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_TestAll2::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::test::TestAll2>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::test::TestAll2> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::test::TestAll2> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

} // mod test
// use self is not allowed, so we need to be a bit roundabout.
mod __f { pub use super::*; }
mod __root {
use super::__f;
pub use __f::*;
pub mod pz {
use super::__f;
pub use __f::*;
} // mod pz
} // mod __root

