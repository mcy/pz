// ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]

#![allow(clippy::borrow_interior_mutable_const)]
#![allow(clippy::declare_interior_mutable_const)]
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
use __rt::reflect as __r;
use __z::std as __s;

use __s::default::Default as _;

pub mod test {
use super::{__, __rt, __z, __s, __r};
use __s::default::Default as _;
/// message `pz.test.TestAll`
pub struct TestAll {
  ptr: __s::ptr::NonNull<__priv_TestAll::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_TestAll::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_TestAll {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_TestAll::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::test::TestAll>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::test::TestAll>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) opt_i32: <__s::primitive::i32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_i64: <__s::primitive::i64 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_u32: <__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_u64: <__s::primitive::u64 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_f32: <__s::primitive::f32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_f64: <__s::primitive::f64 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_str: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_bool: <__s::primitive::bool as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_recursive: <__::pz::test::TestAll as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_nested: <__::pz::test::TestAll_Nested as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_choice: <__::pz::test::TestAll2 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) rep_i32: __z::AVec<<__s::primitive::i32 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_i64: __z::AVec<<__s::primitive::i64 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_u32: __z::AVec<<__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_u64: __z::AVec<<__s::primitive::u64 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_f32: __z::AVec<<__s::primitive::f32 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_f64: __z::AVec<<__s::primitive::f64 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_str: __z::AVec<<__rt::String as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_bool: __z::AVec<<__s::primitive::bool as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_recursive: __z::AVec<<__::pz::test::TestAll as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_nested: __z::AVec<<__::pz::test::TestAll_Nested as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_choice: __z::AVec<<__::pz::test::TestAll2 as __z::Type>::__Storage<__z::Seal>>,
  }
}

