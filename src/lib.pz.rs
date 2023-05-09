// ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]

/// message `pz.Bundle`
pub struct Bundle {
  ptr: crate::rt::__z::ABox<__priv_Bundle::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl Bundle {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Bundle::Storage = __priv_Bundle::Storage {
      __hasbits: [0; 0],
      types: crate::rt::__z::AVec::new(),
      packages: crate::rt::__z::AVec::new(),
      foreign_types: crate::rt::__z::AVec::new(),
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_Bundle::Storage as *mut __priv_Bundle::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_Bundle::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_Bundle::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Bundle::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn types(&self) -> crate::rt::Slice<'_, Type> {
    unsafe {
      let vec = &self.ptr.as_ref().types;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn types_at(&self, idx: usize) -> crate::rt::View<'_, Type> {
    self.types().at(idx)
  }
  pub fn types_mut(&mut self) -> crate::rt::Repeated<'_, Type> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().types) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn packages(&self) -> crate::rt::Slice<'_, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().packages;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn packages_at(&self, idx: usize) -> crate::rt::View<'_, crate::rt::Str> {
    self.packages().at(idx)
  }
  pub fn packages_mut(&mut self) -> crate::rt::Repeated<'_, crate::rt::Str> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().packages) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn foreign_types(&self) -> crate::rt::Slice<'_, Bundle_ForeignType> {
    unsafe {
      let vec = &self.ptr.as_ref().foreign_types;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn foreign_types_at(&self, idx: usize) -> crate::rt::View<'_, Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }
  pub fn foreign_types_mut(&mut self) -> crate::rt::Repeated<'_, Bundle_ForeignType> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().foreign_types) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_Bundle::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Bundle::Storage>()).__hasbits = [0; 0];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_Bundle::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_Bundle::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

}

impl Default for Bundle {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for Bundle {
  type View<'msg> = __priv_Bundle::View<'msg>;
  type Mut<'msg> = __priv_Bundle::Mut<'msg>;
}

impl crate::rt::value::Type for Bundle {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_Bundle::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_Bundle::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_Bundle::View<'msg> {
  pub fn types(self) -> crate::rt::Slice<'msg, Type> {
    unsafe {
      let vec = &self.ptr.as_ref().types;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn types_at(self, idx: usize) -> crate::rt::View<'msg, Type> {
    self.types().at(idx)
  }

  pub fn packages(self) -> crate::rt::Slice<'msg, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().packages;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn packages_at(self, idx: usize) -> crate::rt::View<'msg, crate::rt::Str> {
    self.packages().at(idx)
  }

  pub fn foreign_types(self) -> crate::rt::Slice<'msg, Bundle_ForeignType> {
    unsafe {
      let vec = &self.ptr.as_ref().foreign_types;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn foreign_types_at(self, idx: usize) -> crate::rt::View<'msg, Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
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
    Ok(())
  }
}

impl Default for __priv_Bundle::View<'_> {
  fn default() -> Self {
    Bundle::DEFAULT
  }
}

impl<'msg> __priv_Bundle::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { Bundle::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Bundle::__tdp_info())
  }

  pub fn types(self) -> crate::rt::Slice<'msg, Type> {
    unsafe {
      let vec = &self.ptr.as_ref().types;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn types_at(self, idx: usize) -> crate::rt::View<'msg, Type> {
    self.types().at(idx)
  }
  pub fn types_mut(self) -> crate::rt::Repeated<'msg, Type> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().types) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn packages(self) -> crate::rt::Slice<'msg, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().packages;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn packages_at(self, idx: usize) -> crate::rt::View<'msg, crate::rt::Str> {
    self.packages().at(idx)
  }
  pub fn packages_mut(self) -> crate::rt::Repeated<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().packages) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn foreign_types(self) -> crate::rt::Slice<'msg, Bundle_ForeignType> {
    unsafe {
      let vec = &self.ptr.as_ref().foreign_types;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn foreign_types_at(self, idx: usize) -> crate::rt::View<'msg, Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }
  pub fn foreign_types_mut(self) -> crate::rt::Repeated<'msg, Bundle_ForeignType> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().foreign_types) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

}

impl Drop for Bundle {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_Bundle::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.Bundle ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Bundle::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for Bundle {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Bundle {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 0],
    pub(in super) types: crate::rt::__z::AVec<*mut u8>,
    pub(crate) packages: crate::rt::__z::AVec<(*mut u8, usize)>,
    pub(in super) foreign_types: crate::rt::__z::AVec<*mut u8>,
  }

  pub const FIELD_OFFSET_types: u32 = unsafe {
    let msg = Bundle::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().types as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_packages: u32 = unsafe {
    let msg = Bundle::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().packages as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_foreign_types: u32 = unsafe {
    let msg = Bundle::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().foreign_types as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{3 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{3 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = Bundle::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
            Bundle_ForeignType::__tdp_info,
            Type::__tdp_info,
          ];
          TYS.as_ptr()
        },
        raw_init: Bundle::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (1 << 4),
          offset: __priv_Bundle::FIELD_OFFSET_types,
          ty: 1,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (1 << 4),
          offset: __priv_Bundle::FIELD_OFFSET_packages,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 3,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (1 << 4),
          offset: __priv_Bundle::FIELD_OFFSET_foreign_types,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Bundle::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg Bundle>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Bundle> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Bundle::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut Bundle>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Bundle> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Bundle> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.Bundle.ForeignType`
pub struct Bundle_ForeignType {
  ptr: crate::rt::__z::ABox<__priv_Bundle_ForeignType::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl Bundle_ForeignType {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Bundle_ForeignType::Storage = __priv_Bundle_ForeignType::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      package: 0,
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_Bundle_ForeignType::Storage as *mut __priv_Bundle_ForeignType::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_Bundle_ForeignType::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_Bundle_ForeignType::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Bundle_ForeignType::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Bundle_ForeignType::FIELD_OFFSET_name as usize),
        self.arena,
        Bundle_ForeignType::__hazzer_name,
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(&self) -> crate::rt::View<'_, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(&self) -> Option<crate::rt::View<'_, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().package) })
  }
  pub fn package_mut(&mut self) -> crate::rt::Mut<'_, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(&mut self) -> crate::rt::value::OptMut<'_, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Bundle_ForeignType::FIELD_OFFSET_package as usize),
        self.arena,
        Bundle_ForeignType::__hazzer_package,
      )
    }
  }
  pub fn package_set(&mut self, value: u32) {
    self.package_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_Bundle_ForeignType::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Bundle_ForeignType::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_Bundle_ForeignType::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_Bundle_ForeignType::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_name(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Bundle_ForeignType::FIELD_OFFSET_name as usize;
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
  pub unsafe fn __hazzer_package(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Bundle_ForeignType::FIELD_OFFSET_package as usize;
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
}

impl Default for Bundle_ForeignType {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for Bundle_ForeignType {
  type View<'msg> = __priv_Bundle_ForeignType::View<'msg>;
  type Mut<'msg> = __priv_Bundle_ForeignType::Mut<'msg>;
}

impl crate::rt::value::Type for Bundle_ForeignType {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_Bundle_ForeignType::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_Bundle_ForeignType::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_Bundle_ForeignType::View<'msg> {
  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn package(self) -> crate::rt::View<'msg, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().package) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.package_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("package")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_Bundle_ForeignType::View<'_> {
  fn default() -> Self {
    Bundle_ForeignType::DEFAULT
  }
}

impl<'msg> __priv_Bundle_ForeignType::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { Bundle_ForeignType::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Bundle_ForeignType::__tdp_info())
  }

  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Bundle_ForeignType::FIELD_OFFSET_name as usize),
        self.arena,
        Bundle_ForeignType::__hazzer_name,
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(self) -> crate::rt::View<'msg, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().package) })
  }
  pub fn package_mut(self) -> crate::rt::Mut<'msg, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(self) -> crate::rt::value::OptMut<'msg, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Bundle_ForeignType::FIELD_OFFSET_package as usize),
        self.arena,
        Bundle_ForeignType::__hazzer_package,
      )
    }
  }
  pub fn package_set(self, value: u32) {
    self.package_mut().set(value);
  }

}

impl Drop for Bundle_ForeignType {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_Bundle_ForeignType::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.Bundle.ForeignType ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Bundle_ForeignType::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for Bundle_ForeignType {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Bundle_ForeignType {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: (*mut u8, usize),
    pub(in super) package: u32,
  }

  pub const FIELD_OFFSET_name: u32 = unsafe {
    let msg = Bundle_ForeignType::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().name as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_package: u32 = unsafe {
    let msg = Bundle_ForeignType::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().package as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{2 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{2 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = Bundle_ForeignType::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
          ];
          TYS.as_ptr()
        },
        raw_init: Bundle_ForeignType::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_Bundle_ForeignType::FIELD_OFFSET_name,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Bundle_ForeignType::FIELD_OFFSET_package,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Bundle_ForeignType::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg Bundle_ForeignType>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Bundle_ForeignType> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Bundle_ForeignType::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut Bundle_ForeignType>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Bundle_ForeignType> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Bundle_ForeignType> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.Type`
pub struct Type {
  ptr: crate::rt::__z::ABox<__priv_Type::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl Type {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Type::Storage = __priv_Type::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      package: 0,
      kind: Type_Kind::new().0 as u32,
      declared_in: 0,
      fields: crate::rt::__z::AVec::new(),
      nesteds: crate::rt::__z::AVec::new(),
      attrs: 0 as *mut u8,
      span: 0,
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_Type::Storage as *mut __priv_Type::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_Type::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_Type::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Type::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_name as usize),
        self.arena,
        Type::__hazzer_name,
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(&self) -> crate::rt::View<'_, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(&self) -> Option<crate::rt::View<'_, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().package) })
  }
  pub fn package_mut(&mut self) -> crate::rt::Mut<'_, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(&mut self) -> crate::rt::value::OptMut<'_, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_package as usize),
        self.arena,
        Type::__hazzer_package,
      )
    }
  }
  pub fn package_set(&mut self, value: u32) {
    self.package_mut().set(value);
  }

  pub fn kind(&self) -> crate::rt::View<'_, Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(&self) -> Option<crate::rt::View<'_, Type_Kind>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Type_Kind>(self.ptr.as_ref().kind) })
  }
  pub fn kind_mut(&mut self) -> crate::rt::Mut<'_, Type_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(&mut self) -> crate::rt::value::OptMut<'_, Type_Kind> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_kind as usize),
        self.arena,
        Type::__hazzer_kind,
      )
    }
  }
  pub fn kind_set(&mut self, value: Type_Kind) {
    self.kind_mut().set(value);
  }

  pub fn declared_in(&self) -> crate::rt::View<'_, u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(&self) -> Option<crate::rt::View<'_, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().declared_in) })
  }
  pub fn declared_in_mut(&mut self) -> crate::rt::Mut<'_, u32> {
    self.declared_in_mut_or().into_mut()
  }
  pub fn declared_in_mut_or(&mut self) -> crate::rt::value::OptMut<'_, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_declared_in as usize),
        self.arena,
        Type::__hazzer_declared_in,
      )
    }
  }
  pub fn declared_in_set(&mut self, value: u32) {
    self.declared_in_mut().set(value);
  }

  pub fn fields(&self) -> crate::rt::Slice<'_, Field> {
    unsafe {
      let vec = &self.ptr.as_ref().fields;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn fields_at(&self, idx: usize) -> crate::rt::View<'_, Field> {
    self.fields().at(idx)
  }
  pub fn fields_mut(&mut self) -> crate::rt::Repeated<'_, Field> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().fields) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn nesteds(&self) -> crate::rt::Slice<'_, u32> {
    unsafe {
      let vec = &self.ptr.as_ref().nesteds;
      crate::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn nesteds_at(&self, idx: usize) -> crate::rt::View<'_, u32> {
    self.nesteds().at(idx)
  }
  pub fn nesteds_mut(&mut self) -> crate::rt::Repeated<'_, u32> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().nesteds) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn attrs(&self) -> crate::rt::View<'_, Type_Attrs> {
    self.attrs_or().unwrap_or(Type_Attrs::DEFAULT)
  }
  pub fn attrs_or(&self) -> Option<crate::rt::View<'_, Type_Attrs>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(crate::rt::View::<Type_Attrs> {
      ptr: unsafe { crate::rt::__z::ABox::from_ptr(self.ptr.as_ref().attrs) },
      _ph: std::marker::PhantomData,
    })
  }
  pub fn attrs_mut(&mut self) -> crate::rt::Mut<'_, Type_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(&mut self) -> crate::rt::value::OptMut<'_, Type_Attrs> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_attrs as usize),
        self.arena,
        Type::__hazzer_attrs,
      )
    }
  }

  pub fn span(&self) -> crate::rt::View<'_, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> Option<crate::rt::View<'_, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }
  pub fn span_mut(&mut self) -> crate::rt::Mut<'_, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(&mut self) -> crate::rt::value::OptMut<'_, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_span as usize),
        self.arena,
        Type::__hazzer_span,
      )
    }
  }
  pub fn span_set(&mut self, value: u32) {
    self.span_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_Type::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Type::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_Type::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_Type::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_name(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Type::FIELD_OFFSET_name as usize;
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
  pub unsafe fn __hazzer_package(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Type::FIELD_OFFSET_package as usize;
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
  pub unsafe fn __hazzer_kind(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Type::FIELD_OFFSET_kind as usize;
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
  pub unsafe fn __hazzer_declared_in(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Type::FIELD_OFFSET_declared_in as usize;
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
  pub unsafe fn __hazzer_attrs(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Type::FIELD_OFFSET_attrs as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 16 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !16,
      Some(true) => {
        *word |= 16;
        let storage = &mut *raw.cast::<__priv_Type::Storage>();
        if storage.attrs.is_null() {
          storage.attrs = arena.alloc(Type_Attrs::__LAYOUT).as_ptr();
          Type_Attrs::__raw_init(storage.attrs);
        }
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_span(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Type::FIELD_OFFSET_span as usize;
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
}

impl Default for Type {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for Type {
  type View<'msg> = __priv_Type::View<'msg>;
  type Mut<'msg> = __priv_Type::Mut<'msg>;
}

impl crate::rt::value::Type for Type {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_Type::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_Type::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_Type::View<'msg> {
  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn package(self) -> crate::rt::View<'msg, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().package) })
  }

  pub fn kind(self) -> crate::rt::View<'msg, Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> Option<crate::rt::View<'msg, Type_Kind>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Type_Kind>(self.ptr.as_ref().kind) })
  }

  pub fn declared_in(self) -> crate::rt::View<'msg, u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().declared_in) })
  }

  pub fn fields(self) -> crate::rt::Slice<'msg, Field> {
    unsafe {
      let vec = &self.ptr.as_ref().fields;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn fields_at(self, idx: usize) -> crate::rt::View<'msg, Field> {
    self.fields().at(idx)
  }

  pub fn nesteds(self) -> crate::rt::Slice<'msg, u32> {
    unsafe {
      let vec = &self.ptr.as_ref().nesteds;
      crate::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn nesteds_at(self, idx: usize) -> crate::rt::View<'msg, u32> {
    self.nesteds().at(idx)
  }

  pub fn attrs(self) -> crate::rt::View<'msg, Type_Attrs> {
    self.attrs_or().unwrap_or(Type_Attrs::DEFAULT)
  }
  pub fn attrs_or(self) -> Option<crate::rt::View<'msg, Type_Attrs>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(crate::rt::View::<Type_Attrs> {
      ptr: unsafe { crate::rt::__z::ABox::from_ptr(self.ptr.as_ref().attrs) },
      _ph: std::marker::PhantomData,
    })
  }

  pub fn span(self) -> crate::rt::View<'msg, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.package_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("package")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.kind_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("kind")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.declared_in_or() {
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
    if let Some(value) = self.attrs_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("attrs")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let Some(value) = self.span_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("span")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_Type::View<'_> {
  fn default() -> Self {
    Type::DEFAULT
  }
}

