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

use crate as __rt;
use __rt::__z;
use __rt::reflect as __r;
use __z::std as __s;

use __s::default::Default as _;

/// message `pz.Bundle`
pub struct Bundle {
  ptr: __s::ptr::NonNull<__priv_Bundle::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Bundle::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_Bundle {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_Bundle::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::Bundle>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::Bundle>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 0],
    pub(in super) types: __z::AVec<<__::pz::Type as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) packages: __z::AVec<<__rt::String as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) foreign_types: __z::AVec<<__::pz::Bundle_ForeignType as __z::Type>::__Storage<__z::Seal>>,
  }
}

impl __::pz::Bundle {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_Bundle::Storage {
      __hasbits: [0; 0],
      types: __z::AVec::new(),
      packages: __z::AVec::new(),
      foreign_types: __z::AVec::new(),
    }} as *const __priv_Bundle::Storage as *mut __priv_Bundle::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::Bundle`].
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

  /// Deserializes a new [`__::pz::Bundle`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::Bundle`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Bundle`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Bundle`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Bundle>,
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
    S: __r::Selector<__::pz::Bundle>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Bundle`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn types(&self) -> __rt::Slice<'_, __::pz::Type> {
    self.get(__field_Bundle__types{})
  }
  pub fn types_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::Type> {
    self.types().at(idx)
  }
  pub fn types_mut(&mut self) -> __rt::Repeated<'_, __::pz::Type> {
    self.get_mut(__field_Bundle__types{})
  }
  pub fn set_types(&mut self, value: impl __r::Set<__r::Rep<__::pz::Type>>) -> __r::Mut<'_, __::pz::Bundle> {
    value.apply_to(self.as_mut().types_mut());
    self.as_mut()
  }

  pub fn packages(&self) -> __rt::Slice<'_, __rt::String> {
    self.get(__field_Bundle__packages{})
  }
  pub fn packages_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __rt::String> {
    self.packages().at(idx)
  }
  pub fn packages_mut(&mut self) -> __rt::Repeated<'_, __rt::String> {
    self.get_mut(__field_Bundle__packages{})
  }
  pub fn set_packages(&mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'_, __::pz::Bundle> {
    value.apply_to(self.as_mut().packages_mut());
    self.as_mut()
  }

  pub fn foreign_types(&self) -> __rt::Slice<'_, __::pz::Bundle_ForeignType> {
    self.get(__field_Bundle__foreign_types{})
  }
  pub fn foreign_types_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }
  pub fn foreign_types_mut(&mut self) -> __rt::Repeated<'_, __::pz::Bundle_ForeignType> {
    self.get_mut(__field_Bundle__foreign_types{})
  }
  pub fn set_foreign_types(&mut self, value: impl __r::Set<__r::Rep<__::pz::Bundle_ForeignType>>) -> __r::Mut<'_, __::pz::Bundle> {
    value.apply_to(self.as_mut().foreign_types_mut());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Bundle::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_Bundle::Storage>().as_mut().__hasbits = [0; 0];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::Bundle>, src: __rt::reflect::Ref<__::pz::Bundle>) {
    __r::Set::<<__::pz::Bundle as __r::Field<__field_Bundle__types>>::Type>::apply_to(src.get(__field_Bundle__types{}), dst.as_mut().get_mut(__field_Bundle__types{}));
    __r::Set::<<__::pz::Bundle as __r::Field<__field_Bundle__packages>>::Type>::apply_to(src.get(__field_Bundle__packages{}), dst.as_mut().get_mut(__field_Bundle__packages{}));
    __r::Set::<<__::pz::Bundle as __r::Field<__field_Bundle__foreign_types>>::Type>::apply_to(src.get(__field_Bundle__foreign_types{}), dst.as_mut().get_mut(__field_Bundle__foreign_types{}));
  }
}

impl __z::Message for __::pz::Bundle {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{3 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::Bundle::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::Bundle_ForeignType::__tdp_info,
            __::pz::Type::__tdp_info,
          ];
          DESCS.as_ptr()
        },

        num_hasbit_words: 0,
        kind: __z::tdp::DescKind::Message,
      },

      fields: [
        __z::tdp::FieldStorage {
          number: 1,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Bundle::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().types as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 1,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Bundle::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().packages as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 3,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Bundle::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().foreign_types as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
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

impl __z::Type for __::pz::Bundle {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::Bundle::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::Bundle::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_Bundle::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_Bundle::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_Bundle::Mut {
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

impl __r::Views for __::pz::Bundle {
  type Ref<'a> = __priv_Bundle::Ref<'a>;
  type Mut<'a> = __priv_Bundle::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_Bundle::Ref<'a> {
  type Target = __::pz::Bundle;
  fn as_ref(&self) -> __priv_Bundle::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_Bundle::Mut<'a> {
  type Target = __::pz::Bundle;
  fn as_ref(&self) -> __priv_Bundle::Ref { self.r }
  fn into_ref(self) -> __priv_Bundle::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_Bundle::Mut {
    __priv_Bundle::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::Bundle {
  const DEFAULT: &'static Self = __::pz::Bundle::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_Bundle::Ref<'a> {
  type Message = __::pz::Bundle;
}
impl<'a> __r::MessageMut<'a> for __priv_Bundle::Mut<'a> {
  type Message = __::pz::Bundle;
}

impl __s::default::Default for __::pz::Bundle {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::Bundle>> __s::convert::From<T> for __::pz::Bundle {
  fn from(value: T) -> __::pz::Bundle {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::Bundle> for &__::pz::Bundle {
  fn apply_to(self, mut m: __r::Mut<__::pz::Bundle>) {
    __::pz::Bundle::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Bundle>> for &__::pz::Bundle {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Bundle>>) {
    __::pz::Bundle::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::Bundle> for __rt::reflect::Ref<'_, __::pz::Bundle> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Bundle>) {
    __::pz::Bundle::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::Bundle>> for __rt::reflect::Ref<'_, __::pz::Bundle> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Bundle>>) {
    __::pz::Bundle::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::Bundle> for &__rt::reflect::Mut<'_, __::pz::Bundle> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Bundle>) {
    __::pz::Bundle::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Bundle>> for &__rt::reflect::Mut<'_, __::pz::Bundle> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Bundle>>) {
    __::pz::Bundle::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_Bundle::Ref<'_> {
  fn default() -> Self {
    __::pz::Bundle::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::Bundle {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_Bundle::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Bundle ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Bundle::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::Bundle {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_Bundle__types = __rt::field!(types);
impl __r::Field<__field_Bundle__types> for __::pz::Bundle {
  type Type = __r::Rep<__::pz::Type>;
  type Name = __field_Bundle__types;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "types";
}

type __field_Bundle__packages = __rt::field!(packages);
impl __r::Field<__field_Bundle__packages> for __::pz::Bundle {
  type Type = __r::Rep<__rt::String>;
  type Name = __field_Bundle__packages;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "packages";
}

type __field_Bundle__foreign_types = __rt::field!(foreign_types);
impl __r::Field<__field_Bundle__foreign_types> for __::pz::Bundle {
  type Type = __r::Rep<__::pz::Bundle_ForeignType>;
  type Name = __field_Bundle__foreign_types;
  const NUMBER: __s::primitive::i32 = 3;
  const INDEX: __s::primitive::usize = 2;
  const NAME: &'static __s::primitive::str = "foreign_types";
}

impl<'proto> __priv_Bundle::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_Bundle::Ref { *self }

  /// Serializes this [`__::pz::Bundle`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Bundle`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Bundle>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn types(self) -> __rt::Slice<'proto, __::pz::Type> {
    self.get(__field_Bundle__types{})
  }
  pub fn types_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::Type> {
    self.types().at(idx)
  }

  pub fn packages(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_Bundle__packages{})
  }
  pub fn packages_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.packages().at(idx)
  }

  pub fn foreign_types(self) -> __rt::Slice<'proto, __::pz::Bundle_ForeignType> {
    self.get(__field_Bundle__foreign_types{})
  }
  pub fn foreign_types_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    for value in self.types() {
      if count != 0 { debug.comma(false)?; }
      debug.field("types")?;
      value.__debug(debug)?;
      count += 1;
    }
    if !self.packages().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("packages")?;
      debug.iter(self.packages())?;
      count += 1;
    }
    for value in self.foreign_types() {
      if count != 0 { debug.comma(false)?; }
      debug.field("foreign_types")?;
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

impl<'proto> __priv_Bundle::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_Bundle::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_Bundle::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_Bundle::Mut {
    __priv_Bundle::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::Bundle`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Bundle`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Bundle`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Bundle>,
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
    S: __r::Selector<__::pz::Bundle>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Bundle`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn types(self) -> __rt::Slice<'proto, __::pz::Type> {
    self.get(__field_Bundle__types{})
  }
  pub fn types_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::Type> {
    self.types().at(idx)
  }
  pub fn types_mut(mut self) -> __rt::Repeated<'proto, __::pz::Type> {
    self.get_mut(__field_Bundle__types{})
  }
  pub fn set_types(mut self, value: impl __r::Set<__r::Rep<__::pz::Type>>) -> __r::Mut<'proto, __::pz::Bundle> {
    value.apply_to(self.as_mut().types_mut());
    self
  }

  pub fn packages(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_Bundle__packages{})
  }
  pub fn packages_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.packages().at(idx)
  }
  pub fn packages_mut(mut self) -> __rt::Repeated<'proto, __rt::String> {
    self.get_mut(__field_Bundle__packages{})
  }
  pub fn set_packages(mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'proto, __::pz::Bundle> {
    value.apply_to(self.as_mut().packages_mut());
    self
  }

  pub fn foreign_types(self) -> __rt::Slice<'proto, __::pz::Bundle_ForeignType> {
    self.get(__field_Bundle__foreign_types{})
  }
  pub fn foreign_types_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }
  pub fn foreign_types_mut(mut self) -> __rt::Repeated<'proto, __::pz::Bundle_ForeignType> {
    self.get_mut(__field_Bundle__foreign_types{})
  }
  pub fn set_foreign_types(mut self, value: impl __r::Set<__r::Rep<__::pz::Bundle_ForeignType>>) -> __r::Mut<'proto, __::pz::Bundle> {
    value.apply_to(self.as_mut().foreign_types_mut());
    self
  }

}

/// message `pz.Bundle.ForeignType`
pub struct Bundle_ForeignType {
  ptr: __s::ptr::NonNull<__priv_Bundle_ForeignType::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Bundle_ForeignType::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_Bundle_ForeignType {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_Bundle_ForeignType::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::Bundle_ForeignType>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::Bundle_ForeignType>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) package: <__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>,
  }
}

impl __::pz::Bundle_ForeignType {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_Bundle_ForeignType::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      package: 0,
    }} as *const __priv_Bundle_ForeignType::Storage as *mut __priv_Bundle_ForeignType::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::Bundle_ForeignType`].
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

  /// Deserializes a new [`__::pz::Bundle_ForeignType`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::Bundle_ForeignType`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Bundle_ForeignType`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Bundle_ForeignType`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Bundle_ForeignType>,
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
    S: __r::Selector<__::pz::Bundle_ForeignType>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Bundle_ForeignType`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn name(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_Bundle_ForeignType__name{})
  }
  pub fn name_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_Bundle_ForeignType__name{})
  }
  pub fn set_name(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::Bundle_ForeignType> {
    value.apply_to(self.as_mut().name_mut_or());
    self.as_mut()
  }

  pub fn package(&self) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u32>> {
    self.get(__field_Bundle_ForeignType__package{})
  }
  pub fn package_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u32> {
    self.package_mut_or().into_inner()
  }
  pub fn package_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u32> {
    self.get_mut(__field_Bundle_ForeignType__package{})
  }
  pub fn set_package(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::Bundle_ForeignType> {
    value.apply_to(self.as_mut().package_mut_or());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Bundle_ForeignType::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_Bundle_ForeignType::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::Bundle_ForeignType>, src: __rt::reflect::Ref<__::pz::Bundle_ForeignType>) {
    __r::Set::<<__::pz::Bundle_ForeignType as __r::Field<__field_Bundle_ForeignType__name>>::Type>::apply_to(src.get(__field_Bundle_ForeignType__name{}), dst.as_mut().get_mut(__field_Bundle_ForeignType__name{}));
    __r::Set::<<__::pz::Bundle_ForeignType as __r::Field<__field_Bundle_ForeignType__package>>::Type>::apply_to(src.get(__field_Bundle_ForeignType__package{}), dst.as_mut().get_mut(__field_Bundle_ForeignType__package{}));
  }
}

impl __z::Message for __::pz::Bundle_ForeignType {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{2 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::Bundle_ForeignType::__LAYOUT.size();
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
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Bundle_ForeignType::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Bundle_ForeignType::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().package as *const _ as *const u8;
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

impl __z::Type for __::pz::Bundle_ForeignType {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::Bundle_ForeignType::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::Bundle_ForeignType::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_Bundle_ForeignType::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_Bundle_ForeignType::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_Bundle_ForeignType::Mut {
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

impl __r::Views for __::pz::Bundle_ForeignType {
  type Ref<'a> = __priv_Bundle_ForeignType::Ref<'a>;
  type Mut<'a> = __priv_Bundle_ForeignType::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_Bundle_ForeignType::Ref<'a> {
  type Target = __::pz::Bundle_ForeignType;
  fn as_ref(&self) -> __priv_Bundle_ForeignType::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_Bundle_ForeignType::Mut<'a> {
  type Target = __::pz::Bundle_ForeignType;
  fn as_ref(&self) -> __priv_Bundle_ForeignType::Ref { self.r }
  fn into_ref(self) -> __priv_Bundle_ForeignType::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_Bundle_ForeignType::Mut {
    __priv_Bundle_ForeignType::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::Bundle_ForeignType {
  const DEFAULT: &'static Self = __::pz::Bundle_ForeignType::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_Bundle_ForeignType::Ref<'a> {
  type Message = __::pz::Bundle_ForeignType;
}
impl<'a> __r::MessageMut<'a> for __priv_Bundle_ForeignType::Mut<'a> {
  type Message = __::pz::Bundle_ForeignType;
}

impl __s::default::Default for __::pz::Bundle_ForeignType {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::Bundle_ForeignType>> __s::convert::From<T> for __::pz::Bundle_ForeignType {
  fn from(value: T) -> __::pz::Bundle_ForeignType {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::Bundle_ForeignType> for &__::pz::Bundle_ForeignType {
  fn apply_to(self, mut m: __r::Mut<__::pz::Bundle_ForeignType>) {
    __::pz::Bundle_ForeignType::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Bundle_ForeignType>> for &__::pz::Bundle_ForeignType {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Bundle_ForeignType>>) {
    __::pz::Bundle_ForeignType::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::Bundle_ForeignType> for __rt::reflect::Ref<'_, __::pz::Bundle_ForeignType> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Bundle_ForeignType>) {
    __::pz::Bundle_ForeignType::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::Bundle_ForeignType>> for __rt::reflect::Ref<'_, __::pz::Bundle_ForeignType> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Bundle_ForeignType>>) {
    __::pz::Bundle_ForeignType::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::Bundle_ForeignType> for &__rt::reflect::Mut<'_, __::pz::Bundle_ForeignType> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Bundle_ForeignType>) {
    __::pz::Bundle_ForeignType::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Bundle_ForeignType>> for &__rt::reflect::Mut<'_, __::pz::Bundle_ForeignType> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Bundle_ForeignType>>) {
    __::pz::Bundle_ForeignType::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_Bundle_ForeignType::Ref<'_> {
  fn default() -> Self {
    __::pz::Bundle_ForeignType::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::Bundle_ForeignType {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_Bundle_ForeignType::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Bundle.ForeignType ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Bundle_ForeignType::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::Bundle_ForeignType {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_Bundle_ForeignType__name = __rt::field!(name);
impl __r::Field<__field_Bundle_ForeignType__name> for __::pz::Bundle_ForeignType {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_Bundle_ForeignType__name;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "name";
}

type __field_Bundle_ForeignType__package = __rt::field!(package);
impl __r::Field<__field_Bundle_ForeignType__package> for __::pz::Bundle_ForeignType {
  type Type = __r::Opt<__s::primitive::u32>;
  type Name = __field_Bundle_ForeignType__package;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "package";
}

impl<'proto> __priv_Bundle_ForeignType::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_Bundle_ForeignType::Ref { *self }

  /// Serializes this [`__::pz::Bundle_ForeignType`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Bundle_ForeignType`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Bundle_ForeignType>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Bundle_ForeignType__name{})
  }

  pub fn package(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Bundle_ForeignType__package{})
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.package_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("package")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_Bundle_ForeignType::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_Bundle_ForeignType::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_Bundle_ForeignType::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_Bundle_ForeignType::Mut {
    __priv_Bundle_ForeignType::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::Bundle_ForeignType`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Bundle_ForeignType`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Bundle_ForeignType`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Bundle_ForeignType>,
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
    S: __r::Selector<__::pz::Bundle_ForeignType>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Bundle_ForeignType`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Bundle_ForeignType__name{})
  }
  pub fn name_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_Bundle_ForeignType__name{})
  }
  pub fn set_name(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::Bundle_ForeignType> {
    value.apply_to(self.as_mut().name_mut_or());
    self
  }

  pub fn package(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Bundle_ForeignType__package{})
  }
  pub fn package_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u32> {
    self.package_mut_or().into_inner()
  }
  pub fn package_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u32> {
    self.get_mut(__field_Bundle_ForeignType__package{})
  }
  pub fn set_package(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::Bundle_ForeignType> {
    value.apply_to(self.as_mut().package_mut_or());
    self
  }

}

/// message `pz.Type`
pub struct Type {
  ptr: __s::ptr::NonNull<__priv_Type::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Type::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_Type {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_Type::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::Type>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::Type>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) package: <__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) kind: <__::pz::Type_Kind as __z::Type>::__Storage<__z::Seal>,
    pub(in super) declared_in: <__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) fields: __z::AVec<<__::pz::Field as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) nesteds: __z::AVec<<__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) attrs: <__::pz::Type_Attrs as __z::Type>::__Storage<__z::Seal>,
    pub(in super) span: <__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>,
  }
}