impl __::pz::test::TestAll {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_TestAll::Storage {
      __hasbits: [0; 1],
      opt_i32: 0,
      opt_i64: 0,
      opt_u32: 0,
      opt_u64: 0,
      opt_f32: 0.0,
      opt_f64: 0.0,
      opt_str: __z::RawStr::new(),
      opt_bool: false,
      opt_recursive: __s::option::Option::None,
      opt_nested: __s::option::Option::None,
      opt_choice: __s::option::Option::None,
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
    }} as *const __priv_TestAll::Storage as *mut __priv_TestAll::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::test::TestAll`].
  pub const fn new() -> Self {
    Self {
      ptr: __s::ptr::NonNull::dangling(),
      arena: __s::option::Option::None,
    }
  }

  fn __init(&mut self) {
    if self.arena.is_none() {
      self.arena = __s::option::Option::Some(__z::RawArena::new());
    }
  }

  /// Deserializes a new [`__::pz::test::TestAll`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::test::TestAll`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::test::TestAll`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::test::TestAll`] to an in-memory byte array.
  ///
  /// See [`Message::to_bytes()`][__r::Message::to_bytes].
  fn to_bytes(&self, codec: __rt::Codec) -> __s::vec::Vec<__s::primitive::u8> {
    <Self as __r::Message>::to_bytes(self, codec)
  }

  /// Converts an ordinary Rust reference into a message reference.
  ///
  /// See [`Message::as_ref()`][__r::Message::as_ref].
  pub fn as_ref(&self) -> __rt::reflect::Ref<Self> {
    use __s::convert::From;
    unsafe { <Self as __z::Type>::__ref(
      __z::Seal,
      __s::ptr::NonNull::from(&self.ptr).cast::<__s::option::Option<__z::tdp::Opaque>>(),
    )}
  }

  /// Converts an ordinary Rust reference into a mutable message reference.
  ///
  /// See [`Message::as_mut()`][__r::Message::as_mut].
  pub fn as_mut(&mut self) -> __rt::reflect::Mut<Self> {
    use __s::convert::From;
    self.__init();
    unsafe { <Self as __z::Type>::__mut(
      __z::Seal,
      __s::ptr::NonNull::from(&mut self.ptr).cast::<__s::option::Option<__z::tdp::Opaque>>(),
      self.arena.unwrap(),
    )}
  }

  /// Selects the fields given by `selector` out of this message by reference.
  ///
  /// See [`Message::get()`][__r::Message::get].
  pub fn get<S>(&self, selector: S) -> __r::Ref<S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll>,
  {
    <Self as __r::Message>::get(self, selector)
  }

  /// Selects the fields given by `selector` out of this message by mutable
  /// reference.
  ///
  /// If this would result in aliasing, it generates a post-monomorphization
  /// error.
  ///
  /// See [`Message::get_mut()`][__r::Message::get_mut].
  pub fn get_mut<S>(&mut self, selector: S) -> __r::Mut<S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::test::TestAll`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn opt_i32(&self) -> __rt::reflect::Ref<'_, __s::primitive::i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::i32>> {
    self.get(__field_TestAll__opt_i32{})
  }
  pub fn opt_i32_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::i32> {
    self.opt_i32_mut_or().into_inner()
  }
  pub fn opt_i32_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::i32> {
    self.get_mut(__field_TestAll__opt_i32{})
  }
  pub fn set_opt_i32(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::i32>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_i32_mut_or());
    self.as_mut()
  }

  pub fn opt_i64(&self) -> __rt::reflect::Ref<'_, __s::primitive::i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::i64>> {
    self.get(__field_TestAll__opt_i64{})
  }
  pub fn opt_i64_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::i64> {
    self.opt_i64_mut_or().into_inner()
  }
  pub fn opt_i64_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::i64> {
    self.get_mut(__field_TestAll__opt_i64{})
  }
  pub fn set_opt_i64(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::i64>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_i64_mut_or());
    self.as_mut()
  }

  pub fn opt_u32(&self) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u32>> {
    self.get(__field_TestAll__opt_u32{})
  }
  pub fn opt_u32_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u32> {
    self.opt_u32_mut_or().into_inner()
  }
  pub fn opt_u32_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u32> {
    self.get_mut(__field_TestAll__opt_u32{})
  }
  pub fn set_opt_u32(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_u32_mut_or());
    self.as_mut()
  }

  pub fn opt_u64(&self) -> __rt::reflect::Ref<'_, __s::primitive::u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u64>> {
    self.get(__field_TestAll__opt_u64{})
  }
  pub fn opt_u64_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u64> {
    self.opt_u64_mut_or().into_inner()
  }
  pub fn opt_u64_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u64> {
    self.get_mut(__field_TestAll__opt_u64{})
  }
  pub fn set_opt_u64(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u64>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_u64_mut_or());
    self.as_mut()
  }

  pub fn opt_f32(&self) -> __rt::reflect::Ref<'_, __s::primitive::f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::f32>> {
    self.get(__field_TestAll__opt_f32{})
  }
  pub fn opt_f32_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::f32> {
    self.opt_f32_mut_or().into_inner()
  }
  pub fn opt_f32_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::f32> {
    self.get_mut(__field_TestAll__opt_f32{})
  }
  pub fn set_opt_f32(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::f32>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_f32_mut_or());
    self.as_mut()
  }

  pub fn opt_f64(&self) -> __rt::reflect::Ref<'_, __s::primitive::f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::f64>> {
    self.get(__field_TestAll__opt_f64{})
  }
  pub fn opt_f64_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::f64> {
    self.opt_f64_mut_or().into_inner()
  }
  pub fn opt_f64_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::f64> {
    self.get_mut(__field_TestAll__opt_f64{})
  }
  pub fn set_opt_f64(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::f64>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_f64_mut_or());
    self.as_mut()
  }

  pub fn opt_str(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_TestAll__opt_str{})
  }
  pub fn opt_str_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.opt_str_mut_or().into_inner()
  }
  pub fn opt_str_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_TestAll__opt_str{})
  }
  pub fn set_opt_str(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_str_mut_or());
    self.as_mut()
  }

  pub fn opt_bool(&self) -> __rt::reflect::Ref<'_, __s::primitive::bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::bool>> {
    self.get(__field_TestAll__opt_bool{})
  }
  pub fn opt_bool_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::bool> {
    self.opt_bool_mut_or().into_inner()
  }
  pub fn opt_bool_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::bool> {
    self.get_mut(__field_TestAll__opt_bool{})
  }
  pub fn set_opt_bool(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::bool>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_bool_mut_or());
    self.as_mut()
  }

  pub fn opt_recursive(&self) -> __rt::reflect::Ref<'_, __::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::test::TestAll>> {
    self.get(__field_TestAll__opt_recursive{})
  }
  pub fn opt_recursive_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::test::TestAll> {
    self.opt_recursive_mut_or().into_inner()
  }
  pub fn opt_recursive_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::test::TestAll> {
    self.get_mut(__field_TestAll__opt_recursive{})
  }
  pub fn set_opt_recursive(&mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_recursive_mut_or());
    self.as_mut()
  }

  pub fn opt_nested(&self) -> __rt::reflect::Ref<'_, __::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::test::TestAll_Nested>> {
    self.get(__field_TestAll__opt_nested{})
  }
  pub fn opt_nested_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::test::TestAll_Nested> {
    self.opt_nested_mut_or().into_inner()
  }
  pub fn opt_nested_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::test::TestAll_Nested> {
    self.get_mut(__field_TestAll__opt_nested{})
  }
  pub fn set_opt_nested(&mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll_Nested>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_nested_mut_or());
    self.as_mut()
  }

  pub fn opt_choice(&self) -> __rt::reflect::Ref<'_, __::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::test::TestAll2>> {
    self.get(__field_TestAll__opt_choice{})
  }
  pub fn opt_choice_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::test::TestAll2> {
    self.opt_choice_mut_or().into_inner()
  }
  pub fn opt_choice_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::test::TestAll2> {
    self.get_mut(__field_TestAll__opt_choice{})
  }
  pub fn set_opt_choice(&mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll2>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_choice_mut_or());
    self.as_mut()
  }

  pub fn rep_i32(&self) -> __rt::Slice<'_, __s::primitive::i32> {
    self.get(__field_TestAll__rep_i32{})
  }
  pub fn rep_i32_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::i32> {
    self.get_mut(__field_TestAll__rep_i32{})
  }
  pub fn set_rep_i32(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::i32>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_i32_mut());
    self.as_mut()
  }

  pub fn rep_i64(&self) -> __rt::Slice<'_, __s::primitive::i64> {
    self.get(__field_TestAll__rep_i64{})
  }
  pub fn rep_i64_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::i64> {
    self.get_mut(__field_TestAll__rep_i64{})
  }
  pub fn set_rep_i64(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::i64>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_i64_mut());
    self.as_mut()
  }

  pub fn rep_u32(&self) -> __rt::Slice<'_, __s::primitive::u32> {
    self.get(__field_TestAll__rep_u32{})
  }
  pub fn rep_u32_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::u32> {
    self.get_mut(__field_TestAll__rep_u32{})
  }
  pub fn set_rep_u32(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_u32_mut());
    self.as_mut()
  }

  pub fn rep_u64(&self) -> __rt::Slice<'_, __s::primitive::u64> {
    self.get(__field_TestAll__rep_u64{})
  }
  pub fn rep_u64_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::u64> {
    self.get_mut(__field_TestAll__rep_u64{})
  }
  pub fn set_rep_u64(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::u64>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_u64_mut());
    self.as_mut()
  }

  pub fn rep_f32(&self) -> __rt::Slice<'_, __s::primitive::f32> {
    self.get(__field_TestAll__rep_f32{})
  }
  pub fn rep_f32_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::f32> {
    self.get_mut(__field_TestAll__rep_f32{})
  }
  pub fn set_rep_f32(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::f32>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_f32_mut());
    self.as_mut()
  }

  pub fn rep_f64(&self) -> __rt::Slice<'_, __s::primitive::f64> {
    self.get(__field_TestAll__rep_f64{})
  }
  pub fn rep_f64_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::f64> {
    self.get_mut(__field_TestAll__rep_f64{})
  }
  pub fn set_rep_f64(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::f64>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_f64_mut());
    self.as_mut()
  }

  pub fn rep_str(&self) -> __rt::Slice<'_, __rt::String> {
    self.get(__field_TestAll__rep_str{})
  }
  pub fn rep_str_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __rt::String> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(&mut self) -> __rt::Repeated<'_, __rt::String> {
    self.get_mut(__field_TestAll__rep_str{})
  }
  pub fn set_rep_str(&mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_str_mut());
    self.as_mut()
  }

  pub fn rep_bool(&self) -> __rt::Slice<'_, __s::primitive::bool> {
    self.get(__field_TestAll__rep_bool{})
  }
  pub fn rep_bool_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::bool> {
    self.get_mut(__field_TestAll__rep_bool{})
  }
  pub fn set_rep_bool(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::bool>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_bool_mut());
    self.as_mut()
  }

  pub fn rep_recursive(&self) -> __rt::Slice<'_, __::pz::test::TestAll> {
    self.get(__field_TestAll__rep_recursive{})
  }
  pub fn rep_recursive_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(&mut self) -> __rt::Repeated<'_, __::pz::test::TestAll> {
    self.get_mut(__field_TestAll__rep_recursive{})
  }
  pub fn set_rep_recursive(&mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_recursive_mut());
    self.as_mut()
  }

  pub fn rep_nested(&self) -> __rt::Slice<'_, __::pz::test::TestAll_Nested> {
    self.get(__field_TestAll__rep_nested{})
  }
  pub fn rep_nested_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(&mut self) -> __rt::Repeated<'_, __::pz::test::TestAll_Nested> {
    self.get_mut(__field_TestAll__rep_nested{})
  }
  pub fn set_rep_nested(&mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll_Nested>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_nested_mut());
    self.as_mut()
  }

  pub fn rep_choice(&self) -> __rt::Slice<'_, __::pz::test::TestAll2> {
    self.get(__field_TestAll__rep_choice{})
  }
  pub fn rep_choice_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::test::TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(&mut self) -> __rt::Repeated<'_, __::pz::test::TestAll2> {
    self.get_mut(__field_TestAll__rep_choice{})
  }
  pub fn set_rep_choice(&mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll2>>) -> __r::Mut<'_, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_choice_mut());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_TestAll::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_TestAll::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::test::TestAll>, src: __rt::reflect::Ref<__::pz::test::TestAll>) {
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_i32>>::Type>::apply_to(src.get(__field_TestAll__opt_i32{}), dst.as_mut().get_mut(__field_TestAll__opt_i32{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_i64>>::Type>::apply_to(src.get(__field_TestAll__opt_i64{}), dst.as_mut().get_mut(__field_TestAll__opt_i64{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_u32>>::Type>::apply_to(src.get(__field_TestAll__opt_u32{}), dst.as_mut().get_mut(__field_TestAll__opt_u32{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_u64>>::Type>::apply_to(src.get(__field_TestAll__opt_u64{}), dst.as_mut().get_mut(__field_TestAll__opt_u64{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_f32>>::Type>::apply_to(src.get(__field_TestAll__opt_f32{}), dst.as_mut().get_mut(__field_TestAll__opt_f32{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_f64>>::Type>::apply_to(src.get(__field_TestAll__opt_f64{}), dst.as_mut().get_mut(__field_TestAll__opt_f64{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_str>>::Type>::apply_to(src.get(__field_TestAll__opt_str{}), dst.as_mut().get_mut(__field_TestAll__opt_str{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_bool>>::Type>::apply_to(src.get(__field_TestAll__opt_bool{}), dst.as_mut().get_mut(__field_TestAll__opt_bool{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_recursive>>::Type>::apply_to(src.get(__field_TestAll__opt_recursive{}), dst.as_mut().get_mut(__field_TestAll__opt_recursive{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_nested>>::Type>::apply_to(src.get(__field_TestAll__opt_nested{}), dst.as_mut().get_mut(__field_TestAll__opt_nested{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__opt_choice>>::Type>::apply_to(src.get(__field_TestAll__opt_choice{}), dst.as_mut().get_mut(__field_TestAll__opt_choice{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_i32>>::Type>::apply_to(src.get(__field_TestAll__rep_i32{}), dst.as_mut().get_mut(__field_TestAll__rep_i32{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_i64>>::Type>::apply_to(src.get(__field_TestAll__rep_i64{}), dst.as_mut().get_mut(__field_TestAll__rep_i64{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_u32>>::Type>::apply_to(src.get(__field_TestAll__rep_u32{}), dst.as_mut().get_mut(__field_TestAll__rep_u32{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_u64>>::Type>::apply_to(src.get(__field_TestAll__rep_u64{}), dst.as_mut().get_mut(__field_TestAll__rep_u64{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_f32>>::Type>::apply_to(src.get(__field_TestAll__rep_f32{}), dst.as_mut().get_mut(__field_TestAll__rep_f32{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_f64>>::Type>::apply_to(src.get(__field_TestAll__rep_f64{}), dst.as_mut().get_mut(__field_TestAll__rep_f64{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_str>>::Type>::apply_to(src.get(__field_TestAll__rep_str{}), dst.as_mut().get_mut(__field_TestAll__rep_str{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_bool>>::Type>::apply_to(src.get(__field_TestAll__rep_bool{}), dst.as_mut().get_mut(__field_TestAll__rep_bool{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_recursive>>::Type>::apply_to(src.get(__field_TestAll__rep_recursive{}), dst.as_mut().get_mut(__field_TestAll__rep_recursive{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_nested>>::Type>::apply_to(src.get(__field_TestAll__rep_nested{}), dst.as_mut().get_mut(__field_TestAll__rep_nested{}));
    __r::Set::<<__::pz::test::TestAll as __r::Field<__field_TestAll__rep_choice>>::Type>::apply_to(src.get(__field_TestAll__rep_choice{}), dst.as_mut().get_mut(__field_TestAll__rep_choice{}));
  }
}

impl __z::Message for __::pz::test::TestAll {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{22 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::test::TestAll::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::test::TestAll::__tdp_info,
            __::pz::test::TestAll_Nested::__tdp_info,
            __::pz::test::TestAll2::__tdp_info,
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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
            let msg = __::pz::test::TestAll::DEFAULT;
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

    unsafe { STATIC.get() }
  };

  fn __is_null(&self, _: impl __z::Sealed) -> bool {
    self.ptr == __s::ptr::NonNull::dangling()
  }
  fn __raw(_: impl __z::Sealed, ptr: __r::Ref<Self>) -> __z::tdp::Opaque { ptr.ptr.cast() }
  fn __arena(_: impl __z::Sealed, ptr: &mut __r::Mut<Self>) -> __z::RawArena { ptr.arena }
}

impl __z::Type for __::pz::test::TestAll {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::test::TestAll::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::test::TestAll::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_TestAll::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_TestAll::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_TestAll::Mut {
      r: Self::__ref(s, outer),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize<S: __z::Sealed>(
    _: S,
    vec: &mut __z::AVec<__s::option::Option<__z::tdp::Opaque>>,
    new_len: usize,
    arena: __z::RawArena,
  ) {
    vec.resize(new_len, arena)
  }
}

impl __r::Views for __::pz::test::TestAll {
  type Ref<'a> = __priv_TestAll::Ref<'a>;
  type Mut<'a> = __priv_TestAll::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_TestAll::Ref<'a> {
  type Target = __::pz::test::TestAll;
  fn as_ref(&self) -> __priv_TestAll::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_TestAll::Mut<'a> {
  type Target = __::pz::test::TestAll;
  fn as_ref(&self) -> __priv_TestAll::Ref { self.r }
  fn into_ref(self) -> __priv_TestAll::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_TestAll::Mut {
    __priv_TestAll::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::test::TestAll {
  const DEFAULT: &'static Self = __::pz::test::TestAll::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_TestAll::Ref<'a> {
  type Message = __::pz::test::TestAll;
}
impl<'a> __r::MessageMut<'a> for __priv_TestAll::Mut<'a> {
  type Message = __::pz::test::TestAll;
}

impl __s::default::Default for __::pz::test::TestAll {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::test::TestAll>> __s::convert::From<T> for __::pz::test::TestAll {
  fn from(value: T) -> __::pz::test::TestAll {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::test::TestAll> for &__::pz::test::TestAll {
  fn apply_to(self, mut m: __r::Mut<__::pz::test::TestAll>) {
    __::pz::test::TestAll::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::test::TestAll>> for &__::pz::test::TestAll {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::test::TestAll>>) {
    __::pz::test::TestAll::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::test::TestAll> for __rt::reflect::Ref<'_, __::pz::test::TestAll> {
  fn apply_to(self, mut m: __r::Mut<__::pz::test::TestAll>) {
    __::pz::test::TestAll::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::test::TestAll>> for __rt::reflect::Ref<'_, __::pz::test::TestAll> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::test::TestAll>>) {
    __::pz::test::TestAll::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::test::TestAll> for &__rt::reflect::Mut<'_, __::pz::test::TestAll> {
  fn apply_to(self, mut m: __r::Mut<__::pz::test::TestAll>) {
    __::pz::test::TestAll::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::test::TestAll>> for &__rt::reflect::Mut<'_, __::pz::test::TestAll> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::test::TestAll>>) {
    __::pz::test::TestAll::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_TestAll::Ref<'_> {
  fn default() -> Self {
    __::pz::test::TestAll::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::test::TestAll {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_TestAll::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.test.TestAll ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_TestAll::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::test::TestAll {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_TestAll__opt_i32 = __rt::field!(opt_i32);
impl __r::Field<__field_TestAll__opt_i32> for __::pz::test::TestAll {
  type Type = __r::Opt<__s::primitive::i32>;
  type Name = __field_TestAll__opt_i32;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "opt_i32";
}

type __field_TestAll__opt_i64 = __rt::field!(opt_i64);
impl __r::Field<__field_TestAll__opt_i64> for __::pz::test::TestAll {
  type Type = __r::Opt<__s::primitive::i64>;
  type Name = __field_TestAll__opt_i64;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "opt_i64";
}

type __field_TestAll__opt_u32 = __rt::field!(opt_u32);
impl __r::Field<__field_TestAll__opt_u32> for __::pz::test::TestAll {
  type Type = __r::Opt<__s::primitive::u32>;
  type Name = __field_TestAll__opt_u32;
  const NUMBER: __s::primitive::i32 = 3;
  const INDEX: __s::primitive::usize = 2;
  const NAME: &'static __s::primitive::str = "opt_u32";
}

type __field_TestAll__opt_u64 = __rt::field!(opt_u64);
impl __r::Field<__field_TestAll__opt_u64> for __::pz::test::TestAll {
  type Type = __r::Opt<__s::primitive::u64>;
  type Name = __field_TestAll__opt_u64;
  const NUMBER: __s::primitive::i32 = 4;
  const INDEX: __s::primitive::usize = 3;
  const NAME: &'static __s::primitive::str = "opt_u64";
}

type __field_TestAll__opt_f32 = __rt::field!(opt_f32);
impl __r::Field<__field_TestAll__opt_f32> for __::pz::test::TestAll {
  type Type = __r::Opt<__s::primitive::f32>;
  type Name = __field_TestAll__opt_f32;
  const NUMBER: __s::primitive::i32 = 5;
  const INDEX: __s::primitive::usize = 4;
  const NAME: &'static __s::primitive::str = "opt_f32";
}

type __field_TestAll__opt_f64 = __rt::field!(opt_f64);
impl __r::Field<__field_TestAll__opt_f64> for __::pz::test::TestAll {
  type Type = __r::Opt<__s::primitive::f64>;
  type Name = __field_TestAll__opt_f64;
  const NUMBER: __s::primitive::i32 = 6;
  const INDEX: __s::primitive::usize = 5;
  const NAME: &'static __s::primitive::str = "opt_f64";
}

type __field_TestAll__opt_str = __rt::field!(opt_str);
impl __r::Field<__field_TestAll__opt_str> for __::pz::test::TestAll {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_TestAll__opt_str;
  const NUMBER: __s::primitive::i32 = 7;
  const INDEX: __s::primitive::usize = 6;
  const NAME: &'static __s::primitive::str = "opt_str";
}

type __field_TestAll__opt_bool = __rt::field!(opt_bool);
impl __r::Field<__field_TestAll__opt_bool> for __::pz::test::TestAll {
  type Type = __r::Opt<__s::primitive::bool>;
  type Name = __field_TestAll__opt_bool;
  const NUMBER: __s::primitive::i32 = 8;
  const INDEX: __s::primitive::usize = 7;
  const NAME: &'static __s::primitive::str = "opt_bool";
}

type __field_TestAll__opt_recursive = __rt::field!(opt_recursive);
impl __r::Field<__field_TestAll__opt_recursive> for __::pz::test::TestAll {
  type Type = __r::Opt<__::pz::test::TestAll>;
  type Name = __field_TestAll__opt_recursive;
  const NUMBER: __s::primitive::i32 = 10;
  const INDEX: __s::primitive::usize = 8;
  const NAME: &'static __s::primitive::str = "opt_recursive";
}

type __field_TestAll__opt_nested = __rt::field!(opt_nested);
impl __r::Field<__field_TestAll__opt_nested> for __::pz::test::TestAll {
  type Type = __r::Opt<__::pz::test::TestAll_Nested>;
  type Name = __field_TestAll__opt_nested;
  const NUMBER: __s::primitive::i32 = 11;
  const INDEX: __s::primitive::usize = 9;
  const NAME: &'static __s::primitive::str = "opt_nested";
}

type __field_TestAll__opt_choice = __rt::field!(opt_choice);
impl __r::Field<__field_TestAll__opt_choice> for __::pz::test::TestAll {
  type Type = __r::Opt<__::pz::test::TestAll2>;
  type Name = __field_TestAll__opt_choice;
  const NUMBER: __s::primitive::i32 = 12;
  const INDEX: __s::primitive::usize = 10;
  const NAME: &'static __s::primitive::str = "opt_choice";
}

type __field_TestAll__rep_i32 = __rt::field!(rep_i32);
impl __r::Field<__field_TestAll__rep_i32> for __::pz::test::TestAll {
  type Type = __r::Rep<__s::primitive::i32>;
  type Name = __field_TestAll__rep_i32;
  const NUMBER: __s::primitive::i32 = 21;
  const INDEX: __s::primitive::usize = 11;
  const NAME: &'static __s::primitive::str = "rep_i32";
}

type __field_TestAll__rep_i64 = __rt::field!(rep_i64);
impl __r::Field<__field_TestAll__rep_i64> for __::pz::test::TestAll {
  type Type = __r::Rep<__s::primitive::i64>;
  type Name = __field_TestAll__rep_i64;
  const NUMBER: __s::primitive::i32 = 22;
  const INDEX: __s::primitive::usize = 12;
  const NAME: &'static __s::primitive::str = "rep_i64";
}

type __field_TestAll__rep_u32 = __rt::field!(rep_u32);
impl __r::Field<__field_TestAll__rep_u32> for __::pz::test::TestAll {
  type Type = __r::Rep<__s::primitive::u32>;
  type Name = __field_TestAll__rep_u32;
  const NUMBER: __s::primitive::i32 = 23;
  const INDEX: __s::primitive::usize = 13;
  const NAME: &'static __s::primitive::str = "rep_u32";
}

type __field_TestAll__rep_u64 = __rt::field!(rep_u64);
impl __r::Field<__field_TestAll__rep_u64> for __::pz::test::TestAll {
  type Type = __r::Rep<__s::primitive::u64>;
  type Name = __field_TestAll__rep_u64;
  const NUMBER: __s::primitive::i32 = 24;
  const INDEX: __s::primitive::usize = 14;
  const NAME: &'static __s::primitive::str = "rep_u64";
}

type __field_TestAll__rep_f32 = __rt::field!(rep_f32);
impl __r::Field<__field_TestAll__rep_f32> for __::pz::test::TestAll {
  type Type = __r::Rep<__s::primitive::f32>;
  type Name = __field_TestAll__rep_f32;
  const NUMBER: __s::primitive::i32 = 25;
  const INDEX: __s::primitive::usize = 15;
  const NAME: &'static __s::primitive::str = "rep_f32";
}

type __field_TestAll__rep_f64 = __rt::field!(rep_f64);
impl __r::Field<__field_TestAll__rep_f64> for __::pz::test::TestAll {
  type Type = __r::Rep<__s::primitive::f64>;
  type Name = __field_TestAll__rep_f64;
  const NUMBER: __s::primitive::i32 = 26;
  const INDEX: __s::primitive::usize = 16;
  const NAME: &'static __s::primitive::str = "rep_f64";
}

type __field_TestAll__rep_str = __rt::field!(rep_str);
impl __r::Field<__field_TestAll__rep_str> for __::pz::test::TestAll {
  type Type = __r::Rep<__rt::String>;
  type Name = __field_TestAll__rep_str;
  const NUMBER: __s::primitive::i32 = 27;
  const INDEX: __s::primitive::usize = 17;
  const NAME: &'static __s::primitive::str = "rep_str";
}

type __field_TestAll__rep_bool = __rt::field!(rep_bool);
impl __r::Field<__field_TestAll__rep_bool> for __::pz::test::TestAll {
  type Type = __r::Rep<__s::primitive::bool>;
  type Name = __field_TestAll__rep_bool;
  const NUMBER: __s::primitive::i32 = 28;
  const INDEX: __s::primitive::usize = 18;
  const NAME: &'static __s::primitive::str = "rep_bool";
}

type __field_TestAll__rep_recursive = __rt::field!(rep_recursive);
impl __r::Field<__field_TestAll__rep_recursive> for __::pz::test::TestAll {
  type Type = __r::Rep<__::pz::test::TestAll>;
  type Name = __field_TestAll__rep_recursive;
  const NUMBER: __s::primitive::i32 = 30;
  const INDEX: __s::primitive::usize = 19;
  const NAME: &'static __s::primitive::str = "rep_recursive";
}

type __field_TestAll__rep_nested = __rt::field!(rep_nested);
impl __r::Field<__field_TestAll__rep_nested> for __::pz::test::TestAll {
  type Type = __r::Rep<__::pz::test::TestAll_Nested>;
  type Name = __field_TestAll__rep_nested;
  const NUMBER: __s::primitive::i32 = 31;
  const INDEX: __s::primitive::usize = 20;
  const NAME: &'static __s::primitive::str = "rep_nested";
}

type __field_TestAll__rep_choice = __rt::field!(rep_choice);
impl __r::Field<__field_TestAll__rep_choice> for __::pz::test::TestAll {
  type Type = __r::Rep<__::pz::test::TestAll2>;
  type Name = __field_TestAll__rep_choice;
  const NUMBER: __s::primitive::i32 = 32;
  const INDEX: __s::primitive::usize = 21;
  const NAME: &'static __s::primitive::str = "rep_choice";
}

impl<'proto> __priv_TestAll::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_TestAll::Ref { *self }

  /// Serializes this [`__::pz::test::TestAll`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::test::TestAll`] to an in-memory byte array.
  ///
  /// See [`MessageRef::to_bytes()`][__r::MessageRef::to_bytes].
  fn to_bytes(&self, codec: __rt::Codec) -> __s::vec::Vec<__s::primitive::u8> {
    <Self as __r::MessageRef>::to_bytes(self, codec)
  }

  /// Selects the fields given by `selector` out of this message by reference.
  ///
  /// See [`MessageRef::get()`][__r::MessageRef::get].
  pub fn get<S>(self, selector: S) -> __r::Ref<'proto, S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn opt_i32(self) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i32>> {
    self.get(__field_TestAll__opt_i32{})
  }

  pub fn opt_i64(self) -> __rt::reflect::Ref<'proto, __s::primitive::i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i64>> {
    self.get(__field_TestAll__opt_i64{})
  }

  pub fn opt_u32(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_TestAll__opt_u32{})
  }

  pub fn opt_u64(self) -> __rt::reflect::Ref<'proto, __s::primitive::u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u64>> {
    self.get(__field_TestAll__opt_u64{})
  }

  pub fn opt_f32(self) -> __rt::reflect::Ref<'proto, __s::primitive::f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::f32>> {
    self.get(__field_TestAll__opt_f32{})
  }

  pub fn opt_f64(self) -> __rt::reflect::Ref<'proto, __s::primitive::f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::f64>> {
    self.get(__field_TestAll__opt_f64{})
  }

  pub fn opt_str(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_TestAll__opt_str{})
  }

  pub fn opt_bool(self) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::bool>> {
    self.get(__field_TestAll__opt_bool{})
  }

  pub fn opt_recursive(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll>> {
    self.get(__field_TestAll__opt_recursive{})
  }

  pub fn opt_nested(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested>> {
    self.get(__field_TestAll__opt_nested{})
  }

  pub fn opt_choice(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll2>> {
    self.get(__field_TestAll__opt_choice{})
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, __s::primitive::i32> {
    self.get(__field_TestAll__rep_i32{})
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.rep_i32().at(idx)
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, __s::primitive::i64> {
    self.get(__field_TestAll__rep_i64{})
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::i64> {
    self.rep_i64().at(idx)
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, __s::primitive::u32> {
    self.get(__field_TestAll__rep_u32{})
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.rep_u32().at(idx)
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, __s::primitive::u64> {
    self.get(__field_TestAll__rep_u64{})
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u64> {
    self.rep_u64().at(idx)
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, __s::primitive::f32> {
    self.get(__field_TestAll__rep_f32{})
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::f32> {
    self.rep_f32().at(idx)
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, __s::primitive::f64> {
    self.get(__field_TestAll__rep_f64{})
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::f64> {
    self.rep_f64().at(idx)
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_TestAll__rep_str{})
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.rep_str().at(idx)
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, __s::primitive::bool> {
    self.get(__field_TestAll__rep_bool{})
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.rep_bool().at(idx)
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, __::pz::test::TestAll> {
    self.get(__field_TestAll__rep_recursive{})
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, __::pz::test::TestAll_Nested> {
    self.get(__field_TestAll__rep_nested{})
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, __::pz::test::TestAll2> {
    self.get(__field_TestAll__rep_choice{})
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll2> {
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

impl<'proto> __priv_TestAll::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_TestAll::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_TestAll::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_TestAll::Mut {
    __priv_TestAll::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::test::TestAll`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::test::TestAll`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::test::TestAll`] to an in-memory byte array.
  ///
  /// See [`MessageMut::to_bytes()`][__r::MessageMut::to_bytes].
  fn to_bytes(&self, codec: __rt::Codec) -> __s::vec::Vec<__s::primitive::u8> {
    <Self as __r::MessageMut>::to_bytes(self, codec)
  }

  /// Selects the fields given by `selector` out of this message by reference.
  ///
  /// See [`MessageMut::get()`][__r::MessageMut::get].
  pub fn get<S>(self, selector: S) -> __r::Ref<'proto, S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll>,
  {
    <Self as __r::MessageMut>::get(self, selector)
  }

  /// Selects the fields given by `selector` out of this message by mutable
  /// reference.
  ///
  /// If this would result in aliasing, it generates a post-monomorphization
  /// error.
  ///
  /// See [`MessageMut::get_mut()`][__r::MessageMut::get_mut].
  pub fn get_mut<S>(self, selector: S) -> __r::Mut<'proto, S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::test::TestAll`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn opt_i32(self) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i32>> {
    self.get(__field_TestAll__opt_i32{})
  }
  pub fn opt_i32_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::i32> {
    self.opt_i32_mut_or().into_inner()
  }
  pub fn opt_i32_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::i32> {
    self.get_mut(__field_TestAll__opt_i32{})
  }
  pub fn set_opt_i32(mut self, value: impl __r::Set<__r::Opt<__s::primitive::i32>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_i32_mut_or());
    self
  }

  pub fn opt_i64(self) -> __rt::reflect::Ref<'proto, __s::primitive::i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i64>> {
    self.get(__field_TestAll__opt_i64{})
  }
  pub fn opt_i64_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::i64> {
    self.opt_i64_mut_or().into_inner()
  }
  pub fn opt_i64_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::i64> {
    self.get_mut(__field_TestAll__opt_i64{})
  }
  pub fn set_opt_i64(mut self, value: impl __r::Set<__r::Opt<__s::primitive::i64>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_i64_mut_or());
    self
  }

  pub fn opt_u32(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_TestAll__opt_u32{})
  }
  pub fn opt_u32_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u32> {
    self.opt_u32_mut_or().into_inner()
  }
  pub fn opt_u32_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u32> {
    self.get_mut(__field_TestAll__opt_u32{})
  }
  pub fn set_opt_u32(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_u32_mut_or());
    self
  }

  pub fn opt_u64(self) -> __rt::reflect::Ref<'proto, __s::primitive::u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u64>> {
    self.get(__field_TestAll__opt_u64{})
  }
  pub fn opt_u64_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u64> {
    self.opt_u64_mut_or().into_inner()
  }
  pub fn opt_u64_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u64> {
    self.get_mut(__field_TestAll__opt_u64{})
  }
  pub fn set_opt_u64(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u64>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_u64_mut_or());
    self
  }

  pub fn opt_f32(self) -> __rt::reflect::Ref<'proto, __s::primitive::f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::f32>> {
    self.get(__field_TestAll__opt_f32{})
  }
  pub fn opt_f32_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::f32> {
    self.opt_f32_mut_or().into_inner()
  }
  pub fn opt_f32_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::f32> {
    self.get_mut(__field_TestAll__opt_f32{})
  }
  pub fn set_opt_f32(mut self, value: impl __r::Set<__r::Opt<__s::primitive::f32>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_f32_mut_or());
    self
  }

  pub fn opt_f64(self) -> __rt::reflect::Ref<'proto, __s::primitive::f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::f64>> {
    self.get(__field_TestAll__opt_f64{})
  }
  pub fn opt_f64_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::f64> {
    self.opt_f64_mut_or().into_inner()
  }
  pub fn opt_f64_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::f64> {
    self.get_mut(__field_TestAll__opt_f64{})
  }
  pub fn set_opt_f64(mut self, value: impl __r::Set<__r::Opt<__s::primitive::f64>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_f64_mut_or());
    self
  }

  pub fn opt_str(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_TestAll__opt_str{})
  }
  pub fn opt_str_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.opt_str_mut_or().into_inner()
  }
  pub fn opt_str_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_TestAll__opt_str{})
  }
  pub fn set_opt_str(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_str_mut_or());
    self
  }

  pub fn opt_bool(self) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::bool>> {
    self.get(__field_TestAll__opt_bool{})
  }
  pub fn opt_bool_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::bool> {
    self.opt_bool_mut_or().into_inner()
  }
  pub fn opt_bool_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::bool> {
    self.get_mut(__field_TestAll__opt_bool{})
  }
  pub fn set_opt_bool(mut self, value: impl __r::Set<__r::Opt<__s::primitive::bool>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_bool_mut_or());
    self
  }

  pub fn opt_recursive(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll>> {
    self.get(__field_TestAll__opt_recursive{})
  }
  pub fn opt_recursive_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::test::TestAll> {
    self.opt_recursive_mut_or().into_inner()
  }
  pub fn opt_recursive_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::test::TestAll> {
    self.get_mut(__field_TestAll__opt_recursive{})
  }
  pub fn set_opt_recursive(mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_recursive_mut_or());
    self
  }

  pub fn opt_nested(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested>> {
    self.get(__field_TestAll__opt_nested{})
  }
  pub fn opt_nested_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::test::TestAll_Nested> {
    self.opt_nested_mut_or().into_inner()
  }
  pub fn opt_nested_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::test::TestAll_Nested> {
    self.get_mut(__field_TestAll__opt_nested{})
  }
  pub fn set_opt_nested(mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll_Nested>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_nested_mut_or());
    self
  }

  pub fn opt_choice(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll2>> {
    self.get(__field_TestAll__opt_choice{})
  }
  pub fn opt_choice_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::test::TestAll2> {
    self.opt_choice_mut_or().into_inner()
  }
  pub fn opt_choice_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::test::TestAll2> {
    self.get_mut(__field_TestAll__opt_choice{})
  }
  pub fn set_opt_choice(mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll2>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().opt_choice_mut_or());
    self
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, __s::primitive::i32> {
    self.get(__field_TestAll__rep_i32{})
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::i32> {
    self.get_mut(__field_TestAll__rep_i32{})
  }
  pub fn set_rep_i32(mut self, value: impl __r::Set<__r::Rep<__s::primitive::i32>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_i32_mut());
    self
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, __s::primitive::i64> {
    self.get(__field_TestAll__rep_i64{})
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::i64> {
    self.get_mut(__field_TestAll__rep_i64{})
  }
  pub fn set_rep_i64(mut self, value: impl __r::Set<__r::Rep<__s::primitive::i64>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_i64_mut());
    self
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, __s::primitive::u32> {
    self.get(__field_TestAll__rep_u32{})
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::u32> {
    self.get_mut(__field_TestAll__rep_u32{})
  }
  pub fn set_rep_u32(mut self, value: impl __r::Set<__r::Rep<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_u32_mut());
    self
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, __s::primitive::u64> {
    self.get(__field_TestAll__rep_u64{})
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::u64> {
    self.get_mut(__field_TestAll__rep_u64{})
  }
  pub fn set_rep_u64(mut self, value: impl __r::Set<__r::Rep<__s::primitive::u64>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_u64_mut());
    self
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, __s::primitive::f32> {
    self.get(__field_TestAll__rep_f32{})
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::f32> {
    self.get_mut(__field_TestAll__rep_f32{})
  }
  pub fn set_rep_f32(mut self, value: impl __r::Set<__r::Rep<__s::primitive::f32>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_f32_mut());
    self
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, __s::primitive::f64> {
    self.get(__field_TestAll__rep_f64{})
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::f64> {
    self.get_mut(__field_TestAll__rep_f64{})
  }
  pub fn set_rep_f64(mut self, value: impl __r::Set<__r::Rep<__s::primitive::f64>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_f64_mut());
    self
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_TestAll__rep_str{})
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(mut self) -> __rt::Repeated<'proto, __rt::String> {
    self.get_mut(__field_TestAll__rep_str{})
  }
  pub fn set_rep_str(mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_str_mut());
    self
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, __s::primitive::bool> {
    self.get(__field_TestAll__rep_bool{})
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::bool> {
    self.get_mut(__field_TestAll__rep_bool{})
  }
  pub fn set_rep_bool(mut self, value: impl __r::Set<__r::Rep<__s::primitive::bool>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_bool_mut());
    self
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, __::pz::test::TestAll> {
    self.get(__field_TestAll__rep_recursive{})
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(mut self) -> __rt::Repeated<'proto, __::pz::test::TestAll> {
    self.get_mut(__field_TestAll__rep_recursive{})
  }
  pub fn set_rep_recursive(mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_recursive_mut());
    self
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, __::pz::test::TestAll_Nested> {
    self.get(__field_TestAll__rep_nested{})
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(mut self) -> __rt::Repeated<'proto, __::pz::test::TestAll_Nested> {
    self.get_mut(__field_TestAll__rep_nested{})
  }
  pub fn set_rep_nested(mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll_Nested>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_nested_mut());
    self
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, __::pz::test::TestAll2> {
    self.get(__field_TestAll__rep_choice{})
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(mut self) -> __rt::Repeated<'proto, __::pz::test::TestAll2> {
    self.get_mut(__field_TestAll__rep_choice{})
  }
  pub fn set_rep_choice(mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll2>>) -> __r::Mut<'proto, __::pz::test::TestAll> {
    value.apply_to(self.as_mut().rep_choice_mut());
    self
  }

}

/// message `pz.test.TestAll.Nested`
pub struct TestAll_Nested {
  ptr: __s::ptr::NonNull<__priv_TestAll_Nested::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_TestAll_Nested::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_TestAll_Nested {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_TestAll_Nested::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::test::TestAll_Nested>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::test::TestAll_Nested>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) a: <__s::primitive::i32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) b: __z::AVec<<__rt::String as __z::Type>::__Storage<__z::Seal>>,
  }
}

impl __::pz::test::TestAll_Nested {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_TestAll_Nested::Storage {
      __hasbits: [0; 1],
      a: 0,
      b: __z::AVec::new(),
    }} as *const __priv_TestAll_Nested::Storage as *mut __priv_TestAll_Nested::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::test::TestAll_Nested`].
  pub const fn new() -> Self {
    Self {
      ptr: __s::ptr::NonNull::dangling(),
      arena: __s::option::Option::None,
    }
  }

  fn __init(&mut self) {
    if self.arena.is_none() {
      self.arena = __s::option::Option::Some(__z::RawArena::new());
    }
  }

  /// Deserializes a new [`__::pz::test::TestAll_Nested`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::test::TestAll_Nested`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::test::TestAll_Nested`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::test::TestAll_Nested`] to an in-memory byte array.
  ///
  /// See [`Message::to_bytes()`][__r::Message::to_bytes].
  fn to_bytes(&self, codec: __rt::Codec) -> __s::vec::Vec<__s::primitive::u8> {
    <Self as __r::Message>::to_bytes(self, codec)
  }

  /// Converts an ordinary Rust reference into a message reference.
  ///
  /// See [`Message::as_ref()`][__r::Message::as_ref].
  pub fn as_ref(&self) -> __rt::reflect::Ref<Self> {
    use __s::convert::From;
    unsafe { <Self as __z::Type>::__ref(
      __z::Seal,
      __s::ptr::NonNull::from(&self.ptr).cast::<__s::option::Option<__z::tdp::Opaque>>(),
    )}
  }

  /// Converts an ordinary Rust reference into a mutable message reference.
  ///
  /// See [`Message::as_mut()`][__r::Message::as_mut].
  pub fn as_mut(&mut self) -> __rt::reflect::Mut<Self> {
    use __s::convert::From;
    self.__init();
    unsafe { <Self as __z::Type>::__mut(
      __z::Seal,
      __s::ptr::NonNull::from(&mut self.ptr).cast::<__s::option::Option<__z::tdp::Opaque>>(),
      self.arena.unwrap(),
    )}
  }

  /// Selects the fields given by `selector` out of this message by reference.
  ///
  /// See [`Message::get()`][__r::Message::get].
  pub fn get<S>(&self, selector: S) -> __r::Ref<S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll_Nested>,
  {
    <Self as __r::Message>::get(self, selector)
  }

  /// Selects the fields given by `selector` out of this message by mutable
  /// reference.
  ///
  /// If this would result in aliasing, it generates a post-monomorphization
  /// error.
  ///
  /// See [`Message::get_mut()`][__r::Message::get_mut].
  pub fn get_mut<S>(&mut self, selector: S) -> __r::Mut<S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll_Nested>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::test::TestAll_Nested`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn a(&self) -> __rt::reflect::Ref<'_, __s::primitive::i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::i32>> {
    self.get(__field_TestAll_Nested__a{})
  }
  pub fn a_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::i32> {
    self.a_mut_or().into_inner()
  }
  pub fn a_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::i32> {
    self.get_mut(__field_TestAll_Nested__a{})
  }
  pub fn set_a(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::i32>>) -> __r::Mut<'_, __::pz::test::TestAll_Nested> {
    value.apply_to(self.as_mut().a_mut_or());
    self.as_mut()
  }

  pub fn b(&self) -> __rt::Slice<'_, __rt::String> {
    self.get(__field_TestAll_Nested__b{})
  }
  pub fn b_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __rt::String> {
    self.b().at(idx)
  }
  pub fn b_mut(&mut self) -> __rt::Repeated<'_, __rt::String> {
    self.get_mut(__field_TestAll_Nested__b{})
  }
  pub fn set_b(&mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'_, __::pz::test::TestAll_Nested> {
    value.apply_to(self.as_mut().b_mut());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_TestAll_Nested::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_TestAll_Nested::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::test::TestAll_Nested>, src: __rt::reflect::Ref<__::pz::test::TestAll_Nested>) {
    __r::Set::<<__::pz::test::TestAll_Nested as __r::Field<__field_TestAll_Nested__a>>::Type>::apply_to(src.get(__field_TestAll_Nested__a{}), dst.as_mut().get_mut(__field_TestAll_Nested__a{}));
    __r::Set::<<__::pz::test::TestAll_Nested as __r::Field<__field_TestAll_Nested__b>>::Type>::apply_to(src.get(__field_TestAll_Nested__b{}), dst.as_mut().get_mut(__field_TestAll_Nested__b{}));
  }
}

impl __z::Message for __::pz::test::TestAll_Nested {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{2 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::test::TestAll_Nested::__LAYOUT.size();
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
            let msg = __::pz::test::TestAll_Nested::DEFAULT;
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
            let msg = __::pz::test::TestAll_Nested::DEFAULT;
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

    unsafe { STATIC.get() }
  };

  fn __is_null(&self, _: impl __z::Sealed) -> bool {
    self.ptr == __s::ptr::NonNull::dangling()
  }
  fn __raw(_: impl __z::Sealed, ptr: __r::Ref<Self>) -> __z::tdp::Opaque { ptr.ptr.cast() }
  fn __arena(_: impl __z::Sealed, ptr: &mut __r::Mut<Self>) -> __z::RawArena { ptr.arena }
}

impl __z::Type for __::pz::test::TestAll_Nested {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::test::TestAll_Nested::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::test::TestAll_Nested::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_TestAll_Nested::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_TestAll_Nested::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_TestAll_Nested::Mut {
      r: Self::__ref(s, outer),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize<S: __z::Sealed>(
    _: S,
    vec: &mut __z::AVec<__s::option::Option<__z::tdp::Opaque>>,
    new_len: usize,
    arena: __z::RawArena,
  ) {
    vec.resize(new_len, arena)
  }
}

impl __r::Views for __::pz::test::TestAll_Nested {
  type Ref<'a> = __priv_TestAll_Nested::Ref<'a>;
  type Mut<'a> = __priv_TestAll_Nested::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_TestAll_Nested::Ref<'a> {
  type Target = __::pz::test::TestAll_Nested;
  fn as_ref(&self) -> __priv_TestAll_Nested::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_TestAll_Nested::Mut<'a> {
  type Target = __::pz::test::TestAll_Nested;
  fn as_ref(&self) -> __priv_TestAll_Nested::Ref { self.r }
  fn into_ref(self) -> __priv_TestAll_Nested::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_TestAll_Nested::Mut {
    __priv_TestAll_Nested::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::test::TestAll_Nested {
  const DEFAULT: &'static Self = __::pz::test::TestAll_Nested::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_TestAll_Nested::Ref<'a> {
  type Message = __::pz::test::TestAll_Nested;
}
impl<'a> __r::MessageMut<'a> for __priv_TestAll_Nested::Mut<'a> {
  type Message = __::pz::test::TestAll_Nested;
}

impl __s::default::Default for __::pz::test::TestAll_Nested {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::test::TestAll_Nested>> __s::convert::From<T> for __::pz::test::TestAll_Nested {
  fn from(value: T) -> __::pz::test::TestAll_Nested {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::test::TestAll_Nested> for &__::pz::test::TestAll_Nested {
  fn apply_to(self, mut m: __r::Mut<__::pz::test::TestAll_Nested>) {
    __::pz::test::TestAll_Nested::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::test::TestAll_Nested>> for &__::pz::test::TestAll_Nested {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::test::TestAll_Nested>>) {
    __::pz::test::TestAll_Nested::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::test::TestAll_Nested> for __rt::reflect::Ref<'_, __::pz::test::TestAll_Nested> {
  fn apply_to(self, mut m: __r::Mut<__::pz::test::TestAll_Nested>) {
    __::pz::test::TestAll_Nested::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::test::TestAll_Nested>> for __rt::reflect::Ref<'_, __::pz::test::TestAll_Nested> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::test::TestAll_Nested>>) {
    __::pz::test::TestAll_Nested::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::test::TestAll_Nested> for &__rt::reflect::Mut<'_, __::pz::test::TestAll_Nested> {
  fn apply_to(self, mut m: __r::Mut<__::pz::test::TestAll_Nested>) {
    __::pz::test::TestAll_Nested::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::test::TestAll_Nested>> for &__rt::reflect::Mut<'_, __::pz::test::TestAll_Nested> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::test::TestAll_Nested>>) {
    __::pz::test::TestAll_Nested::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_TestAll_Nested::Ref<'_> {
  fn default() -> Self {
    __::pz::test::TestAll_Nested::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::test::TestAll_Nested {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_TestAll_Nested::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.test.TestAll.Nested ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_TestAll_Nested::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::test::TestAll_Nested {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_TestAll_Nested__a = __rt::field!(a);
impl __r::Field<__field_TestAll_Nested__a> for __::pz::test::TestAll_Nested {
  type Type = __r::Opt<__s::primitive::i32>;
  type Name = __field_TestAll_Nested__a;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "a";
}

type __field_TestAll_Nested__b = __rt::field!(b);
impl __r::Field<__field_TestAll_Nested__b> for __::pz::test::TestAll_Nested {
  type Type = __r::Rep<__rt::String>;
  type Name = __field_TestAll_Nested__b;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "b";
}

impl<'proto> __priv_TestAll_Nested::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_TestAll_Nested::Ref { *self }

  /// Serializes this [`__::pz::test::TestAll_Nested`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::test::TestAll_Nested`] to an in-memory byte array.
  ///
  /// See [`MessageRef::to_bytes()`][__r::MessageRef::to_bytes].
  fn to_bytes(&self, codec: __rt::Codec) -> __s::vec::Vec<__s::primitive::u8> {
    <Self as __r::MessageRef>::to_bytes(self, codec)
  }

  /// Selects the fields given by `selector` out of this message by reference.
  ///
  /// See [`MessageRef::get()`][__r::MessageRef::get].
  pub fn get<S>(self, selector: S) -> __r::Ref<'proto, S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll_Nested>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn a(self) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i32>> {
    self.get(__field_TestAll_Nested__a{})
  }

  pub fn b(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_TestAll_Nested__b{})
  }
  pub fn b_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
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

