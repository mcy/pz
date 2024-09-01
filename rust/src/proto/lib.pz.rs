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

use crate as __rt;
use __rt::__z;
use __z::std as __s;

use __s::default::Default as _;

/// message `pz.Bundle`
pub struct Bundle {
  ptr: __z::ABox<__priv_Bundle::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Bundle::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::Bundle {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Bundle::Storage = __priv_Bundle::Storage {
      __hasbits: [0; 0],
      types: __z::AVec::new(),
      packages: __z::AVec::new(),
      foreign_types: __z::AVec::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_Bundle::Storage as *mut __priv_Bundle::Storage as *mut u8),
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
    __priv_Bundle::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_Bundle::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::Bundle::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn types(&self) -> __rt::Slice<'_, __root::pz::Type> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::Type>(self.ptr.as_ptr())
    }
  }
  pub fn types_at(&self, idx: usize) -> __rt::View<'_, __root::pz::Type> {
    self.types().at(idx)
  }
  pub fn types_mut(&mut self) -> __rt::Repeated<'_, __root::pz::Type> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(0);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::Type>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn packages(&self) -> __rt::Slice<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn packages_at(&self, idx: usize) -> __rt::View<'_, __rt::Str> {
    self.packages().at(idx)
  }
  pub fn packages_mut(&mut self) -> __rt::Repeated<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn foreign_types(&self) -> __rt::Slice<'_, __root::pz::Bundle_ForeignType> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::Bundle_ForeignType>(self.ptr.as_ptr())
    }
  }
  pub fn foreign_types_at(&self, idx: usize) -> __rt::View<'_, __root::pz::Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }
  pub fn foreign_types_mut(&mut self) -> __rt::Repeated<'_, __root::pz::Bundle_ForeignType> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(2);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::Bundle_ForeignType>(self.ptr.as_ptr(), self.arena)
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Bundle::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Bundle::Storage>()).__hasbits = [0; 0];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_Bundle::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::Bundle {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::Bundle {
  type View<'proto> = __priv_Bundle::View<'proto>;
  type Mut<'proto> = __priv_Bundle::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::Bundle {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_Bundle::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_Bundle::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Bundle::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::Bundle> {
    __priv_Bundle::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn types(self) -> __rt::Slice<'proto, __root::pz::Type> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::Type>(self.ptr.as_ptr())
    }
  }
  pub fn types_at(self, idx: usize) -> __rt::View<'proto, __root::pz::Type> {
    self.types().at(idx)
  }

  pub fn packages(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn packages_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.packages().at(idx)
  }

  pub fn foreign_types(self) -> __rt::Slice<'proto, __root::pz::Bundle_ForeignType> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::Bundle_ForeignType>(self.ptr.as_ptr())
    }
  }
  pub fn foreign_types_at(self, idx: usize) -> __rt::View<'proto, __root::pz::Bundle_ForeignType> {
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

impl __s::default::Default for __priv_Bundle::View<'_> {
  fn default() -> Self {
    __root::pz::Bundle::DEFAULT
  }
}

impl<'proto> __priv_Bundle::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::Bundle> {
    __priv_Bundle::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::Bundle> {
    __priv_Bundle::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::Bundle> {
    __priv_Bundle::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::Bundle::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::Bundle::__tdp_info())
  }

  pub fn types(self) -> __rt::Slice<'proto, __root::pz::Type> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::Type>(self.ptr.as_ptr())
    }
  }
  pub fn types_at(self, idx: usize) -> __rt::View<'proto, __root::pz::Type> {
    self.types().at(idx)
  }
  pub fn types_mut(self) -> __rt::Repeated<'proto, __root::pz::Type> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(0);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::Type>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn packages(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn packages_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.packages().at(idx)
  }
  pub fn packages_mut(self) -> __rt::Repeated<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn foreign_types(self) -> __rt::Slice<'proto, __root::pz::Bundle_ForeignType> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::Bundle_ForeignType>(self.ptr.as_ptr())
    }
  }
  pub fn foreign_types_at(self, idx: usize) -> __rt::View<'proto, __root::pz::Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }
  pub fn foreign_types_mut(self) -> __rt::Repeated<'proto, __root::pz::Bundle_ForeignType> {
    unsafe {
      let field = __root::pz::Bundle::__tdp_info().field(2);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::Bundle_ForeignType>(self.ptr.as_ptr(), self.arena)
    }
  }

}

impl __s::ops::Drop for __root::pz::Bundle {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_Bundle::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Bundle ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Bundle::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::Bundle {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Bundle {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 0],
    pub(in super) types: __z::AVec<*mut u8>,
    pub(crate) packages: __z::AVec<(*mut u8, usize)>,
    pub(in super) foreign_types: __z::AVec<*mut u8>,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{3 + 1}> =
    __z::tdp::DescStorage::<{3 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::Bundle::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::Bundle_ForeignType::__tdp_info,
            __root::pz::Type::__tdp_info,
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
            let msg = __root::pz::Bundle::DEFAULT;
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
            let msg = __root::pz::Bundle::DEFAULT;
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
            let msg = __root::pz::Bundle::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Bundle::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::Bundle>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Bundle> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Bundle::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::Bundle>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Bundle> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::Bundle> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.Bundle.ForeignType`
pub struct Bundle_ForeignType {
  ptr: __z::ABox<__priv_Bundle_ForeignType::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Bundle_ForeignType::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::Bundle_ForeignType {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Bundle_ForeignType::Storage = __priv_Bundle_ForeignType::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      package: 0,
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_Bundle_ForeignType::Storage as *mut __priv_Bundle_ForeignType::Storage as *mut u8),
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
    __priv_Bundle_ForeignType::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_Bundle_ForeignType::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::Bundle_ForeignType::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> __rt::View<'_, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::Bundle_ForeignType::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Bundle_ForeignType::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(&self) -> __rt::View<'_, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(&self) -> __s::option::Option<__rt::View<'_, u32>> {
    unsafe {
      let field = __root::pz::Bundle_ForeignType::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn package_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Bundle_ForeignType::__tdp_info().field(1),
      )
    }
  }
  pub fn package_set(&mut self, value: u32) {
    self.package_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Bundle_ForeignType::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Bundle_ForeignType::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_Bundle_ForeignType::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::Bundle_ForeignType {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::Bundle_ForeignType {
  type View<'proto> = __priv_Bundle_ForeignType::View<'proto>;
  type Mut<'proto> = __priv_Bundle_ForeignType::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::Bundle_ForeignType {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_Bundle_ForeignType::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_Bundle_ForeignType::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Bundle_ForeignType::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::Bundle_ForeignType> {
    __priv_Bundle_ForeignType::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::Bundle_ForeignType::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn package(self) -> __rt::View<'proto, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Bundle_ForeignType::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
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

impl __s::default::Default for __priv_Bundle_ForeignType::View<'_> {
  fn default() -> Self {
    __root::pz::Bundle_ForeignType::DEFAULT
  }
}

impl<'proto> __priv_Bundle_ForeignType::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::Bundle_ForeignType> {
    __priv_Bundle_ForeignType::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::Bundle_ForeignType> {
    __priv_Bundle_ForeignType::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::Bundle_ForeignType> {
    __priv_Bundle_ForeignType::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::Bundle_ForeignType::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::Bundle_ForeignType::__tdp_info())
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::Bundle_ForeignType::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Bundle_ForeignType::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(self) -> __rt::View<'proto, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Bundle_ForeignType::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn package_mut(self) -> __rt::Mut<'proto, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Bundle_ForeignType::__tdp_info().field(1),
      )
    }
  }
  pub fn package_set(self, value: u32) {
    self.package_mut().set(value);
  }

}

impl __s::ops::Drop for __root::pz::Bundle_ForeignType {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_Bundle_ForeignType::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Bundle.ForeignType ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Bundle_ForeignType::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::Bundle_ForeignType {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Bundle_ForeignType {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: __z::RawStr,
    pub(in super) package: u32,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{2 + 1}> =
    __z::tdp::DescStorage::<{2 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::Bundle_ForeignType::__LAYOUT.size();
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
            let msg = __root::pz::Bundle_ForeignType::DEFAULT;
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
            let msg = __root::pz::Bundle_ForeignType::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Bundle_ForeignType::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::Bundle_ForeignType>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Bundle_ForeignType> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Bundle_ForeignType::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::Bundle_ForeignType>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Bundle_ForeignType> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::Bundle_ForeignType> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.Type`
pub struct Type {
  ptr: __z::ABox<__priv_Type::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Type::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::Type {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Type::Storage = __priv_Type::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      package: 0,
      kind: __root::pz::Type_Kind::new().0 as u32,
      declared_in: 0,
      fields: __z::AVec::new(),
      nesteds: __z::AVec::new(),
      attrs: 0 as *mut u8,
      span: 0,
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_Type::Storage as *mut __priv_Type::Storage as *mut u8),
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
    __priv_Type::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_Type::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::Type::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> __rt::View<'_, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(&self) -> __rt::View<'_, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(&self) -> __s::option::Option<__rt::View<'_, u32>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn package_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(1),
      )
    }
  }
  pub fn package_set(&mut self, value: u32) {
    self.package_mut().set(value);
  }

  pub fn kind(&self) -> __rt::View<'_, __root::pz::Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::Type_Kind>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Type_Kind>(self.ptr.as_ptr()))
    }
  }
  pub fn kind_mut(&mut self) -> __rt::Mut<'_, __root::pz::Type_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::Type_Kind> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(2),
      )
    }
  }
  pub fn kind_set(&mut self, value: __root::pz::Type_Kind) {
    self.kind_mut().set(value);
  }

  pub fn declared_in(&self) -> __rt::View<'_, u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(&self) -> __s::option::Option<__rt::View<'_, u32>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn declared_in_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.declared_in_mut_or().into_mut()
  }
  pub fn declared_in_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(3),
      )
    }
  }
  pub fn declared_in_set(&mut self, value: u32) {
    self.declared_in_mut().set(value);
  }

  pub fn fields(&self) -> __rt::Slice<'_, __root::pz::Field> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::Field>(self.ptr.as_ptr())
    }
  }
  pub fn fields_at(&self, idx: usize) -> __rt::View<'_, __root::pz::Field> {
    self.fields().at(idx)
  }
  pub fn fields_mut(&mut self) -> __rt::Repeated<'_, __root::pz::Field> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(4);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::Field>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn nesteds(&self) -> __rt::Slice<'_, u32> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn nesteds_at(&self, idx: usize) -> __rt::View<'_, u32> {
    self.nesteds().at(idx)
  }
  pub fn nesteds_mut(&mut self) -> __rt::Repeated<'_, u32> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(5);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn attrs(&self) -> __rt::View<'_, __root::pz::Type_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::Type_Attrs>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Type_Attrs>(self.ptr.as_ptr()))
    }
  }
  pub fn attrs_mut(&mut self) -> __rt::Mut<'_, __root::pz::Type_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::Type_Attrs> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(6),
      )
    }
  }

  pub fn span(&self) -> __rt::View<'_, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> __s::option::Option<__rt::View<'_, u32>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(7);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn span_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(7),
      )
    }
  }
  pub fn span_set(&mut self, value: u32) {
    self.span_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Type::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Type::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_Type::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::Type {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::Type {
  type View<'proto> = __priv_Type::View<'proto>;
  type Mut<'proto> = __priv_Type::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::Type {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_Type::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_Type::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Type::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::Type> {
    __priv_Type::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn package(self) -> __rt::View<'proto, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }

  pub fn kind(self) -> __rt::View<'proto, __root::pz::Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::Type_Kind>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Type_Kind>(self.ptr.as_ptr()))
    }
  }

  pub fn declared_in(self) -> __rt::View<'proto, u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }

  pub fn fields(self) -> __rt::Slice<'proto, __root::pz::Field> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::Field>(self.ptr.as_ptr())
    }
  }
  pub fn fields_at(self, idx: usize) -> __rt::View<'proto, __root::pz::Field> {
    self.fields().at(idx)
  }

  pub fn nesteds(self) -> __rt::Slice<'proto, u32> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn nesteds_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.nesteds().at(idx)
  }

  pub fn attrs(self) -> __rt::View<'proto, __root::pz::Type_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::Type_Attrs>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Type_Attrs>(self.ptr.as_ptr()))
    }
  }

  pub fn span(self) -> __rt::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(7);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
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