impl __::pz::Type {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_Type::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      package: 0,
      kind: __::pz::Type_Kind::new(),
      declared_in: 0,
      fields: __z::AVec::new(),
      nesteds: __z::AVec::new(),
      attrs: __s::option::Option::None,
      span: 0,
    }} as *const __priv_Type::Storage as *mut __priv_Type::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::Type`].
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

  /// Deserializes a new [`__::pz::Type`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::Type`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Type`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Type`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Type>,
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
    S: __r::Selector<__::pz::Type>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Type`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn name(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_Type__name{})
  }
  pub fn name_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_Type__name{})
  }
  pub fn set_name(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::Type> {
    value.apply_to(self.as_mut().name_mut_or());
    self.as_mut()
  }

  pub fn package(&self) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u32>> {
    self.get(__field_Type__package{})
  }
  pub fn package_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u32> {
    self.package_mut_or().into_inner()
  }
  pub fn package_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u32> {
    self.get_mut(__field_Type__package{})
  }
  pub fn set_package(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::Type> {
    value.apply_to(self.as_mut().package_mut_or());
    self.as_mut()
  }

  pub fn kind(&self) -> __rt::reflect::Ref<'_, __::pz::Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::Type_Kind>> {
    self.get(__field_Type__kind{})
  }
  pub fn kind_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::Type_Kind> {
    self.kind_mut_or().into_inner()
  }
  pub fn kind_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::Type_Kind> {
    self.get_mut(__field_Type__kind{})
  }
  pub fn set_kind(&mut self, value: impl __r::Set<__r::Opt<__::pz::Type_Kind>>) -> __r::Mut<'_, __::pz::Type> {
    value.apply_to(self.as_mut().kind_mut_or());
    self.as_mut()
  }

  pub fn declared_in(&self) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u32>> {
    self.get(__field_Type__declared_in{})
  }
  pub fn declared_in_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u32> {
    self.declared_in_mut_or().into_inner()
  }
  pub fn declared_in_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u32> {
    self.get_mut(__field_Type__declared_in{})
  }
  pub fn set_declared_in(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::Type> {
    value.apply_to(self.as_mut().declared_in_mut_or());
    self.as_mut()
  }

  pub fn fields(&self) -> __rt::Slice<'_, __::pz::Field> {
    self.get(__field_Type__fields{})
  }
  pub fn fields_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::Field> {
    self.fields().at(idx)
  }
  pub fn fields_mut(&mut self) -> __rt::Repeated<'_, __::pz::Field> {
    self.get_mut(__field_Type__fields{})
  }
  pub fn set_fields(&mut self, value: impl __r::Set<__r::Rep<__::pz::Field>>) -> __r::Mut<'_, __::pz::Type> {
    value.apply_to(self.as_mut().fields_mut());
    self.as_mut()
  }

  pub fn nesteds(&self) -> __rt::Slice<'_, __s::primitive::u32> {
    self.get(__field_Type__nesteds{})
  }
  pub fn nesteds_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.nesteds().at(idx)
  }
  pub fn nesteds_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::u32> {
    self.get_mut(__field_Type__nesteds{})
  }
  pub fn set_nesteds(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::Type> {
    value.apply_to(self.as_mut().nesteds_mut());
    self.as_mut()
  }

  pub fn attrs(&self) -> __rt::reflect::Ref<'_, __::pz::Type_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::Type_Attrs>> {
    self.get(__field_Type__attrs{})
  }
  pub fn attrs_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::Type_Attrs> {
    self.attrs_mut_or().into_inner()
  }
  pub fn attrs_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::Type_Attrs> {
    self.get_mut(__field_Type__attrs{})
  }
  pub fn set_attrs(&mut self, value: impl __r::Set<__r::Opt<__::pz::Type_Attrs>>) -> __r::Mut<'_, __::pz::Type> {
    value.apply_to(self.as_mut().attrs_mut_or());
    self.as_mut()
  }

  pub fn span(&self) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u32>> {
    self.get(__field_Type__span{})
  }
  pub fn span_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u32> {
    self.span_mut_or().into_inner()
  }
  pub fn span_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u32> {
    self.get_mut(__field_Type__span{})
  }
  pub fn set_span(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::Type> {
    value.apply_to(self.as_mut().span_mut_or());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Type::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_Type::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::Type>, src: __rt::reflect::Ref<__::pz::Type>) {
    __r::Set::<<__::pz::Type as __r::Field<__field_Type__name>>::Type>::apply_to(src.get(__field_Type__name{}), dst.as_mut().get_mut(__field_Type__name{}));
    __r::Set::<<__::pz::Type as __r::Field<__field_Type__package>>::Type>::apply_to(src.get(__field_Type__package{}), dst.as_mut().get_mut(__field_Type__package{}));
    __r::Set::<<__::pz::Type as __r::Field<__field_Type__kind>>::Type>::apply_to(src.get(__field_Type__kind{}), dst.as_mut().get_mut(__field_Type__kind{}));
    __r::Set::<<__::pz::Type as __r::Field<__field_Type__declared_in>>::Type>::apply_to(src.get(__field_Type__declared_in{}), dst.as_mut().get_mut(__field_Type__declared_in{}));
    __r::Set::<<__::pz::Type as __r::Field<__field_Type__fields>>::Type>::apply_to(src.get(__field_Type__fields{}), dst.as_mut().get_mut(__field_Type__fields{}));
    __r::Set::<<__::pz::Type as __r::Field<__field_Type__nesteds>>::Type>::apply_to(src.get(__field_Type__nesteds{}), dst.as_mut().get_mut(__field_Type__nesteds{}));
    __r::Set::<<__::pz::Type as __r::Field<__field_Type__attrs>>::Type>::apply_to(src.get(__field_Type__attrs{}), dst.as_mut().get_mut(__field_Type__attrs{}));
    __r::Set::<<__::pz::Type as __r::Field<__field_Type__span>>::Type>::apply_to(src.get(__field_Type__span{}), dst.as_mut().get_mut(__field_Type__span{}));
  }
}

impl __z::Message for __::pz::Type {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{8 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::Type::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::Field::__tdp_info,
            __::pz::Type_Attrs::__tdp_info,
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
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().package as *const _ as *const u8;
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
            let msg = __::pz::Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().kind as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
        },
        __z::tdp::FieldStorage {
          number: 4,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().declared_in as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 3,
        },
        __z::tdp::FieldStorage {
          number: 10,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().fields as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 4,
        },
        __z::tdp::FieldStorage {
          number: 11,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().nesteds as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 4,
        },
        __z::tdp::FieldStorage {
          number: 12,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().attrs as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 1,
          hasbit: 4,
        },
        __z::tdp::FieldStorage {
          number: 20,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().span as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 5,
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

impl __z::Type for __::pz::Type {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::Type::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::Type::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_Type::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_Type::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_Type::Mut {
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

impl __r::Views for __::pz::Type {
  type Ref<'a> = __priv_Type::Ref<'a>;
  type Mut<'a> = __priv_Type::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_Type::Ref<'a> {
  type Target = __::pz::Type;
  fn as_ref(&self) -> __priv_Type::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_Type::Mut<'a> {
  type Target = __::pz::Type;
  fn as_ref(&self) -> __priv_Type::Ref { self.r }
  fn into_ref(self) -> __priv_Type::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_Type::Mut {
    __priv_Type::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::Type {
  const DEFAULT: &'static Self = __::pz::Type::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_Type::Ref<'a> {
  type Message = __::pz::Type;
}
impl<'a> __r::MessageMut<'a> for __priv_Type::Mut<'a> {
  type Message = __::pz::Type;
}

impl __s::default::Default for __::pz::Type {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::Type>> __s::convert::From<T> for __::pz::Type {
  fn from(value: T) -> __::pz::Type {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::Type> for &__::pz::Type {
  fn apply_to(self, mut m: __r::Mut<__::pz::Type>) {
    __::pz::Type::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Type>> for &__::pz::Type {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Type>>) {
    __::pz::Type::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::Type> for __rt::reflect::Ref<'_, __::pz::Type> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Type>) {
    __::pz::Type::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::Type>> for __rt::reflect::Ref<'_, __::pz::Type> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Type>>) {
    __::pz::Type::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::Type> for &__rt::reflect::Mut<'_, __::pz::Type> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Type>) {
    __::pz::Type::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Type>> for &__rt::reflect::Mut<'_, __::pz::Type> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Type>>) {
    __::pz::Type::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_Type::Ref<'_> {
  fn default() -> Self {
    __::pz::Type::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::Type {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_Type::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Type ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Type::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::Type {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_Type__name = __rt::field!(name);
impl __r::Field<__field_Type__name> for __::pz::Type {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_Type__name;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "name";
}

type __field_Type__package = __rt::field!(package);
impl __r::Field<__field_Type__package> for __::pz::Type {
  type Type = __r::Opt<__s::primitive::u32>;
  type Name = __field_Type__package;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "package";
}

type __field_Type__kind = __rt::field!(kind);
impl __r::Field<__field_Type__kind> for __::pz::Type {
  type Type = __r::Opt<__::pz::Type_Kind>;
  type Name = __field_Type__kind;
  const NUMBER: __s::primitive::i32 = 3;
  const INDEX: __s::primitive::usize = 2;
  const NAME: &'static __s::primitive::str = "kind";
}

type __field_Type__declared_in = __rt::field!(declared_in);
impl __r::Field<__field_Type__declared_in> for __::pz::Type {
  type Type = __r::Opt<__s::primitive::u32>;
  type Name = __field_Type__declared_in;
  const NUMBER: __s::primitive::i32 = 4;
  const INDEX: __s::primitive::usize = 3;
  const NAME: &'static __s::primitive::str = "declared_in";
}

type __field_Type__fields = __rt::field!(fields);
impl __r::Field<__field_Type__fields> for __::pz::Type {
  type Type = __r::Rep<__::pz::Field>;
  type Name = __field_Type__fields;
  const NUMBER: __s::primitive::i32 = 10;
  const INDEX: __s::primitive::usize = 4;
  const NAME: &'static __s::primitive::str = "fields";
}

type __field_Type__nesteds = __rt::field!(nesteds);
impl __r::Field<__field_Type__nesteds> for __::pz::Type {
  type Type = __r::Rep<__s::primitive::u32>;
  type Name = __field_Type__nesteds;
  const NUMBER: __s::primitive::i32 = 11;
  const INDEX: __s::primitive::usize = 5;
  const NAME: &'static __s::primitive::str = "nesteds";
}

type __field_Type__attrs = __rt::field!(attrs);
impl __r::Field<__field_Type__attrs> for __::pz::Type {
  type Type = __r::Opt<__::pz::Type_Attrs>;
  type Name = __field_Type__attrs;
  const NUMBER: __s::primitive::i32 = 12;
  const INDEX: __s::primitive::usize = 6;
  const NAME: &'static __s::primitive::str = "attrs";
}

type __field_Type__span = __rt::field!(span);
impl __r::Field<__field_Type__span> for __::pz::Type {
  type Type = __r::Opt<__s::primitive::u32>;
  type Name = __field_Type__span;
  const NUMBER: __s::primitive::i32 = 20;
  const INDEX: __s::primitive::usize = 7;
  const NAME: &'static __s::primitive::str = "span";
}

impl<'proto> __priv_Type::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_Type::Ref { *self }

  /// Serializes this [`__::pz::Type`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Type`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Type>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Type__name{})
  }

  pub fn package(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Type__package{})
  }

  pub fn kind(self) -> __rt::reflect::Ref<'proto, __::pz::Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::Type_Kind>> {
    self.get(__field_Type__kind{})
  }

  pub fn declared_in(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Type__declared_in{})
  }

  pub fn fields(self) -> __rt::Slice<'proto, __::pz::Field> {
    self.get(__field_Type__fields{})
  }
  pub fn fields_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::Field> {
    self.fields().at(idx)
  }

  pub fn nesteds(self) -> __rt::Slice<'proto, __s::primitive::u32> {
    self.get(__field_Type__nesteds{})
  }
  pub fn nesteds_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.nesteds().at(idx)
  }

  pub fn attrs(self) -> __rt::reflect::Ref<'proto, __::pz::Type_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::Type_Attrs>> {
    self.get(__field_Type__attrs{})
  }

  pub fn span(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Type__span{})
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.package_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("package")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.kind_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("kind")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.declared_in_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("declared_in")?;
      debug.write_debug(value);
      count += 1;
    }
    for value in self.fields() {
      if count != 0 { debug.comma(false)?; }
      debug.field("fields")?;
      value.__debug(debug)?;
      count += 1;
    }
    if !self.nesteds().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("nesteds")?;
      debug.iter(self.nesteds())?;
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.attrs_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("attrs")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.span_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("span")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_Type::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_Type::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_Type::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_Type::Mut {
    __priv_Type::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::Type`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Type`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Type`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Type>,
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
    S: __r::Selector<__::pz::Type>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Type`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Type__name{})
  }
  pub fn name_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_Type__name{})
  }
  pub fn set_name(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::Type> {
    value.apply_to(self.as_mut().name_mut_or());
    self
  }

  pub fn package(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Type__package{})
  }
  pub fn package_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u32> {
    self.package_mut_or().into_inner()
  }
  pub fn package_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u32> {
    self.get_mut(__field_Type__package{})
  }
  pub fn set_package(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::Type> {
    value.apply_to(self.as_mut().package_mut_or());
    self
  }

  pub fn kind(self) -> __rt::reflect::Ref<'proto, __::pz::Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::Type_Kind>> {
    self.get(__field_Type__kind{})
  }
  pub fn kind_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::Type_Kind> {
    self.kind_mut_or().into_inner()
  }
  pub fn kind_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::Type_Kind> {
    self.get_mut(__field_Type__kind{})
  }
  pub fn set_kind(mut self, value: impl __r::Set<__r::Opt<__::pz::Type_Kind>>) -> __r::Mut<'proto, __::pz::Type> {
    value.apply_to(self.as_mut().kind_mut_or());
    self
  }

  pub fn declared_in(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Type__declared_in{})
  }
  pub fn declared_in_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u32> {
    self.declared_in_mut_or().into_inner()
  }
  pub fn declared_in_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u32> {
    self.get_mut(__field_Type__declared_in{})
  }
  pub fn set_declared_in(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::Type> {
    value.apply_to(self.as_mut().declared_in_mut_or());
    self
  }

  pub fn fields(self) -> __rt::Slice<'proto, __::pz::Field> {
    self.get(__field_Type__fields{})
  }
  pub fn fields_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::Field> {
    self.fields().at(idx)
  }
  pub fn fields_mut(mut self) -> __rt::Repeated<'proto, __::pz::Field> {
    self.get_mut(__field_Type__fields{})
  }
  pub fn set_fields(mut self, value: impl __r::Set<__r::Rep<__::pz::Field>>) -> __r::Mut<'proto, __::pz::Type> {
    value.apply_to(self.as_mut().fields_mut());
    self
  }

  pub fn nesteds(self) -> __rt::Slice<'proto, __s::primitive::u32> {
    self.get(__field_Type__nesteds{})
  }
  pub fn nesteds_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.nesteds().at(idx)
  }
  pub fn nesteds_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::u32> {
    self.get_mut(__field_Type__nesteds{})
  }
  pub fn set_nesteds(mut self, value: impl __r::Set<__r::Rep<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::Type> {
    value.apply_to(self.as_mut().nesteds_mut());
    self
  }

  pub fn attrs(self) -> __rt::reflect::Ref<'proto, __::pz::Type_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::Type_Attrs>> {
    self.get(__field_Type__attrs{})
  }
  pub fn attrs_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::Type_Attrs> {
    self.attrs_mut_or().into_inner()
  }
  pub fn attrs_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::Type_Attrs> {
    self.get_mut(__field_Type__attrs{})
  }
  pub fn set_attrs(mut self, value: impl __r::Set<__r::Opt<__::pz::Type_Attrs>>) -> __r::Mut<'proto, __::pz::Type> {
    value.apply_to(self.as_mut().attrs_mut_or());
    self
  }

  pub fn span(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Type__span{})
  }
  pub fn span_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u32> {
    self.span_mut_or().into_inner()
  }
  pub fn span_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u32> {
    self.get_mut(__field_Type__span{})
  }
  pub fn set_span(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::Type> {
    value.apply_to(self.as_mut().span_mut_or());
    self
  }

}

/// enum `pz.Type.Kind`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Type_Kind(pub __s::primitive::i32);

impl __::pz::Type_Kind {
  pub const Message: Self = Self(0);
  pub const Struct: Self = Self(1);
  pub const Choice: Self = Self(2);
  pub const Enum: Self = Self(3);

  pub const fn new() -> Self {
    Self(0)
  }
}

impl __s::default::Default for __::pz::Type_Kind {
  fn default() -> Self {
    Self(0)
  }
}

impl __z::Type for __::pz::Type_Kind {
  type __Storage<S: __z::Sealed> = Self;

  unsafe fn __ref<'a, S: __z::Sealed>(_: S, ptr: __s::ptr::NonNull<Self>) -> __rt::Ref<'a, Self> {
    ptr.read()
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    _: S,
    mut ptr: __s::ptr::NonNull<Self>,
    _: __z::RawArena,
  ) -> __rt::Mut<'a, Self> {
    __rt::ScalarMut::__wrap(ptr.as_mut())
  }
}

impl __r::Views for __::pz::Type_Kind {
  type Ref<'a> = Self;
  type Mut<'a> = __rt::ScalarMut<'a, Self>;
}

impl __r::RefView<'_> for __::pz::Type_Kind {
  type Target = Self;

  fn as_ref(&self) -> Self {
    *self
  }
}

impl __r::Set<__::pz::Type_Kind> for __::pz::Type_Kind {
  fn apply_to(self, mut m: __r::Mut<__::pz::Type_Kind>) {
    m.set(self)
  }
}

impl __r::Set<__r::Opt<__::pz::Type_Kind>> for __::pz::Type_Kind {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Type_Kind>>) {
    m.into_inner().set(self)
  }
}

impl __s::fmt::Debug for __::pz::Type_Kind {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    match *self {
      Self::Message => __s::write!(fmt, "Message"),
      Self::Struct => __s::write!(fmt, "Struct"),
      Self::Choice => __s::write!(fmt, "Choice"),
      Self::Enum => __s::write!(fmt, "Enum"),
      Self(n) => __s::write!(fmt, "pz.Type.Kind({n})"),
    }
  }
}

/// message `pz.Type.Attrs`
pub struct Type_Attrs {
  ptr: __s::ptr::NonNull<__priv_Type_Attrs::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Type_Attrs::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_Type_Attrs {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_Type_Attrs::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::Type_Attrs>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::Type_Attrs>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) deprecated: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) docs: __z::AVec<<__rt::String as __z::Type>::__Storage<__z::Seal>>,
  }
}

impl __::pz::Type_Attrs {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_Type_Attrs::Storage {
      __hasbits: [0; 1],
      deprecated: __z::RawStr::new(),
      docs: __z::AVec::new(),
    }} as *const __priv_Type_Attrs::Storage as *mut __priv_Type_Attrs::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::Type_Attrs`].
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

  /// Deserializes a new [`__::pz::Type_Attrs`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::Type_Attrs`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Type_Attrs`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Type_Attrs`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Type_Attrs>,
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
    S: __r::Selector<__::pz::Type_Attrs>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Type_Attrs`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn deprecated(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_Type_Attrs__deprecated{})
  }
  pub fn deprecated_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.deprecated_mut_or().into_inner()
  }
  pub fn deprecated_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_Type_Attrs__deprecated{})
  }
  pub fn set_deprecated(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::Type_Attrs> {
    value.apply_to(self.as_mut().deprecated_mut_or());
    self.as_mut()
  }

  pub fn docs(&self) -> __rt::Slice<'_, __rt::String> {
    self.get(__field_Type_Attrs__docs{})
  }
  pub fn docs_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __rt::String> {
    self.docs().at(idx)
  }
  pub fn docs_mut(&mut self) -> __rt::Repeated<'_, __rt::String> {
    self.get_mut(__field_Type_Attrs__docs{})
  }
  pub fn set_docs(&mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'_, __::pz::Type_Attrs> {
    value.apply_to(self.as_mut().docs_mut());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Type_Attrs::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_Type_Attrs::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::Type_Attrs>, src: __rt::reflect::Ref<__::pz::Type_Attrs>) {
    __r::Set::<<__::pz::Type_Attrs as __r::Field<__field_Type_Attrs__deprecated>>::Type>::apply_to(src.get(__field_Type_Attrs__deprecated{}), dst.as_mut().get_mut(__field_Type_Attrs__deprecated{}));
    __r::Set::<<__::pz::Type_Attrs as __r::Field<__field_Type_Attrs__docs>>::Type>::apply_to(src.get(__field_Type_Attrs__docs{}), dst.as_mut().get_mut(__field_Type_Attrs__docs{}));
  }
}

impl __z::Message for __::pz::Type_Attrs {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{2 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::Type_Attrs::__LAYOUT.size();
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
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Type_Attrs::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().deprecated as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 100,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Type_Attrs::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().docs as *const _ as *const u8;
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

impl __z::Type for __::pz::Type_Attrs {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::Type_Attrs::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::Type_Attrs::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_Type_Attrs::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_Type_Attrs::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_Type_Attrs::Mut {
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

impl __r::Views for __::pz::Type_Attrs {
  type Ref<'a> = __priv_Type_Attrs::Ref<'a>;
  type Mut<'a> = __priv_Type_Attrs::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_Type_Attrs::Ref<'a> {
  type Target = __::pz::Type_Attrs;
  fn as_ref(&self) -> __priv_Type_Attrs::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_Type_Attrs::Mut<'a> {
  type Target = __::pz::Type_Attrs;
  fn as_ref(&self) -> __priv_Type_Attrs::Ref { self.r }
  fn into_ref(self) -> __priv_Type_Attrs::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_Type_Attrs::Mut {
    __priv_Type_Attrs::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::Type_Attrs {
  const DEFAULT: &'static Self = __::pz::Type_Attrs::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_Type_Attrs::Ref<'a> {
  type Message = __::pz::Type_Attrs;
}
impl<'a> __r::MessageMut<'a> for __priv_Type_Attrs::Mut<'a> {
  type Message = __::pz::Type_Attrs;
}

impl __s::default::Default for __::pz::Type_Attrs {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::Type_Attrs>> __s::convert::From<T> for __::pz::Type_Attrs {
  fn from(value: T) -> __::pz::Type_Attrs {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::Type_Attrs> for &__::pz::Type_Attrs {
  fn apply_to(self, mut m: __r::Mut<__::pz::Type_Attrs>) {
    __::pz::Type_Attrs::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Type_Attrs>> for &__::pz::Type_Attrs {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Type_Attrs>>) {
    __::pz::Type_Attrs::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::Type_Attrs> for __rt::reflect::Ref<'_, __::pz::Type_Attrs> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Type_Attrs>) {
    __::pz::Type_Attrs::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::Type_Attrs>> for __rt::reflect::Ref<'_, __::pz::Type_Attrs> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Type_Attrs>>) {
    __::pz::Type_Attrs::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::Type_Attrs> for &__rt::reflect::Mut<'_, __::pz::Type_Attrs> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Type_Attrs>) {
    __::pz::Type_Attrs::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Type_Attrs>> for &__rt::reflect::Mut<'_, __::pz::Type_Attrs> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Type_Attrs>>) {
    __::pz::Type_Attrs::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_Type_Attrs::Ref<'_> {
  fn default() -> Self {
    __::pz::Type_Attrs::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::Type_Attrs {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_Type_Attrs::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Type.Attrs ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Type_Attrs::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::Type_Attrs {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_Type_Attrs__deprecated = __rt::field!(deprecated);
impl __r::Field<__field_Type_Attrs__deprecated> for __::pz::Type_Attrs {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_Type_Attrs__deprecated;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "deprecated";
}

type __field_Type_Attrs__docs = __rt::field!(docs);
impl __r::Field<__field_Type_Attrs__docs> for __::pz::Type_Attrs {
  type Type = __r::Rep<__rt::String>;
  type Name = __field_Type_Attrs__docs;
  const NUMBER: __s::primitive::i32 = 100;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "docs";
}

impl<'proto> __priv_Type_Attrs::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_Type_Attrs::Ref { *self }

  /// Serializes this [`__::pz::Type_Attrs`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Type_Attrs`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Type_Attrs>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn deprecated(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Type_Attrs__deprecated{})
  }

  pub fn docs(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_Type_Attrs__docs{})
  }
  pub fn docs_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.docs().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.deprecated_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("deprecated")?;
      debug.write_debug(value);
      count += 1;
    }
    if !self.docs().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("docs")?;
      debug.iter(self.docs())?;
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_Type_Attrs::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_Type_Attrs::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_Type_Attrs::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_Type_Attrs::Mut {
    __priv_Type_Attrs::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::Type_Attrs`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Type_Attrs`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Type_Attrs`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Type_Attrs>,
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
    S: __r::Selector<__::pz::Type_Attrs>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Type_Attrs`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn deprecated(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Type_Attrs__deprecated{})
  }
  pub fn deprecated_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.deprecated_mut_or().into_inner()
  }
  pub fn deprecated_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_Type_Attrs__deprecated{})
  }
  pub fn set_deprecated(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::Type_Attrs> {
    value.apply_to(self.as_mut().deprecated_mut_or());
    self
  }

  pub fn docs(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_Type_Attrs__docs{})
  }
  pub fn docs_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.docs().at(idx)
  }
  pub fn docs_mut(mut self) -> __rt::Repeated<'proto, __rt::String> {
    self.get_mut(__field_Type_Attrs__docs{})
  }
  pub fn set_docs(mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'proto, __::pz::Type_Attrs> {
    value.apply_to(self.as_mut().docs_mut());
    self
  }

}

/// message `pz.Field`
pub struct Field {
  ptr: __s::ptr::NonNull<__priv_Field::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Field::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_Field {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_Field::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::Field>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::Field>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) number: <__s::primitive::i32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) is_repeated: <__s::primitive::bool as __z::Type>::__Storage<__z::Seal>,
    pub(in super) r#type: <__::pz::Field_Type as __z::Type>::__Storage<__z::Seal>,
    pub(in super) type_index: <__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) attrs: <__::pz::Field_Attrs as __z::Type>::__Storage<__z::Seal>,
    pub(in super) span: <__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>,
  }
}