impl<'msg> __priv_Type::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { Type::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Type::__tdp_info())
  }

  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_name as usize),
        self.arena,
        Type::__hazzer_name,
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(self) -> crate::rt::View<'msg, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().package) })
  }
  pub fn package_mut(self) -> crate::rt::Mut<'msg, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(self) -> crate::rt::value::OptMut<'msg, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_package as usize),
        self.arena,
        Type::__hazzer_package,
      )
    }
  }
  pub fn package_set(self, value: u32) {
    self.package_mut().set(value);
  }

  pub fn kind(self) -> crate::rt::View<'msg, Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> Option<crate::rt::View<'msg, Type_Kind>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Type_Kind>(self.ptr.as_ref().kind) })
  }
  pub fn kind_mut(self) -> crate::rt::Mut<'msg, Type_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(self) -> crate::rt::value::OptMut<'msg, Type_Kind> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_kind as usize),
        self.arena,
        Type::__hazzer_kind,
      )
    }
  }
  pub fn kind_set(self, value: Type_Kind) {
    self.kind_mut().set(value);
  }

  pub fn declared_in(self) -> crate::rt::View<'msg, u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().declared_in) })
  }
  pub fn declared_in_mut(self) -> crate::rt::Mut<'msg, u32> {
    self.declared_in_mut_or().into_mut()
  }
  pub fn declared_in_mut_or(self) -> crate::rt::value::OptMut<'msg, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_declared_in as usize),
        self.arena,
        Type::__hazzer_declared_in,
      )
    }
  }
  pub fn declared_in_set(self, value: u32) {
    self.declared_in_mut().set(value);
  }

  pub fn fields(self) -> crate::rt::Slice<'msg, Field> {
    unsafe {
      let vec = &self.ptr.as_ref().fields;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn fields_at(self, idx: usize) -> crate::rt::View<'msg, Field> {
    self.fields().at(idx)
  }
  pub fn fields_mut(self) -> crate::rt::Repeated<'msg, Field> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().fields) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn nesteds(self) -> crate::rt::Slice<'msg, u32> {
    unsafe {
      let vec = &self.ptr.as_ref().nesteds;
      crate::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn nesteds_at(self, idx: usize) -> crate::rt::View<'msg, u32> {
    self.nesteds().at(idx)
  }
  pub fn nesteds_mut(self) -> crate::rt::Repeated<'msg, u32> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().nesteds) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn attrs(self) -> crate::rt::View<'msg, Type_Attrs> {
    self.attrs_or().unwrap_or(Type_Attrs::DEFAULT)
  }
  pub fn attrs_or(self) -> Option<crate::rt::View<'msg, Type_Attrs>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(crate::rt::View::<Type_Attrs> {
      ptr: unsafe { crate::rt::__z::ABox::from_ptr(self.ptr.as_ref().attrs) },
      _ph: std::marker::PhantomData,
    })
  }
  pub fn attrs_mut(self) -> crate::rt::Mut<'msg, Type_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(self) -> crate::rt::value::OptMut<'msg, Type_Attrs> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_attrs as usize),
        self.arena,
        Type::__hazzer_attrs,
      )
    }
  }

  pub fn span(self) -> crate::rt::View<'msg, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }
  pub fn span_mut(self) -> crate::rt::Mut<'msg, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(self) -> crate::rt::value::OptMut<'msg, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type::FIELD_OFFSET_span as usize),
        self.arena,
        Type::__hazzer_span,
      )
    }
  }
  pub fn span_set(self, value: u32) {
    self.span_mut().set(value);
  }

}

impl Drop for Type {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_Type::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.Type ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Type::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for Type {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Type {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: (*mut u8, usize),
    pub(in super) package: u32,
    pub(in super) kind: u32,
    pub(in super) declared_in: u32,
    pub(in super) fields: crate::rt::__z::AVec<*mut u8>,
    pub (in super) nesteds: crate::rt::__z::AVec<u32>,
    pub(in super) attrs: *mut u8,
    pub(in super) span: u32,
  }