impl __s::default::Default for __priv_Type::View<'_> {
  fn default() -> Self {
    __root::pz::Type::DEFAULT
  }
}

impl<'proto> __priv_Type::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::Type> {
    __priv_Type::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::Type> {
    __priv_Type::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::Type> {
    __priv_Type::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::Type::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::Type::__tdp_info())
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(self) -> __rt::View<'proto, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn package_mut(self) -> __rt::Mut<'proto, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(1),
      )
    }
  }
  pub fn package_set(self, value: u32) {
    self.package_mut().set(value);
  }

  pub fn kind(self) -> __rt::View<'proto, __root::pz::Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::Type_Kind>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Type_Kind>(self.ptr.as_ptr()))
    }
  }
  pub fn kind_mut(self) -> __rt::Mut<'proto, __root::pz::Type_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::Type_Kind> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(2),
      )
    }
  }
  pub fn kind_set(self, value: __root::pz::Type_Kind) {
    self.kind_mut().set(value);
  }

  pub fn declared_in(self) -> __rt::View<'proto, u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn declared_in_mut(self) -> __rt::Mut<'proto, u32> {
    self.declared_in_mut_or().into_mut()
  }
  pub fn declared_in_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(3),
      )
    }
  }
  pub fn declared_in_set(self, value: u32) {
    self.declared_in_mut().set(value);
  }

  pub fn fields(self) -> __rt::Slice<'proto, __root::pz::Field> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::Field>(self.ptr.as_ptr())
    }
  }
  pub fn fields_at(self, idx: usize) -> __rt::View<'proto, __root::pz::Field> {
    self.fields().at(idx)
  }
  pub fn fields_mut(self) -> __rt::Repeated<'proto, __root::pz::Field> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(4);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::Field>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn nesteds(self) -> __rt::Slice<'proto, u32> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn nesteds_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.nesteds().at(idx)
  }
  pub fn nesteds_mut(self) -> __rt::Repeated<'proto, u32> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(5);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn attrs(self) -> __rt::View<'proto, __root::pz::Type_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::Type_Attrs>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Type_Attrs>(self.ptr.as_ptr()))
    }
  }
  pub fn attrs_mut(self) -> __rt::Mut<'proto, __root::pz::Type_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::Type_Attrs> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(6),
      )
    }
  }

  pub fn span(self) -> __rt::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Type::__tdp_info().field(7);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn span_mut(self) -> __rt::Mut<'proto, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type::__tdp_info().field(7),
      )
    }
  }
  pub fn span_set(self, value: u32) {
    self.span_mut().set(value);
  }

}

impl __s::ops::Drop for __root::pz::Type {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_Type::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Type ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Type::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::Type {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Type {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: __z::RawStr,
    pub(in super) package: u32,
    pub(in super) kind: u32,
    pub(in super) declared_in: u32,
    pub(in super) fields: __z::AVec<*mut u8>,
    pub(in super) nesteds: __z::AVec<u32>,
    pub(in super) attrs: *mut u8,
    pub(in super) span: u32,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{8 + 1}> =
    __z::tdp::DescStorage::<{8 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::Type::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::Field::__tdp_info,
            __root::pz::Type_Attrs::__tdp_info,
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
            let msg = __root::pz::Type::DEFAULT;
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
            let msg = __root::pz::Type::DEFAULT;
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
            let msg = __root::pz::Type::DEFAULT;
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
            let msg = __root::pz::Type::DEFAULT;
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
            let msg = __root::pz::Type::DEFAULT;
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
            let msg = __root::pz::Type::DEFAULT;
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
            let msg = __root::pz::Type::DEFAULT;
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
            let msg = __root::pz::Type::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Type::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::Type>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Type> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Type::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::Type>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Type> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::Type> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// enum `pz.Type.Kind`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Type_Kind(pub i32);

impl __root::pz::Type_Kind {
  pub const Message: Self = Self(0);
  pub const Struct: Self = Self(1);
  pub const Choice: Self = Self(2);
  pub const Enum: Self = Self(3);

  pub const fn new() -> Self {
    Self(0)
  }
}

impl __s::default::Default for __root::pz::Type_Kind {
  fn default() -> Self {
    Self(0)
  }
}

impl __rt::ptr::Proxied for __root::pz::Type_Kind {
  type View<'a> = Self;
  type Mut<'a> = __rt::ptr::ScalarMut<'a, Self>;
}

impl __s::fmt::Debug for __root::pz::Type_Kind {
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
  ptr: __z::ABox<__priv_Type_Attrs::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Type_Attrs::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::Type_Attrs {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Type_Attrs::Storage = __priv_Type_Attrs::Storage {
      __hasbits: [0; 1],
      deprecated: __z::RawStr::new(),
      docs: __z::AVec::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_Type_Attrs::Storage as *mut __priv_Type_Attrs::Storage as *mut u8),
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
    __priv_Type_Attrs::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_Type_Attrs::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::Type_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn deprecated(&self) -> __rt::View<'_, __rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::Type_Attrs::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn deprecated_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type_Attrs::__tdp_info().field(0),
      )
    }
  }
  pub fn deprecated_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(&self) -> __rt::Slice<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::Type_Attrs::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn docs_at(&self, idx: usize) -> __rt::View<'_, __rt::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(&mut self) -> __rt::Repeated<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::Type_Attrs::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Type_Attrs::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Type_Attrs::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_Type_Attrs::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::Type_Attrs {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::Type_Attrs {
  type View<'proto> = __priv_Type_Attrs::View<'proto>;
  type Mut<'proto> = __priv_Type_Attrs::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::Type_Attrs {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_Type_Attrs::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_Type_Attrs::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Type_Attrs::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::Type_Attrs> {
    __priv_Type_Attrs::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn deprecated(self) -> __rt::View<'proto, __rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::Type_Attrs::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn docs(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::Type_Attrs::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn docs_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
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

impl __s::default::Default for __priv_Type_Attrs::View<'_> {
  fn default() -> Self {
    __root::pz::Type_Attrs::DEFAULT
  }
}

impl<'proto> __priv_Type_Attrs::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::Type_Attrs> {
    __priv_Type_Attrs::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::Type_Attrs> {
    __priv_Type_Attrs::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::Type_Attrs> {
    __priv_Type_Attrs::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::Type_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::Type_Attrs::__tdp_info())
  }

  pub fn deprecated(self) -> __rt::View<'proto, __rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::Type_Attrs::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn deprecated_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Type_Attrs::__tdp_info().field(0),
      )
    }
  }
  pub fn deprecated_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::Type_Attrs::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn docs_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(self) -> __rt::Repeated<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::Type_Attrs::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

}