impl __::pz::Field {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_Field::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      number: 0,
      is_repeated: false,
      r#type: __::pz::Field_Type::new(),
      type_index: 0,
      attrs: __s::option::Option::None,
      span: 0,
    }} as *const __priv_Field::Storage as *mut __priv_Field::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::Field`].
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

  /// Deserializes a new [`__::pz::Field`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::Field`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Field`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Field`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Field>,
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
    S: __r::Selector<__::pz::Field>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Field`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn name(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_Field__name{})
  }
  pub fn name_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_Field__name{})
  }
  pub fn set_name(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::Field> {
    value.apply_to(self.as_mut().name_mut_or());
    self.as_mut()
  }

  pub fn number(&self) -> __rt::reflect::Ref<'_, __s::primitive::i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::i32>> {
    self.get(__field_Field__number{})
  }
  pub fn number_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::i32> {
    self.number_mut_or().into_inner()
  }
  pub fn number_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::i32> {
    self.get_mut(__field_Field__number{})
  }
  pub fn set_number(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::i32>>) -> __r::Mut<'_, __::pz::Field> {
    value.apply_to(self.as_mut().number_mut_or());
    self.as_mut()
  }

  pub fn is_repeated(&self) -> __rt::reflect::Ref<'_, __s::primitive::bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::bool>> {
    self.get(__field_Field__is_repeated{})
  }
  pub fn is_repeated_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::bool> {
    self.is_repeated_mut_or().into_inner()
  }
  pub fn is_repeated_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::bool> {
    self.get_mut(__field_Field__is_repeated{})
  }
  pub fn set_is_repeated(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::bool>>) -> __r::Mut<'_, __::pz::Field> {
    value.apply_to(self.as_mut().is_repeated_mut_or());
    self.as_mut()
  }

  pub fn r#type(&self) -> __rt::reflect::Ref<'_, __::pz::Field_Type> {
    self.type_or().unwrap_or_default()
  }
  pub fn type_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::Field_Type>> {
    self.get(__field_Field__type{})
  }
  pub fn type_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::Field_Type> {
    self.type_mut_or().into_inner()
  }
  pub fn type_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::Field_Type> {
    self.get_mut(__field_Field__type{})
  }
  pub fn set_type(&mut self, value: impl __r::Set<__r::Opt<__::pz::Field_Type>>) -> __r::Mut<'_, __::pz::Field> {
    value.apply_to(self.as_mut().type_mut_or());
    self.as_mut()
  }

  pub fn type_index(&self) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u32>> {
    self.get(__field_Field__type_index{})
  }
  pub fn type_index_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u32> {
    self.type_index_mut_or().into_inner()
  }
  pub fn type_index_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u32> {
    self.get_mut(__field_Field__type_index{})
  }
  pub fn set_type_index(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::Field> {
    value.apply_to(self.as_mut().type_index_mut_or());
    self.as_mut()
  }

  pub fn attrs(&self) -> __rt::reflect::Ref<'_, __::pz::Field_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::Field_Attrs>> {
    self.get(__field_Field__attrs{})
  }
  pub fn attrs_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::Field_Attrs> {
    self.attrs_mut_or().into_inner()
  }
  pub fn attrs_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::Field_Attrs> {
    self.get_mut(__field_Field__attrs{})
  }
  pub fn set_attrs(&mut self, value: impl __r::Set<__r::Opt<__::pz::Field_Attrs>>) -> __r::Mut<'_, __::pz::Field> {
    value.apply_to(self.as_mut().attrs_mut_or());
    self.as_mut()
  }

  pub fn span(&self) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u32>> {
    self.get(__field_Field__span{})
  }
  pub fn span_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u32> {
    self.span_mut_or().into_inner()
  }
  pub fn span_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u32> {
    self.get_mut(__field_Field__span{})
  }
  pub fn set_span(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::Field> {
    value.apply_to(self.as_mut().span_mut_or());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Field::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_Field::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::Field>, src: __rt::reflect::Ref<__::pz::Field>) {
    __r::Set::<<__::pz::Field as __r::Field<__field_Field__name>>::Type>::apply_to(src.get(__field_Field__name{}), dst.as_mut().get_mut(__field_Field__name{}));
    __r::Set::<<__::pz::Field as __r::Field<__field_Field__number>>::Type>::apply_to(src.get(__field_Field__number{}), dst.as_mut().get_mut(__field_Field__number{}));
    __r::Set::<<__::pz::Field as __r::Field<__field_Field__is_repeated>>::Type>::apply_to(src.get(__field_Field__is_repeated{}), dst.as_mut().get_mut(__field_Field__is_repeated{}));
    __r::Set::<<__::pz::Field as __r::Field<__field_Field__type>>::Type>::apply_to(src.get(__field_Field__type{}), dst.as_mut().get_mut(__field_Field__type{}));
    __r::Set::<<__::pz::Field as __r::Field<__field_Field__type_index>>::Type>::apply_to(src.get(__field_Field__type_index{}), dst.as_mut().get_mut(__field_Field__type_index{}));
    __r::Set::<<__::pz::Field as __r::Field<__field_Field__attrs>>::Type>::apply_to(src.get(__field_Field__attrs{}), dst.as_mut().get_mut(__field_Field__attrs{}));
    __r::Set::<<__::pz::Field as __r::Field<__field_Field__span>>::Type>::apply_to(src.get(__field_Field__span{}), dst.as_mut().get_mut(__field_Field__span{}));
  }
}

impl __z::Message for __::pz::Field {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{7 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::Field::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::Field_Attrs::__tdp_info,
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
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().number as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        __z::tdp::FieldStorage {
          number: 3,
          flags:
            __z::tdp::Kind::Bool.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().is_repeated as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
        },
        __z::tdp::FieldStorage {
          number: 4,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().r#type as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 3,
        },
        __z::tdp::FieldStorage {
          number: 5,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().type_index as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 4,
        },
        __z::tdp::FieldStorage {
          number: 10,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().attrs as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 5,
        },
        __z::tdp::FieldStorage {
          number: 20,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().span as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 6,
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

impl __z::Type for __::pz::Field {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::Field::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::Field::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_Field::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_Field::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_Field::Mut {
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

impl __r::Views for __::pz::Field {
  type Ref<'a> = __priv_Field::Ref<'a>;
  type Mut<'a> = __priv_Field::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_Field::Ref<'a> {
  type Target = __::pz::Field;
  fn as_ref(&self) -> __priv_Field::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_Field::Mut<'a> {
  type Target = __::pz::Field;
  fn as_ref(&self) -> __priv_Field::Ref { self.r }
  fn into_ref(self) -> __priv_Field::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_Field::Mut {
    __priv_Field::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::Field {
  const DEFAULT: &'static Self = __::pz::Field::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_Field::Ref<'a> {
  type Message = __::pz::Field;
}
impl<'a> __r::MessageMut<'a> for __priv_Field::Mut<'a> {
  type Message = __::pz::Field;
}

impl __s::default::Default for __::pz::Field {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::Field>> __s::convert::From<T> for __::pz::Field {
  fn from(value: T) -> __::pz::Field {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::Field> for &__::pz::Field {
  fn apply_to(self, mut m: __r::Mut<__::pz::Field>) {
    __::pz::Field::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Field>> for &__::pz::Field {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Field>>) {
    __::pz::Field::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::Field> for __rt::reflect::Ref<'_, __::pz::Field> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Field>) {
    __::pz::Field::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::Field>> for __rt::reflect::Ref<'_, __::pz::Field> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Field>>) {
    __::pz::Field::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::Field> for &__rt::reflect::Mut<'_, __::pz::Field> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Field>) {
    __::pz::Field::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Field>> for &__rt::reflect::Mut<'_, __::pz::Field> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Field>>) {
    __::pz::Field::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_Field::Ref<'_> {
  fn default() -> Self {
    __::pz::Field::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::Field {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_Field::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Field ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Field::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::Field {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_Field__name = __rt::field!(name);
impl __r::Field<__field_Field__name> for __::pz::Field {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_Field__name;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "name";
}

type __field_Field__number = __rt::field!(number);
impl __r::Field<__field_Field__number> for __::pz::Field {
  type Type = __r::Opt<__s::primitive::i32>;
  type Name = __field_Field__number;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "number";
}

type __field_Field__is_repeated = __rt::field!(is_repeated);
impl __r::Field<__field_Field__is_repeated> for __::pz::Field {
  type Type = __r::Opt<__s::primitive::bool>;
  type Name = __field_Field__is_repeated;
  const NUMBER: __s::primitive::i32 = 3;
  const INDEX: __s::primitive::usize = 2;
  const NAME: &'static __s::primitive::str = "is_repeated";
}

type __field_Field__type = __rt::field!(type);
impl __r::Field<__field_Field__type> for __::pz::Field {
  type Type = __r::Opt<__::pz::Field_Type>;
  type Name = __field_Field__type;
  const NUMBER: __s::primitive::i32 = 4;
  const INDEX: __s::primitive::usize = 3;
  const NAME: &'static __s::primitive::str = "type";
}

type __field_Field__type_index = __rt::field!(type_index);
impl __r::Field<__field_Field__type_index> for __::pz::Field {
  type Type = __r::Opt<__s::primitive::u32>;
  type Name = __field_Field__type_index;
  const NUMBER: __s::primitive::i32 = 5;
  const INDEX: __s::primitive::usize = 4;
  const NAME: &'static __s::primitive::str = "type_index";
}

type __field_Field__attrs = __rt::field!(attrs);
impl __r::Field<__field_Field__attrs> for __::pz::Field {
  type Type = __r::Opt<__::pz::Field_Attrs>;
  type Name = __field_Field__attrs;
  const NUMBER: __s::primitive::i32 = 10;
  const INDEX: __s::primitive::usize = 5;
  const NAME: &'static __s::primitive::str = "attrs";
}

type __field_Field__span = __rt::field!(span);
impl __r::Field<__field_Field__span> for __::pz::Field {
  type Type = __r::Opt<__s::primitive::u32>;
  type Name = __field_Field__span;
  const NUMBER: __s::primitive::i32 = 20;
  const INDEX: __s::primitive::usize = 6;
  const NAME: &'static __s::primitive::str = "span";
}

impl<'proto> __priv_Field::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_Field::Ref { *self }

  /// Serializes this [`__::pz::Field`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Field`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Field>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Field__name{})
  }

  pub fn number(self) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i32>> {
    self.get(__field_Field__number{})
  }

  pub fn is_repeated(self) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::bool>> {
    self.get(__field_Field__is_repeated{})
  }

  pub fn r#type(self) -> __rt::reflect::Ref<'proto, __::pz::Field_Type> {
    self.type_or().unwrap_or_default()
  }
  pub fn type_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::Field_Type>> {
    self.get(__field_Field__type{})
  }

  pub fn type_index(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Field__type_index{})
  }

  pub fn attrs(self) -> __rt::reflect::Ref<'proto, __::pz::Field_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::Field_Attrs>> {
    self.get(__field_Field__attrs{})
  }

  pub fn span(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Field__span{})
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.number_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("number")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.is_repeated_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("is_repeated")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.type_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("type")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.type_index_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("type_index")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.attrs_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("attrs")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.span_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("span")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_Field::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_Field::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_Field::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_Field::Mut {
    __priv_Field::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::Field`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Field`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Field`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Field>,
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
    S: __r::Selector<__::pz::Field>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Field`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Field__name{})
  }
  pub fn name_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_Field__name{})
  }
  pub fn set_name(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::Field> {
    value.apply_to(self.as_mut().name_mut_or());
    self
  }

  pub fn number(self) -> __rt::reflect::Ref<'proto, __s::primitive::i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::i32>> {
    self.get(__field_Field__number{})
  }
  pub fn number_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::i32> {
    self.number_mut_or().into_inner()
  }
  pub fn number_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::i32> {
    self.get_mut(__field_Field__number{})
  }
  pub fn set_number(mut self, value: impl __r::Set<__r::Opt<__s::primitive::i32>>) -> __r::Mut<'proto, __::pz::Field> {
    value.apply_to(self.as_mut().number_mut_or());
    self
  }

  pub fn is_repeated(self) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::bool>> {
    self.get(__field_Field__is_repeated{})
  }
  pub fn is_repeated_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::bool> {
    self.is_repeated_mut_or().into_inner()
  }
  pub fn is_repeated_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::bool> {
    self.get_mut(__field_Field__is_repeated{})
  }
  pub fn set_is_repeated(mut self, value: impl __r::Set<__r::Opt<__s::primitive::bool>>) -> __r::Mut<'proto, __::pz::Field> {
    value.apply_to(self.as_mut().is_repeated_mut_or());
    self
  }

  pub fn r#type(self) -> __rt::reflect::Ref<'proto, __::pz::Field_Type> {
    self.type_or().unwrap_or_default()
  }
  pub fn type_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::Field_Type>> {
    self.get(__field_Field__type{})
  }
  pub fn type_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::Field_Type> {
    self.type_mut_or().into_inner()
  }
  pub fn type_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::Field_Type> {
    self.get_mut(__field_Field__type{})
  }
  pub fn set_type(mut self, value: impl __r::Set<__r::Opt<__::pz::Field_Type>>) -> __r::Mut<'proto, __::pz::Field> {
    value.apply_to(self.as_mut().type_mut_or());
    self
  }

  pub fn type_index(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Field__type_index{})
  }
  pub fn type_index_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u32> {
    self.type_index_mut_or().into_inner()
  }
  pub fn type_index_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u32> {
    self.get_mut(__field_Field__type_index{})
  }
  pub fn set_type_index(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::Field> {
    value.apply_to(self.as_mut().type_index_mut_or());
    self
  }

  pub fn attrs(self) -> __rt::reflect::Ref<'proto, __::pz::Field_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::Field_Attrs>> {
    self.get(__field_Field__attrs{})
  }
  pub fn attrs_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::Field_Attrs> {
    self.attrs_mut_or().into_inner()
  }
  pub fn attrs_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::Field_Attrs> {
    self.get_mut(__field_Field__attrs{})
  }
  pub fn set_attrs(mut self, value: impl __r::Set<__r::Opt<__::pz::Field_Attrs>>) -> __r::Mut<'proto, __::pz::Field> {
    value.apply_to(self.as_mut().attrs_mut_or());
    self
  }

  pub fn span(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Field__span{})
  }
  pub fn span_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u32> {
    self.span_mut_or().into_inner()
  }
  pub fn span_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u32> {
    self.get_mut(__field_Field__span{})
  }
  pub fn set_span(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::Field> {
    value.apply_to(self.as_mut().span_mut_or());
    self
  }

}

/// enum `pz.Field.Type`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Field_Type(pub __s::primitive::i32);

impl __::pz::Field_Type {
  pub const None: Self = Self(0);
  pub const I32: Self = Self(1);
  pub const U32: Self = Self(2);
  pub const F32: Self = Self(3);
  pub const I64: Self = Self(4);
  pub const U64: Self = Self(5);
  pub const F64: Self = Self(6);
  pub const Bool: Self = Self(7);
  pub const String: Self = Self(8);
  pub const Type: Self = Self(9);
  pub const Foreign: Self = Self(10);

  pub const fn new() -> Self {
    Self(0)
  }
}

impl __s::default::Default for __::pz::Field_Type {
  fn default() -> Self {
    Self(0)
  }
}

impl __z::Type for __::pz::Field_Type {
  type __Storage<S: __z::Sealed> = Self;

  unsafe fn __ref<'a, S: __z::Sealed>(_: S, ptr: __s::ptr::NonNull<Self>) -> __rt::Ref<'a, Self> {
    ptr.read()
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    _: S,
    mut ptr: __s::ptr::NonNull<Self>,
    _: __z::RawArena,
  ) -> __rt::Mut<'a, Self> {
    __rt::ScalarMut::__wrap(ptr.as_mut())
  }
}

impl __r::Views for __::pz::Field_Type {
  type Ref<'a> = Self;
  type Mut<'a> = __rt::ScalarMut<'a, Self>;
}

impl __r::RefView<'_> for __::pz::Field_Type {
  type Target = Self;

  fn as_ref(&self) -> Self {
    *self
  }
}

impl __r::Set<__::pz::Field_Type> for __::pz::Field_Type {
  fn apply_to(self, mut m: __r::Mut<__::pz::Field_Type>) {
    m.set(self)
  }
}

impl __r::Set<__r::Opt<__::pz::Field_Type>> for __::pz::Field_Type {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Field_Type>>) {
    m.into_inner().set(self)
  }
}

impl __s::fmt::Debug for __::pz::Field_Type {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    match *self {
      Self::None => __s::write!(fmt, "None"),
      Self::I32 => __s::write!(fmt, "I32"),
      Self::U32 => __s::write!(fmt, "U32"),
      Self::F32 => __s::write!(fmt, "F32"),
      Self::I64 => __s::write!(fmt, "I64"),
      Self::U64 => __s::write!(fmt, "U64"),
      Self::F64 => __s::write!(fmt, "F64"),
      Self::Bool => __s::write!(fmt, "Bool"),
      Self::String => __s::write!(fmt, "String"),
      Self::Type => __s::write!(fmt, "Type"),
      Self::Foreign => __s::write!(fmt, "Foreign"),
      Self(n) => __s::write!(fmt, "pz.Field.Type({n})"),
    }
  }
}

/// message `pz.Field.Attrs`
pub struct Field_Attrs {
  ptr: __s::ptr::NonNull<__priv_Field_Attrs::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Field_Attrs::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_Field_Attrs {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_Field_Attrs::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::Field_Attrs>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::Field_Attrs>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) deprecated: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) docs: __z::AVec<<__rt::String as __z::Type>::__Storage<__z::Seal>>,
  }
}

impl __::pz::Field_Attrs {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_Field_Attrs::Storage {
      __hasbits: [0; 1],
      deprecated: __z::RawStr::new(),
      docs: __z::AVec::new(),
    }} as *const __priv_Field_Attrs::Storage as *mut __priv_Field_Attrs::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::Field_Attrs`].
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

  /// Deserializes a new [`__::pz::Field_Attrs`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::Field_Attrs`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Field_Attrs`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Field_Attrs`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Field_Attrs>,
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
    S: __r::Selector<__::pz::Field_Attrs>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Field_Attrs`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn deprecated(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_Field_Attrs__deprecated{})
  }
  pub fn deprecated_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.deprecated_mut_or().into_inner()
  }
  pub fn deprecated_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_Field_Attrs__deprecated{})
  }
  pub fn set_deprecated(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::Field_Attrs> {
    value.apply_to(self.as_mut().deprecated_mut_or());
    self.as_mut()
  }

  pub fn docs(&self) -> __rt::Slice<'_, __rt::String> {
    self.get(__field_Field_Attrs__docs{})
  }
  pub fn docs_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __rt::String> {
    self.docs().at(idx)
  }
  pub fn docs_mut(&mut self) -> __rt::Repeated<'_, __rt::String> {
    self.get_mut(__field_Field_Attrs__docs{})
  }
  pub fn set_docs(&mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'_, __::pz::Field_Attrs> {
    value.apply_to(self.as_mut().docs_mut());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Field_Attrs::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_Field_Attrs::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::Field_Attrs>, src: __rt::reflect::Ref<__::pz::Field_Attrs>) {
    __r::Set::<<__::pz::Field_Attrs as __r::Field<__field_Field_Attrs__deprecated>>::Type>::apply_to(src.get(__field_Field_Attrs__deprecated{}), dst.as_mut().get_mut(__field_Field_Attrs__deprecated{}));
    __r::Set::<<__::pz::Field_Attrs as __r::Field<__field_Field_Attrs__docs>>::Type>::apply_to(src.get(__field_Field_Attrs__docs{}), dst.as_mut().get_mut(__field_Field_Attrs__docs{}));
  }
}

impl __z::Message for __::pz::Field_Attrs {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{2 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::Field_Attrs::__LAYOUT.size();
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
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Field_Attrs::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().deprecated as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 100,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::Field_Attrs::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().docs as *const _ as *const u8;
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

impl __z::Type for __::pz::Field_Attrs {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::Field_Attrs::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::Field_Attrs::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_Field_Attrs::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_Field_Attrs::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_Field_Attrs::Mut {
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

impl __r::Views for __::pz::Field_Attrs {
  type Ref<'a> = __priv_Field_Attrs::Ref<'a>;
  type Mut<'a> = __priv_Field_Attrs::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_Field_Attrs::Ref<'a> {
  type Target = __::pz::Field_Attrs;
  fn as_ref(&self) -> __priv_Field_Attrs::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_Field_Attrs::Mut<'a> {
  type Target = __::pz::Field_Attrs;
  fn as_ref(&self) -> __priv_Field_Attrs::Ref { self.r }
  fn into_ref(self) -> __priv_Field_Attrs::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_Field_Attrs::Mut {
    __priv_Field_Attrs::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::Field_Attrs {
  const DEFAULT: &'static Self = __::pz::Field_Attrs::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_Field_Attrs::Ref<'a> {
  type Message = __::pz::Field_Attrs;
}
impl<'a> __r::MessageMut<'a> for __priv_Field_Attrs::Mut<'a> {
  type Message = __::pz::Field_Attrs;
}

impl __s::default::Default for __::pz::Field_Attrs {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::Field_Attrs>> __s::convert::From<T> for __::pz::Field_Attrs {
  fn from(value: T) -> __::pz::Field_Attrs {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::Field_Attrs> for &__::pz::Field_Attrs {
  fn apply_to(self, mut m: __r::Mut<__::pz::Field_Attrs>) {
    __::pz::Field_Attrs::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Field_Attrs>> for &__::pz::Field_Attrs {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Field_Attrs>>) {
    __::pz::Field_Attrs::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::Field_Attrs> for __rt::reflect::Ref<'_, __::pz::Field_Attrs> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Field_Attrs>) {
    __::pz::Field_Attrs::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::Field_Attrs>> for __rt::reflect::Ref<'_, __::pz::Field_Attrs> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Field_Attrs>>) {
    __::pz::Field_Attrs::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::Field_Attrs> for &__rt::reflect::Mut<'_, __::pz::Field_Attrs> {
  fn apply_to(self, mut m: __r::Mut<__::pz::Field_Attrs>) {
    __::pz::Field_Attrs::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::Field_Attrs>> for &__rt::reflect::Mut<'_, __::pz::Field_Attrs> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::Field_Attrs>>) {
    __::pz::Field_Attrs::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_Field_Attrs::Ref<'_> {
  fn default() -> Self {
    __::pz::Field_Attrs::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::Field_Attrs {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_Field_Attrs::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Field.Attrs ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Field_Attrs::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::Field_Attrs {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_Field_Attrs__deprecated = __rt::field!(deprecated);
impl __r::Field<__field_Field_Attrs__deprecated> for __::pz::Field_Attrs {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_Field_Attrs__deprecated;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "deprecated";
}

type __field_Field_Attrs__docs = __rt::field!(docs);
impl __r::Field<__field_Field_Attrs__docs> for __::pz::Field_Attrs {
  type Type = __r::Rep<__rt::String>;
  type Name = __field_Field_Attrs__docs;
  const NUMBER: __s::primitive::i32 = 100;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "docs";
}

impl<'proto> __priv_Field_Attrs::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_Field_Attrs::Ref { *self }

  /// Serializes this [`__::pz::Field_Attrs`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Field_Attrs`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Field_Attrs>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn deprecated(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Field_Attrs__deprecated{})
  }

  pub fn docs(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_Field_Attrs__docs{})
  }
  pub fn docs_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.docs().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.deprecated_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("deprecated")?;
      debug.write_debug(value);
      count += 1;
    }
    if !self.docs().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("docs")?;
      debug.iter(self.docs())?;
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_Field_Attrs::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_Field_Attrs::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_Field_Attrs::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_Field_Attrs::Mut {
    __priv_Field_Attrs::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::Field_Attrs`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::Field_Attrs`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::Field_Attrs`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::Field_Attrs>,
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
    S: __r::Selector<__::pz::Field_Attrs>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::Field_Attrs`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn deprecated(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Field_Attrs__deprecated{})
  }
  pub fn deprecated_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.deprecated_mut_or().into_inner()
  }
  pub fn deprecated_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_Field_Attrs__deprecated{})
  }
  pub fn set_deprecated(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::Field_Attrs> {
    value.apply_to(self.as_mut().deprecated_mut_or());
    self
  }

  pub fn docs(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_Field_Attrs__docs{})
  }
  pub fn docs_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.docs().at(idx)
  }
  pub fn docs_mut(mut self) -> __rt::Repeated<'proto, __rt::String> {
    self.get_mut(__field_Field_Attrs__docs{})
  }
  pub fn set_docs(mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'proto, __::pz::Field_Attrs> {
    value.apply_to(self.as_mut().docs_mut());
    self
  }

}