  pub const FIELD_OFFSET_name: u32 = unsafe {
    let msg = Type::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().name as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_package: u32 = unsafe {
    let msg = Type::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().package as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_kind: u32 = unsafe {
    let msg = Type::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().kind as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_declared_in: u32 = unsafe {
    let msg = Type::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().declared_in as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_fields: u32 = unsafe {
    let msg = Type::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().fields as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_nesteds: u32 = unsafe {
    let msg = Type::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().nesteds as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_attrs: u32 = unsafe {
    let msg = Type::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().attrs as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_span: u32 = unsafe {
    let msg = Type::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().span as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{8 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{8 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = Type::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
            Field::__tdp_info,
            Type_Attrs::__tdp_info,
          ];
          TYS.as_ptr()
        },
        raw_init: Type::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_Type::FIELD_OFFSET_name,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Type::FIELD_OFFSET_package,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field {
          number: 3,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Type::FIELD_OFFSET_kind,
          ty: 0,
          hasbit: 2,
        },
        crate::rt::__z::tdp::Field {
          number: 4,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Type::FIELD_OFFSET_declared_in,
          ty: 0,
          hasbit: 3,
        },
        crate::rt::__z::tdp::Field {
          number: 10,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (1 << 4),
          offset: __priv_Type::FIELD_OFFSET_fields,
          ty: 0,
          hasbit: 4,
        },
        crate::rt::__z::tdp::Field {
          number: 11,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (1 << 4),
          offset: __priv_Type::FIELD_OFFSET_nesteds,
          ty: 0,
          hasbit: 4,
        },
        crate::rt::__z::tdp::Field {
          number: 12,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (0 << 4),
          offset: __priv_Type::FIELD_OFFSET_attrs,
          ty: 1,
          hasbit: 4,
        },
        crate::rt::__z::tdp::Field {
          number: 20,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Type::FIELD_OFFSET_span,
          ty: 0,
          hasbit: 5,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Type::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg Type>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Type> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Type::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut Type>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Type> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Type> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// enum `pz.Type.Kind`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Type_Kind(pub i32);

impl Type_Kind {
  pub const Message: Self = Self(0);
  pub const Struct: Self = Self(1);
  pub const Choice: Self = Self(2);
  pub const Enum: Self = Self(3);

  pub const fn new() -> Self {
    Self(0)
  }
}

impl Default for Type_Kind {
  fn default() -> Self {
    Self(0)
  }
}

impl crate::rt::ptr::Proxied for Type_Kind {
  type View<'a> = Self;
  type Mut<'a> = crate::rt::ptr::ScalarMut<'a, Self>;
}

impl std::fmt::Debug for Type_Kind {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Self::Message => std::write!(fmt, "Message"),
      Self::Struct => std::write!(fmt, "Struct"),
      Self::Choice => std::write!(fmt, "Choice"),
      Self::Enum => std::write!(fmt, "Enum"),
      Self(n) => std::write!(fmt, "pz.Type.Kind({n})"),
    }
  }
}

/// message `pz.Type.Attrs`
pub struct Type_Attrs {
  ptr: crate::rt::__z::ABox<__priv_Type_Attrs::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl Type_Attrs {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Type_Attrs::Storage = __priv_Type_Attrs::Storage {
      __hasbits: [0; 1],
      deprecated: (0 as *mut u8, 0),
      docs: crate::rt::__z::AVec::new(),
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_Type_Attrs::Storage as *mut __priv_Type_Attrs::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_Type_Attrs::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_Type_Attrs::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Type_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn deprecated(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().deprecated;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn deprecated_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type_Attrs::FIELD_OFFSET_deprecated as usize),
        self.arena,
        Type_Attrs::__hazzer_deprecated,
      )
    }
  }
  pub fn deprecated_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(&self) -> crate::rt::Slice<'_, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().docs;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(&self, idx: usize) -> crate::rt::View<'_, crate::rt::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(&mut self) -> crate::rt::Repeated<'_, crate::rt::Str> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().docs) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_Type_Attrs::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Type_Attrs::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_Type_Attrs::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_Type_Attrs::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_deprecated(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Type_Attrs::FIELD_OFFSET_deprecated as usize;
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

impl Default for Type_Attrs {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for Type_Attrs {
  type View<'msg> = __priv_Type_Attrs::View<'msg>;
  type Mut<'msg> = __priv_Type_Attrs::Mut<'msg>;
}

impl crate::rt::value::Type for Type_Attrs {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_Type_Attrs::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_Type_Attrs::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_Type_Attrs::View<'msg> {
  pub fn deprecated(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().deprecated;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn docs(self) -> crate::rt::Slice<'msg, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().docs;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(self, idx: usize) -> crate::rt::View<'msg, crate::rt::Str> {
    self.docs().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.deprecated_or() {
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
    Ok(())
  }
}

impl Default for __priv_Type_Attrs::View<'_> {
  fn default() -> Self {
    Type_Attrs::DEFAULT
  }
}

impl<'msg> __priv_Type_Attrs::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { Type_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Type_Attrs::__tdp_info())
  }

  pub fn deprecated(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().deprecated;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn deprecated_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Type_Attrs::FIELD_OFFSET_deprecated as usize),
        self.arena,
        Type_Attrs::__hazzer_deprecated,
      )
    }
  }
  pub fn deprecated_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(self) -> crate::rt::Slice<'msg, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().docs;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(self, idx: usize) -> crate::rt::View<'msg, crate::rt::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(self) -> crate::rt::Repeated<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().docs) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

}

impl Drop for Type_Attrs {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_Type_Attrs::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.Type.Attrs ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Type_Attrs::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for Type_Attrs {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Type_Attrs {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) deprecated: (*mut u8, usize),
    pub(crate) docs: crate::rt::__z::AVec<(*mut u8, usize)>,
  }

  pub const FIELD_OFFSET_deprecated: u32 = unsafe {
    let msg = Type_Attrs::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().deprecated as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_docs: u32 = unsafe {
    let msg = Type_Attrs::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().docs as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{2 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{2 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = Type_Attrs::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
          ];
          TYS.as_ptr()
        },
        raw_init: Type_Attrs::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_Type_Attrs::FIELD_OFFSET_deprecated,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 100,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (1 << 4),
          offset: __priv_Type_Attrs::FIELD_OFFSET_docs,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Type_Attrs::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg Type_Attrs>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Type_Attrs> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Type_Attrs::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut Type_Attrs>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Type_Attrs> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Type_Attrs> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.Field`
pub struct Field {
  ptr: crate::rt::__z::ABox<__priv_Field::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl Field {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Field::Storage = __priv_Field::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      number: 0,
      is_repeated: false,
      r#type: Field_Type::new().0 as u32,
      type_index: 0,
      attrs: 0 as *mut u8,
      span: 0,
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_Field::Storage as *mut __priv_Field::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_Field::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_Field::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Field::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_name as usize),
        self.arena,
        Field::__hazzer_name,
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn number(&self) -> crate::rt::View<'_, i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(&self) -> Option<crate::rt::View<'_, i32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().number) })
  }
  pub fn number_mut(&mut self) -> crate::rt::Mut<'_, i32> {
    self.number_mut_or().into_mut()
  }
  pub fn number_mut_or(&mut self) -> crate::rt::value::OptMut<'_, i32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_number as usize),
        self.arena,
        Field::__hazzer_number,
      )
    }
  }
  pub fn number_set(&mut self, value: i32) {
    self.number_mut().set(value);
  }

  pub fn is_repeated(&self) -> crate::rt::View<'_, bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(&self) -> Option<crate::rt::View<'_, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().is_repeated) })
  }
  pub fn is_repeated_mut(&mut self) -> crate::rt::Mut<'_, bool> {
    self.is_repeated_mut_or().into_mut()
  }
  pub fn is_repeated_mut_or(&mut self) -> crate::rt::value::OptMut<'_, bool> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_is_repeated as usize),
        self.arena,
        Field::__hazzer_is_repeated,
      )
    }
  }
  pub fn is_repeated_set(&mut self, value: bool) {
    self.is_repeated_mut().set(value);
  }

  pub fn r#type(&self) -> crate::rt::View<'_, Field_Type> {
    self.r#type_or().unwrap_or_default()
  }
  pub fn r#type_or(&self) -> Option<crate::rt::View<'_, Field_Type>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Field_Type>(self.ptr.as_ref().r#type) })
  }
  pub fn r#type_mut(&mut self) -> crate::rt::Mut<'_, Field_Type> {
    self.r#type_mut_or().into_mut()
  }
  pub fn r#type_mut_or(&mut self) -> crate::rt::value::OptMut<'_, Field_Type> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_type as usize),
        self.arena,
        Field::__hazzer_type,
      )
    }
  }
  pub fn r#type_set(&mut self, value: Field_Type) {
    self.r#type_mut().set(value);
  }

  pub fn type_index(&self) -> crate::rt::View<'_, u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(&self) -> Option<crate::rt::View<'_, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().type_index) })
  }
  pub fn type_index_mut(&mut self) -> crate::rt::Mut<'_, u32> {
    self.type_index_mut_or().into_mut()
  }
  pub fn type_index_mut_or(&mut self) -> crate::rt::value::OptMut<'_, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_type_index as usize),
        self.arena,
        Field::__hazzer_type_index,
      )
    }
  }
  pub fn type_index_set(&mut self, value: u32) {
    self.type_index_mut().set(value);
  }

  pub fn attrs(&self) -> crate::rt::View<'_, Field_Attrs> {
    self.attrs_or().unwrap_or(Field_Attrs::DEFAULT)
  }
  pub fn attrs_or(&self) -> Option<crate::rt::View<'_, Field_Attrs>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(crate::rt::View::<Field_Attrs> {
      ptr: unsafe { crate::rt::__z::ABox::from_ptr(self.ptr.as_ref().attrs) },
      _ph: std::marker::PhantomData,
    })
  }
  pub fn attrs_mut(&mut self) -> crate::rt::Mut<'_, Field_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(&mut self) -> crate::rt::value::OptMut<'_, Field_Attrs> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_attrs as usize),
        self.arena,
        Field::__hazzer_attrs,
      )
    }
  }

  pub fn span(&self) -> crate::rt::View<'_, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> Option<crate::rt::View<'_, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 64 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }
  pub fn span_mut(&mut self) -> crate::rt::Mut<'_, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(&mut self) -> crate::rt::value::OptMut<'_, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_span as usize),
        self.arena,
        Field::__hazzer_span,
      )
    }
  }
  pub fn span_set(&mut self, value: u32) {
    self.span_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_Field::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Field::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_Field::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_Field::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_name(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Field::FIELD_OFFSET_name as usize;
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
  pub unsafe fn __hazzer_number(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Field::FIELD_OFFSET_number as usize;
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
  pub unsafe fn __hazzer_is_repeated(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Field::FIELD_OFFSET_is_repeated as usize;
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
  pub unsafe fn __hazzer_type(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Field::FIELD_OFFSET_type as usize;
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
  pub unsafe fn __hazzer_type_index(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Field::FIELD_OFFSET_type_index as usize;
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
  pub unsafe fn __hazzer_attrs(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Field::FIELD_OFFSET_attrs as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 32 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !32,
      Some(true) => {
        *word |= 32;
        let storage = &mut *raw.cast::<__priv_Field::Storage>();
        if storage.attrs.is_null() {
          storage.attrs = arena.alloc(Field_Attrs::__LAYOUT).as_ptr();
          Field_Attrs::__raw_init(storage.attrs);
        }
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_span(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Field::FIELD_OFFSET_span as usize;
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
}

impl Default for Field {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for Field {
  type View<'msg> = __priv_Field::View<'msg>;
  type Mut<'msg> = __priv_Field::Mut<'msg>;
}

impl crate::rt::value::Type for Field {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_Field::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_Field::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_Field::View<'msg> {
  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn number(self) -> crate::rt::View<'msg, i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(self) -> Option<crate::rt::View<'msg, i32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().number) })
  }

  pub fn is_repeated(self) -> crate::rt::View<'msg, bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(self) -> Option<crate::rt::View<'msg, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().is_repeated) })
  }

  pub fn r#type(self) -> crate::rt::View<'msg, Field_Type> {
    self.r#type_or().unwrap_or_default()
  }
  pub fn r#type_or(self) -> Option<crate::rt::View<'msg, Field_Type>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Field_Type>(self.ptr.as_ref().r#type) })
  }

  pub fn type_index(self) -> crate::rt::View<'msg, u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().type_index) })
  }

  pub fn attrs(self) -> crate::rt::View<'msg, Field_Attrs> {
    self.attrs_or().unwrap_or(Field_Attrs::DEFAULT)
  }
  pub fn attrs_or(self) -> Option<crate::rt::View<'msg, Field_Attrs>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(crate::rt::View::<Field_Attrs> {
      ptr: unsafe { crate::rt::__z::ABox::from_ptr(self.ptr.as_ref().attrs) },
      _ph: std::marker::PhantomData,
    })
  }

  pub fn span(self) -> crate::rt::View<'msg, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 64 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.number_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("number")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.is_repeated_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("is_repeated")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.r#type_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("type")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.type_index_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("type_index")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.attrs_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("attrs")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let Some(value) = self.span_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("span")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_Field::View<'_> {
  fn default() -> Self {
    Field::DEFAULT
  }
}

impl<'msg> __priv_Field::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { Field::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Field::__tdp_info())
  }

  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_name as usize),
        self.arena,
        Field::__hazzer_name,
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn number(self) -> crate::rt::View<'msg, i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(self) -> Option<crate::rt::View<'msg, i32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().number) })
  }
  pub fn number_mut(self) -> crate::rt::Mut<'msg, i32> {
    self.number_mut_or().into_mut()
  }
  pub fn number_mut_or(self) -> crate::rt::value::OptMut<'msg, i32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_number as usize),
        self.arena,
        Field::__hazzer_number,
      )
    }
  }
  pub fn number_set(self, value: i32) {
    self.number_mut().set(value);
  }

  pub fn is_repeated(self) -> crate::rt::View<'msg, bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(self) -> Option<crate::rt::View<'msg, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().is_repeated) })
  }
  pub fn is_repeated_mut(self) -> crate::rt::Mut<'msg, bool> {
    self.is_repeated_mut_or().into_mut()
  }
  pub fn is_repeated_mut_or(self) -> crate::rt::value::OptMut<'msg, bool> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_is_repeated as usize),
        self.arena,
        Field::__hazzer_is_repeated,
      )
    }
  }
  pub fn is_repeated_set(self, value: bool) {
    self.is_repeated_mut().set(value);
  }

  pub fn r#type(self) -> crate::rt::View<'msg, Field_Type> {
    self.r#type_or().unwrap_or_default()
  }
  pub fn r#type_or(self) -> Option<crate::rt::View<'msg, Field_Type>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Field_Type>(self.ptr.as_ref().r#type) })
  }
  pub fn r#type_mut(self) -> crate::rt::Mut<'msg, Field_Type> {
    self.r#type_mut_or().into_mut()
  }
  pub fn r#type_mut_or(self) -> crate::rt::value::OptMut<'msg, Field_Type> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_type as usize),
        self.arena,
        Field::__hazzer_type,
      )
    }
  }
  pub fn r#type_set(self, value: Field_Type) {
    self.r#type_mut().set(value);
  }

  pub fn type_index(self) -> crate::rt::View<'msg, u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().type_index) })
  }
  pub fn type_index_mut(self) -> crate::rt::Mut<'msg, u32> {
    self.type_index_mut_or().into_mut()
  }
  pub fn type_index_mut_or(self) -> crate::rt::value::OptMut<'msg, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_type_index as usize),
        self.arena,
        Field::__hazzer_type_index,
      )
    }
  }
  pub fn type_index_set(self, value: u32) {
    self.type_index_mut().set(value);
  }

  pub fn attrs(self) -> crate::rt::View<'msg, Field_Attrs> {
    self.attrs_or().unwrap_or(Field_Attrs::DEFAULT)
  }
  pub fn attrs_or(self) -> Option<crate::rt::View<'msg, Field_Attrs>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(crate::rt::View::<Field_Attrs> {
      ptr: unsafe { crate::rt::__z::ABox::from_ptr(self.ptr.as_ref().attrs) },
      _ph: std::marker::PhantomData,
    })
  }
  pub fn attrs_mut(self) -> crate::rt::Mut<'msg, Field_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(self) -> crate::rt::value::OptMut<'msg, Field_Attrs> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_attrs as usize),
        self.arena,
        Field::__hazzer_attrs,
      )
    }
  }

  pub fn span(self) -> crate::rt::View<'msg, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 64 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }
  pub fn span_mut(self) -> crate::rt::Mut<'msg, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(self) -> crate::rt::value::OptMut<'msg, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field::FIELD_OFFSET_span as usize),
        self.arena,
        Field::__hazzer_span,
      )
    }
  }
  pub fn span_set(self, value: u32) {
    self.span_mut().set(value);
  }

}

impl Drop for Field {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_Field::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.Field ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Field::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for Field {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Field {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: (*mut u8, usize),
    pub(in super) number: u32,
    pub(in super) is_repeated: bool,
    pub(in super) r#type: u32,
    pub(in super) type_index: u32,
    pub(in super) attrs: *mut u8,
    pub(in super) span: u32,
  }

  pub const FIELD_OFFSET_name: u32 = unsafe {
    let msg = Field::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().name as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_number: u32 = unsafe {
    let msg = Field::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().number as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_is_repeated: u32 = unsafe {
    let msg = Field::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().is_repeated as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_type: u32 = unsafe {
    let msg = Field::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().r#type as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_type_index: u32 = unsafe {
    let msg = Field::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().type_index as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_attrs: u32 = unsafe {
    let msg = Field::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().attrs as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_span: u32 = unsafe {
    let msg = Field::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().span as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{7 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{7 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = Field::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
            Field_Attrs::__tdp_info,
          ];
          TYS.as_ptr()
        },
        raw_init: Field::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_Field::FIELD_OFFSET_name,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Field::FIELD_OFFSET_number,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field {
          number: 3,
          flags: (crate::rt::__z::tdp::Kind::Bool as u8 as u32) | (0 << 4),
          offset: __priv_Field::FIELD_OFFSET_is_repeated,
          ty: 0,
          hasbit: 2,
        },
        crate::rt::__z::tdp::Field {
          number: 4,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Field::FIELD_OFFSET_type,
          ty: 0,
          hasbit: 3,
        },
        crate::rt::__z::tdp::Field {
          number: 5,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Field::FIELD_OFFSET_type_index,
          ty: 0,
          hasbit: 4,
        },
        crate::rt::__z::tdp::Field {
          number: 10,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (0 << 4),
          offset: __priv_Field::FIELD_OFFSET_attrs,
          ty: 0,
          hasbit: 5,
        },
        crate::rt::__z::tdp::Field {
          number: 20,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Field::FIELD_OFFSET_span,
          ty: 0,
          hasbit: 6,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Field::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg Field>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Field> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Field::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut Field>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Field> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Field> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// enum `pz.Field.Type`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Field_Type(pub i32);

impl Field_Type {
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

impl Default for Field_Type {
  fn default() -> Self {
    Self(0)
  }
}

impl crate::rt::ptr::Proxied for Field_Type {
  type View<'a> = Self;
  type Mut<'a> = crate::rt::ptr::ScalarMut<'a, Self>;
}

impl std::fmt::Debug for Field_Type {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Self::None => std::write!(fmt, "None"),
      Self::I32 => std::write!(fmt, "I32"),
      Self::U32 => std::write!(fmt, "U32"),
      Self::F32 => std::write!(fmt, "F32"),
      Self::I64 => std::write!(fmt, "I64"),
      Self::U64 => std::write!(fmt, "U64"),
      Self::F64 => std::write!(fmt, "F64"),
      Self::Bool => std::write!(fmt, "Bool"),
      Self::String => std::write!(fmt, "String"),
      Self::Type => std::write!(fmt, "Type"),
      Self::Foreign => std::write!(fmt, "Foreign"),
      Self(n) => std::write!(fmt, "pz.Field.Type({n})"),
    }
  }
}

/// message `pz.Field.Attrs`
pub struct Field_Attrs {
  ptr: crate::rt::__z::ABox<__priv_Field_Attrs::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl Field_Attrs {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Field_Attrs::Storage = __priv_Field_Attrs::Storage {
      __hasbits: [0; 1],
      deprecated: (0 as *mut u8, 0),
      docs: crate::rt::__z::AVec::new(),
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_Field_Attrs::Storage as *mut __priv_Field_Attrs::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_Field_Attrs::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_Field_Attrs::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Field_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn deprecated(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().deprecated;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn deprecated_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field_Attrs::FIELD_OFFSET_deprecated as usize),
        self.arena,
        Field_Attrs::__hazzer_deprecated,
      )
    }
  }
  pub fn deprecated_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(&self) -> crate::rt::Slice<'_, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().docs;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(&self, idx: usize) -> crate::rt::View<'_, crate::rt::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(&mut self) -> crate::rt::Repeated<'_, crate::rt::Str> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().docs) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_Field_Attrs::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Field_Attrs::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_Field_Attrs::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_Field_Attrs::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_deprecated(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Field_Attrs::FIELD_OFFSET_deprecated as usize;
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

impl Default for Field_Attrs {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for Field_Attrs {
  type View<'msg> = __priv_Field_Attrs::View<'msg>;
  type Mut<'msg> = __priv_Field_Attrs::Mut<'msg>;
}

impl crate::rt::value::Type for Field_Attrs {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_Field_Attrs::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_Field_Attrs::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_Field_Attrs::View<'msg> {
  pub fn deprecated(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().deprecated;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn docs(self) -> crate::rt::Slice<'msg, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().docs;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(self, idx: usize) -> crate::rt::View<'msg, crate::rt::Str> {
    self.docs().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.deprecated_or() {
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
    Ok(())
  }
}

impl Default for __priv_Field_Attrs::View<'_> {
  fn default() -> Self {
    Field_Attrs::DEFAULT
  }
}

impl<'msg> __priv_Field_Attrs::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { Field_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Field_Attrs::__tdp_info())
  }

  pub fn deprecated(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().deprecated;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn deprecated_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Field_Attrs::FIELD_OFFSET_deprecated as usize),
        self.arena,
        Field_Attrs::__hazzer_deprecated,
      )
    }
  }
  pub fn deprecated_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(self) -> crate::rt::Slice<'msg, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().docs;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(self, idx: usize) -> crate::rt::View<'msg, crate::rt::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(self) -> crate::rt::Repeated<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().docs) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

}

impl Drop for Field_Attrs {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_Field_Attrs::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.Field.Attrs ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Field_Attrs::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for Field_Attrs {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Field_Attrs {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) deprecated: (*mut u8, usize),
    pub(crate) docs: crate::rt::__z::AVec<(*mut u8, usize)>,
  }

  pub const FIELD_OFFSET_deprecated: u32 = unsafe {
    let msg = Field_Attrs::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().deprecated as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_docs: u32 = unsafe {
    let msg = Field_Attrs::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().docs as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{2 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{2 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = Field_Attrs::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
          ];
          TYS.as_ptr()
        },
        raw_init: Field_Attrs::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_Field_Attrs::FIELD_OFFSET_deprecated,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 100,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (1 << 4),
          offset: __priv_Field_Attrs::FIELD_OFFSET_docs,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Field_Attrs::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg Field_Attrs>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Field_Attrs> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Field_Attrs::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut Field_Attrs>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Field_Attrs> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Field_Attrs> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.AboutRequest`
pub struct AboutRequest {
  ptr: crate::rt::__z::ABox<__priv_AboutRequest::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl AboutRequest {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_AboutRequest::Storage = __priv_AboutRequest::Storage {
      __hasbits: [0; 0],
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_AboutRequest::Storage as *mut __priv_AboutRequest::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_AboutRequest::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_AboutRequest::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { AboutRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_AboutRequest::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_AboutRequest::Storage>()).__hasbits = [0; 0];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_AboutRequest::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_AboutRequest::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

}

impl Default for AboutRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for AboutRequest {
  type View<'msg> = __priv_AboutRequest::View<'msg>;
  type Mut<'msg> = __priv_AboutRequest::Mut<'msg>;
}

impl crate::rt::value::Type for AboutRequest {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_AboutRequest::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_AboutRequest::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_AboutRequest::View<'msg> {

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_AboutRequest::View<'_> {
  fn default() -> Self {
    AboutRequest::DEFAULT
  }
}

impl<'msg> __priv_AboutRequest::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { AboutRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, AboutRequest::__tdp_info())
  }

}

impl Drop for AboutRequest {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_AboutRequest::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.AboutRequest ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_AboutRequest::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for AboutRequest {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_AboutRequest {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 0],
  }

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{0 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{0 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = AboutRequest::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
          ];
          TYS.as_ptr()
        },
        raw_init: AboutRequest::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_AboutRequest::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg AboutRequest>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::AboutRequest> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_AboutRequest::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut AboutRequest>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::AboutRequest> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::AboutRequest> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.AboutResponse`
pub struct AboutResponse {
  ptr: crate::rt::__z::ABox<__priv_AboutResponse::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl AboutResponse {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_AboutResponse::Storage = __priv_AboutResponse::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      version: (0 as *mut u8, 0),
      options: crate::rt::__z::AVec::new(),
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_AboutResponse::Storage as *mut __priv_AboutResponse::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_AboutResponse::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_AboutResponse::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { AboutResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_AboutResponse::FIELD_OFFSET_name as usize),
        self.arena,
        AboutResponse::__hazzer_name,
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn version(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().version;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn version_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.version_mut_or().into_mut()
  }
  pub fn version_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_AboutResponse::FIELD_OFFSET_version as usize),
        self.arena,
        AboutResponse::__hazzer_version,
      )
    }
  }
  pub fn version_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.version_mut().set(value);
  }

  pub fn options(&self) -> crate::rt::Slice<'_, AboutResponse_Option> {
    unsafe {
      let vec = &self.ptr.as_ref().options;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(&self, idx: usize) -> crate::rt::View<'_, AboutResponse_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(&mut self) -> crate::rt::Repeated<'_, AboutResponse_Option> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().options) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_AboutResponse::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_AboutResponse::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_AboutResponse::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_AboutResponse::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_name(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_AboutResponse::FIELD_OFFSET_name as usize;
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
  pub unsafe fn __hazzer_version(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_AboutResponse::FIELD_OFFSET_version as usize;
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
}

impl Default for AboutResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for AboutResponse {
  type View<'msg> = __priv_AboutResponse::View<'msg>;
  type Mut<'msg> = __priv_AboutResponse::Mut<'msg>;
}

impl crate::rt::value::Type for AboutResponse {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_AboutResponse::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_AboutResponse::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_AboutResponse::View<'msg> {
  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn version(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().version;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn options(self) -> crate::rt::Slice<'msg, AboutResponse_Option> {
    unsafe {
      let vec = &self.ptr.as_ref().options;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(self, idx: usize) -> crate::rt::View<'msg, AboutResponse_Option> {
    self.options().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.version_or() {
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
    Ok(())
  }
}

impl Default for __priv_AboutResponse::View<'_> {
  fn default() -> Self {
    AboutResponse::DEFAULT
  }
}

impl<'msg> __priv_AboutResponse::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { AboutResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, AboutResponse::__tdp_info())
  }

  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_AboutResponse::FIELD_OFFSET_name as usize),
        self.arena,
        AboutResponse::__hazzer_name,
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn version(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().version;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn version_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.version_mut_or().into_mut()
  }
  pub fn version_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_AboutResponse::FIELD_OFFSET_version as usize),
        self.arena,
        AboutResponse::__hazzer_version,
      )
    }
  }
  pub fn version_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.version_mut().set(value);
  }

  pub fn options(self) -> crate::rt::Slice<'msg, AboutResponse_Option> {
    unsafe {
      let vec = &self.ptr.as_ref().options;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(self, idx: usize) -> crate::rt::View<'msg, AboutResponse_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(self) -> crate::rt::Repeated<'msg, AboutResponse_Option> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().options) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

}

impl Drop for AboutResponse {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_AboutResponse::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.AboutResponse ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_AboutResponse::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for AboutResponse {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_AboutResponse {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: (*mut u8, usize),
    pub(in super) version: (*mut u8, usize),
    pub(in super) options: crate::rt::__z::AVec<*mut u8>,
  }

  pub const FIELD_OFFSET_name: u32 = unsafe {
    let msg = AboutResponse::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().name as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_version: u32 = unsafe {
    let msg = AboutResponse::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().version as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_options: u32 = unsafe {
    let msg = AboutResponse::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().options as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{3 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{3 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = AboutResponse::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
            AboutResponse_Option::__tdp_info,
          ];
          TYS.as_ptr()
        },
        raw_init: AboutResponse::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_AboutResponse::FIELD_OFFSET_name,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_AboutResponse::FIELD_OFFSET_version,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field {
          number: 10,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (1 << 4),
          offset: __priv_AboutResponse::FIELD_OFFSET_options,
          ty: 0,
          hasbit: 2,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_AboutResponse::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg AboutResponse>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::AboutResponse> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_AboutResponse::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut AboutResponse>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::AboutResponse> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::AboutResponse> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.AboutResponse.Option`
pub struct AboutResponse_Option {
  ptr: crate::rt::__z::ABox<__priv_AboutResponse_Option::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl AboutResponse_Option {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_AboutResponse_Option::Storage = __priv_AboutResponse_Option::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      help: (0 as *mut u8, 0),
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_AboutResponse_Option::Storage as *mut __priv_AboutResponse_Option::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_AboutResponse_Option::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_AboutResponse_Option::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { AboutResponse_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_AboutResponse_Option::FIELD_OFFSET_name as usize),
        self.arena,
        AboutResponse_Option::__hazzer_name,
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn help(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().help;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn help_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.help_mut_or().into_mut()
  }
  pub fn help_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_AboutResponse_Option::FIELD_OFFSET_help as usize),
        self.arena,
        AboutResponse_Option::__hazzer_help,
      )
    }
  }
  pub fn help_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.help_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_AboutResponse_Option::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_AboutResponse_Option::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_AboutResponse_Option::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_AboutResponse_Option::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_name(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_AboutResponse_Option::FIELD_OFFSET_name as usize;
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
  pub unsafe fn __hazzer_help(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_AboutResponse_Option::FIELD_OFFSET_help as usize;
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
}

impl Default for AboutResponse_Option {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for AboutResponse_Option {
  type View<'msg> = __priv_AboutResponse_Option::View<'msg>;
  type Mut<'msg> = __priv_AboutResponse_Option::Mut<'msg>;
}

impl crate::rt::value::Type for AboutResponse_Option {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_AboutResponse_Option::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_AboutResponse_Option::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_AboutResponse_Option::View<'msg> {
  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn help(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().help;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.help_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("help")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_AboutResponse_Option::View<'_> {
  fn default() -> Self {
    AboutResponse_Option::DEFAULT
  }
}

impl<'msg> __priv_AboutResponse_Option::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { AboutResponse_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, AboutResponse_Option::__tdp_info())
  }

  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_AboutResponse_Option::FIELD_OFFSET_name as usize),
        self.arena,
        AboutResponse_Option::__hazzer_name,
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn help(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().help;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn help_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.help_mut_or().into_mut()
  }
  pub fn help_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_AboutResponse_Option::FIELD_OFFSET_help as usize),
        self.arena,
        AboutResponse_Option::__hazzer_help,
      )
    }
  }
  pub fn help_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.help_mut().set(value);
  }

}

impl Drop for AboutResponse_Option {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_AboutResponse_Option::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.AboutResponse.Option ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_AboutResponse_Option::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for AboutResponse_Option {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_AboutResponse_Option {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: (*mut u8, usize),
    pub(in super) help: (*mut u8, usize),
  }