impl __s::ops::Drop for __root::pz::Type_Attrs {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_Type_Attrs::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Type.Attrs ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Type_Attrs::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::Type_Attrs {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Type_Attrs {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) deprecated: __z::RawStr,
    pub(crate) docs: __z::AVec<(*mut u8, usize)>,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{2 + 1}> =
    __z::tdp::DescStorage::<{2 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::Type_Attrs::__LAYOUT.size();
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
            let msg = __root::pz::Type_Attrs::DEFAULT;
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
            let msg = __root::pz::Type_Attrs::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Type_Attrs::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::Type_Attrs>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Type_Attrs> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Type_Attrs::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::Type_Attrs>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Type_Attrs> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::Type_Attrs> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.Field`
pub struct Field {
  ptr: __z::ABox<__priv_Field::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Field::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::Field {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Field::Storage = __priv_Field::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      number: 0,
      is_repeated: false,
      r#type: __root::pz::Field_Type::new().0 as u32,
      type_index: 0,
      attrs: 0 as *mut u8,
      span: 0,
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_Field::Storage as *mut __priv_Field::Storage as *mut u8),
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
    __priv_Field::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_Field::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::Field::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> __rt::View<'_, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn number(&self) -> __rt::View<'_, i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(&self) -> __s::option::Option<__rt::View<'_, i32>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }
  pub fn number_mut(&mut self) -> __rt::Mut<'_, i32> {
    self.number_mut_or().into_mut()
  }
  pub fn number_mut_or(&mut self) -> __rt::value::OptMut<'_, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(1),
      )
    }
  }
  pub fn number_set(&mut self, value: i32) {
    self.number_mut().set(value);
  }

  pub fn is_repeated(&self) -> __rt::View<'_, bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(&self) -> __s::option::Option<__rt::View<'_, bool>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }
  pub fn is_repeated_mut(&mut self) -> __rt::Mut<'_, bool> {
    self.is_repeated_mut_or().into_mut()
  }
  pub fn is_repeated_mut_or(&mut self) -> __rt::value::OptMut<'_, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(2),
      )
    }
  }
  pub fn is_repeated_set(&mut self, value: bool) {
    self.is_repeated_mut().set(value);
  }

  pub fn r#type(&self) -> __rt::View<'_, __root::pz::Field_Type> {
    self.r#type_or().unwrap_or_default()
  }
  pub fn r#type_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::Field_Type>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Field_Type>(self.ptr.as_ptr()))
    }
  }
  pub fn r#type_mut(&mut self) -> __rt::Mut<'_, __root::pz::Field_Type> {
    self.r#type_mut_or().into_mut()
  }
  pub fn r#type_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::Field_Type> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(3),
      )
    }
  }
  pub fn r#type_set(&mut self, value: __root::pz::Field_Type) {
    self.r#type_mut().set(value);
  }

  pub fn type_index(&self) -> __rt::View<'_, u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(&self) -> __s::option::Option<__rt::View<'_, u32>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn type_index_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.type_index_mut_or().into_mut()
  }
  pub fn type_index_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(4),
      )
    }
  }
  pub fn type_index_set(&mut self, value: u32) {
    self.type_index_mut().set(value);
  }

  pub fn attrs(&self) -> __rt::View<'_, __root::pz::Field_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::Field_Attrs>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Field_Attrs>(self.ptr.as_ptr()))
    }
  }
  pub fn attrs_mut(&mut self) -> __rt::Mut<'_, __root::pz::Field_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::Field_Attrs> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(5),
      )
    }
  }

  pub fn span(&self) -> __rt::View<'_, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> __s::option::Option<__rt::View<'_, u32>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn span_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(6),
      )
    }
  }
  pub fn span_set(&mut self, value: u32) {
    self.span_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Field::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Field::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_Field::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::Field {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::Field {
  type View<'proto> = __priv_Field::View<'proto>;
  type Mut<'proto> = __priv_Field::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::Field {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_Field::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_Field::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Field::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::Field> {
    __priv_Field::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn number(self) -> __rt::View<'proto, i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(self) -> __s::option::Option<__rt::View<'proto, i32>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }

  pub fn is_repeated(self) -> __rt::View<'proto, bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(self) -> __s::option::Option<__rt::View<'proto, bool>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }

  pub fn r#type(self) -> __rt::View<'proto, __root::pz::Field_Type> {
    self.r#type_or().unwrap_or_default()
  }
  pub fn r#type_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::Field_Type>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Field_Type>(self.ptr.as_ptr()))
    }
  }

  pub fn type_index(self) -> __rt::View<'proto, u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }

  pub fn attrs(self) -> __rt::View<'proto, __root::pz::Field_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::Field_Attrs>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Field_Attrs>(self.ptr.as_ptr()))
    }
  }

  pub fn span(self) -> __rt::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
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
    if let __s::option::Option::Some(value) = self.r#type_or() {
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

impl __s::default::Default for __priv_Field::View<'_> {
  fn default() -> Self {
    __root::pz::Field::DEFAULT
  }
}

impl<'proto> __priv_Field::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::Field> {
    __priv_Field::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::Field> {
    __priv_Field::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::Field> {
    __priv_Field::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::Field::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::Field::__tdp_info())
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn number(self) -> __rt::View<'proto, i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(self) -> __s::option::Option<__rt::View<'proto, i32>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<i32>(self.ptr.as_ptr()))
    }
  }
  pub fn number_mut(self) -> __rt::Mut<'proto, i32> {
    self.number_mut_or().into_mut()
  }
  pub fn number_mut_or(self) -> __rt::value::OptMut<'proto, i32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(1),
      )
    }
  }
  pub fn number_set(self, value: i32) {
    self.number_mut().set(value);
  }

  pub fn is_repeated(self) -> __rt::View<'proto, bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(self) -> __s::option::Option<__rt::View<'proto, bool>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }
  pub fn is_repeated_mut(self) -> __rt::Mut<'proto, bool> {
    self.is_repeated_mut_or().into_mut()
  }
  pub fn is_repeated_mut_or(self) -> __rt::value::OptMut<'proto, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(2),
      )
    }
  }
  pub fn is_repeated_set(self, value: bool) {
    self.is_repeated_mut().set(value);
  }

  pub fn r#type(self) -> __rt::View<'proto, __root::pz::Field_Type> {
    self.r#type_or().unwrap_or_default()
  }
  pub fn r#type_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::Field_Type>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Field_Type>(self.ptr.as_ptr()))
    }
  }
  pub fn r#type_mut(self) -> __rt::Mut<'proto, __root::pz::Field_Type> {
    self.r#type_mut_or().into_mut()
  }
  pub fn r#type_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::Field_Type> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(3),
      )
    }
  }
  pub fn r#type_set(self, value: __root::pz::Field_Type) {
    self.r#type_mut().set(value);
  }

  pub fn type_index(self) -> __rt::View<'proto, u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(4);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn type_index_mut(self) -> __rt::Mut<'proto, u32> {
    self.type_index_mut_or().into_mut()
  }
  pub fn type_index_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(4),
      )
    }
  }
  pub fn type_index_set(self, value: u32) {
    self.type_index_mut().set(value);
  }

  pub fn attrs(self) -> __rt::View<'proto, __root::pz::Field_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::Field_Attrs>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(5);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Field_Attrs>(self.ptr.as_ptr()))
    }
  }
  pub fn attrs_mut(self) -> __rt::Mut<'proto, __root::pz::Field_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::Field_Attrs> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(5),
      )
    }
  }

  pub fn span(self) -> __rt::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::Field::__tdp_info().field(6);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn span_mut(self) -> __rt::Mut<'proto, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field::__tdp_info().field(6),
      )
    }
  }
  pub fn span_set(self, value: u32) {
    self.span_mut().set(value);
  }

}

impl __s::ops::Drop for __root::pz::Field {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_Field::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Field ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Field::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::Field {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Field {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: __z::RawStr,
    pub(in super) number: u32,
    pub(in super) is_repeated: bool,
    pub(in super) r#type: u32,
    pub(in super) type_index: u32,
    pub(in super) attrs: *mut u8,
    pub(in super) span: u32,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{7 + 1}> =
    __z::tdp::DescStorage::<{7 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::Field::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::Field_Attrs::__tdp_info,
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
            let msg = __root::pz::Field::DEFAULT;
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
            let msg = __root::pz::Field::DEFAULT;
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
            let msg = __root::pz::Field::DEFAULT;
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
            let msg = __root::pz::Field::DEFAULT;
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
            let msg = __root::pz::Field::DEFAULT;
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
            let msg = __root::pz::Field::DEFAULT;
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
            let msg = __root::pz::Field::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Field::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::Field>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Field> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Field::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::Field>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Field> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::Field> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// enum `pz.Field.Type`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Field_Type(pub i32);

impl __root::pz::Field_Type {
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

impl __s::default::Default for __root::pz::Field_Type {
  fn default() -> Self {
    Self(0)
  }
}

impl __rt::ptr::Proxied for __root::pz::Field_Type {
  type View<'a> = Self;
  type Mut<'a> = __rt::ptr::ScalarMut<'a, Self>;
}

impl __s::fmt::Debug for __root::pz::Field_Type {
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
  ptr: __z::ABox<__priv_Field_Attrs::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Field_Attrs::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::Field_Attrs {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Field_Attrs::Storage = __priv_Field_Attrs::Storage {
      __hasbits: [0; 1],
      deprecated: __z::RawStr::new(),
      docs: __z::AVec::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_Field_Attrs::Storage as *mut __priv_Field_Attrs::Storage as *mut u8),
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
    __priv_Field_Attrs::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_Field_Attrs::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::Field_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn deprecated(&self) -> __rt::View<'_, __rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::Field_Attrs::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn deprecated_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field_Attrs::__tdp_info().field(0),
      )
    }
  }
  pub fn deprecated_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(&self) -> __rt::Slice<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::Field_Attrs::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn docs_at(&self, idx: usize) -> __rt::View<'_, __rt::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(&mut self) -> __rt::Repeated<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::Field_Attrs::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Field_Attrs::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Field_Attrs::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_Field_Attrs::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::Field_Attrs {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::Field_Attrs {
  type View<'proto> = __priv_Field_Attrs::View<'proto>;
  type Mut<'proto> = __priv_Field_Attrs::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::Field_Attrs {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_Field_Attrs::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_Field_Attrs::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Field_Attrs::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::Field_Attrs> {
    __priv_Field_Attrs::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn deprecated(self) -> __rt::View<'proto, __rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::Field_Attrs::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn docs(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::Field_Attrs::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn docs_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
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

impl __s::default::Default for __priv_Field_Attrs::View<'_> {
  fn default() -> Self {
    __root::pz::Field_Attrs::DEFAULT
  }
}

impl<'proto> __priv_Field_Attrs::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::Field_Attrs> {
    __priv_Field_Attrs::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::Field_Attrs> {
    __priv_Field_Attrs::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::Field_Attrs> {
    __priv_Field_Attrs::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::Field_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::Field_Attrs::__tdp_info())
  }

  pub fn deprecated(self) -> __rt::View<'proto, __rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::Field_Attrs::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn deprecated_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::Field_Attrs::__tdp_info().field(0),
      )
    }
  }
  pub fn deprecated_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::Field_Attrs::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn docs_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(self) -> __rt::Repeated<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::Field_Attrs::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

}

impl __s::ops::Drop for __root::pz::Field_Attrs {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_Field_Attrs::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.Field.Attrs ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Field_Attrs::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::Field_Attrs {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Field_Attrs {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) deprecated: __z::RawStr,
    pub(crate) docs: __z::AVec<(*mut u8, usize)>,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{2 + 1}> =
    __z::tdp::DescStorage::<{2 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::Field_Attrs::__LAYOUT.size();
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
            let msg = __root::pz::Field_Attrs::DEFAULT;
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
            let msg = __root::pz::Field_Attrs::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Field_Attrs::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::Field_Attrs>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Field_Attrs> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Field_Attrs::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::Field_Attrs>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::Field_Attrs> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::Field_Attrs> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

pub mod plugin {
use super::{__root, __rt, __z, __s};
use __s::default::Default as _;
/// choice `pz.plugin.Request`
pub struct Request {
  ptr: __z::ABox<__priv_Request::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Request::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::Request {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Request::Storage = __priv_Request::Storage {
      which: 0,
      union: __priv_Request::Union { __unset: () },
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_Request::Storage as *mut __priv_Request::Storage as *mut u8),
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
    __priv_Request::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_Request::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(&self) -> __root::pz::plugin::RequestCases<'_, __rt::ptr::select::View> {
    self.as_view().cases()
  }

  pub fn cases_mut(&mut self) -> __root::pz::plugin::RequestCases<'_, __rt::ptr::select::Mut> {
    self.as_mut().cases_mut()
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::Request::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn about(&self) -> __rt::View<'_, __root::pz::plugin::AboutRequest> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::plugin::AboutRequest>> {
    unsafe {
      let field = __root::pz::plugin::Request::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::AboutRequest>(self.ptr.as_ptr()))
    }
  }
  pub fn about_mut(&mut self) -> __rt::Mut<'_, __root::pz::plugin::AboutRequest> {
    self.about_mut_or().into_mut()
  }
  pub fn about_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::plugin::AboutRequest> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Request::__tdp_info().field(0),
      )
    }
  }

  pub fn codegen(&self) -> __rt::View<'_, __root::pz::plugin::CodegenRequest> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::plugin::CodegenRequest>> {
    unsafe {
      let field = __root::pz::plugin::Request::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::CodegenRequest>(self.ptr.as_ptr()))
    }
  }
  pub fn codegen_mut(&mut self) -> __rt::Mut<'_, __root::pz::plugin::CodegenRequest> {
    self.codegen_mut_or().into_mut()
  }
  pub fn codegen_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::plugin::CodegenRequest> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Request::__tdp_info().field(1),
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Request::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Request::Storage>()).which = 0;
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_Request::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

pub enum RequestCases<'proto, Which: __rt::ptr::select::Select> {
  Unset(__s::marker::PhantomData<&'proto Which>),
  About(__rt::ptr::Proxy<'proto, __root::pz::plugin::AboutRequest, Which>),
  Codegen(__rt::ptr::Proxy<'proto, __root::pz::plugin::CodegenRequest, Which>),
}

impl __s::default::Default for __root::pz::plugin::Request {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::Request {
  type View<'proto> = __priv_Request::View<'proto>;
  type Mut<'proto> = __priv_Request::Mut<'proto>;
}

impl<'proto> __priv_Request::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::Request> {
    __priv_Request::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn cases(self) -> __root::pz::plugin::RequestCases<'proto, __rt::ptr::select::View> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      match number {
        0 => __root::pz::plugin::RequestCases::Unset(__s::marker::PhantomData),
        1 => __root::pz::plugin::RequestCases::About(__root::pz::plugin::Request::__tdp_info().field(0).make_view::<__root::pz::plugin::AboutRequest>(self.ptr.as_ptr())),
        2 => __root::pz::plugin::RequestCases::Codegen(__root::pz::plugin::Request::__tdp_info().field(1).make_view::<__root::pz::plugin::CodegenRequest>(self.ptr.as_ptr())),
        _ => __s::unreachable!(),
      }
    }
  }

  pub fn about(self) -> __rt::View<'proto, __root::pz::plugin::AboutRequest> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::plugin::AboutRequest>> {
    unsafe {
      let field = __root::pz::plugin::Request::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::AboutRequest>(self.ptr.as_ptr()))
    }
  }

  pub fn codegen(self) -> __rt::View<'proto, __root::pz::plugin::CodegenRequest> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::plugin::CodegenRequest>> {
    unsafe {
      let field = __root::pz::plugin::Request::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::CodegenRequest>(self.ptr.as_ptr()))
    }
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

impl __s::default::Default for __priv_Request::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::Request::DEFAULT
  }
}

impl<'proto> __priv_Request::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::Request> {
    __priv_Request::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::Request> {
    __priv_Request::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::Request> {
    __priv_Request::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(self) -> __root::pz::plugin::RequestCases<'proto, __rt::ptr::select::View> {
    self.into_view().cases()
  }

  pub fn cases_mut(self) -> __root::pz::plugin::RequestCases<'proto, __rt::ptr::select::Mut> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      match number {
        0 => __root::pz::plugin::RequestCases::Unset(__s::marker::PhantomData),
        1 => __root::pz::plugin::RequestCases::About(__root::pz::plugin::Request::__tdp_info().field(0).make_mut::<__root::pz::plugin::AboutRequest>(self.ptr.as_ptr(), self.arena)),
        2 => __root::pz::plugin::RequestCases::Codegen(__root::pz::plugin::Request::__tdp_info().field(1).make_mut::<__root::pz::plugin::CodegenRequest>(self.ptr.as_ptr(), self.arena)),
        _ => __s::unreachable!(),
      }
    }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::Request::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::Request::__tdp_info())
  }

  pub fn about(self) -> __rt::View<'proto, __root::pz::plugin::AboutRequest> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::plugin::AboutRequest>> {
    unsafe {
      let field = __root::pz::plugin::Request::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::AboutRequest>(self.ptr.as_ptr()))
    }
  }
  pub fn about_mut(self) -> __rt::Mut<'proto, __root::pz::plugin::AboutRequest> {
    self.about_mut_or().into_mut()
  }
  pub fn about_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::plugin::AboutRequest> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Request::__tdp_info().field(0),
      )
    }
  }

  pub fn codegen(self) -> __rt::View<'proto, __root::pz::plugin::CodegenRequest> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::plugin::CodegenRequest>> {
    unsafe {
      let field = __root::pz::plugin::Request::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::CodegenRequest>(self.ptr.as_ptr()))
    }
  }
  pub fn codegen_mut(self) -> __rt::Mut<'proto, __root::pz::plugin::CodegenRequest> {
    self.codegen_mut_or().into_mut()
  }
  pub fn codegen_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::plugin::CodegenRequest> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Request::__tdp_info().field(1),
      )
    }
  }

}

impl __s::ops::Drop for __root::pz::plugin::Request {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_Request::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.Request ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Request::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::Request {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __rt::value::Type for __root::pz::plugin::Request {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_Request::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_Request::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

mod __priv_Request {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(super) which: u32,
    pub(super) union: Union,
  }

  #[repr(C)]
  pub union Union {
    pub(super) __unset: (),
    pub(in super) about: *mut u8,
    pub(in super) codegen: *mut u8,
  }

  pub const UNION_OFFSET: usize = {
    let align = __s::mem::align_of::<__priv_Request::Union>();
    if align < 4 { 4 } else { align }
  };

  pub static TDP_INFO: __z::tdp::DescStorage<{2 + 1}> =
    __z::tdp::DescStorage::<{2 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::Request::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::plugin::AboutRequest::__tdp_info,
            __root::pz::plugin::CodegenRequest::__tdp_info,
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Request::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::Request>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::Request> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Request::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::Request>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::Request> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::Request> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// choice `pz.plugin.Response`
pub struct Response {
  ptr: __z::ABox<__priv_Response::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Response::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::Response {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Response::Storage = __priv_Response::Storage {
      which: 0,
      union: __priv_Response::Union { __unset: () },
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_Response::Storage as *mut __priv_Response::Storage as *mut u8),
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
    __priv_Response::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_Response::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(&self) -> __root::pz::plugin::ResponseCases<'_, __rt::ptr::select::View> {
    self.as_view().cases()
  }

  pub fn cases_mut(&mut self) -> __root::pz::plugin::ResponseCases<'_, __rt::ptr::select::Mut> {
    self.as_mut().cases_mut()
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::Response::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn about(&self) -> __rt::View<'_, __root::pz::plugin::AboutResponse> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::plugin::AboutResponse>> {
    unsafe {
      let field = __root::pz::plugin::Response::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::AboutResponse>(self.ptr.as_ptr()))
    }
  }
  pub fn about_mut(&mut self) -> __rt::Mut<'_, __root::pz::plugin::AboutResponse> {
    self.about_mut_or().into_mut()
  }
  pub fn about_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::plugin::AboutResponse> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Response::__tdp_info().field(0),
      )
    }
  }

  pub fn codegen(&self) -> __rt::View<'_, __root::pz::plugin::CodegenResponse> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::plugin::CodegenResponse>> {
    unsafe {
      let field = __root::pz::plugin::Response::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::CodegenResponse>(self.ptr.as_ptr()))
    }
  }
  pub fn codegen_mut(&mut self) -> __rt::Mut<'_, __root::pz::plugin::CodegenResponse> {
    self.codegen_mut_or().into_mut()
  }
  pub fn codegen_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::plugin::CodegenResponse> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Response::__tdp_info().field(1),
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Response::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Response::Storage>()).which = 0;
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_Response::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

pub enum ResponseCases<'proto, Which: __rt::ptr::select::Select> {
  Unset(__s::marker::PhantomData<&'proto Which>),
  About(__rt::ptr::Proxy<'proto, __root::pz::plugin::AboutResponse, Which>),
  Codegen(__rt::ptr::Proxy<'proto, __root::pz::plugin::CodegenResponse, Which>),
}

impl __s::default::Default for __root::pz::plugin::Response {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::Response {
  type View<'proto> = __priv_Response::View<'proto>;
  type Mut<'proto> = __priv_Response::Mut<'proto>;
}

impl<'proto> __priv_Response::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::Response> {
    __priv_Response::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn cases(self) -> __root::pz::plugin::ResponseCases<'proto, __rt::ptr::select::View> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      match number {
        0 => __root::pz::plugin::ResponseCases::Unset(__s::marker::PhantomData),
        1 => __root::pz::plugin::ResponseCases::About(__root::pz::plugin::Response::__tdp_info().field(0).make_view::<__root::pz::plugin::AboutResponse>(self.ptr.as_ptr())),
        2 => __root::pz::plugin::ResponseCases::Codegen(__root::pz::plugin::Response::__tdp_info().field(1).make_view::<__root::pz::plugin::CodegenResponse>(self.ptr.as_ptr())),
        _ => __s::unreachable!(),
      }
    }
  }

  pub fn about(self) -> __rt::View<'proto, __root::pz::plugin::AboutResponse> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::plugin::AboutResponse>> {
    unsafe {
      let field = __root::pz::plugin::Response::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::AboutResponse>(self.ptr.as_ptr()))
    }
  }

  pub fn codegen(self) -> __rt::View<'proto, __root::pz::plugin::CodegenResponse> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::plugin::CodegenResponse>> {
    unsafe {
      let field = __root::pz::plugin::Response::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::CodegenResponse>(self.ptr.as_ptr()))
    }
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

impl __s::default::Default for __priv_Response::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::Response::DEFAULT
  }
}

impl<'proto> __priv_Response::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::Response> {
    __priv_Response::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::Response> {
    __priv_Response::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::Response> {
    __priv_Response::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(self) -> __root::pz::plugin::ResponseCases<'proto, __rt::ptr::select::View> {
    self.into_view().cases()
  }

  pub fn cases_mut(self) -> __root::pz::plugin::ResponseCases<'proto, __rt::ptr::select::Mut> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      match number {
        0 => __root::pz::plugin::ResponseCases::Unset(__s::marker::PhantomData),
        1 => __root::pz::plugin::ResponseCases::About(__root::pz::plugin::Response::__tdp_info().field(0).make_mut::<__root::pz::plugin::AboutResponse>(self.ptr.as_ptr(), self.arena)),
        2 => __root::pz::plugin::ResponseCases::Codegen(__root::pz::plugin::Response::__tdp_info().field(1).make_mut::<__root::pz::plugin::CodegenResponse>(self.ptr.as_ptr(), self.arena)),
        _ => __s::unreachable!(),
      }
    }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::Response::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::Response::__tdp_info())
  }

  pub fn about(self) -> __rt::View<'proto, __root::pz::plugin::AboutResponse> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::plugin::AboutResponse>> {
    unsafe {
      let field = __root::pz::plugin::Response::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::AboutResponse>(self.ptr.as_ptr()))
    }
  }
  pub fn about_mut(self) -> __rt::Mut<'proto, __root::pz::plugin::AboutResponse> {
    self.about_mut_or().into_mut()
  }
  pub fn about_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::plugin::AboutResponse> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Response::__tdp_info().field(0),
      )
    }
  }

  pub fn codegen(self) -> __rt::View<'proto, __root::pz::plugin::CodegenResponse> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::plugin::CodegenResponse>> {
    unsafe {
      let field = __root::pz::plugin::Response::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::CodegenResponse>(self.ptr.as_ptr()))
    }
  }
  pub fn codegen_mut(self) -> __rt::Mut<'proto, __root::pz::plugin::CodegenResponse> {
    self.codegen_mut_or().into_mut()
  }
  pub fn codegen_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::plugin::CodegenResponse> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Response::__tdp_info().field(1),
      )
    }
  }

}

impl __s::ops::Drop for __root::pz::plugin::Response {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_Response::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.Response ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Response::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::Response {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __rt::value::Type for __root::pz::plugin::Response {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_Response::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_Response::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

mod __priv_Response {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(super) which: u32,
    pub(super) union: Union,
  }

  #[repr(C)]
  pub union Union {
    pub(super) __unset: (),
    pub(in super) about: *mut u8,
    pub(in super) codegen: *mut u8,
  }

  pub const UNION_OFFSET: usize = {
    let align = __s::mem::align_of::<__priv_Response::Union>();
    if align < 4 { 4 } else { align }
  };

  pub static TDP_INFO: __z::tdp::DescStorage<{2 + 1}> =
    __z::tdp::DescStorage::<{2 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::Response::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::plugin::AboutResponse::__tdp_info,
            __root::pz::plugin::CodegenResponse::__tdp_info,
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Response::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::Response>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::Response> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Response::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::Response>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::Response> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::Response> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.AboutRequest`
pub struct AboutRequest {
  ptr: __z::ABox<__priv_AboutRequest::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_AboutRequest::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::AboutRequest {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_AboutRequest::Storage = __priv_AboutRequest::Storage {
      __hasbits: [0; 0],
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_AboutRequest::Storage as *mut __priv_AboutRequest::Storage as *mut u8),
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
    __priv_AboutRequest::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_AboutRequest::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::AboutRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_AboutRequest::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_AboutRequest::Storage>()).__hasbits = [0; 0];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_AboutRequest::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::plugin::AboutRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::AboutRequest {
  type View<'proto> = __priv_AboutRequest::View<'proto>;
  type Mut<'proto> = __priv_AboutRequest::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::plugin::AboutRequest {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_AboutRequest::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_AboutRequest::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_AboutRequest::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::AboutRequest> {
    __priv_AboutRequest::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
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

impl __s::default::Default for __priv_AboutRequest::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::AboutRequest::DEFAULT
  }
}

impl<'proto> __priv_AboutRequest::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::AboutRequest> {
    __priv_AboutRequest::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::AboutRequest> {
    __priv_AboutRequest::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::AboutRequest> {
    __priv_AboutRequest::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::AboutRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::AboutRequest::__tdp_info())
  }

}

impl __s::ops::Drop for __root::pz::plugin::AboutRequest {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_AboutRequest::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.AboutRequest ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_AboutRequest::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::AboutRequest {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_AboutRequest {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 0],
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{0 + 1}> =
    __z::tdp::DescStorage::<{0 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::AboutRequest::__LAYOUT.size();
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_AboutRequest::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::AboutRequest>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::AboutRequest> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_AboutRequest::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::AboutRequest>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::AboutRequest> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::AboutRequest> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.AboutResponse`
pub struct AboutResponse {
  ptr: __z::ABox<__priv_AboutResponse::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_AboutResponse::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::AboutResponse {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_AboutResponse::Storage = __priv_AboutResponse::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      version: __z::RawStr::new(),
      options: __z::AVec::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_AboutResponse::Storage as *mut __priv_AboutResponse::Storage as *mut u8),
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
    __priv_AboutResponse::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_AboutResponse::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::AboutResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> __rt::View<'_, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::AboutResponse::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn version(&self) -> __rt::View<'_, __rt::Str> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn version_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.version_mut_or().into_mut()
  }
  pub fn version_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::AboutResponse::__tdp_info().field(1),
      )
    }
  }
  pub fn version_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.version_mut().set(value);
  }

  pub fn options(&self) -> __rt::Slice<'_, __root::pz::plugin::AboutResponse_Option> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::AboutResponse_Option>(self.ptr.as_ptr())
    }
  }
  pub fn options_at(&self, idx: usize) -> __rt::View<'_, __root::pz::plugin::AboutResponse_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(&mut self) -> __rt::Repeated<'_, __root::pz::plugin::AboutResponse_Option> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(2);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::plugin::AboutResponse_Option>(self.ptr.as_ptr(), self.arena)
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_AboutResponse::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_AboutResponse::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_AboutResponse::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::plugin::AboutResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::AboutResponse {
  type View<'proto> = __priv_AboutResponse::View<'proto>;
  type Mut<'proto> = __priv_AboutResponse::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::plugin::AboutResponse {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_AboutResponse::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_AboutResponse::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_AboutResponse::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::AboutResponse> {
    __priv_AboutResponse::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn version(self) -> __rt::View<'proto, __rt::Str> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn options(self) -> __rt::Slice<'proto, __root::pz::plugin::AboutResponse_Option> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::AboutResponse_Option>(self.ptr.as_ptr())
    }
  }
  pub fn options_at(self, idx: usize) -> __rt::View<'proto, __root::pz::plugin::AboutResponse_Option> {
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

impl __s::default::Default for __priv_AboutResponse::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::AboutResponse::DEFAULT
  }
}

impl<'proto> __priv_AboutResponse::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::AboutResponse> {
    __priv_AboutResponse::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::AboutResponse> {
    __priv_AboutResponse::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::AboutResponse> {
    __priv_AboutResponse::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::AboutResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::AboutResponse::__tdp_info())
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::AboutResponse::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn version(self) -> __rt::View<'proto, __rt::Str> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn version_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.version_mut_or().into_mut()
  }
  pub fn version_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::AboutResponse::__tdp_info().field(1),
      )
    }
  }
  pub fn version_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.version_mut().set(value);
  }

  pub fn options(self) -> __rt::Slice<'proto, __root::pz::plugin::AboutResponse_Option> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::AboutResponse_Option>(self.ptr.as_ptr())
    }
  }
  pub fn options_at(self, idx: usize) -> __rt::View<'proto, __root::pz::plugin::AboutResponse_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(self) -> __rt::Repeated<'proto, __root::pz::plugin::AboutResponse_Option> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse::__tdp_info().field(2);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::plugin::AboutResponse_Option>(self.ptr.as_ptr(), self.arena)
    }
  }

}

impl __s::ops::Drop for __root::pz::plugin::AboutResponse {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_AboutResponse::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.AboutResponse ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_AboutResponse::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::AboutResponse {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_AboutResponse {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: __z::RawStr,
    pub(in super) version: __z::RawStr,
    pub(in super) options: __z::AVec<*mut u8>,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{3 + 1}> =
    __z::tdp::DescStorage::<{3 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::AboutResponse::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::plugin::AboutResponse_Option::__tdp_info,
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
            let msg = __root::pz::plugin::AboutResponse::DEFAULT;
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
            let msg = __root::pz::plugin::AboutResponse::DEFAULT;
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
            let msg = __root::pz::plugin::AboutResponse::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_AboutResponse::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::AboutResponse>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::AboutResponse> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_AboutResponse::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::AboutResponse>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::AboutResponse> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::AboutResponse> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.AboutResponse.Option`
pub struct AboutResponse_Option {
  ptr: __z::ABox<__priv_AboutResponse_Option::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_AboutResponse_Option::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::AboutResponse_Option {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_AboutResponse_Option::Storage = __priv_AboutResponse_Option::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      help: __z::RawStr::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_AboutResponse_Option::Storage as *mut __priv_AboutResponse_Option::Storage as *mut u8),
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
    __priv_AboutResponse_Option::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_AboutResponse_Option::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::AboutResponse_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> __rt::View<'_, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse_Option::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::AboutResponse_Option::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn help(&self) -> __rt::View<'_, __rt::Str> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse_Option::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn help_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.help_mut_or().into_mut()
  }
  pub fn help_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::AboutResponse_Option::__tdp_info().field(1),
      )
    }
  }
  pub fn help_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.help_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_AboutResponse_Option::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_AboutResponse_Option::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_AboutResponse_Option::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::plugin::AboutResponse_Option {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::AboutResponse_Option {
  type View<'proto> = __priv_AboutResponse_Option::View<'proto>;
  type Mut<'proto> = __priv_AboutResponse_Option::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::plugin::AboutResponse_Option {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_AboutResponse_Option::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_AboutResponse_Option::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_AboutResponse_Option::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::AboutResponse_Option> {
    __priv_AboutResponse_Option::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse_Option::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn help(self) -> __rt::View<'proto, __rt::Str> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse_Option::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
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

impl __s::default::Default for __priv_AboutResponse_Option::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::AboutResponse_Option::DEFAULT
  }
}

impl<'proto> __priv_AboutResponse_Option::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::AboutResponse_Option> {
    __priv_AboutResponse_Option::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::AboutResponse_Option> {
    __priv_AboutResponse_Option::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::AboutResponse_Option> {
    __priv_AboutResponse_Option::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::AboutResponse_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::AboutResponse_Option::__tdp_info())
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse_Option::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::AboutResponse_Option::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn help(self) -> __rt::View<'proto, __rt::Str> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::AboutResponse_Option::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn help_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.help_mut_or().into_mut()
  }
  pub fn help_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::AboutResponse_Option::__tdp_info().field(1),
      )
    }
  }
  pub fn help_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.help_mut().set(value);
  }

}