pub mod plugin {
use super::{__, __rt, __z, __s, __r};
use __s::default::Default as _;
/// choice `pz.plugin.Request`
pub struct Request {
  ptr: __s::ptr::NonNull<__priv_Request::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Request::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_Request {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_Request::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::Request>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::Request>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(super) which: u32,
    pub(super) union: Union,
  }

  #[repr(C)]
  pub union Union {
    pub(super) __unset: (),
    pub(in super) about: <__::pz::plugin::AboutRequest as __z::Type>::__Storage<__z::Seal>,
    pub(in super) codegen: <__::pz::plugin::CodegenRequest as __z::Type>::__Storage<__z::Seal>,
  }

  pub const UNION_OFFSET: usize = {
    let align = __s::mem::align_of::<__priv_Request::Union>();
    if align < 4 { 4 } else { align }
  };
}

impl __::pz::plugin::Request {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_Request::Storage {
      which: 0,
      union: __priv_Request::Union { __unset: () },
    }} as *const __priv_Request::Storage as *mut __priv_Request::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::Request`].
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

  /// Deserializes a new [`__::pz::plugin::Request`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::Request`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::Request`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Request`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Request>,
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
    S: __r::Selector<__::pz::plugin::Request>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::Request`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn which(&self) -> i32 {
    self.as_ref().which()
  }

  pub fn cases(&self) -> __::pz::plugin::RequestCases<__r::SelectRef> {
    self.as_ref().cases()
  }

  pub fn cases_mut(&mut self) -> __::pz::plugin::RequestCases<__r::SelectMut> {
    self.as_mut().cases_mut()
  }

  pub fn about(&self) -> __rt::reflect::Ref<'_, __::pz::plugin::AboutRequest> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::plugin::AboutRequest>> {
    self.get(__field_Request__about{})
  }
  pub fn about_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::plugin::AboutRequest> {
    self.about_mut_or().into_inner()
  }
  pub fn about_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::plugin::AboutRequest> {
    self.get_mut(__field_Request__about{})
  }
  pub fn set_about(&mut self, value: impl __r::Set<__r::Opt<__::pz::plugin::AboutRequest>>) -> __r::Mut<'_, __::pz::plugin::Request> {
    value.apply_to(self.as_mut().about_mut_or());
    self.as_mut()
  }

  pub fn codegen(&self) -> __rt::reflect::Ref<'_, __::pz::plugin::CodegenRequest> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::plugin::CodegenRequest>> {
    self.get(__field_Request__codegen{})
  }
  pub fn codegen_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::plugin::CodegenRequest> {
    self.codegen_mut_or().into_inner()
  }
  pub fn codegen_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::plugin::CodegenRequest> {
    self.get_mut(__field_Request__codegen{})
  }
  pub fn set_codegen(&mut self, value: impl __r::Set<__r::Opt<__::pz::plugin::CodegenRequest>>) -> __r::Mut<'_, __::pz::plugin::Request> {
    value.apply_to(self.as_mut().codegen_mut_or());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Request::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_Request::Storage>().as_mut().which = 0;
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::Request>, src: __rt::reflect::Ref<__::pz::plugin::Request>) {
    match src.which() {
      0 => dst.clear(),
      1 => __r::Set::<<__::pz::plugin::Request as __r::Field<__field_Request__about>>::Type>::apply_to(src.get(__field_Request__about{}), dst.get_mut(__field_Request__about{})),
      2 => __r::Set::<<__::pz::plugin::Request as __r::Field<__field_Request__codegen>>::Type>::apply_to(src.get(__field_Request__codegen{}), dst.get_mut(__field_Request__codegen{})),
      _ => __s::unreachable!(),
    }
  }
}

#[non_exhaustive]
pub enum RequestCases<'proto, Which: __r::Select = __r::SelectRef> {
  Unset,
  About(__r::View<'proto, __::pz::plugin::AboutRequest, Which>),
  Codegen(__r::View<'proto, __::pz::plugin::CodegenRequest, Which>),

  #[doc(hidden)]
  __PhantomData(__s::marker::PhantomData<&'proto Which>, __z::Void),
}

impl __z::Message for __::pz::plugin::Request {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{2 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::Request::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::plugin::AboutRequest::__tdp_info,
            __::pz::plugin::CodegenRequest::__tdp_info,
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
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_Request::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_Request::UNION_OFFSET as u32,
          desc: 1,
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

impl __z::Type for __::pz::plugin::Request {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::Request::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::Request::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_Request::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_Request::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_Request::Mut {
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

impl __r::Views for __::pz::plugin::Request {
  type Ref<'a> = __priv_Request::Ref<'a>;
  type Mut<'a> = __priv_Request::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_Request::Ref<'a> {
  type Target = __::pz::plugin::Request;
  fn as_ref(&self) -> __priv_Request::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_Request::Mut<'a> {
  type Target = __::pz::plugin::Request;
  fn as_ref(&self) -> __priv_Request::Ref { self.r }
  fn into_ref(self) -> __priv_Request::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_Request::Mut {
    __priv_Request::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::Request {
  const DEFAULT: &'static Self = __::pz::plugin::Request::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_Request::Ref<'a> {
  type Message = __::pz::plugin::Request;
}
impl<'a> __r::MessageMut<'a> for __priv_Request::Mut<'a> {
  type Message = __::pz::plugin::Request;
}

impl __s::default::Default for __::pz::plugin::Request {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::Request>> __s::convert::From<T> for __::pz::plugin::Request {
  fn from(value: T) -> __::pz::plugin::Request {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::Request> for &__::pz::plugin::Request {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Request>) {
    __::pz::plugin::Request::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Request>> for &__::pz::plugin::Request {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Request>>) {
    __::pz::plugin::Request::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::Request> for __rt::reflect::Ref<'_, __::pz::plugin::Request> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Request>) {
    __::pz::plugin::Request::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Request>> for __rt::reflect::Ref<'_, __::pz::plugin::Request> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Request>>) {
    __::pz::plugin::Request::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::Request> for &__rt::reflect::Mut<'_, __::pz::plugin::Request> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Request>) {
    __::pz::plugin::Request::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Request>> for &__rt::reflect::Mut<'_, __::pz::plugin::Request> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Request>>) {
    __::pz::plugin::Request::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_Request::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::Request::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::Request {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_Request::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.Request ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Request::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::Request {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_Request__about = __rt::field!(about);
impl __r::Field<__field_Request__about> for __::pz::plugin::Request {
  type Type = __r::Opt<__::pz::plugin::AboutRequest>;
  type Name = __field_Request__about;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "about";
}

type __field_Request__codegen = __rt::field!(codegen);
impl __r::Field<__field_Request__codegen> for __::pz::plugin::Request {
  type Type = __r::Opt<__::pz::plugin::CodegenRequest>;
  type Name = __field_Request__codegen;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "codegen";
}

impl<'proto> __priv_Request::Ref<'proto> {
  pub fn which(self) -> i32 {
    unsafe { self.ptr.as_ref() }.which as i32
  }

  pub fn cases(self) -> __::pz::plugin::RequestCases<'proto> {
    unsafe {
      match self.which() {
        0 => __::pz::plugin::RequestCases::Unset,
        1 => __::pz::plugin::RequestCases::About(self.get(__field_Request__about{}).unwrap_unchecked()),
        2 => __::pz::plugin::RequestCases::Codegen(self.get(__field_Request__codegen{}).unwrap_unchecked()),
        _ => __s::unreachable!(),
      }
    }
  }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_Request::Ref { *self }

  /// Serializes this [`__::pz::plugin::Request`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Request`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Request>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn about(self) -> __rt::reflect::Ref<'proto, __::pz::plugin::AboutRequest> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::plugin::AboutRequest>> {
    self.get(__field_Request__about{})
  }

  pub fn codegen(self) -> __rt::reflect::Ref<'proto, __::pz::plugin::CodegenRequest> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::plugin::CodegenRequest>> {
    self.get(__field_Request__codegen{})
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.about_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("about")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.codegen_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("codegen")?;
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

impl<'proto> __priv_Request::Mut<'proto>  {
  pub fn which(&self) -> i32 {
    self.as_ref().which()
  }

  pub fn cases(self) -> __::pz::plugin::RequestCases<'proto> {
    self.into_ref().cases()
  }

  pub fn cases_mut(self) -> __::pz::plugin::RequestCases<'proto, __r::SelectMut> {
    unsafe {
      match self.which() {
        0 => __::pz::plugin::RequestCases::Unset,
        1 => __::pz::plugin::RequestCases::About(self.get_mut(__field_Request__about{}).into_option().unwrap_unchecked()),
        2 => __::pz::plugin::RequestCases::Codegen(self.get_mut(__field_Request__codegen{}).into_option().unwrap_unchecked()),
        _ => __s::unreachable!(),
      }
    }
  }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_Request::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_Request::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_Request::Mut {
    __priv_Request::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::Request`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::Request`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Request`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Request>,
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
    S: __r::Selector<__::pz::plugin::Request>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::Request`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn about(self) -> __rt::reflect::Ref<'proto, __::pz::plugin::AboutRequest> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::plugin::AboutRequest>> {
    self.get(__field_Request__about{})
  }
  pub fn about_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::plugin::AboutRequest> {
    self.about_mut_or().into_inner()
  }
  pub fn about_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::plugin::AboutRequest> {
    self.get_mut(__field_Request__about{})
  }
  pub fn set_about(mut self, value: impl __r::Set<__r::Opt<__::pz::plugin::AboutRequest>>) -> __r::Mut<'proto, __::pz::plugin::Request> {
    value.apply_to(self.as_mut().about_mut_or());
    self
  }

  pub fn codegen(self) -> __rt::reflect::Ref<'proto, __::pz::plugin::CodegenRequest> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::plugin::CodegenRequest>> {
    self.get(__field_Request__codegen{})
  }
  pub fn codegen_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::plugin::CodegenRequest> {
    self.codegen_mut_or().into_inner()
  }
  pub fn codegen_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::plugin::CodegenRequest> {
    self.get_mut(__field_Request__codegen{})
  }
  pub fn set_codegen(mut self, value: impl __r::Set<__r::Opt<__::pz::plugin::CodegenRequest>>) -> __r::Mut<'proto, __::pz::plugin::Request> {
    value.apply_to(self.as_mut().codegen_mut_or());
    self
  }

}

/// choice `pz.plugin.Response`
pub struct Response {
  ptr: __s::ptr::NonNull<__priv_Response::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Response::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_Response {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_Response::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::Response>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::Response>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(super) which: u32,
    pub(super) union: Union,
  }

  #[repr(C)]
  pub union Union {
    pub(super) __unset: (),
    pub(in super) about: <__::pz::plugin::AboutResponse as __z::Type>::__Storage<__z::Seal>,
    pub(in super) codegen: <__::pz::plugin::CodegenResponse as __z::Type>::__Storage<__z::Seal>,
  }

  pub const UNION_OFFSET: usize = {
    let align = __s::mem::align_of::<__priv_Response::Union>();
    if align < 4 { 4 } else { align }
  };
}

impl __::pz::plugin::Response {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_Response::Storage {
      which: 0,
      union: __priv_Response::Union { __unset: () },
    }} as *const __priv_Response::Storage as *mut __priv_Response::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::Response`].
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

  /// Deserializes a new [`__::pz::plugin::Response`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::Response`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::Response`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Response`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Response>,
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
    S: __r::Selector<__::pz::plugin::Response>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::Response`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn which(&self) -> i32 {
    self.as_ref().which()
  }

  pub fn cases(&self) -> __::pz::plugin::ResponseCases<__r::SelectRef> {
    self.as_ref().cases()
  }

  pub fn cases_mut(&mut self) -> __::pz::plugin::ResponseCases<__r::SelectMut> {
    self.as_mut().cases_mut()
  }

  pub fn about(&self) -> __rt::reflect::Ref<'_, __::pz::plugin::AboutResponse> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::plugin::AboutResponse>> {
    self.get(__field_Response__about{})
  }
  pub fn about_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::plugin::AboutResponse> {
    self.about_mut_or().into_inner()
  }
  pub fn about_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::plugin::AboutResponse> {
    self.get_mut(__field_Response__about{})
  }
  pub fn set_about(&mut self, value: impl __r::Set<__r::Opt<__::pz::plugin::AboutResponse>>) -> __r::Mut<'_, __::pz::plugin::Response> {
    value.apply_to(self.as_mut().about_mut_or());
    self.as_mut()
  }

  pub fn codegen(&self) -> __rt::reflect::Ref<'_, __::pz::plugin::CodegenResponse> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::plugin::CodegenResponse>> {
    self.get(__field_Response__codegen{})
  }
  pub fn codegen_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::plugin::CodegenResponse> {
    self.codegen_mut_or().into_inner()
  }
  pub fn codegen_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::plugin::CodegenResponse> {
    self.get_mut(__field_Response__codegen{})
  }
  pub fn set_codegen(&mut self, value: impl __r::Set<__r::Opt<__::pz::plugin::CodegenResponse>>) -> __r::Mut<'_, __::pz::plugin::Response> {
    value.apply_to(self.as_mut().codegen_mut_or());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Response::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_Response::Storage>().as_mut().which = 0;
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::Response>, src: __rt::reflect::Ref<__::pz::plugin::Response>) {
    match src.which() {
      0 => dst.clear(),
      1 => __r::Set::<<__::pz::plugin::Response as __r::Field<__field_Response__about>>::Type>::apply_to(src.get(__field_Response__about{}), dst.get_mut(__field_Response__about{})),
      2 => __r::Set::<<__::pz::plugin::Response as __r::Field<__field_Response__codegen>>::Type>::apply_to(src.get(__field_Response__codegen{}), dst.get_mut(__field_Response__codegen{})),
      _ => __s::unreachable!(),
    }
  }
}

#[non_exhaustive]
pub enum ResponseCases<'proto, Which: __r::Select = __r::SelectRef> {
  Unset,
  About(__r::View<'proto, __::pz::plugin::AboutResponse, Which>),
  Codegen(__r::View<'proto, __::pz::plugin::CodegenResponse, Which>),

  #[doc(hidden)]
  __PhantomData(__s::marker::PhantomData<&'proto Which>, __z::Void),
}

impl __z::Message for __::pz::plugin::Response {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{2 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::Response::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::plugin::AboutResponse::__tdp_info,
            __::pz::plugin::CodegenResponse::__tdp_info,
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
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_Response::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: __priv_Response::UNION_OFFSET as u32,
          desc: 1,
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

impl __z::Type for __::pz::plugin::Response {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::Response::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::Response::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_Response::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_Response::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_Response::Mut {
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

impl __r::Views for __::pz::plugin::Response {
  type Ref<'a> = __priv_Response::Ref<'a>;
  type Mut<'a> = __priv_Response::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_Response::Ref<'a> {
  type Target = __::pz::plugin::Response;
  fn as_ref(&self) -> __priv_Response::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_Response::Mut<'a> {
  type Target = __::pz::plugin::Response;
  fn as_ref(&self) -> __priv_Response::Ref { self.r }
  fn into_ref(self) -> __priv_Response::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_Response::Mut {
    __priv_Response::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::Response {
  const DEFAULT: &'static Self = __::pz::plugin::Response::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_Response::Ref<'a> {
  type Message = __::pz::plugin::Response;
}
impl<'a> __r::MessageMut<'a> for __priv_Response::Mut<'a> {
  type Message = __::pz::plugin::Response;
}

impl __s::default::Default for __::pz::plugin::Response {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::Response>> __s::convert::From<T> for __::pz::plugin::Response {
  fn from(value: T) -> __::pz::plugin::Response {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::Response> for &__::pz::plugin::Response {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Response>) {
    __::pz::plugin::Response::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Response>> for &__::pz::plugin::Response {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Response>>) {
    __::pz::plugin::Response::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::Response> for __rt::reflect::Ref<'_, __::pz::plugin::Response> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Response>) {
    __::pz::plugin::Response::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Response>> for __rt::reflect::Ref<'_, __::pz::plugin::Response> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Response>>) {
    __::pz::plugin::Response::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::Response> for &__rt::reflect::Mut<'_, __::pz::plugin::Response> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Response>) {
    __::pz::plugin::Response::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Response>> for &__rt::reflect::Mut<'_, __::pz::plugin::Response> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Response>>) {
    __::pz::plugin::Response::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_Response::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::Response::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::Response {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_Response::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.Response ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Response::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::Response {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_Response__about = __rt::field!(about);
impl __r::Field<__field_Response__about> for __::pz::plugin::Response {
  type Type = __r::Opt<__::pz::plugin::AboutResponse>;
  type Name = __field_Response__about;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "about";
}

type __field_Response__codegen = __rt::field!(codegen);
impl __r::Field<__field_Response__codegen> for __::pz::plugin::Response {
  type Type = __r::Opt<__::pz::plugin::CodegenResponse>;
  type Name = __field_Response__codegen;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "codegen";
}

impl<'proto> __priv_Response::Ref<'proto> {
  pub fn which(self) -> i32 {
    unsafe { self.ptr.as_ref() }.which as i32
  }

  pub fn cases(self) -> __::pz::plugin::ResponseCases<'proto> {
    unsafe {
      match self.which() {
        0 => __::pz::plugin::ResponseCases::Unset,
        1 => __::pz::plugin::ResponseCases::About(self.get(__field_Response__about{}).unwrap_unchecked()),
        2 => __::pz::plugin::ResponseCases::Codegen(self.get(__field_Response__codegen{}).unwrap_unchecked()),
        _ => __s::unreachable!(),
      }
    }
  }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_Response::Ref { *self }

  /// Serializes this [`__::pz::plugin::Response`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Response`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Response>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn about(self) -> __rt::reflect::Ref<'proto, __::pz::plugin::AboutResponse> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::plugin::AboutResponse>> {
    self.get(__field_Response__about{})
  }

  pub fn codegen(self) -> __rt::reflect::Ref<'proto, __::pz::plugin::CodegenResponse> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::plugin::CodegenResponse>> {
    self.get(__field_Response__codegen{})
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.about_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("about")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.codegen_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("codegen")?;
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

impl<'proto> __priv_Response::Mut<'proto>  {
  pub fn which(&self) -> i32 {
    self.as_ref().which()
  }

  pub fn cases(self) -> __::pz::plugin::ResponseCases<'proto> {
    self.into_ref().cases()
  }

  pub fn cases_mut(self) -> __::pz::plugin::ResponseCases<'proto, __r::SelectMut> {
    unsafe {
      match self.which() {
        0 => __::pz::plugin::ResponseCases::Unset,
        1 => __::pz::plugin::ResponseCases::About(self.get_mut(__field_Response__about{}).into_option().unwrap_unchecked()),
        2 => __::pz::plugin::ResponseCases::Codegen(self.get_mut(__field_Response__codegen{}).into_option().unwrap_unchecked()),
        _ => __s::unreachable!(),
      }
    }
  }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_Response::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_Response::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_Response::Mut {
    __priv_Response::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::Response`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::Response`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Response`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Response>,
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
    S: __r::Selector<__::pz::plugin::Response>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::Response`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn about(self) -> __rt::reflect::Ref<'proto, __::pz::plugin::AboutResponse> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::plugin::AboutResponse>> {
    self.get(__field_Response__about{})
  }
  pub fn about_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::plugin::AboutResponse> {
    self.about_mut_or().into_inner()
  }
  pub fn about_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::plugin::AboutResponse> {
    self.get_mut(__field_Response__about{})
  }
  pub fn set_about(mut self, value: impl __r::Set<__r::Opt<__::pz::plugin::AboutResponse>>) -> __r::Mut<'proto, __::pz::plugin::Response> {
    value.apply_to(self.as_mut().about_mut_or());
    self
  }

  pub fn codegen(self) -> __rt::reflect::Ref<'proto, __::pz::plugin::CodegenResponse> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::plugin::CodegenResponse>> {
    self.get(__field_Response__codegen{})
  }
  pub fn codegen_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::plugin::CodegenResponse> {
    self.codegen_mut_or().into_inner()
  }
  pub fn codegen_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::plugin::CodegenResponse> {
    self.get_mut(__field_Response__codegen{})
  }
  pub fn set_codegen(mut self, value: impl __r::Set<__r::Opt<__::pz::plugin::CodegenResponse>>) -> __r::Mut<'proto, __::pz::plugin::Response> {
    value.apply_to(self.as_mut().codegen_mut_or());
    self
  }

}

/// message `pz.plugin.AboutRequest`
pub struct AboutRequest {
  ptr: __s::ptr::NonNull<__priv_AboutRequest::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_AboutRequest::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_AboutRequest {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_AboutRequest::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::AboutRequest>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::AboutRequest>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 0],
  }
}

impl __::pz::plugin::AboutRequest {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_AboutRequest::Storage {
      __hasbits: [0; 0],
    }} as *const __priv_AboutRequest::Storage as *mut __priv_AboutRequest::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::AboutRequest`].
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

  /// Deserializes a new [`__::pz::plugin::AboutRequest`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::AboutRequest`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::AboutRequest`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::AboutRequest`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::AboutRequest>,
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
    S: __r::Selector<__::pz::plugin::AboutRequest>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::AboutRequest`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_AboutRequest::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_AboutRequest::Storage>().as_mut().__hasbits = [0; 0];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::AboutRequest>, src: __rt::reflect::Ref<__::pz::plugin::AboutRequest>) {
  }
}

impl __z::Message for __::pz::plugin::AboutRequest {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{0 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::AboutRequest::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
          ];
          DESCS.as_ptr()
        },

        num_hasbit_words: 0,
        kind: __z::tdp::DescKind::Message,
      },

      fields: [
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

impl __z::Type for __::pz::plugin::AboutRequest {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::AboutRequest::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::AboutRequest::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_AboutRequest::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_AboutRequest::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_AboutRequest::Mut {
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

impl __r::Views for __::pz::plugin::AboutRequest {
  type Ref<'a> = __priv_AboutRequest::Ref<'a>;
  type Mut<'a> = __priv_AboutRequest::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_AboutRequest::Ref<'a> {
  type Target = __::pz::plugin::AboutRequest;
  fn as_ref(&self) -> __priv_AboutRequest::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_AboutRequest::Mut<'a> {
  type Target = __::pz::plugin::AboutRequest;
  fn as_ref(&self) -> __priv_AboutRequest::Ref { self.r }
  fn into_ref(self) -> __priv_AboutRequest::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_AboutRequest::Mut {
    __priv_AboutRequest::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::AboutRequest {
  const DEFAULT: &'static Self = __::pz::plugin::AboutRequest::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_AboutRequest::Ref<'a> {
  type Message = __::pz::plugin::AboutRequest;
}
impl<'a> __r::MessageMut<'a> for __priv_AboutRequest::Mut<'a> {
  type Message = __::pz::plugin::AboutRequest;
}

impl __s::default::Default for __::pz::plugin::AboutRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::AboutRequest>> __s::convert::From<T> for __::pz::plugin::AboutRequest {
  fn from(value: T) -> __::pz::plugin::AboutRequest {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::AboutRequest> for &__::pz::plugin::AboutRequest {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::AboutRequest>) {
    __::pz::plugin::AboutRequest::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::AboutRequest>> for &__::pz::plugin::AboutRequest {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::AboutRequest>>) {
    __::pz::plugin::AboutRequest::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::AboutRequest> for __rt::reflect::Ref<'_, __::pz::plugin::AboutRequest> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::AboutRequest>) {
    __::pz::plugin::AboutRequest::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::AboutRequest>> for __rt::reflect::Ref<'_, __::pz::plugin::AboutRequest> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::AboutRequest>>) {
    __::pz::plugin::AboutRequest::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::AboutRequest> for &__rt::reflect::Mut<'_, __::pz::plugin::AboutRequest> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::AboutRequest>) {
    __::pz::plugin::AboutRequest::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::AboutRequest>> for &__rt::reflect::Mut<'_, __::pz::plugin::AboutRequest> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::AboutRequest>>) {
    __::pz::plugin::AboutRequest::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_AboutRequest::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::AboutRequest::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::AboutRequest {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_AboutRequest::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.AboutRequest ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_AboutRequest::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::AboutRequest {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl<'proto> __priv_AboutRequest::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_AboutRequest::Ref { *self }

  /// Serializes this [`__::pz::plugin::AboutRequest`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::AboutRequest`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::AboutRequest>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_AboutRequest::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_AboutRequest::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_AboutRequest::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_AboutRequest::Mut {
    __priv_AboutRequest::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::AboutRequest`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::AboutRequest`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::AboutRequest`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::AboutRequest>,
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
    S: __r::Selector<__::pz::plugin::AboutRequest>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::AboutRequest`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

}

