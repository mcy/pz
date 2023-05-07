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

  pub fn types_len(&self) -> usize {
    unsafe { self.ptr.as_ref() }.types.len()
  }
  pub fn types(&self, n: usize) -> Option<crate::rt::View<'_, Type>> {
    unsafe { self.ptr.as_ref().types.as_slice() }.get(n)
      .map(|&ptr| crate::rt::View::<Type> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn types_iter(&self) -> impl Iterator<Item = crate::rt::View<'_, Type>> + '_ {
    unsafe { self.ptr.as_ref().types.as_slice() }.iter()
      .map(|&ptr| crate::rt::View::<Type> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn types_mut(&mut self, n: usize) -> Option<crate::rt::Mut<'_, Type>> {
    unsafe { self.ptr.as_mut().types.as_mut_slice() }.get_mut(n)
      .map(|&mut ptr| crate::rt::Mut::<Type> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
        arena: self.arena,
      })
  }
  pub fn types_add(&mut self) -> crate::rt::Mut<'_, Type> {
    unsafe {
      let vec = &mut self.ptr.as_mut().types;
      let new_len = vec.len() + 1;
      vec.resize_msg(new_len, self.arena,
        Type::__LAYOUT, Type::__raw_init);
      self.types_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn types_resize(&mut self, n: usize) {
    unsafe {
      self.ptr.as_mut().types.resize_msg(
        n, self.arena,
        Type::__LAYOUT, Type::__raw_init);
    }
  }

  pub fn packages_len(&self) -> usize {
    unsafe { self.ptr.as_ref() }.packages.len()
  }
  pub fn packages(&self, n: usize) -> Option<&'_ crate::rt::Str> {
    unsafe { self.ptr.as_ref().packages.as_slice() }.get(n).map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn packages_iter(&self) -> impl Iterator<Item = &'_ crate::rt::Str> + '_ {
    unsafe { self.ptr.as_ref().packages.as_slice() }.iter().map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn packages_mut(&mut self, n: usize) -> Option<crate::rt::StrBuf<'_>> {
    unsafe { self.ptr.as_mut().packages.as_mut_slice() }.get_mut(n)
      .map(|data| crate::rt::StrBuf::__wrap(data, self.arena))
  }
  pub fn packages_add(&mut self) -> crate::rt::StrBuf<'_> {
    unsafe {
      let vec = &mut self.ptr.as_mut().packages;
      let new_len = vec.len() + 1;
      vec.resize(new_len, self.arena);
      self.packages_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn packages_resize(&mut self, n: usize) {
    unsafe {
      self.ptr.as_mut().packages.resize(n, self.arena);
    }
  }

  pub fn foreign_types_len(&self) -> usize {
    unsafe { self.ptr.as_ref() }.foreign_types.len()
  }
  pub fn foreign_types(&self, n: usize) -> Option<&'_ crate::rt::Str> {
    unsafe { self.ptr.as_ref().foreign_types.as_slice() }.get(n).map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn foreign_types_iter(&self) -> impl Iterator<Item = &'_ crate::rt::Str> + '_ {
    unsafe { self.ptr.as_ref().foreign_types.as_slice() }.iter().map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn foreign_types_mut(&mut self, n: usize) -> Option<crate::rt::StrBuf<'_>> {
    unsafe { self.ptr.as_mut().foreign_types.as_mut_slice() }.get_mut(n)
      .map(|data| crate::rt::StrBuf::__wrap(data, self.arena))
  }
  pub fn foreign_types_add(&mut self) -> crate::rt::StrBuf<'_> {
    unsafe {
      let vec = &mut self.ptr.as_mut().foreign_types;
      let new_len = vec.len() + 1;
      vec.resize(new_len, self.arena);
      self.foreign_types_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn foreign_types_resize(&mut self, n: usize) {
    unsafe {
      self.ptr.as_mut().foreign_types.resize(n, self.arena);
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

impl<'msg> __priv_Bundle::View<'msg> {
  pub fn types_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.types.len()
  }
  pub fn types(self, n: usize) -> Option<crate::rt::View<'msg, Type>> {
    unsafe { self.ptr.as_ref().types.as_slice() }.get(n)
      .map(|&ptr| crate::rt::View::<Type> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn types_iter(self) -> impl Iterator<Item = crate::rt::View<'msg, Type>> + 'msg {
    unsafe { self.ptr.as_ref().types.as_slice() }.iter()
      .map(|&ptr| crate::rt::View::<Type> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }

  pub fn packages_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.packages.len()
  }
  pub fn packages(self, n: usize) -> Option<&'msg crate::rt::Str> {
    unsafe { self.ptr.as_ref().packages.as_slice() }.get(n).map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn packages_iter(self) -> impl Iterator<Item = &'msg crate::rt::Str> + 'msg {
    unsafe { self.ptr.as_ref().packages.as_slice() }.iter().map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }

  pub fn foreign_types_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.foreign_types.len()
  }
  pub fn foreign_types(self, n: usize) -> Option<&'msg crate::rt::Str> {
    unsafe { self.ptr.as_ref().foreign_types.as_slice() }.get(n).map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn foreign_types_iter(self) -> impl Iterator<Item = &'msg crate::rt::Str> + 'msg {
    unsafe { self.ptr.as_ref().foreign_types.as_slice() }.iter().map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    for value in self.types_iter() {
      if count != 0 { debug.comma(false)?; }
      debug.field("types")?;
      value.__debug(debug)?;
      count += 1;
    }
    if self.packages_len() != 0 {
      if count != 0 { debug.comma(false)?; }
      debug.field("packages")?;
      debug.iter(self.packages_iter())?;
      count += 1;
    }
    if self.foreign_types_len() != 0 {
      if count != 0 { debug.comma(false)?; }
      debug.field("foreign_types")?;
      debug.iter(self.foreign_types_iter())?;
      count += 1;
    }
    if count != 0 {
      debug.comma(true)?;
    }
    debug.end_block()?;
    Ok(())
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

  pub fn types_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.types.len()
  }
  pub fn types(self, n: usize) -> Option<crate::rt::View<'msg, Type>> {
    unsafe { self.ptr.as_ref().types.as_slice() }.get(n)
      .map(|&ptr| crate::rt::View::<Type> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn types_iter(self) -> impl Iterator<Item = crate::rt::View<'msg, Type>> + 'msg {
    unsafe { self.ptr.as_ref().types.as_slice() }.iter()
      .map(|&ptr| crate::rt::View::<Type> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn types_mut(self, n: usize) -> Option<crate::rt::Mut<'msg, Type>> {
    unsafe { self.ptr.as_mut().types.as_mut_slice() }.get_mut(n)
      .map(|&mut ptr| crate::rt::Mut::<Type> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
        arena: self.arena,
      })
  }
  pub fn types_add(self) -> crate::rt::Mut<'msg, Type> {
    unsafe {
      let vec = &mut self.ptr.as_mut().types;
      let new_len = vec.len() + 1;
      vec.resize_msg(new_len, self.arena,
        Type::__LAYOUT, Type::__raw_init);
      self.types_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn types_resize(self, n: usize) {
    unsafe {
      self.ptr.as_mut().types.resize_msg(
        n, self.arena,
        Type::__LAYOUT, Type::__raw_init);
    }
  }

  pub fn packages_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.packages.len()
  }
  pub fn packages(self, n: usize) -> Option<&'msg crate::rt::Str> {
    unsafe { self.ptr.as_ref().packages.as_slice() }.get(n).map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn packages_iter(self) -> impl Iterator<Item = &'msg crate::rt::Str> + 'msg {
    unsafe { self.ptr.as_ref().packages.as_slice() }.iter().map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn packages_mut(self, n: usize) -> Option<crate::rt::StrBuf<'msg>> {
    unsafe { self.ptr.as_mut().packages.as_mut_slice() }.get_mut(n)
      .map(|data| crate::rt::StrBuf::__wrap(data, self.arena))
  }
  pub fn packages_add(self) -> crate::rt::StrBuf<'msg> {
    unsafe {
      let vec = &mut self.ptr.as_mut().packages;
      let new_len = vec.len() + 1;
      vec.resize(new_len, self.arena);
      self.packages_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn packages_resize(self, n: usize) {
    unsafe {
      self.ptr.as_mut().packages.resize(n, self.arena);
    }
  }

  pub fn foreign_types_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.foreign_types.len()
  }
  pub fn foreign_types(self, n: usize) -> Option<&'msg crate::rt::Str> {
    unsafe { self.ptr.as_ref().foreign_types.as_slice() }.get(n).map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn foreign_types_iter(self) -> impl Iterator<Item = &'msg crate::rt::Str> + 'msg {
    unsafe { self.ptr.as_ref().foreign_types.as_slice() }.iter().map(|&(p, n)| unsafe {
      crate::rt::Str::from_raw_parts(p, n)
    })
  }
  pub fn foreign_types_mut(self, n: usize) -> Option<crate::rt::StrBuf<'msg>> {
    unsafe { self.ptr.as_mut().foreign_types.as_mut_slice() }.get_mut(n)
      .map(|data| crate::rt::StrBuf::__wrap(data, self.arena))
  }
  pub fn foreign_types_add(self) -> crate::rt::StrBuf<'msg> {
    unsafe {
      let vec = &mut self.ptr.as_mut().foreign_types;
      let new_len = vec.len() + 1;
      vec.resize(new_len, self.arena);
      self.foreign_types_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn foreign_types_resize(self, n: usize) {
    unsafe {
      self.ptr.as_mut().foreign_types.resize(n, self.arena);
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
    pub(crate) foreign_types: crate::rt::__z::AVec<(*mut u8, usize)>,
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
          ty: 0,
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
          flags: (crate::rt::__z::tdp::Kind::Str as u8 as u32) | (1 << 4),
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

  pub fn name(&self) -> &'_ crate::rt::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(&self) -> Option<&'_ crate::rt::Str> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (ptr, len) = self.ptr.as_ref().name;
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::rt::StrBuf<'_> {
    unsafe {
      let mut buf = crate::rt::StrBuf::__wrap(&mut self.ptr.as_mut().name, self.arena);
      if self.ptr.as_ref().__hasbits[0] & 1 == 0 {
        buf.clear();
      }
      self.ptr.as_mut().__hasbits[0] |= 1;
      buf
    }
  }
  pub fn name_opt_mut(&mut self) -> Option<crate::rt::StrBuf<'_>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      crate::rt::StrBuf::__wrap(&mut self.ptr.as_mut().name, self.arena)
    })
  }
  pub fn name_set<'a>(&mut self, value: impl crate::rt::str::IntoStrOpt<'a>) {
    match value.into_str_opt() {
      Some(value) => self.name_mut().set(value.as_bytes()),
      None => unsafe {
        self.ptr.as_mut().__hasbits[0] &= !1;
      }
    }
  }

  pub fn package(&self) -> u32 {
    self.package_opt().unwrap_or_default()
  }
  pub fn package_opt(&self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().package) })
  }
  pub fn package_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 2;
        self.ptr.as_mut().package = std::mem::transmute::<u32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !2;
      }
    }
  }

  pub fn kind(&self) -> Type_Kind {
    self.kind_opt().unwrap_or_default()
  }
  pub fn kind_opt(&self) -> Option<Type_Kind> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Type_Kind>(self.ptr.as_ref().kind) })
  }
  pub fn kind_set(&mut self, value: impl Into<Option<Type_Kind>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 4;
        self.ptr.as_mut().kind = std::mem::transmute::<Type_Kind, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !4;
      }
    }
  }

  pub fn declared_in(&self) -> u32 {
    self.declared_in_opt().unwrap_or_default()
  }
  pub fn declared_in_opt(&self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().declared_in) })
  }
  pub fn declared_in_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 8;
        self.ptr.as_mut().declared_in = std::mem::transmute::<u32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !8;
      }
    }
  }

  pub fn fields_len(&self) -> usize {
    unsafe { self.ptr.as_ref() }.fields.len()
  }
  pub fn fields(&self, n: usize) -> Option<crate::rt::View<'_, Field>> {
    unsafe { self.ptr.as_ref().fields.as_slice() }.get(n)
      .map(|&ptr| crate::rt::View::<Field> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn fields_iter(&self) -> impl Iterator<Item = crate::rt::View<'_, Field>> + '_ {
    unsafe { self.ptr.as_ref().fields.as_slice() }.iter()
      .map(|&ptr| crate::rt::View::<Field> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn fields_mut(&mut self, n: usize) -> Option<crate::rt::Mut<'_, Field>> {
    unsafe { self.ptr.as_mut().fields.as_mut_slice() }.get_mut(n)
      .map(|&mut ptr| crate::rt::Mut::<Field> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
        arena: self.arena,
      })
  }
  pub fn fields_add(&mut self) -> crate::rt::Mut<'_, Field> {
    unsafe {
      let vec = &mut self.ptr.as_mut().fields;
      let new_len = vec.len() + 1;
      vec.resize_msg(new_len, self.arena,
        Field::__LAYOUT, Field::__raw_init);
      self.fields_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn fields_resize(&mut self, n: usize) {
    unsafe {
      self.ptr.as_mut().fields.resize_msg(
        n, self.arena,
        Field::__LAYOUT, Field::__raw_init);
    }
  }

  pub fn nesteds(&self) -> &'_ [u32] {
    unsafe {
      let slice = self.ptr.as_ref().nesteds.as_slice();
      std::mem::transmute::<&'_ [u32], &'_ [u32]>(slice)
    }
  }
  pub fn nesteds_mut(&mut self) -> &'_ mut [u32] {
    unsafe {
      let slice = self.ptr.as_mut().nesteds.as_mut_slice();
      std::mem::transmute::<&'_ mut [u32], &'_ mut [u32]>(slice)
    }
  }
  pub fn nesteds_set(&mut self, that: &[u32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().nesteds;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn nesteds_extend(&mut self, that: &[u32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().nesteds;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn span(&self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(&self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }
  pub fn span_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 16;
        self.ptr.as_mut().span = std::mem::transmute::<u32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !16;
      }
    }
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

impl<'msg> __priv_Type::View<'msg> {
  pub fn name(self) -> &'msg crate::rt::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(self) -> Option<&'msg crate::rt::Str> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (ptr, len) = self.ptr.as_ref().name;
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn package(self) -> u32 {
    self.package_opt().unwrap_or_default()
  }
  pub fn package_opt(self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().package) })
  }

  pub fn kind(self) -> Type_Kind {
    self.kind_opt().unwrap_or_default()
  }
  pub fn kind_opt(self) -> Option<Type_Kind> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Type_Kind>(self.ptr.as_ref().kind) })
  }

  pub fn declared_in(self) -> u32 {
    self.declared_in_opt().unwrap_or_default()
  }
  pub fn declared_in_opt(self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().declared_in) })
  }

  pub fn fields_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.fields.len()
  }
  pub fn fields(self, n: usize) -> Option<crate::rt::View<'msg, Field>> {
    unsafe { self.ptr.as_ref().fields.as_slice() }.get(n)
      .map(|&ptr| crate::rt::View::<Field> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn fields_iter(self) -> impl Iterator<Item = crate::rt::View<'msg, Field>> + 'msg {
    unsafe { self.ptr.as_ref().fields.as_slice() }.iter()
      .map(|&ptr| crate::rt::View::<Field> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }

  pub fn nesteds(self) -> &'msg [u32] {
    unsafe {
      let slice = self.ptr.as_ref().nesteds.as_slice();
      std::mem::transmute::<&'msg [u32], &'msg [u32]>(slice)
    }
  }

  pub fn span(self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.name_opt() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.package_opt() {
      if count != 0 { debug.comma(false)?; }
      debug.field("package")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.kind_opt() {
      if count != 0 { debug.comma(false)?; }
      debug.field("kind")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.declared_in_opt() {
      if count != 0 { debug.comma(false)?; }
      debug.field("declared_in")?;
      debug.write_debug(value);
      count += 1;
    }
    for value in self.fields_iter() {
      if count != 0 { debug.comma(false)?; }
      debug.field("fields")?;
      value.__debug(debug)?;
      count += 1;
    }
    let slice = self.nesteds();
    if !slice.is_empty() {
      if count != 0 { debug.comma(false)?; }
      debug.field("nesteds")?;
      debug.iter(slice)?;
      count += 1;
    }
    if let Some(value) = self.span_opt() {
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

impl<'msg> __priv_Type::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { Type::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Type::__tdp_info())
  }

  pub fn name(self) -> &'msg crate::rt::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(self) -> Option<&'msg crate::rt::Str> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (ptr, len) = self.ptr.as_ref().name;
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::rt::StrBuf<'msg> {
    unsafe {
      let mut buf = crate::rt::StrBuf::__wrap(&mut self.ptr.as_mut().name, self.arena);
      if self.ptr.as_ref().__hasbits[0] & 1 == 0 {
        buf.clear();
      }
      self.ptr.as_mut().__hasbits[0] |= 1;
      buf
    }
  }
  pub fn name_opt_mut(self) -> Option<crate::rt::StrBuf<'msg>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      crate::rt::StrBuf::__wrap(&mut self.ptr.as_mut().name, self.arena)
    })
  }
  pub fn name_set<'a>(self, value: impl crate::rt::str::IntoStrOpt<'a>) {
    match value.into_str_opt() {
      Some(value) => self.name_mut().set(value.as_bytes()),
      None => unsafe {
        self.ptr.as_mut().__hasbits[0] &= !1;
      }
    }
  }

  pub fn package(self) -> u32 {
    self.package_opt().unwrap_or_default()
  }
  pub fn package_opt(self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().package) })
  }
  pub fn package_set(self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 2;
        self.ptr.as_mut().package = std::mem::transmute::<u32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !2;
      }
    }
  }

  pub fn kind(self) -> Type_Kind {
    self.kind_opt().unwrap_or_default()
  }
  pub fn kind_opt(self) -> Option<Type_Kind> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Type_Kind>(self.ptr.as_ref().kind) })
  }
  pub fn kind_set(self, value: impl Into<Option<Type_Kind>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 4;
        self.ptr.as_mut().kind = std::mem::transmute::<Type_Kind, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !4;
      }
    }
  }

  pub fn declared_in(self) -> u32 {
    self.declared_in_opt().unwrap_or_default()
  }
  pub fn declared_in_opt(self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().declared_in) })
  }
  pub fn declared_in_set(self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 8;
        self.ptr.as_mut().declared_in = std::mem::transmute::<u32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !8;
      }
    }
  }

  pub fn fields_len(self) -> usize {
    unsafe { self.ptr.as_ref() }.fields.len()
  }
  pub fn fields(self, n: usize) -> Option<crate::rt::View<'msg, Field>> {
    unsafe { self.ptr.as_ref().fields.as_slice() }.get(n)
      .map(|&ptr| crate::rt::View::<Field> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn fields_iter(self) -> impl Iterator<Item = crate::rt::View<'msg, Field>> + 'msg {
    unsafe { self.ptr.as_ref().fields.as_slice() }.iter()
      .map(|&ptr| crate::rt::View::<Field> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
      })
  }
  pub fn fields_mut(self, n: usize) -> Option<crate::rt::Mut<'msg, Field>> {
    unsafe { self.ptr.as_mut().fields.as_mut_slice() }.get_mut(n)
      .map(|&mut ptr| crate::rt::Mut::<Field> {
        ptr: unsafe { crate::rt::__z::ABox::from_ptr(ptr) },
        _ph: std::marker::PhantomData,
        arena: self.arena,
      })
  }
  pub fn fields_add(self) -> crate::rt::Mut<'msg, Field> {
    unsafe {
      let vec = &mut self.ptr.as_mut().fields;
      let new_len = vec.len() + 1;
      vec.resize_msg(new_len, self.arena,
        Field::__LAYOUT, Field::__raw_init);
      self.fields_mut(new_len - 1).unwrap_unchecked()
    }
  }
  pub fn fields_resize(self, n: usize) {
    unsafe {
      self.ptr.as_mut().fields.resize_msg(
        n, self.arena,
        Field::__LAYOUT, Field::__raw_init);
    }
  }

  pub fn nesteds(self) -> &'msg [u32] {
    unsafe {
      let slice = self.ptr.as_ref().nesteds.as_slice();
      std::mem::transmute::<&'msg [u32], &'msg [u32]>(slice)
    }
  }
  pub fn nesteds_mut(self) -> &'msg mut [u32] {
    unsafe {
      let slice = self.ptr.as_mut().nesteds.as_mut_slice();
      std::mem::transmute::<&'msg mut [u32], &'msg mut [u32]>(slice)
    }
  }
  pub fn nesteds_set(self, that: &[u32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().nesteds;
      vec.resize(that.len(), self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr();
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }
  pub fn nesteds_extend(self, that: &[u32]) {
    unsafe {
      let vec = &mut self.ptr.as_mut().nesteds;
      let old_len = vec.len();
      let new_len = old_len + that.len();
      vec.resize(new_len, self.arena);
      let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
      ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
    }
  }

  pub fn span(self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }
  pub fn span_set(self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 16;
        self.ptr.as_mut().span = std::mem::transmute::<u32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !16;
      }
    }
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
  pub const FIELD_OFFSET_span: u32 = unsafe {
    let msg = Type::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().span as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{7 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{7 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = Type::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
            Field::__tdp_info,
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
          number: 20,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Type::FIELD_OFFSET_span,
          ty: 0,
          hasbit: 4,
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

  pub fn name(&self) -> &'_ crate::rt::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(&self) -> Option<&'_ crate::rt::Str> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (ptr, len) = self.ptr.as_ref().name;
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::rt::StrBuf<'_> {
    unsafe {
      let mut buf = crate::rt::StrBuf::__wrap(&mut self.ptr.as_mut().name, self.arena);
      if self.ptr.as_ref().__hasbits[0] & 1 == 0 {
        buf.clear();
      }
      self.ptr.as_mut().__hasbits[0] |= 1;
      buf
    }
  }
  pub fn name_opt_mut(&mut self) -> Option<crate::rt::StrBuf<'_>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      crate::rt::StrBuf::__wrap(&mut self.ptr.as_mut().name, self.arena)
    })
  }
  pub fn name_set<'a>(&mut self, value: impl crate::rt::str::IntoStrOpt<'a>) {
    match value.into_str_opt() {
      Some(value) => self.name_mut().set(value.as_bytes()),
      None => unsafe {
        self.ptr.as_mut().__hasbits[0] &= !1;
      }
    }
  }

  pub fn number(&self) -> i32 {
    self.number_opt().unwrap_or_default()
  }
  pub fn number_opt(&self) -> Option<i32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().number) })
  }
  pub fn number_set(&mut self, value: impl Into<Option<i32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 2;
        self.ptr.as_mut().number = std::mem::transmute::<i32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !2;
      }
    }
  }

  pub fn is_repeated(&self) -> bool {
    self.is_repeated_opt().unwrap_or_default()
  }
  pub fn is_repeated_opt(&self) -> Option<bool> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().is_repeated) })
  }
  pub fn is_repeated_set(&mut self, value: impl Into<Option<bool>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 4;
        self.ptr.as_mut().is_repeated = std::mem::transmute::<bool, bool>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !4;
      }
    }
  }

  pub fn r#type(&self) -> Field_Type {
    self.r#type_opt().unwrap_or_default()
  }
  pub fn r#type_opt(&self) -> Option<Field_Type> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Field_Type>(self.ptr.as_ref().r#type) })
  }
  pub fn r#type_set(&mut self, value: impl Into<Option<Field_Type>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 8;
        self.ptr.as_mut().r#type = std::mem::transmute::<Field_Type, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !8;
      }
    }
  }

  pub fn type_index(&self) -> u32 {
    self.type_index_opt().unwrap_or_default()
  }
  pub fn type_index_opt(&self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().type_index) })
  }
  pub fn type_index_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 16;
        self.ptr.as_mut().type_index = std::mem::transmute::<u32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !16;
      }
    }
  }

  pub fn span(&self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(&self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }
  pub fn span_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 32;
        self.ptr.as_mut().span = std::mem::transmute::<u32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !32;
      }
    }
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