impl __s::ops::Drop for __root::pz::plugin::AboutResponse_Option {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_AboutResponse_Option::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.AboutResponse.Option ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_AboutResponse_Option::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::AboutResponse_Option {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_AboutResponse_Option {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: __z::RawStr,
    pub(in super) help: __z::RawStr,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{2 + 1}> =
    __z::tdp::DescStorage::<{2 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::AboutResponse_Option::__LAYOUT.size();
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
            let msg = __root::pz::plugin::AboutResponse_Option::DEFAULT;
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
            let msg = __root::pz::plugin::AboutResponse_Option::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_AboutResponse_Option::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::AboutResponse_Option>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::AboutResponse_Option> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_AboutResponse_Option::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::AboutResponse_Option>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::AboutResponse_Option> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::AboutResponse_Option> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenRequest`
pub struct CodegenRequest {
  ptr: __z::ABox<__priv_CodegenRequest::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_CodegenRequest::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::CodegenRequest {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenRequest::Storage = __priv_CodegenRequest::Storage {
      __hasbits: [0; 1],
      bundle: 0 as *mut u8,
      requested_indices: __z::AVec::new(),
      options: __z::AVec::new(),
      debug: false,
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_CodegenRequest::Storage as *mut __priv_CodegenRequest::Storage as *mut u8),
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
    __priv_CodegenRequest::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_CodegenRequest::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::CodegenRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn bundle(&self) -> __rt::View<'_, __root::pz::Bundle> {
    self.bundle_or().unwrap_or_default()
  }
  pub fn bundle_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::Bundle>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Bundle>(self.ptr.as_ptr()))
    }
  }
  pub fn bundle_mut(&mut self) -> __rt::Mut<'_, __root::pz::Bundle> {
    self.bundle_mut_or().into_mut()
  }
  pub fn bundle_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::Bundle> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenRequest::__tdp_info().field(0),
      )
    }
  }

  pub fn requested_indices(&self) -> __rt::Slice<'_, u32> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn requested_indices_at(&self, idx: usize) -> __rt::View<'_, u32> {
    self.requested_indices().at(idx)
  }
  pub fn requested_indices_mut(&mut self) -> __rt::Repeated<'_, u32> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn options(&self) -> __rt::Slice<'_, __root::pz::plugin::CodegenRequest_Option> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::CodegenRequest_Option>(self.ptr.as_ptr())
    }
  }
  pub fn options_at(&self, idx: usize) -> __rt::View<'_, __root::pz::plugin::CodegenRequest_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(&mut self) -> __rt::Repeated<'_, __root::pz::plugin::CodegenRequest_Option> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(2);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::plugin::CodegenRequest_Option>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn debug(&self) -> __rt::View<'_, bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(&self) -> __s::option::Option<__rt::View<'_, bool>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }
  pub fn debug_mut(&mut self) -> __rt::Mut<'_, bool> {
    self.debug_mut_or().into_mut()
  }
  pub fn debug_mut_or(&mut self) -> __rt::value::OptMut<'_, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenRequest::__tdp_info().field(3),
      )
    }
  }
  pub fn debug_set(&mut self, value: bool) {
    self.debug_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_CodegenRequest::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_CodegenRequest::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_CodegenRequest::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::plugin::CodegenRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::CodegenRequest {
  type View<'proto> = __priv_CodegenRequest::View<'proto>;
  type Mut<'proto> = __priv_CodegenRequest::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::plugin::CodegenRequest {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_CodegenRequest::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_CodegenRequest::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_CodegenRequest::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::CodegenRequest> {
    __priv_CodegenRequest::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn bundle(self) -> __rt::View<'proto, __root::pz::Bundle> {
    self.bundle_or().unwrap_or_default()
  }
  pub fn bundle_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::Bundle>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Bundle>(self.ptr.as_ptr()))
    }
  }

  pub fn requested_indices(self) -> __rt::Slice<'proto, u32> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn requested_indices_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.requested_indices().at(idx)
  }

  pub fn options(self) -> __rt::Slice<'proto, __root::pz::plugin::CodegenRequest_Option> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::CodegenRequest_Option>(self.ptr.as_ptr())
    }
  }
  pub fn options_at(self, idx: usize) -> __rt::View<'proto, __root::pz::plugin::CodegenRequest_Option> {
    self.options().at(idx)
  }

  pub fn debug(self) -> __rt::View<'proto, bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(self) -> __s::option::Option<__rt::View<'proto, bool>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
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

impl __s::default::Default for __priv_CodegenRequest::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::CodegenRequest::DEFAULT
  }
}

impl<'proto> __priv_CodegenRequest::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::CodegenRequest> {
    __priv_CodegenRequest::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::CodegenRequest> {
    __priv_CodegenRequest::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::CodegenRequest> {
    __priv_CodegenRequest::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::CodegenRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::CodegenRequest::__tdp_info())
  }

  pub fn bundle(self) -> __rt::View<'proto, __root::pz::Bundle> {
    self.bundle_or().unwrap_or_default()
  }
  pub fn bundle_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::Bundle>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::Bundle>(self.ptr.as_ptr()))
    }
  }
  pub fn bundle_mut(self) -> __rt::Mut<'proto, __root::pz::Bundle> {
    self.bundle_mut_or().into_mut()
  }
  pub fn bundle_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::Bundle> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenRequest::__tdp_info().field(0),
      )
    }
  }

  pub fn requested_indices(self) -> __rt::Slice<'proto, u32> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<u32>(self.ptr.as_ptr())
    }
  }
  pub fn requested_indices_at(self, idx: usize) -> __rt::View<'proto, u32> {
    self.requested_indices().at(idx)
  }
  pub fn requested_indices_mut(self) -> __rt::Repeated<'proto, u32> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<u32>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn options(self) -> __rt::Slice<'proto, __root::pz::plugin::CodegenRequest_Option> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::CodegenRequest_Option>(self.ptr.as_ptr())
    }
  }
  pub fn options_at(self, idx: usize) -> __rt::View<'proto, __root::pz::plugin::CodegenRequest_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(self) -> __rt::Repeated<'proto, __root::pz::plugin::CodegenRequest_Option> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(2);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::plugin::CodegenRequest_Option>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn debug(self) -> __rt::View<'proto, bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(self) -> __s::option::Option<__rt::View<'proto, bool>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }
  pub fn debug_mut(self) -> __rt::Mut<'proto, bool> {
    self.debug_mut_or().into_mut()
  }
  pub fn debug_mut_or(self) -> __rt::value::OptMut<'proto, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenRequest::__tdp_info().field(3),
      )
    }
  }
  pub fn debug_set(self, value: bool) {
    self.debug_mut().set(value);
  }

}

impl __s::ops::Drop for __root::pz::plugin::CodegenRequest {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_CodegenRequest::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.CodegenRequest ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_CodegenRequest::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::CodegenRequest {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_CodegenRequest {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) bundle: *mut u8,
    pub(in super) requested_indices: __z::AVec<u32>,
    pub(in super) options: __z::AVec<*mut u8>,
    pub(in super) debug: bool,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{4 + 1}> =
    __z::tdp::DescStorage::<{4 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::CodegenRequest::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::Bundle::__tdp_info,
            __root::pz::plugin::CodegenRequest_Option::__tdp_info,
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
            let msg = __root::pz::plugin::CodegenRequest::DEFAULT;
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
            let msg = __root::pz::plugin::CodegenRequest::DEFAULT;
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
            let msg = __root::pz::plugin::CodegenRequest::DEFAULT;
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
            let msg = __root::pz::plugin::CodegenRequest::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_CodegenRequest::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::CodegenRequest>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::CodegenRequest> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_CodegenRequest::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::CodegenRequest>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::CodegenRequest> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::CodegenRequest> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenRequest.Option`
pub struct CodegenRequest_Option {
  ptr: __z::ABox<__priv_CodegenRequest_Option::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_CodegenRequest_Option::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::CodegenRequest_Option {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenRequest_Option::Storage = __priv_CodegenRequest_Option::Storage {
      __hasbits: [0; 1],
      name: __z::RawStr::new(),
      value: __z::RawStr::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_CodegenRequest_Option::Storage as *mut __priv_CodegenRequest_Option::Storage as *mut u8),
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
    __priv_CodegenRequest_Option::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_CodegenRequest_Option::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::CodegenRequest_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> __rt::View<'_, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest_Option::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenRequest_Option::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn value(&self) -> __rt::View<'_, __rt::Str> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest_Option::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn value_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.value_mut_or().into_mut()
  }
  pub fn value_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenRequest_Option::__tdp_info().field(1),
      )
    }
  }
  pub fn value_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.value_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_CodegenRequest_Option::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_CodegenRequest_Option::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_CodegenRequest_Option::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::plugin::CodegenRequest_Option {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::CodegenRequest_Option {
  type View<'proto> = __priv_CodegenRequest_Option::View<'proto>;
  type Mut<'proto> = __priv_CodegenRequest_Option::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::plugin::CodegenRequest_Option {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_CodegenRequest_Option::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_CodegenRequest_Option::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_CodegenRequest_Option::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::CodegenRequest_Option> {
    __priv_CodegenRequest_Option::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest_Option::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn value(self) -> __rt::View<'proto, __rt::Str> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest_Option::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
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

impl __s::default::Default for __priv_CodegenRequest_Option::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::CodegenRequest_Option::DEFAULT
  }
}

impl<'proto> __priv_CodegenRequest_Option::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::CodegenRequest_Option> {
    __priv_CodegenRequest_Option::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::CodegenRequest_Option> {
    __priv_CodegenRequest_Option::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::CodegenRequest_Option> {
    __priv_CodegenRequest_Option::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::CodegenRequest_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::CodegenRequest_Option::__tdp_info())
  }