  pub const FIELD_OFFSET_name: u32 = unsafe {
    let msg = AboutResponse_Option::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().name as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_help: u32 = unsafe {
    let msg = AboutResponse_Option::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().help as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{2 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{2 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = AboutResponse_Option::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
          ];
          TYS.as_ptr()
        },
        raw_init: AboutResponse_Option::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_AboutResponse_Option::FIELD_OFFSET_name,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_AboutResponse_Option::FIELD_OFFSET_help,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_AboutResponse_Option::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg AboutResponse_Option>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::AboutResponse_Option> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_AboutResponse_Option::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut AboutResponse_Option>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::AboutResponse_Option> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::AboutResponse_Option> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenRequest`
pub struct CodegenRequest {
  ptr: crate::rt::__z::ABox<__priv_CodegenRequest::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl CodegenRequest {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenRequest::Storage = __priv_CodegenRequest::Storage {
      __hasbits: [0; 1],
      bundle: 0 as *mut u8,
      requested_indices: crate::rt::__z::AVec::new(),
      options: crate::rt::__z::AVec::new(),
      debug: false,
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_CodegenRequest::Storage as *mut __priv_CodegenRequest::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_CodegenRequest::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_CodegenRequest::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { CodegenRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn bundle(&self) -> crate::rt::View<'_, Bundle> {
    self.bundle_or().unwrap_or(Bundle::DEFAULT)
  }
  pub fn bundle_or(&self) -> Option<crate::rt::View<'_, Bundle>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(crate::rt::View::<Bundle> {
      ptr: unsafe { crate::rt::__z::ABox::from_ptr(self.ptr.as_ref().bundle) },
      _ph: std::marker::PhantomData,
    })
  }
  pub fn bundle_mut(&mut self) -> crate::rt::Mut<'_, Bundle> {
    self.bundle_mut_or().into_mut()
  }
  pub fn bundle_mut_or(&mut self) -> crate::rt::value::OptMut<'_, Bundle> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenRequest::FIELD_OFFSET_bundle as usize),
        self.arena,
        CodegenRequest::__hazzer_bundle,
      )
    }
  }

  pub fn requested_indices(&self) -> crate::rt::Slice<'_, u32> {
    unsafe {
      let vec = &self.ptr.as_ref().requested_indices;
      crate::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn requested_indices_at(&self, idx: usize) -> crate::rt::View<'_, u32> {
    self.requested_indices().at(idx)
  }
  pub fn requested_indices_mut(&mut self) -> crate::rt::Repeated<'_, u32> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().requested_indices) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn options(&self) -> crate::rt::Slice<'_, CodegenRequest_Option> {
    unsafe {
      let vec = &self.ptr.as_ref().options;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(&self, idx: usize) -> crate::rt::View<'_, CodegenRequest_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(&mut self) -> crate::rt::Repeated<'_, CodegenRequest_Option> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().options) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn debug(&self) -> crate::rt::View<'_, bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(&self) -> Option<crate::rt::View<'_, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().debug) })
  }
  pub fn debug_mut(&mut self) -> crate::rt::Mut<'_, bool> {
    self.debug_mut_or().into_mut()
  }
  pub fn debug_mut_or(&mut self) -> crate::rt::value::OptMut<'_, bool> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenRequest::FIELD_OFFSET_debug as usize),
        self.arena,
        CodegenRequest::__hazzer_debug,
      )
    }
  }
  pub fn debug_set(&mut self, value: bool) {
    self.debug_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_CodegenRequest::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_CodegenRequest::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_CodegenRequest::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_CodegenRequest::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_bundle(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_CodegenRequest::FIELD_OFFSET_bundle as usize;
    let word = &mut *raw.sub(offset).cast::<u32>().add(0);
    let has = *word & 1 != 0;
    match flag {
      None => {},
      Some(false) => *word &= !1,
      Some(true) => {
        *word |= 1;
        let storage = &mut *raw.cast::<__priv_CodegenRequest::Storage>();
        if storage.bundle.is_null() {
          storage.bundle = arena.alloc(Bundle::__LAYOUT).as_ptr();
          Bundle::__raw_init(storage.bundle);
        }
      }
    }
    has
  }
  #[doc(hidden)]
  pub unsafe fn __hazzer_debug(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_CodegenRequest::FIELD_OFFSET_debug as usize;
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
}

impl Default for CodegenRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for CodegenRequest {
  type View<'msg> = __priv_CodegenRequest::View<'msg>;
  type Mut<'msg> = __priv_CodegenRequest::Mut<'msg>;
}

impl crate::rt::value::Type for CodegenRequest {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_CodegenRequest::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_CodegenRequest::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_CodegenRequest::View<'msg> {
  pub fn bundle(self) -> crate::rt::View<'msg, Bundle> {
    self.bundle_or().unwrap_or(Bundle::DEFAULT)
  }
  pub fn bundle_or(self) -> Option<crate::rt::View<'msg, Bundle>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(crate::rt::View::<Bundle> {
      ptr: unsafe { crate::rt::__z::ABox::from_ptr(self.ptr.as_ref().bundle) },
      _ph: std::marker::PhantomData,
    })
  }

  pub fn requested_indices(self) -> crate::rt::Slice<'msg, u32> {
    unsafe {
      let vec = &self.ptr.as_ref().requested_indices;
      crate::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn requested_indices_at(self, idx: usize) -> crate::rt::View<'msg, u32> {
    self.requested_indices().at(idx)
  }

  pub fn options(self) -> crate::rt::Slice<'msg, CodegenRequest_Option> {
    unsafe {
      let vec = &self.ptr.as_ref().options;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(self, idx: usize) -> crate::rt::View<'msg, CodegenRequest_Option> {
    self.options().at(idx)
  }

  pub fn debug(self) -> crate::rt::View<'msg, bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(self) -> Option<crate::rt::View<'msg, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().debug) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.bundle_or() {
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
    if let Some(value) = self.debug_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("debug")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_CodegenRequest::View<'_> {
  fn default() -> Self {
    CodegenRequest::DEFAULT
  }
}

impl<'msg> __priv_CodegenRequest::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { CodegenRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, CodegenRequest::__tdp_info())
  }

  pub fn bundle(self) -> crate::rt::View<'msg, Bundle> {
    self.bundle_or().unwrap_or(Bundle::DEFAULT)
  }
  pub fn bundle_or(self) -> Option<crate::rt::View<'msg, Bundle>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(crate::rt::View::<Bundle> {
      ptr: unsafe { crate::rt::__z::ABox::from_ptr(self.ptr.as_ref().bundle) },
      _ph: std::marker::PhantomData,
    })
  }
  pub fn bundle_mut(self) -> crate::rt::Mut<'msg, Bundle> {
    self.bundle_mut_or().into_mut()
  }
  pub fn bundle_mut_or(self) -> crate::rt::value::OptMut<'msg, Bundle> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenRequest::FIELD_OFFSET_bundle as usize),
        self.arena,
        CodegenRequest::__hazzer_bundle,
      )
    }
  }

  pub fn requested_indices(self) -> crate::rt::Slice<'msg, u32> {
    unsafe {
      let vec = &self.ptr.as_ref().requested_indices;
      crate::rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn requested_indices_at(self, idx: usize) -> crate::rt::View<'msg, u32> {
    self.requested_indices().at(idx)
  }
  pub fn requested_indices_mut(self) -> crate::rt::Repeated<'msg, u32> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().requested_indices) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn options(self) -> crate::rt::Slice<'msg, CodegenRequest_Option> {
    unsafe {
      let vec = &self.ptr.as_ref().options;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(self, idx: usize) -> crate::rt::View<'msg, CodegenRequest_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(self) -> crate::rt::Repeated<'msg, CodegenRequest_Option> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().options) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn debug(self) -> crate::rt::View<'msg, bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(self) -> Option<crate::rt::View<'msg, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().debug) })
  }
  pub fn debug_mut(self) -> crate::rt::Mut<'msg, bool> {
    self.debug_mut_or().into_mut()
  }
  pub fn debug_mut_or(self) -> crate::rt::value::OptMut<'msg, bool> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenRequest::FIELD_OFFSET_debug as usize),
        self.arena,
        CodegenRequest::__hazzer_debug,
      )
    }
  }
  pub fn debug_set(self, value: bool) {
    self.debug_mut().set(value);
  }

}

impl Drop for CodegenRequest {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_CodegenRequest::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.CodegenRequest ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_CodegenRequest::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for CodegenRequest {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_CodegenRequest {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) bundle: *mut u8,
    pub (in super) requested_indices: crate::rt::__z::AVec<u32>,
    pub(in super) options: crate::rt::__z::AVec<*mut u8>,
    pub(in super) debug: bool,
  }

  pub const FIELD_OFFSET_bundle: u32 = unsafe {
    let msg = CodegenRequest::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().bundle as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_requested_indices: u32 = unsafe {
    let msg = CodegenRequest::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().requested_indices as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_options: u32 = unsafe {
    let msg = CodegenRequest::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().options as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_debug: u32 = unsafe {
    let msg = CodegenRequest::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().debug as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{4 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{4 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = CodegenRequest::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
            Bundle::__tdp_info,
            CodegenRequest_Option::__tdp_info,
          ];
          TYS.as_ptr()
        },
        raw_init: CodegenRequest::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (0 << 4),
          offset: __priv_CodegenRequest::FIELD_OFFSET_bundle,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (1 << 4),
          offset: __priv_CodegenRequest::FIELD_OFFSET_requested_indices,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field {
          number: 3,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (1 << 4),
          offset: __priv_CodegenRequest::FIELD_OFFSET_options,
          ty: 1,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field {
          number: 4,
          flags: (crate::rt::__z::tdp::Kind::Bool as u8 as u32) | (0 << 4),
          offset: __priv_CodegenRequest::FIELD_OFFSET_debug,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_CodegenRequest::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg CodegenRequest>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::CodegenRequest> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_CodegenRequest::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut CodegenRequest>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::CodegenRequest> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::CodegenRequest> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenRequest.Option`
pub struct CodegenRequest_Option {
  ptr: crate::rt::__z::ABox<__priv_CodegenRequest_Option::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl CodegenRequest_Option {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenRequest_Option::Storage = __priv_CodegenRequest_Option::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      value: (0 as *mut u8, 0),
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_CodegenRequest_Option::Storage as *mut __priv_CodegenRequest_Option::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_CodegenRequest_Option::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_CodegenRequest_Option::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { CodegenRequest_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenRequest_Option::FIELD_OFFSET_name as usize),
        self.arena,
        CodegenRequest_Option::__hazzer_name,
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn value(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().value;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn value_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.value_mut_or().into_mut()
  }
  pub fn value_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenRequest_Option::FIELD_OFFSET_value as usize),
        self.arena,
        CodegenRequest_Option::__hazzer_value,
      )
    }
  }
  pub fn value_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.value_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_CodegenRequest_Option::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_CodegenRequest_Option::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_CodegenRequest_Option::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_CodegenRequest_Option::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_name(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_CodegenRequest_Option::FIELD_OFFSET_name as usize;
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
  pub unsafe fn __hazzer_value(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_CodegenRequest_Option::FIELD_OFFSET_value as usize;
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
}

impl Default for CodegenRequest_Option {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for CodegenRequest_Option {
  type View<'msg> = __priv_CodegenRequest_Option::View<'msg>;
  type Mut<'msg> = __priv_CodegenRequest_Option::Mut<'msg>;
}

impl crate::rt::value::Type for CodegenRequest_Option {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_CodegenRequest_Option::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_CodegenRequest_Option::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_CodegenRequest_Option::View<'msg> {
  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn value(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().value;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.name_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.value_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("value")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_CodegenRequest_Option::View<'_> {
  fn default() -> Self {
    CodegenRequest_Option::DEFAULT
  }
}

impl<'msg> __priv_CodegenRequest_Option::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { CodegenRequest_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, CodegenRequest_Option::__tdp_info())
  }

  pub fn name(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().name;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenRequest_Option::FIELD_OFFSET_name as usize),
        self.arena,
        CodegenRequest_Option::__hazzer_name,
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn value(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().value;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn value_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.value_mut_or().into_mut()
  }
  pub fn value_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenRequest_Option::FIELD_OFFSET_value as usize),
        self.arena,
        CodegenRequest_Option::__hazzer_value,
      )
    }
  }
  pub fn value_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.value_mut().set(value);
  }

}