impl<'msg> __priv_Field::View<'msg> {
  pub fn name(self) -> &'msg crate::rt::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(self) -> Option<&'msg crate::rt::Str> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (ptr, len) = self.ptr.as_ref().name;
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn number(self) -> i32 {
    self.number_opt().unwrap_or_default()
  }
  pub fn number_opt(self) -> Option<i32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().number) })
  }

  pub fn is_repeated(self) -> bool {
    self.is_repeated_opt().unwrap_or_default()
  }
  pub fn is_repeated_opt(self) -> Option<bool> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().is_repeated) })
  }

  pub fn r#type(self) -> Field_Type {
    self.r#type_opt().unwrap_or_default()
  }
  pub fn r#type_opt(self) -> Option<Field_Type> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Field_Type>(self.ptr.as_ref().r#type) })
  }

  pub fn type_index(self) -> u32 {
    self.type_index_opt().unwrap_or_default()
  }
  pub fn type_index_opt(self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().type_index) })
  }

  pub fn span(self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::rt::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.name_opt() {
      if count != 0 { debug.comma(false)?; }
      debug.field("name")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.number_opt() {
      if count != 0 { debug.comma(false)?; }
      debug.field("number")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.is_repeated_opt() {
      if count != 0 { debug.comma(false)?; }
      debug.field("is_repeated")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.r#type_opt() {
      if count != 0 { debug.comma(false)?; }
      debug.field("type")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.type_index_opt() {
      if count != 0 { debug.comma(false)?; }
      debug.field("type_index")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.span_opt() {
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

impl<'msg> __priv_Field::Mut<'msg>  {
  pub fn clear(self) {
    unsafe { Field::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::rt::Error> {
    let mut ctx = crate::rt::__z::tdp::ParseCtx::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Field::__tdp_info())
  }

  pub fn name(self) -> &'msg crate::rt::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(self) -> Option<&'msg crate::rt::Str> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      let (ptr, len) = self.ptr.as_ref().name;
      crate::rt::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::rt::StrBuf<'msg> {
    unsafe {
      let mut buf = crate::rt::StrBuf::__wrap(&mut self.ptr.as_mut().name, self.arena);
      if self.ptr.as_ref().__hasbits[0] & 1 == 0 {
        buf.clear();
      }
      self.ptr.as_mut().__hasbits[0] |= 1;
      buf
    }
  }
  pub fn name_opt_mut(self) -> Option<crate::rt::StrBuf<'msg>> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 1 == 0 { return None }
    Some(unsafe {
      crate::rt::StrBuf::__wrap(&mut self.ptr.as_mut().name, self.arena)
    })
  }
  pub fn name_set<'a>(self, value: impl crate::rt::str::IntoStrOpt<'a>) {
    match value.into_str_opt() {
      Some(value) => self.name_mut().set(value.as_bytes()),
      None => unsafe {
        self.ptr.as_mut().__hasbits[0] &= !1;
      }
    }
  }

  pub fn number(self) -> i32 {
    self.number_opt().unwrap_or_default()
  }
  pub fn number_opt(self) -> Option<i32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 2 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(self.ptr.as_ref().number) })
  }
  pub fn number_set(self, value: impl Into<Option<i32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 2;
        self.ptr.as_mut().number = std::mem::transmute::<i32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !2;
      }
    }
  }

  pub fn is_repeated(self) -> bool {
    self.is_repeated_opt().unwrap_or_default()
  }
  pub fn is_repeated_opt(self) -> Option<bool> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 4 == 0 { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(self.ptr.as_ref().is_repeated) })
  }
  pub fn is_repeated_set(self, value: impl Into<Option<bool>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 4;
        self.ptr.as_mut().is_repeated = std::mem::transmute::<bool, bool>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !4;
      }
    }
  }

  pub fn r#type(self) -> Field_Type {
    self.r#type_opt().unwrap_or_default()
  }
  pub fn r#type_opt(self) -> Option<Field_Type> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 8 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, Field_Type>(self.ptr.as_ref().r#type) })
  }
  pub fn r#type_set(self, value: impl Into<Option<Field_Type>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 8;
        self.ptr.as_mut().r#type = std::mem::transmute::<Field_Type, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !8;
      }
    }
  }

  pub fn type_index(self) -> u32 {
    self.type_index_opt().unwrap_or_default()
  }
  pub fn type_index_opt(self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 16 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().type_index) })
  }
  pub fn type_index_set(self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 16;
        self.ptr.as_mut().type_index = std::mem::transmute::<u32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !16;
      }
    }
  }

  pub fn span(self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(self) -> Option<u32> {
    if unsafe { self.ptr.as_ref() }.__hasbits[0] & 32 == 0 { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(self.ptr.as_ref().span) })
  }
  pub fn span_set(self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => unsafe {
        self.ptr.as_mut().__hasbits[0] |= 32;
        self.ptr.as_mut().span = std::mem::transmute::<u32, u32>(value);
      }
      None => {
        unsafe { self.ptr.as_mut() }.__hasbits[0] &= !32;
      }
    }
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
  pub const FIELD_OFFSET_span: u32 = unsafe {
    let msg = Field::DEFAULT;
    let top = msg.ptr.as_ptr().cast::<u8>();
    let field = &msg.ptr.as_ref().span as *const _ as *const u8;
    field.offset_from(top) as u32
  };

  pub static TDP_INFO: crate::rt::__z::tdp::MessageAndFields<{6 + 1}> =
    crate::rt::__z::tdp::MessageAndFields::<{6 + 1}> {
      msg: crate::rt::__z::tdp::Message {
        size: {
          let size = Field::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        tys: {
          const TYS: &[fn() -> *const crate::rt::__z::tdp::Message] = &[
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
          number: 20,
          flags: (crate::rt::__z::tdp::Kind::I32 as u8 as u32) | (0 << 4),
          offset: __priv_Field::FIELD_OFFSET_span,
          ty: 0,
          hasbit: 5,
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