  pub fn name(self) -> __rt::View<'proto, __rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest_Option::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn name_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenRequest_Option::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.name_mut().set(value);
  }

  pub fn value(self) -> __rt::View<'proto, __rt::Str> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenRequest_Option::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn value_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.value_mut_or().into_mut()
  }
  pub fn value_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenRequest_Option::__tdp_info().field(1),
      )
    }
  }
  pub fn value_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.value_mut().set(value);
  }

}

impl __s::ops::Drop for __root::pz::plugin::CodegenRequest_Option {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_CodegenRequest_Option::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.CodegenRequest.Option ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_CodegenRequest_Option::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::CodegenRequest_Option {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_CodegenRequest_Option {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: __z::RawStr,
    pub(in super) value: __z::RawStr,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{2 + 1}> =
    __z::tdp::DescStorage::<{2 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::CodegenRequest_Option::__LAYOUT.size();
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
            let msg = __root::pz::plugin::CodegenRequest_Option::DEFAULT;
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
            let msg = __root::pz::plugin::CodegenRequest_Option::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_CodegenRequest_Option::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::CodegenRequest_Option>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::CodegenRequest_Option> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_CodegenRequest_Option::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::CodegenRequest_Option>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::CodegenRequest_Option> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::CodegenRequest_Option> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenResponse`
pub struct CodegenResponse {
  ptr: __z::ABox<__priv_CodegenResponse::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_CodegenResponse::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::CodegenResponse {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenResponse::Storage = __priv_CodegenResponse::Storage {
      __hasbits: [0; 0],
      files: __z::AVec::new(),
      report: __z::AVec::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_CodegenResponse::Storage as *mut __priv_CodegenResponse::Storage as *mut u8),
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
    __priv_CodegenResponse::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_CodegenResponse::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::CodegenResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn files(&self) -> __rt::Slice<'_, __root::pz::plugin::CodegenResponse_File> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::CodegenResponse_File>(self.ptr.as_ptr())
    }
  }
  pub fn files_at(&self, idx: usize) -> __rt::View<'_, __root::pz::plugin::CodegenResponse_File> {
    self.files().at(idx)
  }
  pub fn files_mut(&mut self) -> __rt::Repeated<'_, __root::pz::plugin::CodegenResponse_File> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse::__tdp_info().field(0);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::plugin::CodegenResponse_File>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn report(&self) -> __rt::Slice<'_, __root::pz::plugin::Diagnostic> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::Diagnostic>(self.ptr.as_ptr())
    }
  }
  pub fn report_at(&self, idx: usize) -> __rt::View<'_, __root::pz::plugin::Diagnostic> {
    self.report().at(idx)
  }
  pub fn report_mut(&mut self) -> __rt::Repeated<'_, __root::pz::plugin::Diagnostic> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::plugin::Diagnostic>(self.ptr.as_ptr(), self.arena)
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_CodegenResponse::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_CodegenResponse::Storage>()).__hasbits = [0; 0];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_CodegenResponse::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::plugin::CodegenResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::CodegenResponse {
  type View<'proto> = __priv_CodegenResponse::View<'proto>;
  type Mut<'proto> = __priv_CodegenResponse::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::plugin::CodegenResponse {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_CodegenResponse::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_CodegenResponse::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_CodegenResponse::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::CodegenResponse> {
    __priv_CodegenResponse::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn files(self) -> __rt::Slice<'proto, __root::pz::plugin::CodegenResponse_File> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::CodegenResponse_File>(self.ptr.as_ptr())
    }
  }
  pub fn files_at(self, idx: usize) -> __rt::View<'proto, __root::pz::plugin::CodegenResponse_File> {
    self.files().at(idx)
  }

  pub fn report(self) -> __rt::Slice<'proto, __root::pz::plugin::Diagnostic> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::Diagnostic>(self.ptr.as_ptr())
    }
  }
  pub fn report_at(self, idx: usize) -> __rt::View<'proto, __root::pz::plugin::Diagnostic> {
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

impl __s::default::Default for __priv_CodegenResponse::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::CodegenResponse::DEFAULT
  }
}

impl<'proto> __priv_CodegenResponse::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::CodegenResponse> {
    __priv_CodegenResponse::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::CodegenResponse> {
    __priv_CodegenResponse::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::CodegenResponse> {
    __priv_CodegenResponse::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::CodegenResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::CodegenResponse::__tdp_info())
  }

  pub fn files(self) -> __rt::Slice<'proto, __root::pz::plugin::CodegenResponse_File> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::CodegenResponse_File>(self.ptr.as_ptr())
    }
  }
  pub fn files_at(self, idx: usize) -> __rt::View<'proto, __root::pz::plugin::CodegenResponse_File> {
    self.files().at(idx)
  }
  pub fn files_mut(self) -> __rt::Repeated<'proto, __root::pz::plugin::CodegenResponse_File> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse::__tdp_info().field(0);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::plugin::CodegenResponse_File>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn report(self) -> __rt::Slice<'proto, __root::pz::plugin::Diagnostic> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::Diagnostic>(self.ptr.as_ptr())
    }
  }
  pub fn report_at(self, idx: usize) -> __rt::View<'proto, __root::pz::plugin::Diagnostic> {
    self.report().at(idx)
  }
  pub fn report_mut(self) -> __rt::Repeated<'proto, __root::pz::plugin::Diagnostic> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse::__tdp_info().field(1);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::plugin::Diagnostic>(self.ptr.as_ptr(), self.arena)
    }
  }

}

impl __s::ops::Drop for __root::pz::plugin::CodegenResponse {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_CodegenResponse::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.CodegenResponse ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_CodegenResponse::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::CodegenResponse {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_CodegenResponse {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 0],
    pub(in super) files: __z::AVec<*mut u8>,
    pub(in super) report: __z::AVec<*mut u8>,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{2 + 1}> =
    __z::tdp::DescStorage::<{2 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::CodegenResponse::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::plugin::CodegenResponse_File::__tdp_info,
            __root::pz::plugin::Diagnostic::__tdp_info,
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
            let msg = __root::pz::plugin::CodegenResponse::DEFAULT;
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
            let msg = __root::pz::plugin::CodegenResponse::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_CodegenResponse::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::CodegenResponse>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::CodegenResponse> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_CodegenResponse::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::CodegenResponse>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::CodegenResponse> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::CodegenResponse> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenResponse.File`
pub struct CodegenResponse_File {
  ptr: __z::ABox<__priv_CodegenResponse_File::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_CodegenResponse_File::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::CodegenResponse_File {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenResponse_File::Storage = __priv_CodegenResponse_File::Storage {
      __hasbits: [0; 1],
      path: __z::RawStr::new(),
      content: __z::RawStr::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_CodegenResponse_File::Storage as *mut __priv_CodegenResponse_File::Storage as *mut u8),
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
    __priv_CodegenResponse_File::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_CodegenResponse_File::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::CodegenResponse_File::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn path(&self) -> __rt::View<'_, __rt::Str> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse_File::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn path_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.path_mut_or().into_mut()
  }
  pub fn path_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenResponse_File::__tdp_info().field(0),
      )
    }
  }
  pub fn path_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.path_mut().set(value);
  }