impl Drop for CodegenRequest_Option {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_CodegenRequest_Option::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.CodegenRequest.Option ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_CodegenRequest_Option::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for CodegenRequest_Option {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_CodegenRequest_Option {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) name: (*mut u8, usize),
    pub(in super) value: (*mut u8, usize),
  }

  pub const FIELD_OFFSET_name: u32 = unsafe {
    let msg = CodegenRequest_Option::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().name as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_value: u32 = unsafe {
    let msg = CodegenRequest_Option::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().value as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{2 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{2 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = CodegenRequest_Option::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
          ];
          TYS.as_ptr()
        },
        raw_init: CodegenRequest_Option::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_CodegenRequest_Option::FIELD_OFFSET_name,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_CodegenRequest_Option::FIELD_OFFSET_value,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_CodegenRequest_Option::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg CodegenRequest_Option>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::CodegenRequest_Option> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_CodegenRequest_Option::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut CodegenRequest_Option>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::CodegenRequest_Option> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::CodegenRequest_Option> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenResponse`
pub struct CodegenResponse {
  ptr: crate::rt::__z::ABox<__priv_CodegenResponse::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl CodegenResponse {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenResponse::Storage = __priv_CodegenResponse::Storage {
      __hasbits: [0; 0],
      files: crate::rt::__z::AVec::new(),
      report: crate::rt::__z::AVec::new(),
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_CodegenResponse::Storage as *mut __priv_CodegenResponse::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_CodegenResponse::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_CodegenResponse::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { CodegenResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn files(&self) -> crate::rt::Slice<'_, CodegenResponse_File> {
    unsafe {
      let vec = &self.ptr.as_ref().files;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn files_at(&self, idx: usize) -> crate::rt::View<'_, CodegenResponse_File> {
    self.files().at(idx)
  }
  pub fn files_mut(&mut self) -> crate::rt::Repeated<'_, CodegenResponse_File> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().files) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn report(&self) -> crate::rt::Slice<'_, Diagnostic> {
    unsafe {
      let vec = &self.ptr.as_ref().report;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn report_at(&self, idx: usize) -> crate::rt::View<'_, Diagnostic> {
    self.report().at(idx)
  }
  pub fn report_mut(&mut self) -> crate::rt::Repeated<'_, Diagnostic> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().report) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_CodegenResponse::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_CodegenResponse::Storage>()).__hasbits = [0; 0];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_CodegenResponse::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_CodegenResponse::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

}

impl Default for CodegenResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for CodegenResponse {
  type View<'msg> = __priv_CodegenResponse::View<'msg>;
  type Mut<'msg> = __priv_CodegenResponse::Mut<'msg>;
}

impl crate::rt::value::Type for CodegenResponse {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_CodegenResponse::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_CodegenResponse::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_CodegenResponse::View<'msg> {
  pub fn files(self) -> crate::rt::Slice<'msg, CodegenResponse_File> {
    unsafe {
      let vec = &self.ptr.as_ref().files;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn files_at(self, idx: usize) -> crate::rt::View<'msg, CodegenResponse_File> {
    self.files().at(idx)
  }

  pub fn report(self) -> crate::rt::Slice<'msg, Diagnostic> {
    unsafe {
      let vec = &self.ptr.as_ref().report;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn report_at(self, idx: usize) -> crate::rt::View<'msg, Diagnostic> {
    self.report().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
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
    Ok(())
  }
}

impl Default for __priv_CodegenResponse::View<'_> {
  fn default() -> Self {
    CodegenResponse::DEFAULT
  }
}

impl<'msg> __priv_CodegenResponse::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { CodegenResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, CodegenResponse::__tdp_info())
  }

  pub fn files(self) -> crate::rt::Slice<'msg, CodegenResponse_File> {
    unsafe {
      let vec = &self.ptr.as_ref().files;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn files_at(self, idx: usize) -> crate::rt::View<'msg, CodegenResponse_File> {
    self.files().at(idx)
  }
  pub fn files_mut(self) -> crate::rt::Repeated<'msg, CodegenResponse_File> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().files) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn report(self) -> crate::rt::Slice<'msg, Diagnostic> {
    unsafe {
      let vec = &self.ptr.as_ref().report;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn report_at(self, idx: usize) -> crate::rt::View<'msg, Diagnostic> {
    self.report().at(idx)
  }
  pub fn report_mut(self) -> crate::rt::Repeated<'msg, Diagnostic> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().report) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

}

impl Drop for CodegenResponse {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_CodegenResponse::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.CodegenResponse ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_CodegenResponse::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for CodegenResponse {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_CodegenResponse {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 0],
    pub(in super) files: crate::rt::__z::AVec<*mut u8>,
    pub(in super) report: crate::rt::__z::AVec<*mut u8>,
  }

  pub const FIELD_OFFSET_files: u32 = unsafe {
    let msg = CodegenResponse::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().files as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_report: u32 = unsafe {
    let msg = CodegenResponse::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().report as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{2 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{2 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = CodegenResponse::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
            CodegenResponse_File::__tdp_info,
            Diagnostic::__tdp_info,
          ];
          TYS.as_ptr()
        },
        raw_init: CodegenResponse::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (1 << 4),
          offset: __priv_CodegenResponse::FIELD_OFFSET_files,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (1 << 4),
          offset: __priv_CodegenResponse::FIELD_OFFSET_report,
          ty: 1,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_CodegenResponse::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg CodegenResponse>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::CodegenResponse> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_CodegenResponse::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut CodegenResponse>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::CodegenResponse> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::CodegenResponse> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenResponse.File`
pub struct CodegenResponse_File {
  ptr: crate::rt::__z::ABox<__priv_CodegenResponse_File::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl CodegenResponse_File {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenResponse_File::Storage = __priv_CodegenResponse_File::Storage {
      __hasbits: [0; 1],
      path: (0 as *mut u8, 0),
      content: (0 as *mut u8, 0),
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_CodegenResponse_File::Storage as *mut __priv_CodegenResponse_File::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_CodegenResponse_File::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_CodegenResponse_File::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { CodegenResponse_File::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn path(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().path;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn path_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.path_mut_or().into_mut()
  }
  pub fn path_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenResponse_File::FIELD_OFFSET_path as usize),
        self.arena,
        CodegenResponse_File::__hazzer_path,
      )
    }
  }
  pub fn path_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.path_mut().set(value);
  }

  pub fn content(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().content;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn content_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.content_mut_or().into_mut()
  }
  pub fn content_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenResponse_File::FIELD_OFFSET_content as usize),
        self.arena,
        CodegenResponse_File::__hazzer_content,
      )
    }
  }
  pub fn content_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.content_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_CodegenResponse_File::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_CodegenResponse_File::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_CodegenResponse_File::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_CodegenResponse_File::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_path(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_CodegenResponse_File::FIELD_OFFSET_path as usize;
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
  pub unsafe fn __hazzer_content(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_CodegenResponse_File::FIELD_OFFSET_content as usize;
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
}

impl Default for CodegenResponse_File {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for CodegenResponse_File {
  type View<'msg> = __priv_CodegenResponse_File::View<'msg>;
  type Mut<'msg> = __priv_CodegenResponse_File::Mut<'msg>;
}

impl crate::rt::value::Type for CodegenResponse_File {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_CodegenResponse_File::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_CodegenResponse_File::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_CodegenResponse_File::View<'msg> {
  pub fn path(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().path;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn content(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().content;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.path_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("path")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.content_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("content")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_CodegenResponse_File::View<'_> {
  fn default() -> Self {
    CodegenResponse_File::DEFAULT
  }
}

impl<'msg> __priv_CodegenResponse_File::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { CodegenResponse_File::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, CodegenResponse_File::__tdp_info())
  }

  pub fn path(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().path;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn path_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.path_mut_or().into_mut()
  }
  pub fn path_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenResponse_File::FIELD_OFFSET_path as usize),
        self.arena,
        CodegenResponse_File::__hazzer_path,
      )
    }
  }
  pub fn path_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.path_mut().set(value);
  }

  pub fn content(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().content;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn content_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.content_mut_or().into_mut()
  }
  pub fn content_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_CodegenResponse_File::FIELD_OFFSET_content as usize),
        self.arena,
        CodegenResponse_File::__hazzer_content,
      )
    }
  }
  pub fn content_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.content_mut().set(value);
  }

}