impl<'proto> __priv_TestAll_Nested::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_TestAll_Nested::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_TestAll_Nested::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_TestAll_Nested::Mut {
    __priv_TestAll_Nested::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::test::TestAll_Nested`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::test::TestAll_Nested`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::test::TestAll_Nested`] to an in-memory byte array.
  ///
  /// See [`MessageMut::to_bytes()`][__r::MessageMut::to_bytes].
  fn to_bytes(&self, codec: __rt::Codec) -> __s::vec::Vec<__s::primitive::u8> {
    <Self as __r::MessageMut>::to_bytes(self, codec)
  }

  /// Selects the fields given by `selector` out of this message by reference.
  ///
  /// See [`MessageMut::get()`][__r::MessageMut::get].
  pub fn get<S>(self, selector: S) -> __r::Ref<'proto, S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll_Nested>,
  {
    <Self as __r::MessageMut>::get(self, selector)
  }

  /// Selects the fields given by `selector` out of this message by mutable
  /// reference.
  ///
  /// If this would result in aliasing, it generates a post-monomorphization
  /// error.
  ///
  /// See [`MessageMut::get_mut()`][__r::MessageMut::get_mut].
  pub fn get_mut<S>(self, selector: S) -> __r::Mut<'proto, S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll_Nested>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::test::TestAll_Nested`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn a(self) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.a_or().unwrap_or_default()
  }
  pub fn a_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i32>> {
    self.get(__field_TestAll_Nested__a{})
  }
  pub fn a_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::i32> {
    self.a_mut_or().into_inner()
  }
  pub fn a_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::i32> {
    self.get_mut(__field_TestAll_Nested__a{})
  }
  pub fn set_a(mut self, value: impl __r::Set<__r::Opt<__s::primitive::i32>>) -> __r::Mut<'proto, __::pz::test::TestAll_Nested> {
    value.apply_to(self.as_mut().a_mut_or());
    self
  }

  pub fn b(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_TestAll_Nested__b{})
  }
  pub fn b_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.b().at(idx)
  }
  pub fn b_mut(mut self) -> __rt::Repeated<'proto, __rt::String> {
    self.get_mut(__field_TestAll_Nested__b{})
  }
  pub fn set_b(mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'proto, __::pz::test::TestAll_Nested> {
    value.apply_to(self.as_mut().b_mut());
    self
  }

}