/// message `pz.plugin.AboutResponse`
pub struct AboutResponse {
  ptr: __s::ptr::NonNull<__priv_AboutResponse::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_AboutResponse::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_AboutResponse {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_AboutResponse::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::AboutResponse>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::AboutResponse>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) version: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) options: __z::AVec<<__::pz::plugin::AboutResponse_Option as __z::Type>::__Storage<__z::Seal>>,
  }
}

impl __::pz::plugin::AboutResponse {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_AboutResponse::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      version: __z::RawStr::new(),
      options: __z::AVec::new(),
    }} as *const __priv_AboutResponse::Storage as *mut __priv_AboutResponse::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::AboutResponse`].
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

  /// Deserializes a new [`__::pz::plugin::AboutResponse`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::AboutResponse`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::AboutResponse`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::AboutResponse`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::AboutResponse>,
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
    S: __r::Selector<__::pz::plugin::AboutResponse>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::AboutResponse`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn name(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_AboutResponse__name{})
  }
  pub fn name_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_AboutResponse__name{})
  }
  pub fn set_name(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::AboutResponse> {
    value.apply_to(self.as_mut().name_mut_or());
    self.as_mut()
  }

  pub fn version(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_AboutResponse__version{})
  }
  pub fn version_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.version_mut_or().into_inner()
  }
  pub fn version_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_AboutResponse__version{})
  }
  pub fn set_version(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::AboutResponse> {
    value.apply_to(self.as_mut().version_mut_or());
    self.as_mut()
  }

  pub fn options(&self) -> __rt::Slice<'_, __::pz::plugin::AboutResponse_Option> {
    self.get(__field_AboutResponse__options{})
  }
  pub fn options_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::plugin::AboutResponse_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(&mut self) -> __rt::Repeated<'_, __::pz::plugin::AboutResponse_Option> {
    self.get_mut(__field_AboutResponse__options{})
  }
  pub fn set_options(&mut self, value: impl __r::Set<__r::Rep<__::pz::plugin::AboutResponse_Option>>) -> __r::Mut<'_, __::pz::plugin::AboutResponse> {
    value.apply_to(self.as_mut().options_mut());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_AboutResponse::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_AboutResponse::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::AboutResponse>, src: __rt::reflect::Ref<__::pz::plugin::AboutResponse>) {
    __r::Set::<<__::pz::plugin::AboutResponse as __r::Field<__field_AboutResponse__name>>::Type>::apply_to(src.get(__field_AboutResponse__name{}), dst.as_mut().get_mut(__field_AboutResponse__name{}));
    __r::Set::<<__::pz::plugin::AboutResponse as __r::Field<__field_AboutResponse__version>>::Type>::apply_to(src.get(__field_AboutResponse__version{}), dst.as_mut().get_mut(__field_AboutResponse__version{}));
    __r::Set::<<__::pz::plugin::AboutResponse as __r::Field<__field_AboutResponse__options>>::Type>::apply_to(src.get(__field_AboutResponse__options{}), dst.as_mut().get_mut(__field_AboutResponse__options{}));
  }
}

impl __z::Message for __::pz::plugin::AboutResponse {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{3 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::AboutResponse::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::plugin::AboutResponse_Option::__tdp_info,
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
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::AboutResponse::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::AboutResponse::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().version as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        __z::tdp::FieldStorage {
          number: 10,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::AboutResponse::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().options as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
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

impl __z::Type for __::pz::plugin::AboutResponse {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::AboutResponse::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::AboutResponse::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_AboutResponse::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_AboutResponse::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_AboutResponse::Mut {
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

impl __r::Views for __::pz::plugin::AboutResponse {
  type Ref<'a> = __priv_AboutResponse::Ref<'a>;
  type Mut<'a> = __priv_AboutResponse::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_AboutResponse::Ref<'a> {
  type Target = __::pz::plugin::AboutResponse;
  fn as_ref(&self) -> __priv_AboutResponse::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_AboutResponse::Mut<'a> {
  type Target = __::pz::plugin::AboutResponse;
  fn as_ref(&self) -> __priv_AboutResponse::Ref { self.r }
  fn into_ref(self) -> __priv_AboutResponse::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_AboutResponse::Mut {
    __priv_AboutResponse::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::AboutResponse {
  const DEFAULT: &'static Self = __::pz::plugin::AboutResponse::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_AboutResponse::Ref<'a> {
  type Message = __::pz::plugin::AboutResponse;
}
impl<'a> __r::MessageMut<'a> for __priv_AboutResponse::Mut<'a> {
  type Message = __::pz::plugin::AboutResponse;
}

impl __s::default::Default for __::pz::plugin::AboutResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::AboutResponse>> __s::convert::From<T> for __::pz::plugin::AboutResponse {
  fn from(value: T) -> __::pz::plugin::AboutResponse {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::AboutResponse> for &__::pz::plugin::AboutResponse {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::AboutResponse>) {
    __::pz::plugin::AboutResponse::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::AboutResponse>> for &__::pz::plugin::AboutResponse {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::AboutResponse>>) {
    __::pz::plugin::AboutResponse::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::AboutResponse> for __rt::reflect::Ref<'_, __::pz::plugin::AboutResponse> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::AboutResponse>) {
    __::pz::plugin::AboutResponse::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::AboutResponse>> for __rt::reflect::Ref<'_, __::pz::plugin::AboutResponse> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::AboutResponse>>) {
    __::pz::plugin::AboutResponse::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::AboutResponse> for &__rt::reflect::Mut<'_, __::pz::plugin::AboutResponse> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::AboutResponse>) {
    __::pz::plugin::AboutResponse::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::AboutResponse>> for &__rt::reflect::Mut<'_, __::pz::plugin::AboutResponse> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::AboutResponse>>) {
    __::pz::plugin::AboutResponse::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_AboutResponse::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::AboutResponse::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::AboutResponse {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_AboutResponse::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.AboutResponse ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_AboutResponse::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::AboutResponse {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_AboutResponse__name = __rt::field!(name);
impl __r::Field<__field_AboutResponse__name> for __::pz::plugin::AboutResponse {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_AboutResponse__name;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "name";
}

type __field_AboutResponse__version = __rt::field!(version);
impl __r::Field<__field_AboutResponse__version> for __::pz::plugin::AboutResponse {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_AboutResponse__version;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "version";
}

type __field_AboutResponse__options = __rt::field!(options);
impl __r::Field<__field_AboutResponse__options> for __::pz::plugin::AboutResponse {
  type Type = __r::Rep<__::pz::plugin::AboutResponse_Option>;
  type Name = __field_AboutResponse__options;
  const NUMBER: __s::primitive::i32 = 10;
  const INDEX: __s::primitive::usize = 2;
  const NAME: &'static __s::primitive::str = "options";
}

impl<'proto> __priv_AboutResponse::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_AboutResponse::Ref { *self }

  /// Serializes this [`__::pz::plugin::AboutResponse`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::AboutResponse`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::AboutResponse>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_AboutResponse__name{})
  }

  pub fn version(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_AboutResponse__version{})
  }

  pub fn options(self) -> __rt::Slice<'proto, __::pz::plugin::AboutResponse_Option> {
    self.get(__field_AboutResponse__options{})
  }
  pub fn options_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::plugin::AboutResponse_Option> {
    self.options().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.version_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("version")?;
      debug.write_debug(value);
      count += 1;
    }
    for value in self.options() {
      if count != 0 { debug.comma(false)?; }
      debug.field("options")?;
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

impl<'proto> __priv_AboutResponse::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_AboutResponse::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_AboutResponse::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_AboutResponse::Mut {
    __priv_AboutResponse::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::AboutResponse`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::AboutResponse`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::AboutResponse`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::AboutResponse>,
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
    S: __r::Selector<__::pz::plugin::AboutResponse>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::AboutResponse`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_AboutResponse__name{})
  }
  pub fn name_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_AboutResponse__name{})
  }
  pub fn set_name(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::AboutResponse> {
    value.apply_to(self.as_mut().name_mut_or());
    self
  }

  pub fn version(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_AboutResponse__version{})
  }
  pub fn version_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.version_mut_or().into_inner()
  }
  pub fn version_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_AboutResponse__version{})
  }
  pub fn set_version(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::AboutResponse> {
    value.apply_to(self.as_mut().version_mut_or());
    self
  }

  pub fn options(self) -> __rt::Slice<'proto, __::pz::plugin::AboutResponse_Option> {
    self.get(__field_AboutResponse__options{})
  }
  pub fn options_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::plugin::AboutResponse_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(mut self) -> __rt::Repeated<'proto, __::pz::plugin::AboutResponse_Option> {
    self.get_mut(__field_AboutResponse__options{})
  }
  pub fn set_options(mut self, value: impl __r::Set<__r::Rep<__::pz::plugin::AboutResponse_Option>>) -> __r::Mut<'proto, __::pz::plugin::AboutResponse> {
    value.apply_to(self.as_mut().options_mut());
    self
  }

}

/// message `pz.plugin.AboutResponse.Option`
pub struct AboutResponse_Option {
  ptr: __s::ptr::NonNull<__priv_AboutResponse_Option::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_AboutResponse_Option::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_AboutResponse_Option {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_AboutResponse_Option::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::AboutResponse_Option>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::AboutResponse_Option>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) help: <__rt::String as __z::Type>::__Storage<__z::Seal>,
  }
}

impl __::pz::plugin::AboutResponse_Option {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_AboutResponse_Option::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      help: __z::RawStr::new(),
    }} as *const __priv_AboutResponse_Option::Storage as *mut __priv_AboutResponse_Option::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::AboutResponse_Option`].
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

  /// Deserializes a new [`__::pz::plugin::AboutResponse_Option`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::AboutResponse_Option`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::AboutResponse_Option`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::AboutResponse_Option`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::AboutResponse_Option>,
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
    S: __r::Selector<__::pz::plugin::AboutResponse_Option>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::AboutResponse_Option`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn name(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_AboutResponse_Option__name{})
  }
  pub fn name_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_AboutResponse_Option__name{})
  }
  pub fn set_name(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::AboutResponse_Option> {
    value.apply_to(self.as_mut().name_mut_or());
    self.as_mut()
  }

  pub fn help(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_AboutResponse_Option__help{})
  }
  pub fn help_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.help_mut_or().into_inner()
  }
  pub fn help_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_AboutResponse_Option__help{})
  }
  pub fn set_help(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::AboutResponse_Option> {
    value.apply_to(self.as_mut().help_mut_or());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_AboutResponse_Option::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_AboutResponse_Option::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::AboutResponse_Option>, src: __rt::reflect::Ref<__::pz::plugin::AboutResponse_Option>) {
    __r::Set::<<__::pz::plugin::AboutResponse_Option as __r::Field<__field_AboutResponse_Option__name>>::Type>::apply_to(src.get(__field_AboutResponse_Option__name{}), dst.as_mut().get_mut(__field_AboutResponse_Option__name{}));
    __r::Set::<<__::pz::plugin::AboutResponse_Option as __r::Field<__field_AboutResponse_Option__help>>::Type>::apply_to(src.get(__field_AboutResponse_Option__help{}), dst.as_mut().get_mut(__field_AboutResponse_Option__help{}));
  }
}

impl __z::Message for __::pz::plugin::AboutResponse_Option {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{2 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::AboutResponse_Option::__LAYOUT.size();
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
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::AboutResponse_Option::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::AboutResponse_Option::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().help as *const _ as *const u8;
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

impl __z::Type for __::pz::plugin::AboutResponse_Option {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::AboutResponse_Option::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::AboutResponse_Option::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_AboutResponse_Option::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_AboutResponse_Option::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_AboutResponse_Option::Mut {
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

impl __r::Views for __::pz::plugin::AboutResponse_Option {
  type Ref<'a> = __priv_AboutResponse_Option::Ref<'a>;
  type Mut<'a> = __priv_AboutResponse_Option::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_AboutResponse_Option::Ref<'a> {
  type Target = __::pz::plugin::AboutResponse_Option;
  fn as_ref(&self) -> __priv_AboutResponse_Option::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_AboutResponse_Option::Mut<'a> {
  type Target = __::pz::plugin::AboutResponse_Option;
  fn as_ref(&self) -> __priv_AboutResponse_Option::Ref { self.r }
  fn into_ref(self) -> __priv_AboutResponse_Option::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_AboutResponse_Option::Mut {
    __priv_AboutResponse_Option::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::AboutResponse_Option {
  const DEFAULT: &'static Self = __::pz::plugin::AboutResponse_Option::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_AboutResponse_Option::Ref<'a> {
  type Message = __::pz::plugin::AboutResponse_Option;
}
impl<'a> __r::MessageMut<'a> for __priv_AboutResponse_Option::Mut<'a> {
  type Message = __::pz::plugin::AboutResponse_Option;
}

impl __s::default::Default for __::pz::plugin::AboutResponse_Option {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::AboutResponse_Option>> __s::convert::From<T> for __::pz::plugin::AboutResponse_Option {
  fn from(value: T) -> __::pz::plugin::AboutResponse_Option {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::AboutResponse_Option> for &__::pz::plugin::AboutResponse_Option {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::AboutResponse_Option>) {
    __::pz::plugin::AboutResponse_Option::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::AboutResponse_Option>> for &__::pz::plugin::AboutResponse_Option {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::AboutResponse_Option>>) {
    __::pz::plugin::AboutResponse_Option::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::AboutResponse_Option> for __rt::reflect::Ref<'_, __::pz::plugin::AboutResponse_Option> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::AboutResponse_Option>) {
    __::pz::plugin::AboutResponse_Option::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::AboutResponse_Option>> for __rt::reflect::Ref<'_, __::pz::plugin::AboutResponse_Option> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::AboutResponse_Option>>) {
    __::pz::plugin::AboutResponse_Option::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::AboutResponse_Option> for &__rt::reflect::Mut<'_, __::pz::plugin::AboutResponse_Option> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::AboutResponse_Option>) {
    __::pz::plugin::AboutResponse_Option::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::AboutResponse_Option>> for &__rt::reflect::Mut<'_, __::pz::plugin::AboutResponse_Option> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::AboutResponse_Option>>) {
    __::pz::plugin::AboutResponse_Option::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_AboutResponse_Option::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::AboutResponse_Option::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::AboutResponse_Option {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_AboutResponse_Option::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.AboutResponse.Option ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_AboutResponse_Option::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::AboutResponse_Option {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_AboutResponse_Option__name = __rt::field!(name);
impl __r::Field<__field_AboutResponse_Option__name> for __::pz::plugin::AboutResponse_Option {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_AboutResponse_Option__name;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "name";
}

type __field_AboutResponse_Option__help = __rt::field!(help);
impl __r::Field<__field_AboutResponse_Option__help> for __::pz::plugin::AboutResponse_Option {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_AboutResponse_Option__help;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "help";
}

impl<'proto> __priv_AboutResponse_Option::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_AboutResponse_Option::Ref { *self }

  /// Serializes this [`__::pz::plugin::AboutResponse_Option`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::AboutResponse_Option`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::AboutResponse_Option>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_AboutResponse_Option__name{})
  }

  pub fn help(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_AboutResponse_Option__help{})
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.help_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("help")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_AboutResponse_Option::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_AboutResponse_Option::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_AboutResponse_Option::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_AboutResponse_Option::Mut {
    __priv_AboutResponse_Option::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::AboutResponse_Option`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::AboutResponse_Option`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::AboutResponse_Option`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::AboutResponse_Option>,
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
    S: __r::Selector<__::pz::plugin::AboutResponse_Option>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::AboutResponse_Option`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_AboutResponse_Option__name{})
  }
  pub fn name_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_AboutResponse_Option__name{})
  }
  pub fn set_name(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::AboutResponse_Option> {
    value.apply_to(self.as_mut().name_mut_or());
    self
  }

  pub fn help(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_AboutResponse_Option__help{})
  }
  pub fn help_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.help_mut_or().into_inner()
  }
  pub fn help_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_AboutResponse_Option__help{})
  }
  pub fn set_help(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::AboutResponse_Option> {
    value.apply_to(self.as_mut().help_mut_or());
    self
  }

}

/// message `pz.plugin.CodegenRequest`
pub struct CodegenRequest {
  ptr: __s::ptr::NonNull<__priv_CodegenRequest::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_CodegenRequest::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_CodegenRequest {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_CodegenRequest::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::CodegenRequest>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::CodegenRequest>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) bundle: <__::pz::Bundle as __z::Type>::__Storage<__z::Seal>,
    pub(in super) requested_indices: __z::AVec<<__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) options: __z::AVec<<__::pz::plugin::CodegenRequest_Option as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) debug: <__s::primitive::bool as __z::Type>::__Storage<__z::Seal>,
  }
}