  pub fn content(&self) -> __rt::View<'_, __rt::Str> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse_File::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn content_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.content_mut_or().into_mut()
  }
  pub fn content_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenResponse_File::__tdp_info().field(1),
      )
    }
  }
  pub fn content_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.content_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_CodegenResponse_File::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_CodegenResponse_File::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_CodegenResponse_File::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::plugin::CodegenResponse_File {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::CodegenResponse_File {
  type View<'proto> = __priv_CodegenResponse_File::View<'proto>;
  type Mut<'proto> = __priv_CodegenResponse_File::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::plugin::CodegenResponse_File {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_CodegenResponse_File::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_CodegenResponse_File::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_CodegenResponse_File::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::CodegenResponse_File> {
    __priv_CodegenResponse_File::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn path(self) -> __rt::View<'proto, __rt::Str> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse_File::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn content(self) -> __rt::View<'proto, __rt::Str> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse_File::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
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

impl __s::default::Default for __priv_CodegenResponse_File::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::CodegenResponse_File::DEFAULT
  }
}

impl<'proto> __priv_CodegenResponse_File::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::CodegenResponse_File> {
    __priv_CodegenResponse_File::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::CodegenResponse_File> {
    __priv_CodegenResponse_File::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::CodegenResponse_File> {
    __priv_CodegenResponse_File::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::CodegenResponse_File::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::CodegenResponse_File::__tdp_info())
  }

  pub fn path(self) -> __rt::View<'proto, __rt::Str> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse_File::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn path_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.path_mut_or().into_mut()
  }
  pub fn path_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenResponse_File::__tdp_info().field(0),
      )
    }
  }
  pub fn path_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.path_mut().set(value);
  }

  pub fn content(self) -> __rt::View<'proto, __rt::Str> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::CodegenResponse_File::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn content_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.content_mut_or().into_mut()
  }
  pub fn content_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::CodegenResponse_File::__tdp_info().field(1),
      )
    }
  }
  pub fn content_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.content_mut().set(value);
  }

}