/// choice `pz.test.TestAll2`
pub struct TestAll2 {
  ptr: __s::ptr::NonNull<__priv_TestAll2::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_TestAll2::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_TestAll2 {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_TestAll2::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::test::TestAll2>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::test::TestAll2>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(super) which: u32,
    pub(super) union: Union,
  }

  #[repr(C)]
  pub union Union {
    pub(super) __unset: (),
    pub(in super) opt_i32: <__s::primitive::i32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_i64: <__s::primitive::i64 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_u32: <__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_u64: <__s::primitive::u64 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_f32: <__s::primitive::f32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_f64: <__s::primitive::f64 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_str: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_bool: <__s::primitive::bool as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_recursive: <__::pz::test::TestAll as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_nested: <__::pz::test::TestAll_Nested as __z::Type>::__Storage<__z::Seal>,
    pub(in super) opt_choice: <__::pz::test::TestAll2 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) rep_i32: __z::AVec<<__s::primitive::i32 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_i64: __z::AVec<<__s::primitive::i64 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_u32: __z::AVec<<__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_u64: __z::AVec<<__s::primitive::u64 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_f32: __z::AVec<<__s::primitive::f32 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_f64: __z::AVec<<__s::primitive::f64 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_str: __z::AVec<<__rt::String as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_bool: __z::AVec<<__s::primitive::bool as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_recursive: __z::AVec<<__::pz::test::TestAll as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_nested: __z::AVec<<__::pz::test::TestAll_Nested as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) rep_choice: __z::AVec<<__::pz::test::TestAll2 as __z::Type>::__Storage<__z::Seal>>,
  }

  pub const UNION_OFFSET: usize = {
    let align = __s::mem::align_of::<__priv_TestAll2::Union>();
    if align < 4 { 4 } else { align }
  };
}