impl __::pz::plugin::CodegenRequest {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_CodegenRequest::Storage {
      __hasbits: [0; 1],
      bundle: __s::option::Option::None,
      requested_indices: __z::AVec::new(),
      options: __z::AVec::new(),
      debug: false,
    }} as *const __priv_CodegenRequest::Storage as *mut __priv_CodegenRequest::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::CodegenRequest`].
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

  /// Deserializes a new [`__::pz::plugin::CodegenRequest`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::CodegenRequest`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::CodegenRequest`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenRequest`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenRequest>,
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
    S: __r::Selector<__::pz::plugin::CodegenRequest>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::CodegenRequest`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn bundle(&self) -> __rt::reflect::Ref<'_, __::pz::Bundle> {
    self.bundle_or().unwrap_or_default()
  }
  pub fn bundle_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::Bundle>> {
    self.get(__field_CodegenRequest__bundle{})
  }
  pub fn bundle_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::Bundle> {
    self.bundle_mut_or().into_inner()
  }
  pub fn bundle_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::Bundle> {
    self.get_mut(__field_CodegenRequest__bundle{})
  }
  pub fn set_bundle(&mut self, value: impl __r::Set<__r::Opt<__::pz::Bundle>>) -> __r::Mut<'_, __::pz::plugin::CodegenRequest> {
    value.apply_to(self.as_mut().bundle_mut_or());
    self.as_mut()
  }

  pub fn requested_indices(&self) -> __rt::Slice<'_, __s::primitive::u32> {
    self.get(__field_CodegenRequest__requested_indices{})
  }
  pub fn requested_indices_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.requested_indices().at(idx)
  }
  pub fn requested_indices_mut(&mut self) -> __rt::Repeated<'_, __s::primitive::u32> {
    self.get_mut(__field_CodegenRequest__requested_indices{})
  }
  pub fn set_requested_indices(&mut self, value: impl __r::Set<__r::Rep<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::plugin::CodegenRequest> {
    value.apply_to(self.as_mut().requested_indices_mut());
    self.as_mut()
  }

  pub fn options(&self) -> __rt::Slice<'_, __::pz::plugin::CodegenRequest_Option> {
    self.get(__field_CodegenRequest__options{})
  }
  pub fn options_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::plugin::CodegenRequest_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(&mut self) -> __rt::Repeated<'_, __::pz::plugin::CodegenRequest_Option> {
    self.get_mut(__field_CodegenRequest__options{})
  }
  pub fn set_options(&mut self, value: impl __r::Set<__r::Rep<__::pz::plugin::CodegenRequest_Option>>) -> __r::Mut<'_, __::pz::plugin::CodegenRequest> {
    value.apply_to(self.as_mut().options_mut());
    self.as_mut()
  }

  pub fn debug(&self) -> __rt::reflect::Ref<'_, __s::primitive::bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::bool>> {
    self.get(__field_CodegenRequest__debug{})
  }
  pub fn debug_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::bool> {
    self.debug_mut_or().into_inner()
  }
  pub fn debug_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::bool> {
    self.get_mut(__field_CodegenRequest__debug{})
  }
  pub fn set_debug(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::bool>>) -> __r::Mut<'_, __::pz::plugin::CodegenRequest> {
    value.apply_to(self.as_mut().debug_mut_or());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_CodegenRequest::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_CodegenRequest::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::CodegenRequest>, src: __rt::reflect::Ref<__::pz::plugin::CodegenRequest>) {
    __r::Set::<<__::pz::plugin::CodegenRequest as __r::Field<__field_CodegenRequest__bundle>>::Type>::apply_to(src.get(__field_CodegenRequest__bundle{}), dst.as_mut().get_mut(__field_CodegenRequest__bundle{}));
    __r::Set::<<__::pz::plugin::CodegenRequest as __r::Field<__field_CodegenRequest__requested_indices>>::Type>::apply_to(src.get(__field_CodegenRequest__requested_indices{}), dst.as_mut().get_mut(__field_CodegenRequest__requested_indices{}));
    __r::Set::<<__::pz::plugin::CodegenRequest as __r::Field<__field_CodegenRequest__options>>::Type>::apply_to(src.get(__field_CodegenRequest__options{}), dst.as_mut().get_mut(__field_CodegenRequest__options{}));
    __r::Set::<<__::pz::plugin::CodegenRequest as __r::Field<__field_CodegenRequest__debug>>::Type>::apply_to(src.get(__field_CodegenRequest__debug{}), dst.as_mut().get_mut(__field_CodegenRequest__debug{}));
  }
}

impl __z::Message for __::pz::plugin::CodegenRequest {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{4 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::CodegenRequest::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::Bundle::__tdp_info,
            __::pz::plugin::CodegenRequest_Option::__tdp_info,
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
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::CodegenRequest::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().bundle as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::I32.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::CodegenRequest::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().requested_indices as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        __z::tdp::FieldStorage {
          number: 3,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::CodegenRequest::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().options as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 1,
          hasbit: 1,
        },
        __z::tdp::FieldStorage {
          number: 4,
          flags:
            __z::tdp::Kind::Bool.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::CodegenRequest::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().debug as *const _ as *const u8;
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

impl __z::Type for __::pz::plugin::CodegenRequest {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::CodegenRequest::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::CodegenRequest::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_CodegenRequest::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_CodegenRequest::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_CodegenRequest::Mut {
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

impl __r::Views for __::pz::plugin::CodegenRequest {
  type Ref<'a> = __priv_CodegenRequest::Ref<'a>;
  type Mut<'a> = __priv_CodegenRequest::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_CodegenRequest::Ref<'a> {
  type Target = __::pz::plugin::CodegenRequest;
  fn as_ref(&self) -> __priv_CodegenRequest::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_CodegenRequest::Mut<'a> {
  type Target = __::pz::plugin::CodegenRequest;
  fn as_ref(&self) -> __priv_CodegenRequest::Ref { self.r }
  fn into_ref(self) -> __priv_CodegenRequest::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_CodegenRequest::Mut {
    __priv_CodegenRequest::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::CodegenRequest {
  const DEFAULT: &'static Self = __::pz::plugin::CodegenRequest::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_CodegenRequest::Ref<'a> {
  type Message = __::pz::plugin::CodegenRequest;
}
impl<'a> __r::MessageMut<'a> for __priv_CodegenRequest::Mut<'a> {
  type Message = __::pz::plugin::CodegenRequest;
}

impl __s::default::Default for __::pz::plugin::CodegenRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::CodegenRequest>> __s::convert::From<T> for __::pz::plugin::CodegenRequest {
  fn from(value: T) -> __::pz::plugin::CodegenRequest {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::CodegenRequest> for &__::pz::plugin::CodegenRequest {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenRequest>) {
    __::pz::plugin::CodegenRequest::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenRequest>> for &__::pz::plugin::CodegenRequest {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenRequest>>) {
    __::pz::plugin::CodegenRequest::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::CodegenRequest> for __rt::reflect::Ref<'_, __::pz::plugin::CodegenRequest> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenRequest>) {
    __::pz::plugin::CodegenRequest::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenRequest>> for __rt::reflect::Ref<'_, __::pz::plugin::CodegenRequest> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenRequest>>) {
    __::pz::plugin::CodegenRequest::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::CodegenRequest> for &__rt::reflect::Mut<'_, __::pz::plugin::CodegenRequest> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenRequest>) {
    __::pz::plugin::CodegenRequest::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenRequest>> for &__rt::reflect::Mut<'_, __::pz::plugin::CodegenRequest> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenRequest>>) {
    __::pz::plugin::CodegenRequest::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_CodegenRequest::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::CodegenRequest::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::CodegenRequest {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_CodegenRequest::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.CodegenRequest ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_CodegenRequest::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::CodegenRequest {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_CodegenRequest__bundle = __rt::field!(bundle);
impl __r::Field<__field_CodegenRequest__bundle> for __::pz::plugin::CodegenRequest {
  type Type = __r::Opt<__::pz::Bundle>;
  type Name = __field_CodegenRequest__bundle;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "bundle";
}

type __field_CodegenRequest__requested_indices = __rt::field!(requested_indices);
impl __r::Field<__field_CodegenRequest__requested_indices> for __::pz::plugin::CodegenRequest {
  type Type = __r::Rep<__s::primitive::u32>;
  type Name = __field_CodegenRequest__requested_indices;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "requested_indices";
}

type __field_CodegenRequest__options = __rt::field!(options);
impl __r::Field<__field_CodegenRequest__options> for __::pz::plugin::CodegenRequest {
  type Type = __r::Rep<__::pz::plugin::CodegenRequest_Option>;
  type Name = __field_CodegenRequest__options;
  const NUMBER: __s::primitive::i32 = 3;
  const INDEX: __s::primitive::usize = 2;
  const NAME: &'static __s::primitive::str = "options";
}

type __field_CodegenRequest__debug = __rt::field!(debug);
impl __r::Field<__field_CodegenRequest__debug> for __::pz::plugin::CodegenRequest {
  type Type = __r::Opt<__s::primitive::bool>;
  type Name = __field_CodegenRequest__debug;
  const NUMBER: __s::primitive::i32 = 4;
  const INDEX: __s::primitive::usize = 3;
  const NAME: &'static __s::primitive::str = "debug";
}

impl<'proto> __priv_CodegenRequest::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_CodegenRequest::Ref { *self }

  /// Serializes this [`__::pz::plugin::CodegenRequest`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenRequest`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenRequest>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn bundle(self) -> __rt::reflect::Ref<'proto, __::pz::Bundle> {
    self.bundle_or().unwrap_or_default()
  }
  pub fn bundle_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::Bundle>> {
    self.get(__field_CodegenRequest__bundle{})
  }

  pub fn requested_indices(self) -> __rt::Slice<'proto, __s::primitive::u32> {
    self.get(__field_CodegenRequest__requested_indices{})
  }
  pub fn requested_indices_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.requested_indices().at(idx)
  }

  pub fn options(self) -> __rt::Slice<'proto, __::pz::plugin::CodegenRequest_Option> {
    self.get(__field_CodegenRequest__options{})
  }
  pub fn options_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::plugin::CodegenRequest_Option> {
    self.options().at(idx)
  }

  pub fn debug(self) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::bool>> {
    self.get(__field_CodegenRequest__debug{})
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.bundle_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("bundle")?;
      value.__debug(debug)?;
      count += 1;
    }
    if !self.requested_indices().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("requested_indices")?;
      debug.iter(self.requested_indices())?;
      count += 1;
    }
    for value in self.options() {
      if count != 0 { debug.comma(false)?; }
      debug.field("options")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.debug_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("debug")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_CodegenRequest::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_CodegenRequest::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_CodegenRequest::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_CodegenRequest::Mut {
    __priv_CodegenRequest::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::CodegenRequest`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::CodegenRequest`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenRequest`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenRequest>,
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
    S: __r::Selector<__::pz::plugin::CodegenRequest>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::CodegenRequest`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn bundle(self) -> __rt::reflect::Ref<'proto, __::pz::Bundle> {
    self.bundle_or().unwrap_or_default()
  }
  pub fn bundle_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::Bundle>> {
    self.get(__field_CodegenRequest__bundle{})
  }
  pub fn bundle_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::Bundle> {
    self.bundle_mut_or().into_inner()
  }
  pub fn bundle_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::Bundle> {
    self.get_mut(__field_CodegenRequest__bundle{})
  }
  pub fn set_bundle(mut self, value: impl __r::Set<__r::Opt<__::pz::Bundle>>) -> __r::Mut<'proto, __::pz::plugin::CodegenRequest> {
    value.apply_to(self.as_mut().bundle_mut_or());
    self
  }

  pub fn requested_indices(self) -> __rt::Slice<'proto, __s::primitive::u32> {
    self.get(__field_CodegenRequest__requested_indices{})
  }
  pub fn requested_indices_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.requested_indices().at(idx)
  }
  pub fn requested_indices_mut(mut self) -> __rt::Repeated<'proto, __s::primitive::u32> {
    self.get_mut(__field_CodegenRequest__requested_indices{})
  }
  pub fn set_requested_indices(mut self, value: impl __r::Set<__r::Rep<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::plugin::CodegenRequest> {
    value.apply_to(self.as_mut().requested_indices_mut());
    self
  }

  pub fn options(self) -> __rt::Slice<'proto, __::pz::plugin::CodegenRequest_Option> {
    self.get(__field_CodegenRequest__options{})
  }
  pub fn options_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::plugin::CodegenRequest_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(mut self) -> __rt::Repeated<'proto, __::pz::plugin::CodegenRequest_Option> {
    self.get_mut(__field_CodegenRequest__options{})
  }
  pub fn set_options(mut self, value: impl __r::Set<__r::Rep<__::pz::plugin::CodegenRequest_Option>>) -> __r::Mut<'proto, __::pz::plugin::CodegenRequest> {
    value.apply_to(self.as_mut().options_mut());
    self
  }

  pub fn debug(self) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::bool>> {
    self.get(__field_CodegenRequest__debug{})
  }
  pub fn debug_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::bool> {
    self.debug_mut_or().into_inner()
  }
  pub fn debug_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::bool> {
    self.get_mut(__field_CodegenRequest__debug{})
  }
  pub fn set_debug(mut self, value: impl __r::Set<__r::Opt<__s::primitive::bool>>) -> __r::Mut<'proto, __::pz::plugin::CodegenRequest> {
    value.apply_to(self.as_mut().debug_mut_or());
    self
  }

}

/// message `pz.plugin.CodegenRequest.Option`
pub struct CodegenRequest_Option {
  ptr: __s::ptr::NonNull<__priv_CodegenRequest_Option::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_CodegenRequest_Option::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_CodegenRequest_Option {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_CodegenRequest_Option::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::CodegenRequest_Option>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::CodegenRequest_Option>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) value: <__rt::String as __z::Type>::__Storage<__z::Seal>,
  }
}

impl __::pz::plugin::CodegenRequest_Option {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_CodegenRequest_Option::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      value: __z::RawStr::new(),
    }} as *const __priv_CodegenRequest_Option::Storage as *mut __priv_CodegenRequest_Option::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::CodegenRequest_Option`].
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

  /// Deserializes a new [`__::pz::plugin::CodegenRequest_Option`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::CodegenRequest_Option`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::CodegenRequest_Option`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenRequest_Option`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenRequest_Option>,
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
    S: __r::Selector<__::pz::plugin::CodegenRequest_Option>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::CodegenRequest_Option`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn name(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_CodegenRequest_Option__name{})
  }
  pub fn name_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_CodegenRequest_Option__name{})
  }
  pub fn set_name(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::CodegenRequest_Option> {
    value.apply_to(self.as_mut().name_mut_or());
    self.as_mut()
  }

  pub fn value(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_CodegenRequest_Option__value{})
  }
  pub fn value_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.value_mut_or().into_inner()
  }
  pub fn value_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_CodegenRequest_Option__value{})
  }
  pub fn set_value(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::CodegenRequest_Option> {
    value.apply_to(self.as_mut().value_mut_or());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_CodegenRequest_Option::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_CodegenRequest_Option::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::CodegenRequest_Option>, src: __rt::reflect::Ref<__::pz::plugin::CodegenRequest_Option>) {
    __r::Set::<<__::pz::plugin::CodegenRequest_Option as __r::Field<__field_CodegenRequest_Option__name>>::Type>::apply_to(src.get(__field_CodegenRequest_Option__name{}), dst.as_mut().get_mut(__field_CodegenRequest_Option__name{}));
    __r::Set::<<__::pz::plugin::CodegenRequest_Option as __r::Field<__field_CodegenRequest_Option__value>>::Type>::apply_to(src.get(__field_CodegenRequest_Option__value{}), dst.as_mut().get_mut(__field_CodegenRequest_Option__value{}));
  }
}

impl __z::Message for __::pz::plugin::CodegenRequest_Option {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{2 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::CodegenRequest_Option::__LAYOUT.size();
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
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::CodegenRequest_Option::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::CodegenRequest_Option::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().value as *const _ as *const u8;
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

impl __z::Type for __::pz::plugin::CodegenRequest_Option {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::CodegenRequest_Option::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::CodegenRequest_Option::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_CodegenRequest_Option::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_CodegenRequest_Option::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_CodegenRequest_Option::Mut {
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

impl __r::Views for __::pz::plugin::CodegenRequest_Option {
  type Ref<'a> = __priv_CodegenRequest_Option::Ref<'a>;
  type Mut<'a> = __priv_CodegenRequest_Option::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_CodegenRequest_Option::Ref<'a> {
  type Target = __::pz::plugin::CodegenRequest_Option;
  fn as_ref(&self) -> __priv_CodegenRequest_Option::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_CodegenRequest_Option::Mut<'a> {
  type Target = __::pz::plugin::CodegenRequest_Option;
  fn as_ref(&self) -> __priv_CodegenRequest_Option::Ref { self.r }
  fn into_ref(self) -> __priv_CodegenRequest_Option::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_CodegenRequest_Option::Mut {
    __priv_CodegenRequest_Option::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::CodegenRequest_Option {
  const DEFAULT: &'static Self = __::pz::plugin::CodegenRequest_Option::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_CodegenRequest_Option::Ref<'a> {
  type Message = __::pz::plugin::CodegenRequest_Option;
}
impl<'a> __r::MessageMut<'a> for __priv_CodegenRequest_Option::Mut<'a> {
  type Message = __::pz::plugin::CodegenRequest_Option;
}

impl __s::default::Default for __::pz::plugin::CodegenRequest_Option {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::CodegenRequest_Option>> __s::convert::From<T> for __::pz::plugin::CodegenRequest_Option {
  fn from(value: T) -> __::pz::plugin::CodegenRequest_Option {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::CodegenRequest_Option> for &__::pz::plugin::CodegenRequest_Option {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenRequest_Option>) {
    __::pz::plugin::CodegenRequest_Option::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenRequest_Option>> for &__::pz::plugin::CodegenRequest_Option {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenRequest_Option>>) {
    __::pz::plugin::CodegenRequest_Option::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::CodegenRequest_Option> for __rt::reflect::Ref<'_, __::pz::plugin::CodegenRequest_Option> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenRequest_Option>) {
    __::pz::plugin::CodegenRequest_Option::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenRequest_Option>> for __rt::reflect::Ref<'_, __::pz::plugin::CodegenRequest_Option> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenRequest_Option>>) {
    __::pz::plugin::CodegenRequest_Option::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::CodegenRequest_Option> for &__rt::reflect::Mut<'_, __::pz::plugin::CodegenRequest_Option> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenRequest_Option>) {
    __::pz::plugin::CodegenRequest_Option::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenRequest_Option>> for &__rt::reflect::Mut<'_, __::pz::plugin::CodegenRequest_Option> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenRequest_Option>>) {
    __::pz::plugin::CodegenRequest_Option::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_CodegenRequest_Option::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::CodegenRequest_Option::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::CodegenRequest_Option {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_CodegenRequest_Option::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.CodegenRequest.Option ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_CodegenRequest_Option::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::CodegenRequest_Option {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_CodegenRequest_Option__name = __rt::field!(name);
impl __r::Field<__field_CodegenRequest_Option__name> for __::pz::plugin::CodegenRequest_Option {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_CodegenRequest_Option__name;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "name";
}

type __field_CodegenRequest_Option__value = __rt::field!(value);
impl __r::Field<__field_CodegenRequest_Option__value> for __::pz::plugin::CodegenRequest_Option {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_CodegenRequest_Option__value;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "value";
}

impl<'proto> __priv_CodegenRequest_Option::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_CodegenRequest_Option::Ref { *self }

  /// Serializes this [`__::pz::plugin::CodegenRequest_Option`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenRequest_Option`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenRequest_Option>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_CodegenRequest_Option__name{})
  }

  pub fn value(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_CodegenRequest_Option__value{})
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.value_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("value")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_CodegenRequest_Option::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_CodegenRequest_Option::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_CodegenRequest_Option::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_CodegenRequest_Option::Mut {
    __priv_CodegenRequest_Option::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::CodegenRequest_Option`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::CodegenRequest_Option`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenRequest_Option`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenRequest_Option>,
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
    S: __r::Selector<__::pz::plugin::CodegenRequest_Option>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::CodegenRequest_Option`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn name(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_CodegenRequest_Option__name{})
  }
  pub fn name_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.name_mut_or().into_inner()
  }
  pub fn name_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_CodegenRequest_Option__name{})
  }
  pub fn set_name(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::CodegenRequest_Option> {
    value.apply_to(self.as_mut().name_mut_or());
    self
  }

  pub fn value(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_CodegenRequest_Option__value{})
  }
  pub fn value_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.value_mut_or().into_inner()
  }
  pub fn value_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_CodegenRequest_Option__value{})
  }
  pub fn set_value(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::CodegenRequest_Option> {
    value.apply_to(self.as_mut().value_mut_or());
    self
  }

}

/// message `pz.plugin.CodegenResponse`
pub struct CodegenResponse {
  ptr: __s::ptr::NonNull<__priv_CodegenResponse::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_CodegenResponse::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_CodegenResponse {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_CodegenResponse::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::CodegenResponse>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::CodegenResponse>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 0],
    pub(in super) files: __z::AVec<<__::pz::plugin::CodegenResponse_File as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) report: __z::AVec<<__::pz::plugin::Diagnostic as __z::Type>::__Storage<__z::Seal>>,
  }
}

impl __::pz::plugin::CodegenResponse {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_CodegenResponse::Storage {
      __hasbits: [0; 0],
      files: __z::AVec::new(),
      report: __z::AVec::new(),
    }} as *const __priv_CodegenResponse::Storage as *mut __priv_CodegenResponse::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::CodegenResponse`].
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

  /// Deserializes a new [`__::pz::plugin::CodegenResponse`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::CodegenResponse`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::CodegenResponse`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenResponse`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenResponse>,
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
    S: __r::Selector<__::pz::plugin::CodegenResponse>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::CodegenResponse`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn files(&self) -> __rt::Slice<'_, __::pz::plugin::CodegenResponse_File> {
    self.get(__field_CodegenResponse__files{})
  }
  pub fn files_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::plugin::CodegenResponse_File> {
    self.files().at(idx)
  }
  pub fn files_mut(&mut self) -> __rt::Repeated<'_, __::pz::plugin::CodegenResponse_File> {
    self.get_mut(__field_CodegenResponse__files{})
  }
  pub fn set_files(&mut self, value: impl __r::Set<__r::Rep<__::pz::plugin::CodegenResponse_File>>) -> __r::Mut<'_, __::pz::plugin::CodegenResponse> {
    value.apply_to(self.as_mut().files_mut());
    self.as_mut()
  }

  pub fn report(&self) -> __rt::Slice<'_, __::pz::plugin::Diagnostic> {
    self.get(__field_CodegenResponse__report{})
  }
  pub fn report_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::plugin::Diagnostic> {
    self.report().at(idx)
  }
  pub fn report_mut(&mut self) -> __rt::Repeated<'_, __::pz::plugin::Diagnostic> {
    self.get_mut(__field_CodegenResponse__report{})
  }
  pub fn set_report(&mut self, value: impl __r::Set<__r::Rep<__::pz::plugin::Diagnostic>>) -> __r::Mut<'_, __::pz::plugin::CodegenResponse> {
    value.apply_to(self.as_mut().report_mut());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_CodegenResponse::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_CodegenResponse::Storage>().as_mut().__hasbits = [0; 0];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::CodegenResponse>, src: __rt::reflect::Ref<__::pz::plugin::CodegenResponse>) {
    __r::Set::<<__::pz::plugin::CodegenResponse as __r::Field<__field_CodegenResponse__files>>::Type>::apply_to(src.get(__field_CodegenResponse__files{}), dst.as_mut().get_mut(__field_CodegenResponse__files{}));
    __r::Set::<<__::pz::plugin::CodegenResponse as __r::Field<__field_CodegenResponse__report>>::Type>::apply_to(src.get(__field_CodegenResponse__report{}), dst.as_mut().get_mut(__field_CodegenResponse__report{}));
  }
}

impl __z::Message for __::pz::plugin::CodegenResponse {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{2 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::CodegenResponse::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::plugin::CodegenResponse_File::__tdp_info,
            __::pz::plugin::Diagnostic::__tdp_info,
          ];
          DESCS.as_ptr()
        },

        num_hasbit_words: 0,
        kind: __z::tdp::DescKind::Message,
      },

      fields: [
        __z::tdp::FieldStorage {
          number: 1,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::CodegenResponse::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().files as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::CodegenResponse::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().report as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 1,
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

impl __z::Type for __::pz::plugin::CodegenResponse {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::CodegenResponse::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::CodegenResponse::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_CodegenResponse::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_CodegenResponse::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_CodegenResponse::Mut {
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

impl __r::Views for __::pz::plugin::CodegenResponse {
  type Ref<'a> = __priv_CodegenResponse::Ref<'a>;
  type Mut<'a> = __priv_CodegenResponse::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_CodegenResponse::Ref<'a> {
  type Target = __::pz::plugin::CodegenResponse;
  fn as_ref(&self) -> __priv_CodegenResponse::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_CodegenResponse::Mut<'a> {
  type Target = __::pz::plugin::CodegenResponse;
  fn as_ref(&self) -> __priv_CodegenResponse::Ref { self.r }
  fn into_ref(self) -> __priv_CodegenResponse::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_CodegenResponse::Mut {
    __priv_CodegenResponse::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::CodegenResponse {
  const DEFAULT: &'static Self = __::pz::plugin::CodegenResponse::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_CodegenResponse::Ref<'a> {
  type Message = __::pz::plugin::CodegenResponse;
}
impl<'a> __r::MessageMut<'a> for __priv_CodegenResponse::Mut<'a> {
  type Message = __::pz::plugin::CodegenResponse;
}

impl __s::default::Default for __::pz::plugin::CodegenResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::CodegenResponse>> __s::convert::From<T> for __::pz::plugin::CodegenResponse {
  fn from(value: T) -> __::pz::plugin::CodegenResponse {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::CodegenResponse> for &__::pz::plugin::CodegenResponse {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenResponse>) {
    __::pz::plugin::CodegenResponse::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenResponse>> for &__::pz::plugin::CodegenResponse {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenResponse>>) {
    __::pz::plugin::CodegenResponse::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::CodegenResponse> for __rt::reflect::Ref<'_, __::pz::plugin::CodegenResponse> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenResponse>) {
    __::pz::plugin::CodegenResponse::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenResponse>> for __rt::reflect::Ref<'_, __::pz::plugin::CodegenResponse> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenResponse>>) {
    __::pz::plugin::CodegenResponse::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::CodegenResponse> for &__rt::reflect::Mut<'_, __::pz::plugin::CodegenResponse> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenResponse>) {
    __::pz::plugin::CodegenResponse::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenResponse>> for &__rt::reflect::Mut<'_, __::pz::plugin::CodegenResponse> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenResponse>>) {
    __::pz::plugin::CodegenResponse::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_CodegenResponse::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::CodegenResponse::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::CodegenResponse {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_CodegenResponse::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.CodegenResponse ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_CodegenResponse::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::CodegenResponse {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_CodegenResponse__files = __rt::field!(files);
impl __r::Field<__field_CodegenResponse__files> for __::pz::plugin::CodegenResponse {
  type Type = __r::Rep<__::pz::plugin::CodegenResponse_File>;
  type Name = __field_CodegenResponse__files;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "files";
}

type __field_CodegenResponse__report = __rt::field!(report);
impl __r::Field<__field_CodegenResponse__report> for __::pz::plugin::CodegenResponse {
  type Type = __r::Rep<__::pz::plugin::Diagnostic>;
  type Name = __field_CodegenResponse__report;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "report";
}

impl<'proto> __priv_CodegenResponse::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_CodegenResponse::Ref { *self }

  /// Serializes this [`__::pz::plugin::CodegenResponse`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenResponse`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenResponse>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn files(self) -> __rt::Slice<'proto, __::pz::plugin::CodegenResponse_File> {
    self.get(__field_CodegenResponse__files{})
  }
  pub fn files_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::plugin::CodegenResponse_File> {
    self.files().at(idx)
  }

  pub fn report(self) -> __rt::Slice<'proto, __::pz::plugin::Diagnostic> {
    self.get(__field_CodegenResponse__report{})
  }
  pub fn report_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::plugin::Diagnostic> {
    self.report().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    for value in self.files() {
      if count != 0 { debug.comma(false)?; }
      debug.field("files")?;
      value.__debug(debug)?;
      count += 1;
    }
    for value in self.report() {
      if count != 0 { debug.comma(false)?; }
      debug.field("report")?;
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

impl<'proto> __priv_CodegenResponse::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_CodegenResponse::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_CodegenResponse::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_CodegenResponse::Mut {
    __priv_CodegenResponse::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::CodegenResponse`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::CodegenResponse`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenResponse`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenResponse>,
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
    S: __r::Selector<__::pz::plugin::CodegenResponse>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::CodegenResponse`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn files(self) -> __rt::Slice<'proto, __::pz::plugin::CodegenResponse_File> {
    self.get(__field_CodegenResponse__files{})
  }
  pub fn files_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::plugin::CodegenResponse_File> {
    self.files().at(idx)
  }
  pub fn files_mut(mut self) -> __rt::Repeated<'proto, __::pz::plugin::CodegenResponse_File> {
    self.get_mut(__field_CodegenResponse__files{})
  }
  pub fn set_files(mut self, value: impl __r::Set<__r::Rep<__::pz::plugin::CodegenResponse_File>>) -> __r::Mut<'proto, __::pz::plugin::CodegenResponse> {
    value.apply_to(self.as_mut().files_mut());
    self
  }

  pub fn report(self) -> __rt::Slice<'proto, __::pz::plugin::Diagnostic> {
    self.get(__field_CodegenResponse__report{})
  }
  pub fn report_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::plugin::Diagnostic> {
    self.report().at(idx)
  }
  pub fn report_mut(mut self) -> __rt::Repeated<'proto, __::pz::plugin::Diagnostic> {
    self.get_mut(__field_CodegenResponse__report{})
  }
  pub fn set_report(mut self, value: impl __r::Set<__r::Rep<__::pz::plugin::Diagnostic>>) -> __r::Mut<'proto, __::pz::plugin::CodegenResponse> {
    value.apply_to(self.as_mut().report_mut());
    self
  }

}