impl __s::ops::Drop for __root::pz::plugin::CodegenResponse_File {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_CodegenResponse_File::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.CodegenResponse.File ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_CodegenResponse_File::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::CodegenResponse_File {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_CodegenResponse_File {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) path: __z::RawStr,
    pub(in super) content: __z::RawStr,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{2 + 1}> =
    __z::tdp::DescStorage::<{2 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::CodegenResponse_File::__LAYOUT.size();
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
            let msg = __root::pz::plugin::CodegenResponse_File::DEFAULT;
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
            let msg = __root::pz::plugin::CodegenResponse_File::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_CodegenResponse_File::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::CodegenResponse_File>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::CodegenResponse_File> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_CodegenResponse_File::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::CodegenResponse_File>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::CodegenResponse_File> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::CodegenResponse_File> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.Diagnostic`
pub struct Diagnostic {
  ptr: __z::ABox<__priv_Diagnostic::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Diagnostic::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::Diagnostic {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Diagnostic::Storage = __priv_Diagnostic::Storage {
      __hasbits: [0; 1],
      kind: __root::pz::plugin::Diagnostic_Kind::new().0 as u32,
      msg: __z::RawStr::new(),
      snippets: __z::AVec::new(),
      notes: __z::AVec::new(),
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_Diagnostic::Storage as *mut __priv_Diagnostic::Storage as *mut u8),
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
    __priv_Diagnostic::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_Diagnostic::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::Diagnostic::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn kind(&self) -> __rt::View<'_, __root::pz::plugin::Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(&self) -> __s::option::Option<__rt::View<'_, __root::pz::plugin::Diagnostic_Kind>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::Diagnostic_Kind>(self.ptr.as_ptr()))
    }
  }
  pub fn kind_mut(&mut self) -> __rt::Mut<'_, __root::pz::plugin::Diagnostic_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(&mut self) -> __rt::value::OptMut<'_, __root::pz::plugin::Diagnostic_Kind> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Diagnostic::__tdp_info().field(0),
      )
    }
  }
  pub fn kind_set(&mut self, value: __root::pz::plugin::Diagnostic_Kind) {
    self.kind_mut().set(value);
  }

  pub fn msg(&self) -> __rt::View<'_, __rt::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn msg_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.msg_mut_or().into_mut()
  }
  pub fn msg_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Diagnostic::__tdp_info().field(1),
      )
    }
  }
  pub fn msg_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.msg_mut().set(value);
  }

  pub fn snippets(&self) -> __rt::Slice<'_, __root::pz::plugin::Diagnostic_Snippet> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::Diagnostic_Snippet>(self.ptr.as_ptr())
    }
  }
  pub fn snippets_at(&self, idx: usize) -> __rt::View<'_, __root::pz::plugin::Diagnostic_Snippet> {
    self.snippets().at(idx)
  }
  pub fn snippets_mut(&mut self) -> __rt::Repeated<'_, __root::pz::plugin::Diagnostic_Snippet> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(2);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::plugin::Diagnostic_Snippet>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn notes(&self) -> __rt::Slice<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn notes_at(&self, idx: usize) -> __rt::View<'_, __rt::Str> {
    self.notes().at(idx)
  }
  pub fn notes_mut(&mut self) -> __rt::Repeated<'_, __rt::Str> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(3);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Diagnostic::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Diagnostic::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_Diagnostic::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::plugin::Diagnostic {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::Diagnostic {
  type View<'proto> = __priv_Diagnostic::View<'proto>;
  type Mut<'proto> = __priv_Diagnostic::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::plugin::Diagnostic {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_Diagnostic::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_Diagnostic::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Diagnostic::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::Diagnostic> {
    __priv_Diagnostic::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn kind(self) -> __rt::View<'proto, __root::pz::plugin::Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::plugin::Diagnostic_Kind>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::Diagnostic_Kind>(self.ptr.as_ptr()))
    }
  }

  pub fn msg(self) -> __rt::View<'proto, __rt::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn snippets(self) -> __rt::Slice<'proto, __root::pz::plugin::Diagnostic_Snippet> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::Diagnostic_Snippet>(self.ptr.as_ptr())
    }
  }
  pub fn snippets_at(self, idx: usize) -> __rt::View<'proto, __root::pz::plugin::Diagnostic_Snippet> {
    self.snippets().at(idx)
  }

  pub fn notes(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn notes_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
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

impl __s::default::Default for __priv_Diagnostic::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::Diagnostic::DEFAULT
  }
}

impl<'proto> __priv_Diagnostic::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::Diagnostic> {
    __priv_Diagnostic::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::Diagnostic> {
    __priv_Diagnostic::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::Diagnostic> {
    __priv_Diagnostic::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::Diagnostic::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::Diagnostic::__tdp_info())
  }

  pub fn kind(self) -> __rt::View<'proto, __root::pz::plugin::Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> __s::option::Option<__rt::View<'proto, __root::pz::plugin::Diagnostic_Kind>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__root::pz::plugin::Diagnostic_Kind>(self.ptr.as_ptr()))
    }
  }
  pub fn kind_mut(self) -> __rt::Mut<'proto, __root::pz::plugin::Diagnostic_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(self) -> __rt::value::OptMut<'proto, __root::pz::plugin::Diagnostic_Kind> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Diagnostic::__tdp_info().field(0),
      )
    }
  }
  pub fn kind_set(self, value: __root::pz::plugin::Diagnostic_Kind) {
    self.kind_mut().set(value);
  }

  pub fn msg(self) -> __rt::View<'proto, __rt::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn msg_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.msg_mut_or().into_mut()
  }
  pub fn msg_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Diagnostic::__tdp_info().field(1),
      )
    }
  }
  pub fn msg_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.msg_mut().set(value);
  }

  pub fn snippets(self) -> __rt::Slice<'proto, __root::pz::plugin::Diagnostic_Snippet> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__root::pz::plugin::Diagnostic_Snippet>(self.ptr.as_ptr())
    }
  }
  pub fn snippets_at(self, idx: usize) -> __rt::View<'proto, __root::pz::plugin::Diagnostic_Snippet> {
    self.snippets().at(idx)
  }
  pub fn snippets_mut(self) -> __rt::Repeated<'proto, __root::pz::plugin::Diagnostic_Snippet> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(2);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__root::pz::plugin::Diagnostic_Snippet>(self.ptr.as_ptr(), self.arena)
    }
  }

  pub fn notes(self) -> __rt::Slice<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(3);
      if field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
      field.make_slice::<__rt::Str>(self.ptr.as_ptr())
    }
  }
  pub fn notes_at(self, idx: usize) -> __rt::View<'proto, __rt::Str> {
    self.notes().at(idx)
  }
  pub fn notes_mut(self) -> __rt::Repeated<'proto, __rt::Str> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic::__tdp_info().field(3);
      field.init(self.ptr.as_ptr(), self.arena);
      field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
    }
  }

}

impl __s::ops::Drop for __root::pz::plugin::Diagnostic {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_Diagnostic::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.Diagnostic ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Diagnostic::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::Diagnostic {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Diagnostic {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) kind: u32,
    pub(in super) msg: __z::RawStr,
    pub(in super) snippets: __z::AVec<*mut u8>,
    pub(crate) notes: __z::AVec<(*mut u8, usize)>,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{4 + 1}> =
    __z::tdp::DescStorage::<{4 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::Diagnostic::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> __z::tdp::Desc] = &[
            __root::pz::plugin::Diagnostic_Snippet::__tdp_info,
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
            let msg = __root::pz::plugin::Diagnostic::DEFAULT;
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
            let msg = __root::pz::plugin::Diagnostic::DEFAULT;
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
            let msg = __root::pz::plugin::Diagnostic::DEFAULT;
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
            let msg = __root::pz::plugin::Diagnostic::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Diagnostic::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::Diagnostic>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::Diagnostic> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Diagnostic::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::Diagnostic>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::Diagnostic> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::Diagnostic> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

/// enum `pz.plugin.Diagnostic.Kind`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Diagnostic_Kind(pub i32);

impl __root::pz::plugin::Diagnostic_Kind {
  pub const Error: Self = Self(0);
  pub const Warning: Self = Self(1);

  pub const fn new() -> Self {
    Self(0)
  }
}

impl __s::default::Default for __root::pz::plugin::Diagnostic_Kind {
  fn default() -> Self {
    Self(0)
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::Diagnostic_Kind {
  type View<'a> = Self;
  type Mut<'a> = __rt::ptr::ScalarMut<'a, Self>;
}

impl __s::fmt::Debug for __root::pz::plugin::Diagnostic_Kind {
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
  ptr: __z::ABox<__priv_Diagnostic_Snippet::Storage>,
  arena: __z::RawArena,
}

const _: () = {
  assert!(
    __s::mem::size_of::<__priv_Diagnostic_Snippet::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl __root::pz::plugin::Diagnostic_Snippet {
  pub const DEFAULT: __rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Diagnostic_Snippet::Storage = __priv_Diagnostic_Snippet::Storage {
      __hasbits: [0; 1],
      span: 0,
      message: __z::RawStr::new(),
      is_remark: false,
    };
    __rt::View::<Self> {
      ptr: __z::ABox::from_ptr(&VALUE as *const __priv_Diagnostic_Snippet::Storage as *mut __priv_Diagnostic_Snippet::Storage as *mut u8),
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
    __priv_Diagnostic_Snippet::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<Self> {
    __priv_Diagnostic_Snippet::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { __root::pz::plugin::Diagnostic_Snippet::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn span(&self) -> __rt::View<'_, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> __s::option::Option<__rt::View<'_, u32>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn span_mut(&mut self) -> __rt::Mut<'_, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(&mut self) -> __rt::value::OptMut<'_, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(0),
      )
    }
  }
  pub fn span_set(&mut self, value: u32) {
    self.span_mut().set(value);
  }

  pub fn message(&self) -> __rt::View<'_, __rt::Str> {
    self.message_or().unwrap_or_default()
  }
  pub fn message_or(&self) -> __s::option::Option<__rt::View<'_, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn message_mut(&mut self) -> __rt::Mut<'_, __rt::Str> {
    self.message_mut_or().into_mut()
  }
  pub fn message_mut_or(&mut self) -> __rt::value::OptMut<'_, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(1),
      )
    }
  }
  pub fn message_set(&mut self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.message_mut().set(value);
  }

  pub fn is_remark(&self) -> __rt::View<'_, bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(&self) -> __s::option::Option<__rt::View<'_, bool>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }
  pub fn is_remark_mut(&mut self) -> __rt::Mut<'_, bool> {
    self.is_remark_mut_or().into_mut()
  }
  pub fn is_remark_mut_or(&mut self) -> __rt::value::OptMut<'_, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(2),
      )
    }
  }
  pub fn is_remark_set(&mut self, value: bool) {
    self.is_remark_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: __s::alloc::Layout = __s::alloc::Layout::new::<__priv_Diagnostic_Snippet::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Diagnostic_Snippet::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> __z::tdp::Desc {
    unsafe { __priv_Diagnostic_Snippet::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    __s::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl __s::default::Default for __root::pz::plugin::Diagnostic_Snippet {
  fn default() -> Self {
    Self::new()
  }
}

impl __rt::ptr::Proxied for __root::pz::plugin::Diagnostic_Snippet {
  type View<'proto> = __priv_Diagnostic_Snippet::View<'proto>;
  type Mut<'proto> = __priv_Diagnostic_Snippet::Mut<'proto>;
}

impl __rt::value::Type for __root::pz::plugin::Diagnostic_Snippet {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *const *mut u8) -> __rt::View<'a, Self> {
    __priv_Diagnostic_Snippet::View {
      ptr: __z::ABox::from_ptr(ptr.read()),
      _ph: __s::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut *mut u8, arena: __z::RawArena) -> __rt::Mut<'a, Self> {
    __priv_Diagnostic_Snippet::Mut {
      ptr: __z::ABox::from_ptr(ptr.read()),
      arena,
      _ph: __s::marker::PhantomData,
    }
  }

  unsafe fn __resize(vec: &mut __z::AVec<*mut u8>, new_len: usize, arena: __z::RawArena) {
    vec.resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Diagnostic_Snippet::View<'proto> {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::Diagnostic_Snippet> {
    __priv_Diagnostic_Snippet::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn span(self) -> __rt::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }

  pub fn message(self) -> __rt::View<'proto, __rt::Str> {
    self.message_or().unwrap_or_default()
  }
  pub fn message_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }

  pub fn is_remark(self) -> __rt::View<'proto, bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(self) -> __s::option::Option<__rt::View<'proto, bool>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
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

impl __s::default::Default for __priv_Diagnostic_Snippet::View<'_> {
  fn default() -> Self {
    __root::pz::plugin::Diagnostic_Snippet::DEFAULT
  }
}

impl<'proto> __priv_Diagnostic_Snippet::Mut<'proto>  {
  pub fn as_view(&self) -> __rt::View<__root::pz::plugin::Diagnostic_Snippet> {
    __priv_Diagnostic_Snippet::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn into_view(self) -> __rt::View<'proto, __root::pz::plugin::Diagnostic_Snippet> {
    __priv_Diagnostic_Snippet::View { ptr: self.ptr, _ph: __s::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> __rt::Mut<__root::pz::plugin::Diagnostic_Snippet> {
    __priv_Diagnostic_Snippet::Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { __root::pz::plugin::Diagnostic_Snippet::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn __s::io::Read) -> __s::result::Result<(), __rt::Error> {
    let mut ctx = __z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, __root::pz::plugin::Diagnostic_Snippet::__tdp_info())
  }

  pub fn span(self) -> __rt::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> __s::option::Option<__rt::View<'proto, u32>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(0);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<u32>(self.ptr.as_ptr()))
    }
  }
  pub fn span_mut(self) -> __rt::Mut<'proto, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(self) -> __rt::value::OptMut<'proto, u32> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(0),
      )
    }
  }
  pub fn span_set(self, value: u32) {
    self.span_mut().set(value);
  }

  pub fn message(self) -> __rt::View<'proto, __rt::Str> {
    self.message_or().unwrap_or_default()
  }
  pub fn message_or(self) -> __s::option::Option<__rt::View<'proto, __rt::Str>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(1);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
    }
  }
  pub fn message_mut(self) -> __rt::Mut<'proto, __rt::Str> {
    self.message_mut_or().into_mut()
  }
  pub fn message_mut_or(self) -> __rt::value::OptMut<'proto, __rt::Str> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(1),
      )
    }
  }
  pub fn message_set(self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
    self.message_mut().set(value);
  }

  pub fn is_remark(self) -> __rt::View<'proto, bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(self) -> __s::option::Option<__rt::View<'proto, bool>> {
    unsafe {
      let field = __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(2);
      if field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
      __s::option::Option::Some(field.make_view::<bool>(self.ptr.as_ptr()))
    }
  }
  pub fn is_remark_mut(self) -> __rt::Mut<'proto, bool> {
    self.is_remark_mut_or().into_mut()
  }
  pub fn is_remark_mut_or(self) -> __rt::value::OptMut<'proto, bool> {
    unsafe {
      __rt::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        __root::pz::plugin::Diagnostic_Snippet::__tdp_info().field(2),
      )
    }
  }
  pub fn is_remark_set(self, value: bool) {
    self.is_remark_mut().set(value);
  }

}

impl __s::ops::Drop for __root::pz::plugin::Diagnostic_Snippet {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl __s::fmt::Debug for __priv_Diagnostic_Snippet::View<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    fmt.write_str("pz.plugin.Diagnostic.Snippet ")?;
    let mut debug = __z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl __s::fmt::Debug for __priv_Diagnostic_Snippet::Mut<'_> {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    use __rt::ptr::ViewFor;
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl __s::fmt::Debug for __root::pz::plugin::Diagnostic_Snippet {
  fn fmt(&self, fmt: &mut __s::fmt::Formatter) -> __s::fmt::Result {
    __s::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Diagnostic_Snippet {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) span: u32,
    pub(in super) message: __z::RawStr,
    pub(in super) is_remark: bool,
  }

  pub static TDP_INFO: __z::tdp::DescStorage<{3 + 1}> =
    __z::tdp::DescStorage::<{3 + 1}> {
      header: __z::tdp::DescHeader {
        size: {
          let size = __root::pz::plugin::Diagnostic_Snippet::__LAYOUT.size();
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
            let msg = __root::pz::plugin::Diagnostic_Snippet::DEFAULT;
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
            let msg = __root::pz::plugin::Diagnostic_Snippet::DEFAULT;
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
            let msg = __root::pz::plugin::Diagnostic_Snippet::DEFAULT;
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

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Diagnostic_Snippet::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto __root::pz::plugin::Diagnostic_Snippet>,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::Diagnostic_Snippet> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: __z::ABox<__priv_Diagnostic_Snippet::Storage>,
    pub(in super) _ph: __s::marker::PhantomData<&'proto mut __root::pz::plugin::Diagnostic_Snippet>,
    pub(in super) arena: __z::RawArena,
  }

  impl<'proto> __rt::ptr::ViewFor<'proto, super::__root::pz::plugin::Diagnostic_Snippet> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }
  }

  impl<'proto> __rt::ptr::MutFor<'proto, super::__root::pz::plugin::Diagnostic_Snippet> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: __s::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: __s::marker::PhantomData, arena: self.arena }
    }
  }
}

} // mod plugin
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