impl __::pz::test::TestAll2 {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_TestAll2::Storage {
      which: 0,
      union: __priv_TestAll2::Union { __unset: () },
    }} as *const __priv_TestAll2::Storage as *mut __priv_TestAll2::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::test::TestAll2`].
  pub const fn new() -> Self {
    Self {
      ptr: __s::ptr::NonNull::dangling(),
      arena: __s::option::Option::None,
    }
  }

  fn __init(&mut self) {
    if self.arena.is_none() {
      self.arena = __s::option::Option::Some(__z::RawArena::new());
    }
  }

  /// Deserializes a new [`__::pz::test::TestAll2`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::test::TestAll2`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::test::TestAll2`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::test::TestAll2`] to an in-memory byte array.
  ///
  /// See [`Message::to_bytes()`][__r::Message::to_bytes].
  fn to_bytes(&self, codec: __rt::Codec) -> __s::vec::Vec<__s::primitive::u8> {
    <Self as __r::Message>::to_bytes(self, codec)
  }

  /// Converts an ordinary Rust reference into a message reference.
  ///
  /// See [`Message::as_ref()`][__r::Message::as_ref].
  pub fn as_ref(&self) -> __rt::reflect::Ref<Self> {
    use __s::convert::From;
    unsafe { <Self as __z::Type>::__ref(
      __z::Seal,
      __s::ptr::NonNull::from(&self.ptr).cast::<__s::option::Option<__z::tdp::Opaque>>(),
    )}
  }

  /// Converts an ordinary Rust reference into a mutable message reference.
  ///
  /// See [`Message::as_mut()`][__r::Message::as_mut].
  pub fn as_mut(&mut self) -> __rt::reflect::Mut<Self> {
    use __s::convert::From;
    self.__init();
    unsafe { <Self as __z::Type>::__mut(
      __z::Seal,
      __s::ptr::NonNull::from(&mut self.ptr).cast::<__s::option::Option<__z::tdp::Opaque>>(),
      self.arena.unwrap(),
    )}
  }

  /// Selects the fields given by `selector` out of this message by reference.
  ///
  /// See [`Message::get()`][__r::Message::get].
  pub fn get<S>(&self, selector: S) -> __r::Ref<S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll2>,
  {
    <Self as __r::Message>::get(self, selector)
  }

  /// Selects the fields given by `selector` out of this message by mutable
  /// reference.
  ///
  /// If this would result in aliasing, it generates a post-monomorphization
  /// error.
  ///
  /// See [`Message::get_mut()`][__r::Message::get_mut].
  pub fn get_mut<S>(&mut self, selector: S) -> __r::Mut<S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll2>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::test::TestAll2`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn which(&self) -> i32 {
    self.as_ref().which()
  }

  pub fn cases(&self) -> __::pz::test::TestAll2Cases<__r::SelectRef> {
    self.as_ref().cases()
  }

  pub fn cases_mut(&mut self) -> __::pz::test::TestAll2Cases<__r::SelectMut> {
    self.as_mut().cases_mut()
  }

  pub fn opt_i32(&self) -> __rt::reflect::Ref<'_, __s::primitive::i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::i32>> {
    self.get(__field_TestAll2__opt_i32{})
  }
  pub fn opt_i32_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::i32> {
    self.opt_i32_mut_or().into_inner()
  }
  pub fn opt_i32_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::i32> {
    self.get_mut(__field_TestAll2__opt_i32{})
  }
  pub fn set_opt_i32(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::i32>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_i32_mut_or());
    self.as_mut()
  }

  pub fn opt_i64(&self) -> __rt::reflect::Ref<'_, __s::primitive::i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::i64>> {
    self.get(__field_TestAll2__opt_i64{})
  }
  pub fn opt_i64_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::i64> {
    self.opt_i64_mut_or().into_inner()
  }
  pub fn opt_i64_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::i64> {
    self.get_mut(__field_TestAll2__opt_i64{})
  }
  pub fn set_opt_i64(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::i64>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_i64_mut_or());
    self.as_mut()
  }

  pub fn opt_u32(&self) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u32>> {
    self.get(__field_TestAll2__opt_u32{})
  }
  pub fn opt_u32_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u32> {
    self.opt_u32_mut_or().into_inner()
  }
  pub fn opt_u32_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u32> {
    self.get_mut(__field_TestAll2__opt_u32{})
  }
  pub fn set_opt_u32(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_u32_mut_or());
    self.as_mut()
  }

  pub fn opt_u64(&self) -> __rt::reflect::Ref<'_, __s::primitive::u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u64>> {
    self.get(__field_TestAll2__opt_u64{})
  }
  pub fn opt_u64_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u64> {
    self.opt_u64_mut_or().into_inner()
  }
  pub fn opt_u64_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u64> {
    self.get_mut(__field_TestAll2__opt_u64{})
  }
  pub fn set_opt_u64(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u64>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_u64_mut_or());
    self.as_mut()
  }

  pub fn opt_f32(&self) -> __rt::reflect::Ref<'_, __s::primitive::f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::f32>> {
    self.get(__field_TestAll2__opt_f32{})
  }
  pub fn opt_f32_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::f32> {
    self.opt_f32_mut_or().into_inner()
  }
  pub fn opt_f32_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::f32> {
    self.get_mut(__field_TestAll2__opt_f32{})
  }
  pub fn set_opt_f32(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::f32>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_f32_mut_or());
    self.as_mut()
  }

  pub fn opt_f64(&self) -> __rt::reflect::Ref<'_, __s::primitive::f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::f64>> {
    self.get(__field_TestAll2__opt_f64{})
  }
  pub fn opt_f64_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::f64> {
    self.opt_f64_mut_or().into_inner()
  }
  pub fn opt_f64_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::f64> {
    self.get_mut(__field_TestAll2__opt_f64{})
  }
  pub fn set_opt_f64(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::f64>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_f64_mut_or());
    self.as_mut()
  }

  pub fn opt_str(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_TestAll2__opt_str{})
  }
  pub fn opt_str_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.opt_str_mut_or().into_inner()
  }
  pub fn opt_str_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_TestAll2__opt_str{})
  }
  pub fn set_opt_str(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_str_mut_or());
    self.as_mut()
  }

  pub fn opt_bool(&self) -> __rt::reflect::Ref<'_, __s::primitive::bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::bool>> {
    self.get(__field_TestAll2__opt_bool{})
  }
  pub fn opt_bool_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::bool> {
    self.opt_bool_mut_or().into_inner()
  }
  pub fn opt_bool_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::bool> {
    self.get_mut(__field_TestAll2__opt_bool{})
  }
  pub fn set_opt_bool(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::bool>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_bool_mut_or());
    self.as_mut()
  }

  pub fn opt_recursive(&self) -> __rt::reflect::Ref<'_, __::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::test::TestAll>> {
    self.get(__field_TestAll2__opt_recursive{})
  }
  pub fn opt_recursive_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::test::TestAll> {
    self.opt_recursive_mut_or().into_inner()
  }
  pub fn opt_recursive_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::test::TestAll> {
    self.get_mut(__field_TestAll2__opt_recursive{})
  }
  pub fn set_opt_recursive(&mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_recursive_mut_or());
    self.as_mut()
  }

  pub fn opt_nested(&self) -> __rt::reflect::Ref<'_, __::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::test::TestAll_Nested>> {
    self.get(__field_TestAll2__opt_nested{})
  }
  pub fn opt_nested_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::test::TestAll_Nested> {
    self.opt_nested_mut_or().into_inner()
  }
  pub fn opt_nested_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::test::TestAll_Nested> {
    self.get_mut(__field_TestAll2__opt_nested{})
  }
  pub fn set_opt_nested(&mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll_Nested>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_nested_mut_or());
    self.as_mut()
  }

  pub fn opt_choice(&self) -> __rt::reflect::Ref<'_, __::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::test::TestAll2>> {
    self.get(__field_TestAll2__opt_choice{})
  }
  pub fn opt_choice_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::test::TestAll2> {
    self.opt_choice_mut_or().into_inner()
  }
  pub fn opt_choice_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::test::TestAll2> {
    self.get_mut(__field_TestAll2__opt_choice{})
  }
  pub fn set_opt_choice(&mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll2>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_choice_mut_or());
    self.as_mut()
  }

  pub fn rep_i32(&self) -> __rt::Slice<'_, __s::primitive::i32> {
    self.get(__field_TestAll2__rep_i32{})
  }
  pub fn rep_i32_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::i32> {
    self.get_mut(__field_TestAll2__rep_i32{})
  }
  pub fn set_rep_i32(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::i32>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_i32_mut());
    self.as_mut()
  }

  pub fn rep_i64(&self) -> __rt::Slice<'_, __s::primitive::i64> {
    self.get(__field_TestAll2__rep_i64{})
  }
  pub fn rep_i64_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::i64> {
    self.get_mut(__field_TestAll2__rep_i64{})
  }
  pub fn set_rep_i64(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::i64>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_i64_mut());
    self.as_mut()
  }

  pub fn rep_u32(&self) -> __rt::Slice<'_, __s::primitive::u32> {
    self.get(__field_TestAll2__rep_u32{})
  }
  pub fn rep_u32_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::u32> {
    self.get_mut(__field_TestAll2__rep_u32{})
  }
  pub fn set_rep_u32(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_u32_mut());
    self.as_mut()
  }

  pub fn rep_u64(&self) -> __rt::Slice<'_, __s::primitive::u64> {
    self.get(__field_TestAll2__rep_u64{})
  }
  pub fn rep_u64_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::u64> {
    self.get_mut(__field_TestAll2__rep_u64{})
  }
  pub fn set_rep_u64(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::u64>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_u64_mut());
    self.as_mut()
  }

  pub fn rep_f32(&self) -> __rt::Slice<'_, __s::primitive::f32> {
    self.get(__field_TestAll2__rep_f32{})
  }
  pub fn rep_f32_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::f32> {
    self.get_mut(__field_TestAll2__rep_f32{})
  }
  pub fn set_rep_f32(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::f32>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_f32_mut());
    self.as_mut()
  }

  pub fn rep_f64(&self) -> __rt::Slice<'_, __s::primitive::f64> {
    self.get(__field_TestAll2__rep_f64{})
  }
  pub fn rep_f64_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::f64> {
    self.get_mut(__field_TestAll2__rep_f64{})
  }
  pub fn set_rep_f64(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::f64>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_f64_mut());
    self.as_mut()
  }

  pub fn rep_str(&self) -> __rt::Slice<'_, __rt::String> {
    self.get(__field_TestAll2__rep_str{})
  }
  pub fn rep_str_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __rt::String> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(&mut self) -> __rt::Repeated<'_, __rt::String> {
    self.get_mut(__field_TestAll2__rep_str{})
  }
  pub fn set_rep_str(&mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_str_mut());
    self.as_mut()
  }

  pub fn rep_bool(&self) -> __rt::Slice<'_, __s::primitive::bool> {
    self.get(__field_TestAll2__rep_bool{})
  }
  pub fn rep_bool_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::bool> {
    self.get_mut(__field_TestAll2__rep_bool{})
  }
  pub fn set_rep_bool(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::bool>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_bool_mut());
    self.as_mut()
  }

  pub fn rep_recursive(&self) -> __rt::Slice<'_, __::pz::test::TestAll> {
    self.get(__field_TestAll2__rep_recursive{})
  }
  pub fn rep_recursive_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(&mut self) -> __rt::Repeated<'_, __::pz::test::TestAll> {
    self.get_mut(__field_TestAll2__rep_recursive{})
  }
  pub fn set_rep_recursive(&mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_recursive_mut());
    self.as_mut()
  }

  pub fn rep_nested(&self) -> __rt::Slice<'_, __::pz::test::TestAll_Nested> {
    self.get(__field_TestAll2__rep_nested{})
  }
  pub fn rep_nested_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(&mut self) -> __rt::Repeated<'_, __::pz::test::TestAll_Nested> {
    self.get_mut(__field_TestAll2__rep_nested{})
  }
  pub fn set_rep_nested(&mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll_Nested>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_nested_mut());
    self.as_mut()
  }

  pub fn rep_choice(&self) -> __rt::Slice<'_, __::pz::test::TestAll2> {
    self.get(__field_TestAll2__rep_choice{})
  }
  pub fn rep_choice_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::test::TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(&mut self) -> __rt::Repeated<'_, __::pz::test::TestAll2> {
    self.get_mut(__field_TestAll2__rep_choice{})
  }
  pub fn set_rep_choice(&mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll2>>) -> __r::Mut<'_, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_choice_mut());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_TestAll2::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_TestAll2::Storage>().as_mut().which = 0;
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::test::TestAll2>, src: __rt::reflect::Ref<__::pz::test::TestAll2>) {
    match src.which() {
      0 => dst.clear(),
      1 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_i32>>::Type>::apply_to(src.get(__field_TestAll2__opt_i32{}), dst.get_mut(__field_TestAll2__opt_i32{})),
      2 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_i64>>::Type>::apply_to(src.get(__field_TestAll2__opt_i64{}), dst.get_mut(__field_TestAll2__opt_i64{})),
      3 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_u32>>::Type>::apply_to(src.get(__field_TestAll2__opt_u32{}), dst.get_mut(__field_TestAll2__opt_u32{})),
      4 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_u64>>::Type>::apply_to(src.get(__field_TestAll2__opt_u64{}), dst.get_mut(__field_TestAll2__opt_u64{})),
      5 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_f32>>::Type>::apply_to(src.get(__field_TestAll2__opt_f32{}), dst.get_mut(__field_TestAll2__opt_f32{})),
      6 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_f64>>::Type>::apply_to(src.get(__field_TestAll2__opt_f64{}), dst.get_mut(__field_TestAll2__opt_f64{})),
      7 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_str>>::Type>::apply_to(src.get(__field_TestAll2__opt_str{}), dst.get_mut(__field_TestAll2__opt_str{})),
      8 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_bool>>::Type>::apply_to(src.get(__field_TestAll2__opt_bool{}), dst.get_mut(__field_TestAll2__opt_bool{})),
      10 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_recursive>>::Type>::apply_to(src.get(__field_TestAll2__opt_recursive{}), dst.get_mut(__field_TestAll2__opt_recursive{})),
      11 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_nested>>::Type>::apply_to(src.get(__field_TestAll2__opt_nested{}), dst.get_mut(__field_TestAll2__opt_nested{})),
      12 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__opt_choice>>::Type>::apply_to(src.get(__field_TestAll2__opt_choice{}), dst.get_mut(__field_TestAll2__opt_choice{})),
      21 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_i32>>::Type>::apply_to(src.get(__field_TestAll2__rep_i32{}), dst.get_mut(__field_TestAll2__rep_i32{})),
      22 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_i64>>::Type>::apply_to(src.get(__field_TestAll2__rep_i64{}), dst.get_mut(__field_TestAll2__rep_i64{})),
      23 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_u32>>::Type>::apply_to(src.get(__field_TestAll2__rep_u32{}), dst.get_mut(__field_TestAll2__rep_u32{})),
      24 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_u64>>::Type>::apply_to(src.get(__field_TestAll2__rep_u64{}), dst.get_mut(__field_TestAll2__rep_u64{})),
      25 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_f32>>::Type>::apply_to(src.get(__field_TestAll2__rep_f32{}), dst.get_mut(__field_TestAll2__rep_f32{})),
      26 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_f64>>::Type>::apply_to(src.get(__field_TestAll2__rep_f64{}), dst.get_mut(__field_TestAll2__rep_f64{})),
      27 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_str>>::Type>::apply_to(src.get(__field_TestAll2__rep_str{}), dst.get_mut(__field_TestAll2__rep_str{})),
      28 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_bool>>::Type>::apply_to(src.get(__field_TestAll2__rep_bool{}), dst.get_mut(__field_TestAll2__rep_bool{})),
      30 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_recursive>>::Type>::apply_to(src.get(__field_TestAll2__rep_recursive{}), dst.get_mut(__field_TestAll2__rep_recursive{})),
      31 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_nested>>::Type>::apply_to(src.get(__field_TestAll2__rep_nested{}), dst.get_mut(__field_TestAll2__rep_nested{})),
      32 => __r::Set::<<__::pz::test::TestAll2 as __r::Field<__field_TestAll2__rep_choice>>::Type>::apply_to(src.get(__field_TestAll2__rep_choice{}), dst.get_mut(__field_TestAll2__rep_choice{})),
      _ => __s::unreachable!(),
    }
  }
}

#[non_exhaustive]
pub enum TestAll2Cases<'proto, Which: __r::Select = __r::SelectRef> {
  Unset,
  OptI32(__r::View<'proto, __s::primitive::i32, Which>),
  OptI64(__r::View<'proto, __s::primitive::i64, Which>),
  OptU32(__r::View<'proto, __s::primitive::u32, Which>),
  OptU64(__r::View<'proto, __s::primitive::u64, Which>),
  OptF32(__r::View<'proto, __s::primitive::f32, Which>),
  OptF64(__r::View<'proto, __s::primitive::f64, Which>),
  OptStr(__r::View<'proto, __rt::String, Which>),
  OptBool(__r::View<'proto, __s::primitive::bool, Which>),
  OptRecursive(__r::View<'proto, __::pz::test::TestAll, Which>),
  OptNested(__r::View<'proto, __::pz::test::TestAll_Nested, Which>),
  OptChoice(__r::View<'proto, __::pz::test::TestAll2, Which>),
  RepI32(__r::View<'proto, __r::Rep<__s::primitive::i32>, Which>),
  RepI64(__r::View<'proto, __r::Rep<__s::primitive::i64>, Which>),
  RepU32(__r::View<'proto, __r::Rep<__s::primitive::u32>, Which>),
  RepU64(__r::View<'proto, __r::Rep<__s::primitive::u64>, Which>),
  RepF32(__r::View<'proto, __r::Rep<__s::primitive::f32>, Which>),
  RepF64(__r::View<'proto, __r::Rep<__s::primitive::f64>, Which>),
  RepStr(__r::View<'proto, __r::Rep<__rt::String>, Which>),
  RepBool(__r::View<'proto, __r::Rep<__s::primitive::bool>, Which>),
  RepRecursive(__r::View<'proto, __r::Rep<__::pz::test::TestAll>, Which>),
  RepNested(__r::View<'proto, __r::Rep<__::pz::test::TestAll_Nested>, Which>),
  RepChoice(__r::View<'proto, __r::Rep<__::pz::test::TestAll2>, Which>),

  #[doc(hidden)]
  __PhantomData(__s::marker::PhantomData<&'proto Which>, __z::Void),
}

impl __z::Message for __::pz::test::TestAll2 {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{22 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::test::TestAll2::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::test::TestAll::__tdp_info,
            __::pz::test::TestAll_Nested::__tdp_info,
            __::pz::test::TestAll2::__tdp_info,
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

    unsafe { STATIC.get() }
  };

  fn __is_null(&self, _: impl __z::Sealed) -> bool {
    self.ptr == __s::ptr::NonNull::dangling()
  }
  fn __raw(_: impl __z::Sealed, ptr: __r::Ref<Self>) -> __z::tdp::Opaque { ptr.ptr.cast() }
  fn __arena(_: impl __z::Sealed, ptr: &mut __r::Mut<Self>) -> __z::RawArena { ptr.arena }
}

impl __z::Type for __::pz::test::TestAll2 {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::test::TestAll2::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::test::TestAll2::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_TestAll2::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_TestAll2::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_TestAll2::Mut {
      r: Self::__ref(s, outer),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize<S: __z::Sealed>(
    _: S,
    vec: &mut __z::AVec<__s::option::Option<__z::tdp::Opaque>>,
    new_len: usize,
    arena: __z::RawArena,
  ) {
    vec.resize(new_len, arena)
  }
}

impl __r::Views for __::pz::test::TestAll2 {
  type Ref<'a> = __priv_TestAll2::Ref<'a>;
  type Mut<'a> = __priv_TestAll2::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_TestAll2::Ref<'a> {
  type Target = __::pz::test::TestAll2;
  fn as_ref(&self) -> __priv_TestAll2::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_TestAll2::Mut<'a> {
  type Target = __::pz::test::TestAll2;
  fn as_ref(&self) -> __priv_TestAll2::Ref { self.r }
  fn into_ref(self) -> __priv_TestAll2::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_TestAll2::Mut {
    __priv_TestAll2::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::test::TestAll2 {
  const DEFAULT: &'static Self = __::pz::test::TestAll2::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_TestAll2::Ref<'a> {
  type Message = __::pz::test::TestAll2;
}
impl<'a> __r::MessageMut<'a> for __priv_TestAll2::Mut<'a> {
  type Message = __::pz::test::TestAll2;
}

impl __s::default::Default for __::pz::test::TestAll2 {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::test::TestAll2>> __s::convert::From<T> for __::pz::test::TestAll2 {
  fn from(value: T) -> __::pz::test::TestAll2 {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::test::TestAll2> for &__::pz::test::TestAll2 {
  fn apply_to(self, mut m: __r::Mut<__::pz::test::TestAll2>) {
    __::pz::test::TestAll2::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::test::TestAll2>> for &__::pz::test::TestAll2 {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::test::TestAll2>>) {
    __::pz::test::TestAll2::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::test::TestAll2> for __rt::reflect::Ref<'_, __::pz::test::TestAll2> {
  fn apply_to(self, mut m: __r::Mut<__::pz::test::TestAll2>) {
    __::pz::test::TestAll2::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::test::TestAll2>> for __rt::reflect::Ref<'_, __::pz::test::TestAll2> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::test::TestAll2>>) {
    __::pz::test::TestAll2::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::test::TestAll2> for &__rt::reflect::Mut<'_, __::pz::test::TestAll2> {
  fn apply_to(self, mut m: __r::Mut<__::pz::test::TestAll2>) {
    __::pz::test::TestAll2::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::test::TestAll2>> for &__rt::reflect::Mut<'_, __::pz::test::TestAll2> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::test::TestAll2>>) {
    __::pz::test::TestAll2::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_TestAll2::Ref<'_> {
  fn default() -> Self {
    __::pz::test::TestAll2::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::test::TestAll2 {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_TestAll2::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.test.TestAll2 ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_TestAll2::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::test::TestAll2 {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_TestAll2__opt_i32 = __rt::field!(opt_i32);
impl __r::Field<__field_TestAll2__opt_i32> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__s::primitive::i32>;
  type Name = __field_TestAll2__opt_i32;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "opt_i32";
}

type __field_TestAll2__opt_i64 = __rt::field!(opt_i64);
impl __r::Field<__field_TestAll2__opt_i64> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__s::primitive::i64>;
  type Name = __field_TestAll2__opt_i64;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "opt_i64";
}

type __field_TestAll2__opt_u32 = __rt::field!(opt_u32);
impl __r::Field<__field_TestAll2__opt_u32> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__s::primitive::u32>;
  type Name = __field_TestAll2__opt_u32;
  const NUMBER: __s::primitive::i32 = 3;
  const INDEX: __s::primitive::usize = 2;
  const NAME: &'static __s::primitive::str = "opt_u32";
}

type __field_TestAll2__opt_u64 = __rt::field!(opt_u64);
impl __r::Field<__field_TestAll2__opt_u64> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__s::primitive::u64>;
  type Name = __field_TestAll2__opt_u64;
  const NUMBER: __s::primitive::i32 = 4;
  const INDEX: __s::primitive::usize = 3;
  const NAME: &'static __s::primitive::str = "opt_u64";
}

type __field_TestAll2__opt_f32 = __rt::field!(opt_f32);
impl __r::Field<__field_TestAll2__opt_f32> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__s::primitive::f32>;
  type Name = __field_TestAll2__opt_f32;
  const NUMBER: __s::primitive::i32 = 5;
  const INDEX: __s::primitive::usize = 4;
  const NAME: &'static __s::primitive::str = "opt_f32";
}

type __field_TestAll2__opt_f64 = __rt::field!(opt_f64);
impl __r::Field<__field_TestAll2__opt_f64> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__s::primitive::f64>;
  type Name = __field_TestAll2__opt_f64;
  const NUMBER: __s::primitive::i32 = 6;
  const INDEX: __s::primitive::usize = 5;
  const NAME: &'static __s::primitive::str = "opt_f64";
}

type __field_TestAll2__opt_str = __rt::field!(opt_str);
impl __r::Field<__field_TestAll2__opt_str> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_TestAll2__opt_str;
  const NUMBER: __s::primitive::i32 = 7;
  const INDEX: __s::primitive::usize = 6;
  const NAME: &'static __s::primitive::str = "opt_str";
}

type __field_TestAll2__opt_bool = __rt::field!(opt_bool);
impl __r::Field<__field_TestAll2__opt_bool> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__s::primitive::bool>;
  type Name = __field_TestAll2__opt_bool;
  const NUMBER: __s::primitive::i32 = 8;
  const INDEX: __s::primitive::usize = 7;
  const NAME: &'static __s::primitive::str = "opt_bool";
}

type __field_TestAll2__opt_recursive = __rt::field!(opt_recursive);
impl __r::Field<__field_TestAll2__opt_recursive> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__::pz::test::TestAll>;
  type Name = __field_TestAll2__opt_recursive;
  const NUMBER: __s::primitive::i32 = 10;
  const INDEX: __s::primitive::usize = 8;
  const NAME: &'static __s::primitive::str = "opt_recursive";
}

type __field_TestAll2__opt_nested = __rt::field!(opt_nested);
impl __r::Field<__field_TestAll2__opt_nested> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__::pz::test::TestAll_Nested>;
  type Name = __field_TestAll2__opt_nested;
  const NUMBER: __s::primitive::i32 = 11;
  const INDEX: __s::primitive::usize = 9;
  const NAME: &'static __s::primitive::str = "opt_nested";
}

type __field_TestAll2__opt_choice = __rt::field!(opt_choice);
impl __r::Field<__field_TestAll2__opt_choice> for __::pz::test::TestAll2 {
  type Type = __r::Opt<__::pz::test::TestAll2>;
  type Name = __field_TestAll2__opt_choice;
  const NUMBER: __s::primitive::i32 = 12;
  const INDEX: __s::primitive::usize = 10;
  const NAME: &'static __s::primitive::str = "opt_choice";
}

type __field_TestAll2__rep_i32 = __rt::field!(rep_i32);
impl __r::Field<__field_TestAll2__rep_i32> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__s::primitive::i32>;
  type Name = __field_TestAll2__rep_i32;
  const NUMBER: __s::primitive::i32 = 21;
  const INDEX: __s::primitive::usize = 11;
  const NAME: &'static __s::primitive::str = "rep_i32";
}

type __field_TestAll2__rep_i64 = __rt::field!(rep_i64);
impl __r::Field<__field_TestAll2__rep_i64> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__s::primitive::i64>;
  type Name = __field_TestAll2__rep_i64;
  const NUMBER: __s::primitive::i32 = 22;
  const INDEX: __s::primitive::usize = 12;
  const NAME: &'static __s::primitive::str = "rep_i64";
}

type __field_TestAll2__rep_u32 = __rt::field!(rep_u32);
impl __r::Field<__field_TestAll2__rep_u32> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__s::primitive::u32>;
  type Name = __field_TestAll2__rep_u32;
  const NUMBER: __s::primitive::i32 = 23;
  const INDEX: __s::primitive::usize = 13;
  const NAME: &'static __s::primitive::str = "rep_u32";
}

type __field_TestAll2__rep_u64 = __rt::field!(rep_u64);
impl __r::Field<__field_TestAll2__rep_u64> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__s::primitive::u64>;
  type Name = __field_TestAll2__rep_u64;
  const NUMBER: __s::primitive::i32 = 24;
  const INDEX: __s::primitive::usize = 14;
  const NAME: &'static __s::primitive::str = "rep_u64";
}

type __field_TestAll2__rep_f32 = __rt::field!(rep_f32);
impl __r::Field<__field_TestAll2__rep_f32> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__s::primitive::f32>;
  type Name = __field_TestAll2__rep_f32;
  const NUMBER: __s::primitive::i32 = 25;
  const INDEX: __s::primitive::usize = 15;
  const NAME: &'static __s::primitive::str = "rep_f32";
}

type __field_TestAll2__rep_f64 = __rt::field!(rep_f64);
impl __r::Field<__field_TestAll2__rep_f64> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__s::primitive::f64>;
  type Name = __field_TestAll2__rep_f64;
  const NUMBER: __s::primitive::i32 = 26;
  const INDEX: __s::primitive::usize = 16;
  const NAME: &'static __s::primitive::str = "rep_f64";
}

type __field_TestAll2__rep_str = __rt::field!(rep_str);
impl __r::Field<__field_TestAll2__rep_str> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__rt::String>;
  type Name = __field_TestAll2__rep_str;
  const NUMBER: __s::primitive::i32 = 27;
  const INDEX: __s::primitive::usize = 17;
  const NAME: &'static __s::primitive::str = "rep_str";
}

type __field_TestAll2__rep_bool = __rt::field!(rep_bool);
impl __r::Field<__field_TestAll2__rep_bool> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__s::primitive::bool>;
  type Name = __field_TestAll2__rep_bool;
  const NUMBER: __s::primitive::i32 = 28;
  const INDEX: __s::primitive::usize = 18;
  const NAME: &'static __s::primitive::str = "rep_bool";
}

type __field_TestAll2__rep_recursive = __rt::field!(rep_recursive);
impl __r::Field<__field_TestAll2__rep_recursive> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__::pz::test::TestAll>;
  type Name = __field_TestAll2__rep_recursive;
  const NUMBER: __s::primitive::i32 = 30;
  const INDEX: __s::primitive::usize = 19;
  const NAME: &'static __s::primitive::str = "rep_recursive";
}

type __field_TestAll2__rep_nested = __rt::field!(rep_nested);
impl __r::Field<__field_TestAll2__rep_nested> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__::pz::test::TestAll_Nested>;
  type Name = __field_TestAll2__rep_nested;
  const NUMBER: __s::primitive::i32 = 31;
  const INDEX: __s::primitive::usize = 20;
  const NAME: &'static __s::primitive::str = "rep_nested";
}

type __field_TestAll2__rep_choice = __rt::field!(rep_choice);
impl __r::Field<__field_TestAll2__rep_choice> for __::pz::test::TestAll2 {
  type Type = __r::Rep<__::pz::test::TestAll2>;
  type Name = __field_TestAll2__rep_choice;
  const NUMBER: __s::primitive::i32 = 32;
  const INDEX: __s::primitive::usize = 21;
  const NAME: &'static __s::primitive::str = "rep_choice";
}

impl<'proto> __priv_TestAll2::Ref<'proto> {
  pub fn which(self) -> i32 {
    unsafe { self.ptr.as_ref() }.which as i32
  }

  pub fn cases(self) -> __::pz::test::TestAll2Cases<'proto> {
    unsafe {
      match self.which() {
        0 => __::pz::test::TestAll2Cases::Unset,
        1 => __::pz::test::TestAll2Cases::OptI32(self.get(__field_TestAll2__opt_i32{}).unwrap_unchecked()),
        2 => __::pz::test::TestAll2Cases::OptI64(self.get(__field_TestAll2__opt_i64{}).unwrap_unchecked()),
        3 => __::pz::test::TestAll2Cases::OptU32(self.get(__field_TestAll2__opt_u32{}).unwrap_unchecked()),
        4 => __::pz::test::TestAll2Cases::OptU64(self.get(__field_TestAll2__opt_u64{}).unwrap_unchecked()),
        5 => __::pz::test::TestAll2Cases::OptF32(self.get(__field_TestAll2__opt_f32{}).unwrap_unchecked()),
        6 => __::pz::test::TestAll2Cases::OptF64(self.get(__field_TestAll2__opt_f64{}).unwrap_unchecked()),
        7 => __::pz::test::TestAll2Cases::OptStr(self.get(__field_TestAll2__opt_str{}).unwrap_unchecked()),
        8 => __::pz::test::TestAll2Cases::OptBool(self.get(__field_TestAll2__opt_bool{}).unwrap_unchecked()),
        10 => __::pz::test::TestAll2Cases::OptRecursive(self.get(__field_TestAll2__opt_recursive{}).unwrap_unchecked()),
        11 => __::pz::test::TestAll2Cases::OptNested(self.get(__field_TestAll2__opt_nested{}).unwrap_unchecked()),
        12 => __::pz::test::TestAll2Cases::OptChoice(self.get(__field_TestAll2__opt_choice{}).unwrap_unchecked()),
        21 => __::pz::test::TestAll2Cases::RepI32(self.get(__field_TestAll2__rep_i32{})),
        22 => __::pz::test::TestAll2Cases::RepI64(self.get(__field_TestAll2__rep_i64{})),
        23 => __::pz::test::TestAll2Cases::RepU32(self.get(__field_TestAll2__rep_u32{})),
        24 => __::pz::test::TestAll2Cases::RepU64(self.get(__field_TestAll2__rep_u64{})),
        25 => __::pz::test::TestAll2Cases::RepF32(self.get(__field_TestAll2__rep_f32{})),
        26 => __::pz::test::TestAll2Cases::RepF64(self.get(__field_TestAll2__rep_f64{})),
        27 => __::pz::test::TestAll2Cases::RepStr(self.get(__field_TestAll2__rep_str{})),
        28 => __::pz::test::TestAll2Cases::RepBool(self.get(__field_TestAll2__rep_bool{})),
        30 => __::pz::test::TestAll2Cases::RepRecursive(self.get(__field_TestAll2__rep_recursive{})),
        31 => __::pz::test::TestAll2Cases::RepNested(self.get(__field_TestAll2__rep_nested{})),
        32 => __::pz::test::TestAll2Cases::RepChoice(self.get(__field_TestAll2__rep_choice{})),
        _ => __s::unreachable!(),
      }
    }
  }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_TestAll2::Ref { *self }

  /// Serializes this [`__::pz::test::TestAll2`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::test::TestAll2`] to an in-memory byte array.
  ///
  /// See [`MessageRef::to_bytes()`][__r::MessageRef::to_bytes].
  fn to_bytes(&self, codec: __rt::Codec) -> __s::vec::Vec<__s::primitive::u8> {
    <Self as __r::MessageRef>::to_bytes(self, codec)
  }

  /// Selects the fields given by `selector` out of this message by reference.
  ///
  /// See [`MessageRef::get()`][__r::MessageRef::get].
  pub fn get<S>(self, selector: S) -> __r::Ref<'proto, S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll2>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn opt_i32(self) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i32>> {
    self.get(__field_TestAll2__opt_i32{})
  }

  pub fn opt_i64(self) -> __rt::reflect::Ref<'proto, __s::primitive::i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i64>> {
    self.get(__field_TestAll2__opt_i64{})
  }

  pub fn opt_u32(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_TestAll2__opt_u32{})
  }

  pub fn opt_u64(self) -> __rt::reflect::Ref<'proto, __s::primitive::u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u64>> {
    self.get(__field_TestAll2__opt_u64{})
  }

  pub fn opt_f32(self) -> __rt::reflect::Ref<'proto, __s::primitive::f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::f32>> {
    self.get(__field_TestAll2__opt_f32{})
  }

  pub fn opt_f64(self) -> __rt::reflect::Ref<'proto, __s::primitive::f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::f64>> {
    self.get(__field_TestAll2__opt_f64{})
  }

  pub fn opt_str(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_TestAll2__opt_str{})
  }

  pub fn opt_bool(self) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::bool>> {
    self.get(__field_TestAll2__opt_bool{})
  }

  pub fn opt_recursive(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll>> {
    self.get(__field_TestAll2__opt_recursive{})
  }

  pub fn opt_nested(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested>> {
    self.get(__field_TestAll2__opt_nested{})
  }

  pub fn opt_choice(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll2>> {
    self.get(__field_TestAll2__opt_choice{})
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, __s::primitive::i32> {
    self.get(__field_TestAll2__rep_i32{})
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.rep_i32().at(idx)
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, __s::primitive::i64> {
    self.get(__field_TestAll2__rep_i64{})
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::i64> {
    self.rep_i64().at(idx)
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, __s::primitive::u32> {
    self.get(__field_TestAll2__rep_u32{})
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.rep_u32().at(idx)
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, __s::primitive::u64> {
    self.get(__field_TestAll2__rep_u64{})
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u64> {
    self.rep_u64().at(idx)
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, __s::primitive::f32> {
    self.get(__field_TestAll2__rep_f32{})
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::f32> {
    self.rep_f32().at(idx)
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, __s::primitive::f64> {
    self.get(__field_TestAll2__rep_f64{})
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::f64> {
    self.rep_f64().at(idx)
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_TestAll2__rep_str{})
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.rep_str().at(idx)
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, __s::primitive::bool> {
    self.get(__field_TestAll2__rep_bool{})
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.rep_bool().at(idx)
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, __::pz::test::TestAll> {
    self.get(__field_TestAll2__rep_recursive{})
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, __::pz::test::TestAll_Nested> {
    self.get(__field_TestAll2__rep_nested{})
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, __::pz::test::TestAll2> {
    self.get(__field_TestAll2__rep_choice{})
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll2> {
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

impl<'proto> __priv_TestAll2::Mut<'proto>  {
  pub fn which(&self) -> i32 {
    self.as_ref().which()
  }

  pub fn cases(self) -> __::pz::test::TestAll2Cases<'proto> {
    self.into_ref().cases()
  }

  pub fn cases_mut(self) -> __::pz::test::TestAll2Cases<'proto, __r::SelectMut> {
    unsafe {
      match self.which() {
        0 => __::pz::test::TestAll2Cases::Unset,
        1 => __::pz::test::TestAll2Cases::OptI32(self.get_mut(__field_TestAll2__opt_i32{}).into_option().unwrap_unchecked()),
        2 => __::pz::test::TestAll2Cases::OptI64(self.get_mut(__field_TestAll2__opt_i64{}).into_option().unwrap_unchecked()),
        3 => __::pz::test::TestAll2Cases::OptU32(self.get_mut(__field_TestAll2__opt_u32{}).into_option().unwrap_unchecked()),
        4 => __::pz::test::TestAll2Cases::OptU64(self.get_mut(__field_TestAll2__opt_u64{}).into_option().unwrap_unchecked()),
        5 => __::pz::test::TestAll2Cases::OptF32(self.get_mut(__field_TestAll2__opt_f32{}).into_option().unwrap_unchecked()),
        6 => __::pz::test::TestAll2Cases::OptF64(self.get_mut(__field_TestAll2__opt_f64{}).into_option().unwrap_unchecked()),
        7 => __::pz::test::TestAll2Cases::OptStr(self.get_mut(__field_TestAll2__opt_str{}).into_option().unwrap_unchecked()),
        8 => __::pz::test::TestAll2Cases::OptBool(self.get_mut(__field_TestAll2__opt_bool{}).into_option().unwrap_unchecked()),
        10 => __::pz::test::TestAll2Cases::OptRecursive(self.get_mut(__field_TestAll2__opt_recursive{}).into_option().unwrap_unchecked()),
        11 => __::pz::test::TestAll2Cases::OptNested(self.get_mut(__field_TestAll2__opt_nested{}).into_option().unwrap_unchecked()),
        12 => __::pz::test::TestAll2Cases::OptChoice(self.get_mut(__field_TestAll2__opt_choice{}).into_option().unwrap_unchecked()),
        21 => __::pz::test::TestAll2Cases::RepI32(self.get_mut(__field_TestAll2__rep_i32{})),
        22 => __::pz::test::TestAll2Cases::RepI64(self.get_mut(__field_TestAll2__rep_i64{})),
        23 => __::pz::test::TestAll2Cases::RepU32(self.get_mut(__field_TestAll2__rep_u32{})),
        24 => __::pz::test::TestAll2Cases::RepU64(self.get_mut(__field_TestAll2__rep_u64{})),
        25 => __::pz::test::TestAll2Cases::RepF32(self.get_mut(__field_TestAll2__rep_f32{})),
        26 => __::pz::test::TestAll2Cases::RepF64(self.get_mut(__field_TestAll2__rep_f64{})),
        27 => __::pz::test::TestAll2Cases::RepStr(self.get_mut(__field_TestAll2__rep_str{})),
        28 => __::pz::test::TestAll2Cases::RepBool(self.get_mut(__field_TestAll2__rep_bool{})),
        30 => __::pz::test::TestAll2Cases::RepRecursive(self.get_mut(__field_TestAll2__rep_recursive{})),
        31 => __::pz::test::TestAll2Cases::RepNested(self.get_mut(__field_TestAll2__rep_nested{})),
        32 => __::pz::test::TestAll2Cases::RepChoice(self.get_mut(__field_TestAll2__rep_choice{})),
        _ => __s::unreachable!(),
      }
    }
  }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_TestAll2::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_TestAll2::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_TestAll2::Mut {
    __priv_TestAll2::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::test::TestAll2`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::test::TestAll2`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::test::TestAll2`] to an in-memory byte array.
  ///
  /// See [`MessageMut::to_bytes()`][__r::MessageMut::to_bytes].
  fn to_bytes(&self, codec: __rt::Codec) -> __s::vec::Vec<__s::primitive::u8> {
    <Self as __r::MessageMut>::to_bytes(self, codec)
  }

  /// Selects the fields given by `selector` out of this message by reference.
  ///
  /// See [`MessageMut::get()`][__r::MessageMut::get].
  pub fn get<S>(self, selector: S) -> __r::Ref<'proto, S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll2>,
  {
    <Self as __r::MessageMut>::get(self, selector)
  }

  /// Selects the fields given by `selector` out of this message by mutable
  /// reference.
  ///
  /// If this would result in aliasing, it generates a post-monomorphization
  /// error.
  ///
  /// See [`MessageMut::get_mut()`][__r::MessageMut::get_mut].
  pub fn get_mut<S>(self, selector: S) -> __r::Mut<'proto, S::Type>
  where
    S: __r::Selector<__::pz::test::TestAll2>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::test::TestAll2`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn opt_i32(self) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.opt_i32_or().unwrap_or_default()
  }
  pub fn opt_i32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i32>> {
    self.get(__field_TestAll2__opt_i32{})
  }
  pub fn opt_i32_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::i32> {
    self.opt_i32_mut_or().into_inner()
  }
  pub fn opt_i32_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::i32> {
    self.get_mut(__field_TestAll2__opt_i32{})
  }
  pub fn set_opt_i32(mut self, value: impl __r::Set<__r::Opt<__s::primitive::i32>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_i32_mut_or());
    self
  }

  pub fn opt_i64(self) -> __rt::reflect::Ref<'proto, __s::primitive::i64> {
    self.opt_i64_or().unwrap_or_default()
  }
  pub fn opt_i64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i64>> {
    self.get(__field_TestAll2__opt_i64{})
  }
  pub fn opt_i64_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::i64> {
    self.opt_i64_mut_or().into_inner()
  }
  pub fn opt_i64_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::i64> {
    self.get_mut(__field_TestAll2__opt_i64{})
  }
  pub fn set_opt_i64(mut self, value: impl __r::Set<__r::Opt<__s::primitive::i64>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_i64_mut_or());
    self
  }

  pub fn opt_u32(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.opt_u32_or().unwrap_or_default()
  }
  pub fn opt_u32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_TestAll2__opt_u32{})
  }
  pub fn opt_u32_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u32> {
    self.opt_u32_mut_or().into_inner()
  }
  pub fn opt_u32_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u32> {
    self.get_mut(__field_TestAll2__opt_u32{})
  }
  pub fn set_opt_u32(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_u32_mut_or());
    self
  }

  pub fn opt_u64(self) -> __rt::reflect::Ref<'proto, __s::primitive::u64> {
    self.opt_u64_or().unwrap_or_default()
  }
  pub fn opt_u64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u64>> {
    self.get(__field_TestAll2__opt_u64{})
  }
  pub fn opt_u64_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u64> {
    self.opt_u64_mut_or().into_inner()
  }
  pub fn opt_u64_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u64> {
    self.get_mut(__field_TestAll2__opt_u64{})
  }
  pub fn set_opt_u64(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u64>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_u64_mut_or());
    self
  }

  pub fn opt_f32(self) -> __rt::reflect::Ref<'proto, __s::primitive::f32> {
    self.opt_f32_or().unwrap_or_default()
  }
  pub fn opt_f32_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::f32>> {
    self.get(__field_TestAll2__opt_f32{})
  }
  pub fn opt_f32_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::f32> {
    self.opt_f32_mut_or().into_inner()
  }
  pub fn opt_f32_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::f32> {
    self.get_mut(__field_TestAll2__opt_f32{})
  }
  pub fn set_opt_f32(mut self, value: impl __r::Set<__r::Opt<__s::primitive::f32>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_f32_mut_or());
    self
  }

  pub fn opt_f64(self) -> __rt::reflect::Ref<'proto, __s::primitive::f64> {
    self.opt_f64_or().unwrap_or_default()
  }
  pub fn opt_f64_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::f64>> {
    self.get(__field_TestAll2__opt_f64{})
  }
  pub fn opt_f64_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::f64> {
    self.opt_f64_mut_or().into_inner()
  }
  pub fn opt_f64_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::f64> {
    self.get_mut(__field_TestAll2__opt_f64{})
  }
  pub fn set_opt_f64(mut self, value: impl __r::Set<__r::Opt<__s::primitive::f64>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_f64_mut_or());
    self
  }

  pub fn opt_str(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.opt_str_or().unwrap_or_default()
  }
  pub fn opt_str_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_TestAll2__opt_str{})
  }
  pub fn opt_str_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.opt_str_mut_or().into_inner()
  }
  pub fn opt_str_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_TestAll2__opt_str{})
  }
  pub fn set_opt_str(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_str_mut_or());
    self
  }

  pub fn opt_bool(self) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.opt_bool_or().unwrap_or_default()
  }
  pub fn opt_bool_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::bool>> {
    self.get(__field_TestAll2__opt_bool{})
  }
  pub fn opt_bool_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::bool> {
    self.opt_bool_mut_or().into_inner()
  }
  pub fn opt_bool_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::bool> {
    self.get_mut(__field_TestAll2__opt_bool{})
  }
  pub fn set_opt_bool(mut self, value: impl __r::Set<__r::Opt<__s::primitive::bool>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_bool_mut_or());
    self
  }

  pub fn opt_recursive(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll> {
    self.opt_recursive_or().unwrap_or_default()
  }
  pub fn opt_recursive_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll>> {
    self.get(__field_TestAll2__opt_recursive{})
  }
  pub fn opt_recursive_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::test::TestAll> {
    self.opt_recursive_mut_or().into_inner()
  }
  pub fn opt_recursive_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::test::TestAll> {
    self.get_mut(__field_TestAll2__opt_recursive{})
  }
  pub fn set_opt_recursive(mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_recursive_mut_or());
    self
  }

  pub fn opt_nested(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested> {
    self.opt_nested_or().unwrap_or_default()
  }
  pub fn opt_nested_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested>> {
    self.get(__field_TestAll2__opt_nested{})
  }
  pub fn opt_nested_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::test::TestAll_Nested> {
    self.opt_nested_mut_or().into_inner()
  }
  pub fn opt_nested_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::test::TestAll_Nested> {
    self.get_mut(__field_TestAll2__opt_nested{})
  }
  pub fn set_opt_nested(mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll_Nested>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_nested_mut_or());
    self
  }

  pub fn opt_choice(self) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll2> {
    self.opt_choice_or().unwrap_or_default()
  }
  pub fn opt_choice_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::test::TestAll2>> {
    self.get(__field_TestAll2__opt_choice{})
  }
  pub fn opt_choice_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::test::TestAll2> {
    self.opt_choice_mut_or().into_inner()
  }
  pub fn opt_choice_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::test::TestAll2> {
    self.get_mut(__field_TestAll2__opt_choice{})
  }
  pub fn set_opt_choice(mut self, value: impl __r::Set<__r::Opt<__::pz::test::TestAll2>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().opt_choice_mut_or());
    self
  }

  pub fn rep_i32(self) -> __rt::Slice<'proto, __s::primitive::i32> {
    self.get(__field_TestAll2__rep_i32{})
  }
  pub fn rep_i32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.rep_i32().at(idx)
  }
  pub fn rep_i32_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::i32> {
    self.get_mut(__field_TestAll2__rep_i32{})
  }
  pub fn set_rep_i32(mut self, value: impl __r::Set<__r::Rep<__s::primitive::i32>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_i32_mut());
    self
  }

  pub fn rep_i64(self) -> __rt::Slice<'proto, __s::primitive::i64> {
    self.get(__field_TestAll2__rep_i64{})
  }
  pub fn rep_i64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::i64> {
    self.rep_i64().at(idx)
  }
  pub fn rep_i64_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::i64> {
    self.get_mut(__field_TestAll2__rep_i64{})
  }
  pub fn set_rep_i64(mut self, value: impl __r::Set<__r::Rep<__s::primitive::i64>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_i64_mut());
    self
  }

  pub fn rep_u32(self) -> __rt::Slice<'proto, __s::primitive::u32> {
    self.get(__field_TestAll2__rep_u32{})
  }
  pub fn rep_u32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.rep_u32().at(idx)
  }
  pub fn rep_u32_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::u32> {
    self.get_mut(__field_TestAll2__rep_u32{})
  }
  pub fn set_rep_u32(mut self, value: impl __r::Set<__r::Rep<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_u32_mut());
    self
  }

  pub fn rep_u64(self) -> __rt::Slice<'proto, __s::primitive::u64> {
    self.get(__field_TestAll2__rep_u64{})
  }
  pub fn rep_u64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u64> {
    self.rep_u64().at(idx)
  }
  pub fn rep_u64_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::u64> {
    self.get_mut(__field_TestAll2__rep_u64{})
  }
  pub fn set_rep_u64(mut self, value: impl __r::Set<__r::Rep<__s::primitive::u64>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_u64_mut());
    self
  }

  pub fn rep_f32(self) -> __rt::Slice<'proto, __s::primitive::f32> {
    self.get(__field_TestAll2__rep_f32{})
  }
  pub fn rep_f32_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::f32> {
    self.rep_f32().at(idx)
  }
  pub fn rep_f32_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::f32> {
    self.get_mut(__field_TestAll2__rep_f32{})
  }
  pub fn set_rep_f32(mut self, value: impl __r::Set<__r::Rep<__s::primitive::f32>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_f32_mut());
    self
  }

  pub fn rep_f64(self) -> __rt::Slice<'proto, __s::primitive::f64> {
    self.get(__field_TestAll2__rep_f64{})
  }
  pub fn rep_f64_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::f64> {
    self.rep_f64().at(idx)
  }
  pub fn rep_f64_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::f64> {
    self.get_mut(__field_TestAll2__rep_f64{})
  }
  pub fn set_rep_f64(mut self, value: impl __r::Set<__r::Rep<__s::primitive::f64>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_f64_mut());
    self
  }

  pub fn rep_str(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_TestAll2__rep_str{})
  }
  pub fn rep_str_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.rep_str().at(idx)
  }
  pub fn rep_str_mut(mut self) -> __rt::Repeated<'proto, __rt::String> {
    self.get_mut(__field_TestAll2__rep_str{})
  }
  pub fn set_rep_str(mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_str_mut());
    self
  }

  pub fn rep_bool(self) -> __rt::Slice<'proto, __s::primitive::bool> {
    self.get(__field_TestAll2__rep_bool{})
  }
  pub fn rep_bool_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.rep_bool().at(idx)
  }
  pub fn rep_bool_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::bool> {
    self.get_mut(__field_TestAll2__rep_bool{})
  }
  pub fn set_rep_bool(mut self, value: impl __r::Set<__r::Rep<__s::primitive::bool>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_bool_mut());
    self
  }

  pub fn rep_recursive(self) -> __rt::Slice<'proto, __::pz::test::TestAll> {
    self.get(__field_TestAll2__rep_recursive{})
  }
  pub fn rep_recursive_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll> {
    self.rep_recursive().at(idx)
  }
  pub fn rep_recursive_mut(mut self) -> __rt::Repeated<'proto, __::pz::test::TestAll> {
    self.get_mut(__field_TestAll2__rep_recursive{})
  }
  pub fn set_rep_recursive(mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_recursive_mut());
    self
  }

  pub fn rep_nested(self) -> __rt::Slice<'proto, __::pz::test::TestAll_Nested> {
    self.get(__field_TestAll2__rep_nested{})
  }
  pub fn rep_nested_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll_Nested> {
    self.rep_nested().at(idx)
  }
  pub fn rep_nested_mut(mut self) -> __rt::Repeated<'proto, __::pz::test::TestAll_Nested> {
    self.get_mut(__field_TestAll2__rep_nested{})
  }
  pub fn set_rep_nested(mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll_Nested>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_nested_mut());
    self
  }

  pub fn rep_choice(self) -> __rt::Slice<'proto, __::pz::test::TestAll2> {
    self.get(__field_TestAll2__rep_choice{})
  }
  pub fn rep_choice_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::test::TestAll2> {
    self.rep_choice().at(idx)
  }
  pub fn rep_choice_mut(mut self) -> __rt::Repeated<'proto, __::pz::test::TestAll2> {
    self.get_mut(__field_TestAll2__rep_choice{})
  }
  pub fn set_rep_choice(mut self, value: impl __r::Set<__r::Rep<__::pz::test::TestAll2>>) -> __r::Mut<'proto, __::pz::test::TestAll2> {
    value.apply_to(self.as_mut().rep_choice_mut());
    self
  }

}

} // mod test

// The __ module exports the package universe needed for this module, to
// simplify cross-type references.
mod __f { pub use super::*; }
mod __ {
use super::__f;
pub use __f::*;
pub mod pz {
use super::__f;
pub use __f::*;
} // mod pz
} // mod __