/// message `pz.plugin.CodegenResponse.File`
pub struct CodegenResponse_File {
  ptr: __s::ptr::NonNull<__priv_CodegenResponse_File::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_CodegenResponse_File::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_CodegenResponse_File {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_CodegenResponse_File::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::CodegenResponse_File>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::CodegenResponse_File>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) path: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) content: <__rt::String as __z::Type>::__Storage<__z::Seal>,
  }
}

impl __::pz::plugin::CodegenResponse_File {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_CodegenResponse_File::Storage {
      __hasbits: [0; 1],
      path: __z::RawStr::new(),
      content: __z::RawStr::new(),
    }} as *const __priv_CodegenResponse_File::Storage as *mut __priv_CodegenResponse_File::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::CodegenResponse_File`].
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

  /// Deserializes a new [`__::pz::plugin::CodegenResponse_File`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::CodegenResponse_File`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::CodegenResponse_File`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenResponse_File`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenResponse_File>,
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
    S: __r::Selector<__::pz::plugin::CodegenResponse_File>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::CodegenResponse_File`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn path(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_CodegenResponse_File__path{})
  }
  pub fn path_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.path_mut_or().into_inner()
  }
  pub fn path_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_CodegenResponse_File__path{})
  }
  pub fn set_path(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::CodegenResponse_File> {
    value.apply_to(self.as_mut().path_mut_or());
    self.as_mut()
  }

  pub fn content(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_CodegenResponse_File__content{})
  }
  pub fn content_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.content_mut_or().into_inner()
  }
  pub fn content_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_CodegenResponse_File__content{})
  }
  pub fn set_content(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::CodegenResponse_File> {
    value.apply_to(self.as_mut().content_mut_or());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_CodegenResponse_File::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_CodegenResponse_File::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::CodegenResponse_File>, src: __rt::reflect::Ref<__::pz::plugin::CodegenResponse_File>) {
    __r::Set::<<__::pz::plugin::CodegenResponse_File as __r::Field<__field_CodegenResponse_File__path>>::Type>::apply_to(src.get(__field_CodegenResponse_File__path{}), dst.as_mut().get_mut(__field_CodegenResponse_File__path{}));
    __r::Set::<<__::pz::plugin::CodegenResponse_File as __r::Field<__field_CodegenResponse_File__content>>::Type>::apply_to(src.get(__field_CodegenResponse_File__content{}), dst.as_mut().get_mut(__field_CodegenResponse_File__content{}));
  }
}

impl __z::Message for __::pz::plugin::CodegenResponse_File {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{2 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::CodegenResponse_File::__LAYOUT.size();
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
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::CodegenResponse_File::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().path as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::CodegenResponse_File::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().content as *const _ as *const u8;
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

impl __z::Type for __::pz::plugin::CodegenResponse_File {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::CodegenResponse_File::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::CodegenResponse_File::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_CodegenResponse_File::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_CodegenResponse_File::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_CodegenResponse_File::Mut {
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

impl __r::Views for __::pz::plugin::CodegenResponse_File {
  type Ref<'a> = __priv_CodegenResponse_File::Ref<'a>;
  type Mut<'a> = __priv_CodegenResponse_File::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_CodegenResponse_File::Ref<'a> {
  type Target = __::pz::plugin::CodegenResponse_File;
  fn as_ref(&self) -> __priv_CodegenResponse_File::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_CodegenResponse_File::Mut<'a> {
  type Target = __::pz::plugin::CodegenResponse_File;
  fn as_ref(&self) -> __priv_CodegenResponse_File::Ref { self.r }
  fn into_ref(self) -> __priv_CodegenResponse_File::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_CodegenResponse_File::Mut {
    __priv_CodegenResponse_File::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::CodegenResponse_File {
  const DEFAULT: &'static Self = __::pz::plugin::CodegenResponse_File::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_CodegenResponse_File::Ref<'a> {
  type Message = __::pz::plugin::CodegenResponse_File;
}
impl<'a> __r::MessageMut<'a> for __priv_CodegenResponse_File::Mut<'a> {
  type Message = __::pz::plugin::CodegenResponse_File;
}

impl __s::default::Default for __::pz::plugin::CodegenResponse_File {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::CodegenResponse_File>> __s::convert::From<T> for __::pz::plugin::CodegenResponse_File {
  fn from(value: T) -> __::pz::plugin::CodegenResponse_File {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::CodegenResponse_File> for &__::pz::plugin::CodegenResponse_File {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenResponse_File>) {
    __::pz::plugin::CodegenResponse_File::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenResponse_File>> for &__::pz::plugin::CodegenResponse_File {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenResponse_File>>) {
    __::pz::plugin::CodegenResponse_File::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::CodegenResponse_File> for __rt::reflect::Ref<'_, __::pz::plugin::CodegenResponse_File> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenResponse_File>) {
    __::pz::plugin::CodegenResponse_File::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenResponse_File>> for __rt::reflect::Ref<'_, __::pz::plugin::CodegenResponse_File> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenResponse_File>>) {
    __::pz::plugin::CodegenResponse_File::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::CodegenResponse_File> for &__rt::reflect::Mut<'_, __::pz::plugin::CodegenResponse_File> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::CodegenResponse_File>) {
    __::pz::plugin::CodegenResponse_File::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::CodegenResponse_File>> for &__rt::reflect::Mut<'_, __::pz::plugin::CodegenResponse_File> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::CodegenResponse_File>>) {
    __::pz::plugin::CodegenResponse_File::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_CodegenResponse_File::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::CodegenResponse_File::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::CodegenResponse_File {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_CodegenResponse_File::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.CodegenResponse.File ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_CodegenResponse_File::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::CodegenResponse_File {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_CodegenResponse_File__path = __rt::field!(path);
impl __r::Field<__field_CodegenResponse_File__path> for __::pz::plugin::CodegenResponse_File {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_CodegenResponse_File__path;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "path";
}

type __field_CodegenResponse_File__content = __rt::field!(content);
impl __r::Field<__field_CodegenResponse_File__content> for __::pz::plugin::CodegenResponse_File {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_CodegenResponse_File__content;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "content";
}

impl<'proto> __priv_CodegenResponse_File::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_CodegenResponse_File::Ref { *self }

  /// Serializes this [`__::pz::plugin::CodegenResponse_File`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenResponse_File`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenResponse_File>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn path(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_CodegenResponse_File__path{})
  }

  pub fn content(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_CodegenResponse_File__content{})
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.path_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("path")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.content_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("content")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_CodegenResponse_File::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_CodegenResponse_File::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_CodegenResponse_File::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_CodegenResponse_File::Mut {
    __priv_CodegenResponse_File::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::CodegenResponse_File`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::CodegenResponse_File`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::CodegenResponse_File`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::CodegenResponse_File>,
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
    S: __r::Selector<__::pz::plugin::CodegenResponse_File>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::CodegenResponse_File`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn path(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_CodegenResponse_File__path{})
  }
  pub fn path_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.path_mut_or().into_inner()
  }
  pub fn path_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_CodegenResponse_File__path{})
  }
  pub fn set_path(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::CodegenResponse_File> {
    value.apply_to(self.as_mut().path_mut_or());
    self
  }

  pub fn content(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_CodegenResponse_File__content{})
  }
  pub fn content_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.content_mut_or().into_inner()
  }
  pub fn content_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_CodegenResponse_File__content{})
  }
  pub fn set_content(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::CodegenResponse_File> {
    value.apply_to(self.as_mut().content_mut_or());
    self
  }

}

/// message `pz.plugin.Diagnostic`
pub struct Diagnostic {
  ptr: __s::ptr::NonNull<__priv_Diagnostic::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Diagnostic::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_Diagnostic {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_Diagnostic::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::Diagnostic>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::Diagnostic>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) kind: <__::pz::plugin::Diagnostic_Kind as __z::Type>::__Storage<__z::Seal>,
    pub(in super) msg: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) snippets: __z::AVec<<__::pz::plugin::Diagnostic_Snippet as __z::Type>::__Storage<__z::Seal>>,
    pub(in super) notes: __z::AVec<<__rt::String as __z::Type>::__Storage<__z::Seal>>,
  }
}

impl __::pz::plugin::Diagnostic {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_Diagnostic::Storage {
      __hasbits: [0; 1],
      kind: __::pz::plugin::Diagnostic_Kind::new(),
      msg: __z::RawStr::new(),
      snippets: __z::AVec::new(),
      notes: __z::AVec::new(),
    }} as *const __priv_Diagnostic::Storage as *mut __priv_Diagnostic::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::Diagnostic`].
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

  /// Deserializes a new [`__::pz::plugin::Diagnostic`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::Diagnostic`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::Diagnostic`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Diagnostic`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Diagnostic>,
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
    S: __r::Selector<__::pz::plugin::Diagnostic>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::Diagnostic`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn kind(&self) -> __rt::reflect::Ref<'_, __::pz::plugin::Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __::pz::plugin::Diagnostic_Kind>> {
    self.get(__field_Diagnostic__kind{})
  }
  pub fn kind_mut(&mut self) -> __rt::reflect::Mut<'_, __::pz::plugin::Diagnostic_Kind> {
    self.kind_mut_or().into_inner()
  }
  pub fn kind_mut_or(&mut self) -> __rt::OptMut<'_, __::pz::plugin::Diagnostic_Kind> {
    self.get_mut(__field_Diagnostic__kind{})
  }
  pub fn set_kind(&mut self, value: impl __r::Set<__r::Opt<__::pz::plugin::Diagnostic_Kind>>) -> __r::Mut<'_, __::pz::plugin::Diagnostic> {
    value.apply_to(self.as_mut().kind_mut_or());
    self.as_mut()
  }

  pub fn msg(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_Diagnostic__msg{})
  }
  pub fn msg_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.msg_mut_or().into_inner()
  }
  pub fn msg_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_Diagnostic__msg{})
  }
  pub fn set_msg(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::Diagnostic> {
    value.apply_to(self.as_mut().msg_mut_or());
    self.as_mut()
  }

  pub fn snippets(&self) -> __rt::Slice<'_, __::pz::plugin::Diagnostic_Snippet> {
    self.get(__field_Diagnostic__snippets{})
  }
  pub fn snippets_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __::pz::plugin::Diagnostic_Snippet> {
    self.snippets().at(idx)
  }
  pub fn snippets_mut(&mut self) -> __rt::Repeated<'_, __::pz::plugin::Diagnostic_Snippet> {
    self.get_mut(__field_Diagnostic__snippets{})
  }
  pub fn set_snippets(&mut self, value: impl __r::Set<__r::Rep<__::pz::plugin::Diagnostic_Snippet>>) -> __r::Mut<'_, __::pz::plugin::Diagnostic> {
    value.apply_to(self.as_mut().snippets_mut());
    self.as_mut()
  }

  pub fn notes(&self) -> __rt::Slice<'_, __rt::String> {
    self.get(__field_Diagnostic__notes{})
  }
  pub fn notes_at(&self, idx: usize) -> __rt::reflect::Ref<'_, __rt::String> {
    self.notes().at(idx)
  }
  pub fn notes_mut(&mut self) -> __rt::Repeated<'_, __rt::String> {
    self.get_mut(__field_Diagnostic__notes{})
  }
  pub fn set_notes(&mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::Diagnostic> {
    value.apply_to(self.as_mut().notes_mut());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Diagnostic::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_Diagnostic::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::Diagnostic>, src: __rt::reflect::Ref<__::pz::plugin::Diagnostic>) {
    __r::Set::<<__::pz::plugin::Diagnostic as __r::Field<__field_Diagnostic__kind>>::Type>::apply_to(src.get(__field_Diagnostic__kind{}), dst.as_mut().get_mut(__field_Diagnostic__kind{}));
    __r::Set::<<__::pz::plugin::Diagnostic as __r::Field<__field_Diagnostic__msg>>::Type>::apply_to(src.get(__field_Diagnostic__msg{}), dst.as_mut().get_mut(__field_Diagnostic__msg{}));
    __r::Set::<<__::pz::plugin::Diagnostic as __r::Field<__field_Diagnostic__snippets>>::Type>::apply_to(src.get(__field_Diagnostic__snippets{}), dst.as_mut().get_mut(__field_Diagnostic__snippets{}));
    __r::Set::<<__::pz::plugin::Diagnostic as __r::Field<__field_Diagnostic__notes>>::Type>::apply_to(src.get(__field_Diagnostic__notes{}), dst.as_mut().get_mut(__field_Diagnostic__notes{}));
  }
}

impl __z::Message for __::pz::plugin::Diagnostic {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{4 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::Diagnostic::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },

        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __::pz::plugin::Diagnostic_Snippet::__tdp_info,
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
            let msg = __::pz::plugin::Diagnostic::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().kind as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::Diagnostic::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().msg as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        __z::tdp::FieldStorage {
          number: 3,
          flags:
            __z::tdp::Kind::Type.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::Diagnostic::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().snippets as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
        },
        __z::tdp::FieldStorage {
          number: 4,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            1 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::Diagnostic::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().notes as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
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

impl __z::Type for __::pz::plugin::Diagnostic {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::Diagnostic::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::Diagnostic::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_Diagnostic::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_Diagnostic::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_Diagnostic::Mut {
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

impl __r::Views for __::pz::plugin::Diagnostic {
  type Ref<'a> = __priv_Diagnostic::Ref<'a>;
  type Mut<'a> = __priv_Diagnostic::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_Diagnostic::Ref<'a> {
  type Target = __::pz::plugin::Diagnostic;
  fn as_ref(&self) -> __priv_Diagnostic::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_Diagnostic::Mut<'a> {
  type Target = __::pz::plugin::Diagnostic;
  fn as_ref(&self) -> __priv_Diagnostic::Ref { self.r }
  fn into_ref(self) -> __priv_Diagnostic::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_Diagnostic::Mut {
    __priv_Diagnostic::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::Diagnostic {
  const DEFAULT: &'static Self = __::pz::plugin::Diagnostic::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_Diagnostic::Ref<'a> {
  type Message = __::pz::plugin::Diagnostic;
}
impl<'a> __r::MessageMut<'a> for __priv_Diagnostic::Mut<'a> {
  type Message = __::pz::plugin::Diagnostic;
}

impl __s::default::Default for __::pz::plugin::Diagnostic {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::Diagnostic>> __s::convert::From<T> for __::pz::plugin::Diagnostic {
  fn from(value: T) -> __::pz::plugin::Diagnostic {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::Diagnostic> for &__::pz::plugin::Diagnostic {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Diagnostic>) {
    __::pz::plugin::Diagnostic::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Diagnostic>> for &__::pz::plugin::Diagnostic {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Diagnostic>>) {
    __::pz::plugin::Diagnostic::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::Diagnostic> for __rt::reflect::Ref<'_, __::pz::plugin::Diagnostic> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Diagnostic>) {
    __::pz::plugin::Diagnostic::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Diagnostic>> for __rt::reflect::Ref<'_, __::pz::plugin::Diagnostic> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Diagnostic>>) {
    __::pz::plugin::Diagnostic::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::Diagnostic> for &__rt::reflect::Mut<'_, __::pz::plugin::Diagnostic> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Diagnostic>) {
    __::pz::plugin::Diagnostic::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Diagnostic>> for &__rt::reflect::Mut<'_, __::pz::plugin::Diagnostic> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Diagnostic>>) {
    __::pz::plugin::Diagnostic::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_Diagnostic::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::Diagnostic::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::Diagnostic {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_Diagnostic::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.Diagnostic ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Diagnostic::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::Diagnostic {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_Diagnostic__kind = __rt::field!(kind);
impl __r::Field<__field_Diagnostic__kind> for __::pz::plugin::Diagnostic {
  type Type = __r::Opt<__::pz::plugin::Diagnostic_Kind>;
  type Name = __field_Diagnostic__kind;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "kind";
}

type __field_Diagnostic__msg = __rt::field!(msg);
impl __r::Field<__field_Diagnostic__msg> for __::pz::plugin::Diagnostic {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_Diagnostic__msg;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "msg";
}

type __field_Diagnostic__snippets = __rt::field!(snippets);
impl __r::Field<__field_Diagnostic__snippets> for __::pz::plugin::Diagnostic {
  type Type = __r::Rep<__::pz::plugin::Diagnostic_Snippet>;
  type Name = __field_Diagnostic__snippets;
  const NUMBER: __s::primitive::i32 = 3;
  const INDEX: __s::primitive::usize = 2;
  const NAME: &'static __s::primitive::str = "snippets";
}

type __field_Diagnostic__notes = __rt::field!(notes);
impl __r::Field<__field_Diagnostic__notes> for __::pz::plugin::Diagnostic {
  type Type = __r::Rep<__rt::String>;
  type Name = __field_Diagnostic__notes;
  const NUMBER: __s::primitive::i32 = 4;
  const INDEX: __s::primitive::usize = 3;
  const NAME: &'static __s::primitive::str = "notes";
}

impl<'proto> __priv_Diagnostic::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_Diagnostic::Ref { *self }

  /// Serializes this [`__::pz::plugin::Diagnostic`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Diagnostic`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Diagnostic>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn kind(self) -> __rt::reflect::Ref<'proto, __::pz::plugin::Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::plugin::Diagnostic_Kind>> {
    self.get(__field_Diagnostic__kind{})
  }

  pub fn msg(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Diagnostic__msg{})
  }

  pub fn snippets(self) -> __rt::Slice<'proto, __::pz::plugin::Diagnostic_Snippet> {
    self.get(__field_Diagnostic__snippets{})
  }
  pub fn snippets_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::plugin::Diagnostic_Snippet> {
    self.snippets().at(idx)
  }

  pub fn notes(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_Diagnostic__notes{})
  }
  pub fn notes_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.notes().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.kind_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("kind")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.msg_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("msg")?;
      debug.write_debug(value);
      count += 1;
    }
    for value in self.snippets() {
      if count != 0 { debug.comma(false)?; }
      debug.field("snippets")?;
      value.__debug(debug)?;
      count += 1;
    }
    if !self.notes().is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("notes")?;
      debug.iter(self.notes())?;
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_Diagnostic::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_Diagnostic::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_Diagnostic::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_Diagnostic::Mut {
    __priv_Diagnostic::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::Diagnostic`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::Diagnostic`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Diagnostic`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Diagnostic>,
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
    S: __r::Selector<__::pz::plugin::Diagnostic>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::Diagnostic`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn kind(self) -> __rt::reflect::Ref<'proto, __::pz::plugin::Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __::pz::plugin::Diagnostic_Kind>> {
    self.get(__field_Diagnostic__kind{})
  }
  pub fn kind_mut(mut self) -> __rt::reflect::Mut<'proto, __::pz::plugin::Diagnostic_Kind> {
    self.kind_mut_or().into_inner()
  }
  pub fn kind_mut_or(mut self) -> __rt::OptMut<'proto, __::pz::plugin::Diagnostic_Kind> {
    self.get_mut(__field_Diagnostic__kind{})
  }
  pub fn set_kind(mut self, value: impl __r::Set<__r::Opt<__::pz::plugin::Diagnostic_Kind>>) -> __r::Mut<'proto, __::pz::plugin::Diagnostic> {
    value.apply_to(self.as_mut().kind_mut_or());
    self
  }

  pub fn msg(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Diagnostic__msg{})
  }
  pub fn msg_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.msg_mut_or().into_inner()
  }
  pub fn msg_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_Diagnostic__msg{})
  }
  pub fn set_msg(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::Diagnostic> {
    value.apply_to(self.as_mut().msg_mut_or());
    self
  }

  pub fn snippets(self) -> __rt::Slice<'proto, __::pz::plugin::Diagnostic_Snippet> {
    self.get(__field_Diagnostic__snippets{})
  }
  pub fn snippets_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __::pz::plugin::Diagnostic_Snippet> {
    self.snippets().at(idx)
  }
  pub fn snippets_mut(mut self) -> __rt::Repeated<'proto, __::pz::plugin::Diagnostic_Snippet> {
    self.get_mut(__field_Diagnostic__snippets{})
  }
  pub fn set_snippets(mut self, value: impl __r::Set<__r::Rep<__::pz::plugin::Diagnostic_Snippet>>) -> __r::Mut<'proto, __::pz::plugin::Diagnostic> {
    value.apply_to(self.as_mut().snippets_mut());
    self
  }

  pub fn notes(self) -> __rt::Slice<'proto, __rt::String> {
    self.get(__field_Diagnostic__notes{})
  }
  pub fn notes_at(self, idx: usize) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.notes().at(idx)
  }
  pub fn notes_mut(mut self) -> __rt::Repeated<'proto, __rt::String> {
    self.get_mut(__field_Diagnostic__notes{})
  }
  pub fn set_notes(mut self, value: impl __r::Set<__r::Rep<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::Diagnostic> {
    value.apply_to(self.as_mut().notes_mut());
    self
  }

}