impl Drop for CodegenResponse_File {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_CodegenResponse_File::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.CodegenResponse.File ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_CodegenResponse_File::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for CodegenResponse_File {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_CodegenResponse_File {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) path: (*mut u8, usize),
    pub(in super) content: (*mut u8, usize),
  }

  pub const FIELD_OFFSET_path: u32 = unsafe {
    let msg = CodegenResponse_File::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().path as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_content: u32 = unsafe {
    let msg = CodegenResponse_File::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().content as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{2 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{2 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = CodegenResponse_File::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
          ];
          TYS.as_ptr()
        },
        raw_init: CodegenResponse_File::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_CodegenResponse_File::FIELD_OFFSET_path,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_CodegenResponse_File::FIELD_OFFSET_content,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_CodegenResponse_File::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg CodegenResponse_File>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::CodegenResponse_File> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_CodegenResponse_File::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut CodegenResponse_File>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::CodegenResponse_File> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::CodegenResponse_File> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.Diagnostic`
pub struct Diagnostic {
  ptr: crate::rt::__z::ABox<__priv_Diagnostic::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl Diagnostic {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Diagnostic::Storage = __priv_Diagnostic::Storage {
      __hasbits: [0; 1],
      kind: Diagnostic_Kind::new().0 as u32,
      msg: (0 as *mut u8, 0),
      snippets: crate::rt::__z::AVec::new(),
      notes: crate::rt::__z::AVec::new(),
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_Diagnostic::Storage as *mut __priv_Diagnostic::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_Diagnostic::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_Diagnostic::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Diagnostic::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn kind(&self) -> crate::rt::View<'_, Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(&self) -> Option<crate::rt::View<'_, Diagnostic_Kind>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Diagnostic_Kind>(self.ptr.as_ref().kind) })
  }
  pub fn kind_mut(&mut self) -> crate::rt::Mut<'_, Diagnostic_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(&mut self) -> crate::rt::value::OptMut<'_, Diagnostic_Kind> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Diagnostic::FIELD_OFFSET_kind as usize),
        self.arena,
        Diagnostic::__hazzer_kind,
      )
    }
  }
  pub fn kind_set(&mut self, value: Diagnostic_Kind) {
    self.kind_mut().set(value);
  }

  pub fn msg(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().msg;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn msg_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.msg_mut_or().into_mut()
  }
  pub fn msg_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Diagnostic::FIELD_OFFSET_msg as usize),
        self.arena,
        Diagnostic::__hazzer_msg,
      )
    }
  }
  pub fn msg_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.msg_mut().set(value);
  }

  pub fn snippets(&self) -> crate::rt::Slice<'_, Diagnostic_Snippet> {
    unsafe {
      let vec = &self.ptr.as_ref().snippets;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn snippets_at(&self, idx: usize) -> crate::rt::View<'_, Diagnostic_Snippet> {
    self.snippets().at(idx)
  }
  pub fn snippets_mut(&mut self) -> crate::rt::Repeated<'_, Diagnostic_Snippet> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().snippets) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn notes(&self) -> crate::rt::Slice<'_, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().notes;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn notes_at(&self, idx: usize) -> crate::rt::View<'_, crate::rt::Str> {
    self.notes().at(idx)
  }
  pub fn notes_mut(&mut self) -> crate::rt::Repeated<'_, crate::rt::Str> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().notes) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_Diagnostic::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Diagnostic::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_Diagnostic::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_Diagnostic::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_kind(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Diagnostic::FIELD_OFFSET_kind as usize;
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
  pub unsafe fn __hazzer_msg(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Diagnostic::FIELD_OFFSET_msg as usize;
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
}

impl Default for Diagnostic {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for Diagnostic {
  type View<'msg> = __priv_Diagnostic::View<'msg>;
  type Mut<'msg> = __priv_Diagnostic::Mut<'msg>;
}

impl crate::rt::value::Type for Diagnostic {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_Diagnostic::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_Diagnostic::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_Diagnostic::View<'msg> {
  pub fn kind(self) -> crate::rt::View<'msg, Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> Option<crate::rt::View<'msg, Diagnostic_Kind>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Diagnostic_Kind>(self.ptr.as_ref().kind) })
  }

  pub fn msg(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().msg;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn snippets(self) -> crate::rt::Slice<'msg, Diagnostic_Snippet> {
    unsafe {
      let vec = &self.ptr.as_ref().snippets;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn snippets_at(self, idx: usize) -> crate::rt::View<'msg, Diagnostic_Snippet> {
    self.snippets().at(idx)
  }

  pub fn notes(self) -> crate::rt::Slice<'msg, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().notes;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn notes_at(self, idx: usize) -> crate::rt::View<'msg, crate::rt::Str> {
    self.notes().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.kind_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("kind")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.msg_or() {
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
    Ok(())
  }
}

impl Default for __priv_Diagnostic::View<'_> {
  fn default() -> Self {
    Diagnostic::DEFAULT
  }
}

impl<'msg> __priv_Diagnostic::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { Diagnostic::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Diagnostic::__tdp_info())
  }

  pub fn kind(self) -> crate::rt::View<'msg, Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> Option<crate::rt::View<'msg, Diagnostic_Kind>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Diagnostic_Kind>(self.ptr.as_ref().kind) })
  }
  pub fn kind_mut(self) -> crate::rt::Mut<'msg, Diagnostic_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(self) -> crate::rt::value::OptMut<'msg, Diagnostic_Kind> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Diagnostic::FIELD_OFFSET_kind as usize),
        self.arena,
        Diagnostic::__hazzer_kind,
      )
    }
  }
  pub fn kind_set(self, value: Diagnostic_Kind) {
    self.kind_mut().set(value);
  }

  pub fn msg(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().msg;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn msg_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.msg_mut_or().into_mut()
  }
  pub fn msg_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Diagnostic::FIELD_OFFSET_msg as usize),
        self.arena,
        Diagnostic::__hazzer_msg,
      )
    }
  }
  pub fn msg_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.msg_mut().set(value);
  }

  pub fn snippets(self) -> crate::rt::Slice<'msg, Diagnostic_Snippet> {
    unsafe {
      let vec = &self.ptr.as_ref().snippets;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn snippets_at(self, idx: usize) -> crate::rt::View<'msg, Diagnostic_Snippet> {
    self.snippets().at(idx)
  }
  pub fn snippets_mut(self) -> crate::rt::Repeated<'msg, Diagnostic_Snippet> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().snippets) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn notes(self) -> crate::rt::Slice<'msg, crate::rt::Str> {
    unsafe {
      let vec = &self.ptr.as_ref().notes;
      crate::rt::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn notes_at(self, idx: usize) -> crate::rt::View<'msg, crate::rt::Str> {
    self.notes().at(idx)
  }
  pub fn notes_mut(self) -> crate::rt::Repeated<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::Repeated::__wrap(
        (&mut self.ptr.as_mut().notes) as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

}

impl Drop for Diagnostic {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_Diagnostic::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.Diagnostic ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Diagnostic::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for Diagnostic {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Diagnostic {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) kind: u32,
    pub(in super) msg: (*mut u8, usize),
    pub(in super) snippets: crate::rt::__z::AVec<*mut u8>,
    pub(crate) notes: crate::rt::__z::AVec<(*mut u8, usize)>,
  }

  pub const FIELD_OFFSET_kind: u32 = unsafe {
    let msg = Diagnostic::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().kind as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_msg: u32 = unsafe {
    let msg = Diagnostic::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().msg as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_snippets: u32 = unsafe {
    let msg = Diagnostic::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().snippets as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_notes: u32 = unsafe {
    let msg = Diagnostic::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().notes as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{4 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{4 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = Diagnostic::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
            Diagnostic_Snippet::__tdp_info,
          ];
          TYS.as_ptr()
        },
        raw_init: Diagnostic::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Diagnostic::FIELD_OFFSET_kind,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_Diagnostic::FIELD_OFFSET_msg,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field {
          number: 3,
          flags: (crate::rt::__z::tdp::Kind::Msg as u8 as u32) | (1 << 4),
          offset: __priv_Diagnostic::FIELD_OFFSET_snippets,
          ty: 0,
          hasbit: 2,
        },
        crate::rt::__z::tdp::Field {
          number: 4,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (1 << 4),
          offset: __priv_Diagnostic::FIELD_OFFSET_notes,
          ty: 0,
          hasbit: 2,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Diagnostic::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg Diagnostic>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Diagnostic> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Diagnostic::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut Diagnostic>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Diagnostic> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Diagnostic> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// enum `pz.plugin.Diagnostic.Kind`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Diagnostic_Kind(pub i32);

impl Diagnostic_Kind {
  pub const Error: Self = Self(0);
  pub const Warning: Self = Self(1);

  pub const fn new() -> Self {
    Self(0)
  }
}

impl Default for Diagnostic_Kind {
  fn default() -> Self {
    Self(0)
  }
}

impl crate::rt::ptr::Proxied for Diagnostic_Kind {
  type View<'a> = Self;
  type Mut<'a> = crate::rt::ptr::ScalarMut<'a, Self>;
}

impl std::fmt::Debug for Diagnostic_Kind {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Self::Error => std::write!(fmt, "Error"),
      Self::Warning => std::write!(fmt, "Warning"),
      Self(n) => std::write!(fmt, "pz.plugin.Diagnostic.Kind({n})"),
    }
  }
}

/// message `pz.plugin.Diagnostic.Snippet`
pub struct Diagnostic_Snippet {
  ptr: crate::rt::__z::ABox<__priv_Diagnostic_Snippet::Storage>,
  arena: crate::rt::__z::RawArena,
}

impl Diagnostic_Snippet {
  pub const DEFAULT: crate::rt::View<'static, Self> = unsafe {
    const VALUE: __priv_Diagnostic_Snippet::Storage = __priv_Diagnostic_Snippet::Storage {
      __hasbits: [0; 1],
      span: 0,
      msg: (0 as *mut u8, 0),
      is_remark: false,
    };
    crate::rt::View::<Self> {
      ptr: crate::rt::__z::ABox::from_ptr(&VALUE as *const __priv_Diagnostic_Snippet::Storage as *mut __priv_Diagnostic_Snippet::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::rt::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      Self::__raw_init(ptr);
      Self {
        ptr: crate::rt::__z::ABox::from_ptr(ptr),
        arena,
      }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::rt::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::rt::View<Self> {
    __priv_Diagnostic_Snippet::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::rt::Mut<Self> {
    __priv_Diagnostic_Snippet::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Diagnostic_Snippet::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn span(&self) -> crate::rt::View<'_, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> Option<crate::rt::View<'_, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }
  pub fn span_mut(&mut self) -> crate::rt::Mut<'_, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(&mut self) -> crate::rt::value::OptMut<'_, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Diagnostic_Snippet::FIELD_OFFSET_span as usize),
        self.arena,
        Diagnostic_Snippet::__hazzer_span,
      )
    }
  }
  pub fn span_set(&mut self, value: u32) {
    self.span_mut().set(value);
  }

  pub fn msg(&self) -> crate::rt::View<'_, crate::rt::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(&self) -> Option<crate::rt::View<'_, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().msg;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn msg_mut(&mut self) -> crate::rt::Mut<'_, crate::rt::Str> {
    self.msg_mut_or().into_mut()
  }
  pub fn msg_mut_or(&mut self) -> crate::rt::value::OptMut<'_, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Diagnostic_Snippet::FIELD_OFFSET_msg as usize),
        self.arena,
        Diagnostic_Snippet::__hazzer_msg,
      )
    }
  }
  pub fn msg_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.msg_mut().set(value);
  }

  pub fn is_remark(&self) -> crate::rt::View<'_, bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(&self) -> Option<crate::rt::View<'_, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().is_remark) })
  }
  pub fn is_remark_mut(&mut self) -> crate::rt::Mut<'_, bool> {
    self.is_remark_mut_or().into_mut()
  }
  pub fn is_remark_mut_or(&mut self) -> crate::rt::value::OptMut<'_, bool> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Diagnostic_Snippet::FIELD_OFFSET_is_remark as usize),
        self.arena,
        Diagnostic_Snippet::__hazzer_is_remark,
      )
    }
  }
  pub fn is_remark_set(&mut self, value: bool) {
    self.is_remark_mut().set(value);
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_Diagnostic_Snippet::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Diagnostic_Snippet::Storage>()).__hasbits = [0; 1];
  }
  #[doc(hidden)]
  pub unsafe fn __raw_init(raw: *mut u8) {
    raw.cast::<__priv_Diagnostic_Snippet::Storage>()
      .copy_from_nonoverlapping(Self::DEFAULT.ptr.as_ptr().cast(), 1);
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> *const crate::rt::__z::tdp::Message {
    &__priv_Diagnostic_Snippet::TDP_INFO as *const _ as *const crate::rt::__z::tdp::Message
  }

  #[doc(hidden)]
  pub unsafe fn __hazzer_span(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Diagnostic_Snippet::FIELD_OFFSET_span as usize;
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
  pub unsafe fn __hazzer_msg(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Diagnostic_Snippet::FIELD_OFFSET_msg as usize;
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
  pub unsafe fn __hazzer_is_remark(
    raw: *mut u8,
    arena: crate::rt::__z::RawArena,
    flag: Option<bool>,
  ) -> bool {
    let offset = __priv_Diagnostic_Snippet::FIELD_OFFSET_is_remark as usize;
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
}

impl Default for Diagnostic_Snippet {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::rt::ptr::Proxied for Diagnostic_Snippet {
  type View<'msg> = __priv_Diagnostic_Snippet::View<'msg>;
  type Mut<'msg> = __priv_Diagnostic_Snippet::Mut<'msg>;
}

impl crate::rt::value::Type for Diagnostic_Snippet {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::rt::View<'a, Self> {
    __priv_Diagnostic_Snippet::View {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::rt::__z::RawArena) -> crate::rt::Mut<'a, Self> {
    __priv_Diagnostic_Snippet::Mut {
      ptr: crate::rt::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::rt::__z::RawArena) {
    (&mut *ptr.cast::<crate::rt::__z::AVec<*mut u8>>()).resize_msg(
      new_len, arena, Self::__LAYOUT, Self::__raw_init)
  }
}

impl<'msg> __priv_Diagnostic_Snippet::View<'msg> {
  pub fn span(self) -> crate::rt::View<'msg, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }

  pub fn msg(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().msg;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn is_remark(self) -> crate::rt::View<'msg, bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(self) -> Option<crate::rt::View<'msg, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().is_remark) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.span_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("span")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.msg_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("msg")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.is_remark_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("is_remark")?;
      debug.write_debug(value);
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
  }
}

impl Default for __priv_Diagnostic_Snippet::View<'_> {
  fn default() -> Self {
    Diagnostic_Snippet::DEFAULT
  }
}

impl<'msg> __priv_Diagnostic_Snippet::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { Diagnostic_Snippet::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Diagnostic_Snippet::__tdp_info())
  }

  pub fn span(self) -> crate::rt::View<'msg, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::rt::View<'msg, u32>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }
  pub fn span_mut(self) -> crate::rt::Mut<'msg, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(self) -> crate::rt::value::OptMut<'msg, u32> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Diagnostic_Snippet::FIELD_OFFSET_span as usize),
        self.arena,
        Diagnostic_Snippet::__hazzer_span,
      )
    }
  }
  pub fn span_set(self, value: u32) {
    self.span_mut().set(value);
  }

  pub fn msg(self) -> crate::rt::View<'msg, crate::rt::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(self) -> Option<crate::rt::View<'msg, crate::rt::Str>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe {
      let (mut ptr, len) = self.ptr.as_ref().msg;
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn msg_mut(self) -> crate::rt::Mut<'msg, crate::rt::Str> {
    self.msg_mut_or().into_mut()
  }
  pub fn msg_mut_or(self) -> crate::rt::value::OptMut<'msg, crate::rt::Str> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Diagnostic_Snippet::FIELD_OFFSET_msg as usize),
        self.arena,
        Diagnostic_Snippet::__hazzer_msg,
      )
    }
  }
  pub fn msg_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.msg_mut().set(value);
  }

  pub fn is_remark(self) -> crate::rt::View<'msg, bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(self) -> Option<crate::rt::View<'msg, bool>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().is_remark) })
  }
  pub fn is_remark_mut(self) -> crate::rt::Mut<'msg, bool> {
    self.is_remark_mut_or().into_mut()
  }
  pub fn is_remark_mut_or(self) -> crate::rt::value::OptMut<'msg, bool> {
    unsafe {
      crate::rt::value::OptMut::__wrap(
        self.ptr.as_ptr().add(__priv_Diagnostic_Snippet::FIELD_OFFSET_is_remark as usize),
        self.arena,
        Diagnostic_Snippet::__hazzer_is_remark,
      )
    }
  }
  pub fn is_remark_set(self, value: bool) {
    self.is_remark_mut().set(value);
  }

}

impl Drop for Diagnostic_Snippet {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_Diagnostic_Snippet::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.Diagnostic.Snippet ")?;
    let mut debug = crate::rt::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Diagnostic_Snippet::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::rt::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for Diagnostic_Snippet {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

mod __priv_Diagnostic_Snippet {
  pub use super::*;

  #[repr(C)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(in super) span: u32,
    pub(in super) msg: (*mut u8, usize),
    pub(in super) is_remark: bool,
  }

  pub const FIELD_OFFSET_span: u32 = unsafe {
    let msg = Diagnostic_Snippet::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().span as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_msg: u32 = unsafe {
    let msg = Diagnostic_Snippet::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().msg as *const _ as *const u8;
    field.offset_from(top) as u32
  };
  pub const FIELD_OFFSET_is_remark: u32 = unsafe {
    let msg = Diagnostic_Snippet::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().is_remark as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{3 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{3 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = Diagnostic_Snippet::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
          ];
          TYS.as_ptr()
        },
        raw_init: Diagnostic_Snippet::__raw_init,
      },
      fields: [
        crate::rt::__z::tdp::Field {
          number: 1,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Diagnostic_Snippet::FIELD_OFFSET_span,
          ty: 0,
          hasbit: 0,
        },
        crate::rt::__z::tdp::Field {
          number: 2,
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (0 << 4),
          offset: __priv_Diagnostic_Snippet::FIELD_OFFSET_msg,
          ty: 0,
          hasbit: 1,
        },
        crate::rt::__z::tdp::Field {
          number: 3,
          flags: (crate::rt::__z::tdp::Kind::Bool as u8 as u32) | (0 << 4),
          offset: __priv_Diagnostic_Snippet::FIELD_OFFSET_is_remark,
          ty: 0,
          hasbit: 2,
        },
        crate::rt::__z::tdp::Field { number: 0, flags: 0, offset: 0, ty: 0, hasbit: 0, },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Diagnostic_Snippet::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg Diagnostic_Snippet>,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Diagnostic_Snippet> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'msg> {
    pub(in super) ptr: crate::rt::__z::ABox<__priv_Diagnostic_Snippet::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'msg mut Diagnostic_Snippet>,
    pub(in super) arena: crate::rt::__z::RawArena,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Diagnostic_Snippet> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Diagnostic_Snippet> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