/// enum `pz.plugin.Diagnostic.Kind`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Diagnostic_Kind(pub __s::primitive::i32);

impl __::pz::plugin::Diagnostic_Kind {
  pub const Error: Self = Self(0);
  pub const Warning: Self = Self(1);

  pub const fn new() -> Self {
    Self(0)
  }
}

impl __s::default::Default for __::pz::plugin::Diagnostic_Kind {
  fn default() -> Self {
    Self(0)
  }
}

impl __z::Type for __::pz::plugin::Diagnostic_Kind {
  type __Storage<S: __z::Sealed> = Self;

  unsafe fn __ref<'a, S: __z::Sealed>(_: S, ptr: __s::ptr::NonNull<Self>) -> __rt::Ref<'a, Self> {
    ptr.read()
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    _: S,
    mut ptr: __s::ptr::NonNull<Self>,
    _: __z::RawArena,
  ) -> __rt::Mut<'a, Self> {
    __rt::ScalarMut::__wrap(ptr.as_mut())
  }
}

impl __r::Views for __::pz::plugin::Diagnostic_Kind {
  type Ref<'a> = Self;
  type Mut<'a> = __rt::ScalarMut<'a, Self>;
}

impl __r::RefView<'_> for __::pz::plugin::Diagnostic_Kind {
  type Target = Self;

  fn as_ref(&self) -> Self {
    *self
  }
}

impl __r::Set<__::pz::plugin::Diagnostic_Kind> for __::pz::plugin::Diagnostic_Kind {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Diagnostic_Kind>) {
    m.set(self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Diagnostic_Kind>> for __::pz::plugin::Diagnostic_Kind {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Diagnostic_Kind>>) {
    m.into_inner().set(self)
  }
}

impl __s::fmt::Debug for __::pz::plugin::Diagnostic_Kind {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    match *self {
      Self::Error => __s::write!(fmt, "Error"),
      Self::Warning => __s::write!(fmt, "Warning"),
      Self(n) => __s::write!(fmt, "pz.plugin.Diagnostic.Kind({n})"),
    }
  }
}

/// message `pz.plugin.Diagnostic.Snippet`
pub struct Diagnostic_Snippet {
  ptr: __s::ptr::NonNull<__priv_Diagnostic_Snippet::Storage>,
  arena: __s::option::Option<__z::RawArena>,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Diagnostic_Snippet::Storage>() < (u32::MAX as usize),
    "storage size exceeds 4GB",
  );
};

mod __priv_Diagnostic_Snippet {
  pub use super::*;

  #[derive(Copy, Clone)]
  pub struct Ref<'proto> {
    pub(in super) ptr: __s::ptr::NonNull<__priv_Diagnostic_Snippet::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __::pz::plugin::Diagnostic_Snippet>,
  }

  pub struct Mut<'proto> {
    pub(in super) r: Ref<'proto>,
    pub(in super) arena: __z::RawArena,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __::pz::plugin::Diagnostic_Snippet>,
  }

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) span: <__s::primitive::u32 as __z::Type>::__Storage<__z::Seal>,
    pub(in super) message: <__rt::String as __z::Type>::__Storage<__z::Seal>,
    pub(in super) is_remark: <__s::primitive::bool as __z::Type>::__Storage<__z::Seal>,
  }
}

impl __::pz::plugin::Diagnostic_Snippet {
  /// The default value for [`Type`], provided as a static constant.
  ///
  /// See [`Message::DEFAULT`][__r::Message::DEFAULT].
  pub const DEFAULT: &'static Self = unsafe { &Self {
    ptr: __s::ptr::NonNull::new_unchecked(&const { __priv_Diagnostic_Snippet::Storage {
      __hasbits: [0; 1],
      span: 0,
      message: __z::RawStr::new(),
      is_remark: false,
    }} as *const __priv_Diagnostic_Snippet::Storage as *mut __priv_Diagnostic_Snippet::Storage),
    arena: __s::option::Option::None,
  }};

  /// Constructs a new, empty [`__::pz::plugin::Diagnostic_Snippet`].
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

  /// Deserializes a new [`__::pz::plugin::Diagnostic_Snippet`] from the given stream.
  ///
  /// See [`Message::parse()`][__r::Message::parse].
  pub fn parse(codec: __rt::Codec, input: &mut dyn __s::io::Read) -> __s::result::Result<Self, __rt::Error> {
    <Self as __r::Message>::parse(codec, input)
  }

  /// Deserializes onto this [`__::pz::plugin::Diagnostic_Snippet`] in place from the given stream.
  ///
  /// See [`Message::parse_in_place()`][__r::Message::parse_in_place].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::Diagnostic_Snippet`] to the given stream.
  ///
  /// See [`Message::emit()`][__r::Message::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::Message>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Diagnostic_Snippet`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Diagnostic_Snippet>,
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
    S: __r::Selector<__::pz::plugin::Diagnostic_Snippet>,
  {
    <Self as __r::Message>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::Diagnostic_Snippet`] to its default state.
  ///
  /// See [`Message::clear()`][__r::Message::clear].
  pub fn clear(&mut self) {
    <Self as __r::Message>::clear(self)
  }

  pub fn span(&self) -> __rt::reflect::Ref<'_, __s::primitive::u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::u32>> {
    self.get(__field_Diagnostic_Snippet__span{})
  }
  pub fn span_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::u32> {
    self.span_mut_or().into_inner()
  }
  pub fn span_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::u32> {
    self.get_mut(__field_Diagnostic_Snippet__span{})
  }
  pub fn set_span(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'_, __::pz::plugin::Diagnostic_Snippet> {
    value.apply_to(self.as_mut().span_mut_or());
    self.as_mut()
  }

  pub fn message(&self) -> __rt::reflect::Ref<'_, __rt::String> {
    self.message_or().unwrap_or_default()
  }
  pub fn message_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __rt::String>> {
    self.get(__field_Diagnostic_Snippet__message{})
  }
  pub fn message_mut(&mut self) -> __rt::reflect::Mut<'_, __rt::String> {
    self.message_mut_or().into_inner()
  }
  pub fn message_mut_or(&mut self) -> __rt::OptMut<'_, __rt::String> {
    self.get_mut(__field_Diagnostic_Snippet__message{})
  }
  pub fn set_message(&mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'_, __::pz::plugin::Diagnostic_Snippet> {
    value.apply_to(self.as_mut().message_mut_or());
    self.as_mut()
  }

  pub fn is_remark(&self) -> __rt::reflect::Ref<'_, __s::primitive::bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(&self) -> __s::option::Option<__rt::reflect::Ref<'_, __s::primitive::bool>> {
    self.get(__field_Diagnostic_Snippet__is_remark{})
  }
  pub fn is_remark_mut(&mut self) -> __rt::reflect::Mut<'_, __s::primitive::bool> {
    self.is_remark_mut_or().into_inner()
  }
  pub fn is_remark_mut_or(&mut self) -> __rt::OptMut<'_, __s::primitive::bool> {
    self.get_mut(__field_Diagnostic_Snippet__is_remark{})
  }
  pub fn set_is_remark(&mut self, value: impl __r::Set<__r::Opt<__s::primitive::bool>>) -> __r::Mut<'_, __::pz::plugin::Diagnostic_Snippet> {
    value.apply_to(self.as_mut().is_remark_mut_or());
    self.as_mut()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Diagnostic_Snippet::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: __z::tdp::Opaque) {
    raw.cast::<__priv_Diagnostic_Snippet::Storage>().as_mut().__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    <Self as __z::Message>::__TDP
  }

  #[doc(hidden)]
  fn __memcpy(mut dst: __rt::reflect::Mut<__::pz::plugin::Diagnostic_Snippet>, src: __rt::reflect::Ref<__::pz::plugin::Diagnostic_Snippet>) {
    __r::Set::<<__::pz::plugin::Diagnostic_Snippet as __r::Field<__field_Diagnostic_Snippet__span>>::Type>::apply_to(src.get(__field_Diagnostic_Snippet__span{}), dst.as_mut().get_mut(__field_Diagnostic_Snippet__span{}));
    __r::Set::<<__::pz::plugin::Diagnostic_Snippet as __r::Field<__field_Diagnostic_Snippet__message>>::Type>::apply_to(src.get(__field_Diagnostic_Snippet__message{}), dst.as_mut().get_mut(__field_Diagnostic_Snippet__message{}));
    __r::Set::<<__::pz::plugin::Diagnostic_Snippet as __r::Field<__field_Diagnostic_Snippet__is_remark>>::Type>::apply_to(src.get(__field_Diagnostic_Snippet__is_remark{}), dst.as_mut().get_mut(__field_Diagnostic_Snippet__is_remark{}));
  }
}

impl __z::Message for __::pz::plugin::Diagnostic_Snippet {
  const __TDP: __z::tdp::Desc = {
    type Tdp = __z::tdp::DescStorage<{3 + 1}>;
    const STATIC: Tdp = Tdp {
      header: __z::tdp::DescHeader {
        size: {
          let size = __::pz::plugin::Diagnostic_Snippet::__LAYOUT.size();
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
            let msg = __::pz::plugin::Diagnostic_Snippet::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().span as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        __z::tdp::FieldStorage {
          number: 2,
          flags:
            __z::tdp::Kind::Str.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::Diagnostic_Snippet::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().message as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        __z::tdp::FieldStorage {
          number: 3,
          flags:
            __z::tdp::Kind::Bool.raw() << __z::tdp::Field::KIND_SHIFT |
            0 << __z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = __::pz::plugin::Diagnostic_Snippet::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().is_remark as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
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

impl __z::Type for __::pz::plugin::Diagnostic_Snippet {
  type __Storage<S: __z::Sealed> = __s::option::Option<__z::tdp::Opaque>;

  unsafe fn __ref<'a, S: __z::Sealed>(
    _: S,
    ptr: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
  ) -> __r::Ref<'a, Self> {
    match ptr.read() {
      __s::option::Option::None => __::pz::plugin::Diagnostic_Snippet::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) if ptr == __s::ptr::NonNull::dangling() => __::pz::plugin::Diagnostic_Snippet::DEFAULT.as_ref(),
      __s::option::Option::Some(ptr) => __priv_Diagnostic_Snippet::Ref { ptr: ptr.cast(), _ph: __s::marker::PhantomData }
    }
  }

  unsafe fn __mut<'a, S: __z::Sealed>(
    s: S,
    mut outer: __s::ptr::NonNull<__s::option::Option<__z::tdp::Opaque>>,
    arena: __z::RawArena,
  ) -> __r::Mut<'a, Self> {
    let ptr = outer.as_mut();
    if ptr.is_none() || *ptr == __s::option::Option::Some(__s::ptr::NonNull::<__priv_Diagnostic_Snippet::Storage>::dangling().cast::<u8>()) {
      let new = arena.alloc(Self::__LAYOUT);
      new.write_bytes(0, Self::__LAYOUT.size());
      *ptr = __s::option::Option::Some(new);
    }

    __priv_Diagnostic_Snippet::Mut {
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

impl __r::Views for __::pz::plugin::Diagnostic_Snippet {
  type Ref<'a> = __priv_Diagnostic_Snippet::Ref<'a>;
  type Mut<'a> = __priv_Diagnostic_Snippet::Mut<'a>;
}

impl<'a> __r::RefView<'a> for __priv_Diagnostic_Snippet::Ref<'a> {
  type Target = __::pz::plugin::Diagnostic_Snippet;
  fn as_ref(&self) -> __priv_Diagnostic_Snippet::Ref { *self }
}

impl<'a> __r::MutView<'a> for __priv_Diagnostic_Snippet::Mut<'a> {
  type Target = __::pz::plugin::Diagnostic_Snippet;
  fn as_ref(&self) -> __priv_Diagnostic_Snippet::Ref { self.r }
  fn into_ref(self) -> __priv_Diagnostic_Snippet::Ref<'a> { self.r }
  fn as_mut(&mut self) -> __priv_Diagnostic_Snippet::Mut {
    __priv_Diagnostic_Snippet::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }
}

impl __r::Message for __::pz::plugin::Diagnostic_Snippet {
  const DEFAULT: &'static Self = __::pz::plugin::Diagnostic_Snippet::DEFAULT;

  fn as_ref(&self) -> __r::Ref<Self> {
    Self::as_ref(self)
  }
  fn as_mut(&mut self) -> __r::Mut<Self> {
    Self::as_mut(self)
  }
}

impl<'a> __r::MessageRef<'a> for __priv_Diagnostic_Snippet::Ref<'a> {
  type Message = __::pz::plugin::Diagnostic_Snippet;
}
impl<'a> __r::MessageMut<'a> for __priv_Diagnostic_Snippet::Mut<'a> {
  type Message = __::pz::plugin::Diagnostic_Snippet;
}

impl __s::default::Default for __::pz::plugin::Diagnostic_Snippet {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: __r::Set<__::pz::plugin::Diagnostic_Snippet>> __s::convert::From<T> for __::pz::plugin::Diagnostic_Snippet {
  fn from(value: T) -> __::pz::plugin::Diagnostic_Snippet {
    let mut msg = Self::default();
    value.apply_to(msg.as_mut());
    msg
  }
}

impl __r::Set<__::pz::plugin::Diagnostic_Snippet> for &__::pz::plugin::Diagnostic_Snippet {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Diagnostic_Snippet>) {
    __::pz::plugin::Diagnostic_Snippet::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Diagnostic_Snippet>> for &__::pz::plugin::Diagnostic_Snippet {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Diagnostic_Snippet>>) {
    __::pz::plugin::Diagnostic_Snippet::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __r::Set<__::pz::plugin::Diagnostic_Snippet> for __rt::reflect::Ref<'_, __::pz::plugin::Diagnostic_Snippet> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Diagnostic_Snippet>) {
    __::pz::plugin::Diagnostic_Snippet::__memcpy(m, self)
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Diagnostic_Snippet>> for __rt::reflect::Ref<'_, __::pz::plugin::Diagnostic_Snippet> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Diagnostic_Snippet>>) {
    __::pz::plugin::Diagnostic_Snippet::__memcpy(m.into_inner(), self)
  }
}

impl __r::Set<__::pz::plugin::Diagnostic_Snippet> for &__rt::reflect::Mut<'_, __::pz::plugin::Diagnostic_Snippet> {
  fn apply_to(self, mut m: __r::Mut<__::pz::plugin::Diagnostic_Snippet>) {
    __::pz::plugin::Diagnostic_Snippet::__memcpy(m, self.as_ref())
  }
}

impl __r::Set<__r::Opt<__::pz::plugin::Diagnostic_Snippet>> for &__rt::reflect::Mut<'_, __::pz::plugin::Diagnostic_Snippet> {
  fn apply_to(self, m: __r::Mut<__r::Opt<__::pz::plugin::Diagnostic_Snippet>>) {
    __::pz::plugin::Diagnostic_Snippet::__memcpy(m.into_inner(), self.as_ref())
  }
}

impl __s::default::Default for __priv_Diagnostic_Snippet::Ref<'_> {
  fn default() -> Self {
    __::pz::plugin::Diagnostic_Snippet::DEFAULT.as_ref()
  }
}

impl __s::ops::Drop for __::pz::plugin::Diagnostic_Snippet {
  fn drop(&mut self) {
    if let __s::option::Option::Some(arena) = self.arena {
      unsafe { arena.destroy() }
    }
  }
}

impl __s::fmt::Debug for __priv_Diagnostic_Snippet::Ref<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.Diagnostic.Snippet ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Diagnostic_Snippet::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

impl __s::fmt::Debug for __::pz::plugin::Diagnostic_Snippet {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_ref(), fmt)
  }
}

type __field_Diagnostic_Snippet__span = __rt::field!(span);
impl __r::Field<__field_Diagnostic_Snippet__span> for __::pz::plugin::Diagnostic_Snippet {
  type Type = __r::Opt<__s::primitive::u32>;
  type Name = __field_Diagnostic_Snippet__span;
  const NUMBER: __s::primitive::i32 = 1;
  const INDEX: __s::primitive::usize = 0;
  const NAME: &'static __s::primitive::str = "span";
}

type __field_Diagnostic_Snippet__message = __rt::field!(message);
impl __r::Field<__field_Diagnostic_Snippet__message> for __::pz::plugin::Diagnostic_Snippet {
  type Type = __r::Opt<__rt::String>;
  type Name = __field_Diagnostic_Snippet__message;
  const NUMBER: __s::primitive::i32 = 2;
  const INDEX: __s::primitive::usize = 1;
  const NAME: &'static __s::primitive::str = "message";
}

type __field_Diagnostic_Snippet__is_remark = __rt::field!(is_remark);
impl __r::Field<__field_Diagnostic_Snippet__is_remark> for __::pz::plugin::Diagnostic_Snippet {
  type Type = __r::Opt<__s::primitive::bool>;
  type Name = __field_Diagnostic_Snippet__is_remark;
  const NUMBER: __s::primitive::i32 = 3;
  const INDEX: __s::primitive::usize = 2;
  const NAME: &'static __s::primitive::str = "is_remark";
}

impl<'proto> __priv_Diagnostic_Snippet::Ref<'proto> {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`RefView::as_ref()`][__r::RefView::as_ref].
  pub fn as_ref(&self) -> __priv_Diagnostic_Snippet::Ref { *self }

  /// Serializes this [`__::pz::plugin::Diagnostic_Snippet`] to the given stream.
  ///
  /// See [`MessageRef::emit()`][__r::MessageRef::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageRef>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Diagnostic_Snippet`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Diagnostic_Snippet>,
  {
    <Self as __r::MessageRef>::get(self, selector)
  }

  pub fn span(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Diagnostic_Snippet__span{})
  }

  pub fn message(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.message_or().unwrap_or_default()
  }
  pub fn message_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Diagnostic_Snippet__message{})
  }

  pub fn is_remark(self) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::bool>> {
    self.get(__field_Diagnostic_Snippet__is_remark{})
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut __z::Debug) -> __s::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let __s::option::Option::Some(value) = self.span_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("span")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.message_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("message")?;
      debug.write_debug(value);
      count += 1;
    }
    if let __s::option::Option::Some(value) = self.is_remark_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("is_remark")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    __s::result::Result::Ok(())
  }
}

impl<'proto> __priv_Diagnostic_Snippet::Mut<'proto>  {
  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_ref()`][__r::MutView::as_ref].
  pub fn as_ref(&self) -> __priv_Diagnostic_Snippet::Ref { self.r }

  /// Consumes this mutator, converting it into an immutable view.
  ///
  /// See [`MutView::into_ref()`][__r::MutView::into_ref].
  pub fn into_ref(self) -> __priv_Diagnostic_Snippet::Ref<'proto> { self.r }

  /// Shortens this view's lifetime, analogous to reborrowing.
  ///
  /// See [`MutView::as_mut()`][__r::MutView::as_mut].
  pub fn as_mut(&mut self) -> __priv_Diagnostic_Snippet::Mut {
    __priv_Diagnostic_Snippet::Mut { r: self.r, arena: self.arena, _ph: __s::marker::PhantomData, }
  }

  /// Parses onto this [`__::pz::plugin::Diagnostic_Snippet`] in place from the given stream.
  ///
  /// See [`MessageMut::parse_in_place()`][__r::MessageMut::parse_in_place()].
  pub fn parse_in_place(
    &mut self,
    codec: __rt::Codec,
    input: &mut dyn __s::io::Read,
  ) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::parse_in_place(self, codec, input)
  }

  /// Serializes this [`__::pz::plugin::Diagnostic_Snippet`] to the given stream.
  ///
  /// See [`MessageMut::emit()`][__r::MessageMut::emit].
  fn emit(&self, codec: __rt::Codec, output: &mut dyn __s::io::Write) -> __s::result::Result<(), __rt::Error> {
    <Self as __r::MessageMut>::emit(self, codec, output)
  }

  /// Serializes this [`__::pz::plugin::Diagnostic_Snippet`] to an in-memory byte array.
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
    S: __r::Selector<__::pz::plugin::Diagnostic_Snippet>,
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
    S: __r::Selector<__::pz::plugin::Diagnostic_Snippet>,
  {
    <Self as __r::MessageMut>::get_mut(self, selector)
  }

  /// Resets this [`__::pz::plugin::Diagnostic_Snippet`] to its default state.
  ///
  /// See [`MessageMut::clear()`][__r::MessageMut::clear].
  pub fn clear(&mut self) {
    <Self as __r::MessageMut>::clear(self)
  }

  pub fn span(self) -> __rt::reflect::Ref<'proto, __s::primitive::u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::u32>> {
    self.get(__field_Diagnostic_Snippet__span{})
  }
  pub fn span_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::u32> {
    self.span_mut_or().into_inner()
  }
  pub fn span_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::u32> {
    self.get_mut(__field_Diagnostic_Snippet__span{})
  }
  pub fn set_span(mut self, value: impl __r::Set<__r::Opt<__s::primitive::u32>>) -> __r::Mut<'proto, __::pz::plugin::Diagnostic_Snippet> {
    value.apply_to(self.as_mut().span_mut_or());
    self
  }

  pub fn message(self) -> __rt::reflect::Ref<'proto, __rt::String> {
    self.message_or().unwrap_or_default()
  }
  pub fn message_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __rt::String>> {
    self.get(__field_Diagnostic_Snippet__message{})
  }
  pub fn message_mut(mut self) -> __rt::reflect::Mut<'proto, __rt::String> {
    self.message_mut_or().into_inner()
  }
  pub fn message_mut_or(mut self) -> __rt::OptMut<'proto, __rt::String> {
    self.get_mut(__field_Diagnostic_Snippet__message{})
  }
  pub fn set_message(mut self, value: impl __r::Set<__r::Opt<__rt::String>>) -> __r::Mut<'proto, __::pz::plugin::Diagnostic_Snippet> {
    value.apply_to(self.as_mut().message_mut_or());
    self
  }

  pub fn is_remark(self) -> __rt::reflect::Ref<'proto, __s::primitive::bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(self) -> __s::option::Option<__rt::reflect::Ref<'proto, __s::primitive::bool>> {
    self.get(__field_Diagnostic_Snippet__is_remark{})
  }
  pub fn is_remark_mut(mut self) -> __rt::reflect::Mut<'proto, __s::primitive::bool> {
    self.is_remark_mut_or().into_inner()
  }
  pub fn is_remark_mut_or(mut self) -> __rt::OptMut<'proto, __s::primitive::bool> {
    self.get_mut(__field_Diagnostic_Snippet__is_remark{})
  }
  pub fn set_is_remark(mut self, value: impl __r::Set<__r::Opt<__s::primitive::bool>>) -> __r::Mut<'proto, __::pz::plugin::Diagnostic_Snippet> {
    value.apply_to(self.as_mut().is_remark_mut_or());
    self
  }

}

} // mod plugin

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

