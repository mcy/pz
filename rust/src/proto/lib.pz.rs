// ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]

/// choice `pz.plugin.Request`
pub struct Request {
  ptr: crate::__z::ABox<__priv_Request::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_Request::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl Request {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_Request::Storage = __priv_Request::Storage {
      which: 0,
      union: __priv_Request::Union { __unset: () },
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_Request::Storage as *mut __priv_Request::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Request::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Request::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(&self) -> RequestCases<'_, crate::ptr::select::View> {
    self.as_view().cases()
  }

  pub fn cases_mut(&mut self) -> RequestCases<'_, crate::ptr::select::Mut> {
    self.as_mut().cases_mut()
  }

  pub fn clear(&mut self) {
    unsafe { Request::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn about(&self) -> crate::View<'_, AboutRequest> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(&self) -> Option<crate::View<'_, AboutRequest>> {
    if !unsafe { Request::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<AboutRequest as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.about } as *mut _ as *mut u8)) }
  }
  pub fn about_mut(&mut self) -> crate::Mut<'_, AboutRequest> {
    self.about_mut_or().into_mut()
  }
  pub fn about_mut_or(&mut self) -> crate::value::OptMut<'_, AboutRequest> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Request::__tdp_info().field(0),
      )
    }
  }

  pub fn codegen(&self) -> crate::View<'_, CodegenRequest> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(&self) -> Option<crate::View<'_, CodegenRequest>> {
    if !unsafe { Request::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<CodegenRequest as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.codegen } as *mut _ as *mut u8)) }
  }
  pub fn codegen_mut(&mut self) -> crate::Mut<'_, CodegenRequest> {
    self.codegen_mut_or().into_mut()
  }
  pub fn codegen_mut_or(&mut self) -> crate::value::OptMut<'_, CodegenRequest> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Request::__tdp_info().field(1),
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_Request::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Request::Storage>()).which = 0;
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_Request::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

pub enum RequestCases<'proto, Which: crate::ptr::select::Select> {
  Unset(std::marker::PhantomData<&'proto Which>),
  About(crate::ptr::Proxy<'proto, AboutRequest, Which>),
  Codegen(crate::ptr::Proxy<'proto, CodegenRequest, Which>),
}

impl Default for Request {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for Request {
  type View<'proto> = __priv_Request::View<'proto>;
  type Mut<'proto> = __priv_Request::Mut<'proto>;
}

impl<'proto> __priv_Request::View<'proto> {
  pub fn as_view(&self) -> crate::View<Request> {
    __priv_Request::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn cases(self) -> RequestCases<'proto, crate::ptr::select::View> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      let raw = self.ptr.as_ptr().add(__priv_Request::UNION_OFFSET);
      match number {
        0 => RequestCases::Unset(std::marker::PhantomData),
        1 => RequestCases::About(<AboutRequest as crate::value::Type>::__make_view(raw)),
        2 => RequestCases::Codegen(<CodegenRequest as crate::value::Type>::__make_view(raw)),
        _ => unreachable!(),
      }
    }
  }

  pub fn about(self) -> crate::View<'proto, AboutRequest> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> Option<crate::View<'proto, AboutRequest>> {
    if !unsafe { Request::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<AboutRequest as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.about } as *mut _ as *mut u8)) }
  }

  pub fn codegen(self) -> crate::View<'proto, CodegenRequest> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> Option<crate::View<'proto, CodegenRequest>> {
    if !unsafe { Request::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<CodegenRequest as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.codegen } as *mut _ as *mut u8)) }
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.about_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("about")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let Some(value) = self.codegen_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("codegen")?;
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

impl Default for __priv_Request::View<'_> {
  fn default() -> Self {
    Request::DEFAULT
  }
}

impl<'proto> __priv_Request::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<Request> {
    __priv_Request::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, Request> {
    __priv_Request::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Request> {
    __priv_Request::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(self) -> RequestCases<'proto, crate::ptr::select::View> {
    self.into_view().cases()
  }

  pub fn cases_mut(self) -> RequestCases<'proto, crate::ptr::select::Mut> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      let raw = self.ptr.as_ptr().add(__priv_Request::UNION_OFFSET);
      match number {
        0 => RequestCases::Unset(std::marker::PhantomData),
        1 => RequestCases::About(<AboutRequest as crate::value::Type>::__make_mut(raw, self.arena)),
        2 => RequestCases::Codegen(<CodegenRequest as crate::value::Type>::__make_mut(raw, self.arena)),
        _ => unreachable!(),
      }
    }
  }

  pub fn clear(self) {
    unsafe { Request::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Request::__tdp_info())
  }

  pub fn about(self) -> crate::View<'proto, AboutRequest> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> Option<crate::View<'proto, AboutRequest>> {
    if !unsafe { Request::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<AboutRequest as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.about } as *mut _ as *mut u8)) }
  }
  pub fn about_mut(self) -> crate::Mut<'proto, AboutRequest> {
    self.about_mut_or().into_mut()
  }
  pub fn about_mut_or(self) -> crate::value::OptMut<'proto, AboutRequest> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Request::__tdp_info().field(0),
      )
    }
  }

  pub fn codegen(self) -> crate::View<'proto, CodegenRequest> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> Option<crate::View<'proto, CodegenRequest>> {
    if !unsafe { Request::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<CodegenRequest as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.codegen } as *mut _ as *mut u8)) }
  }
  pub fn codegen_mut(self) -> crate::Mut<'proto, CodegenRequest> {
    self.codegen_mut_or().into_mut()
  }
  pub fn codegen_mut_or(self) -> crate::value::OptMut<'proto, CodegenRequest> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Request::__tdp_info().field(1),
      )
    }
  }

}

impl Drop for Request {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_Request::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.Request ")?;
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Request::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for Request {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl crate::value::Type for Request {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_Request::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_Request::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
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
    let align = std::mem::align_of::<__priv_Request::Union>();
    if align < 4 { 4 } else { align }
  };

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{2 + 1}> =
    crate::__z::tdp::DescStorage::<{2 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = Request::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
            AboutRequest::__tdp_info,
            CodegenRequest::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 0,
        kind: crate::__z::tdp::DescKind::Choice,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: __priv_Request::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: __priv_Request::UNION_OFFSET as u32,
          desc: 1,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Request::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto Request>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Request> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Request::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut Request>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Request> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::Request> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// choice `pz.plugin.Response`
pub struct Response {
  ptr: crate::__z::ABox<__priv_Response::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_Response::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl Response {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_Response::Storage = __priv_Response::Storage {
      which: 0,
      union: __priv_Response::Union { __unset: () },
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_Response::Storage as *mut __priv_Response::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Response::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Response::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(&self) -> ResponseCases<'_, crate::ptr::select::View> {
    self.as_view().cases()
  }

  pub fn cases_mut(&mut self) -> ResponseCases<'_, crate::ptr::select::Mut> {
    self.as_mut().cases_mut()
  }

  pub fn clear(&mut self) {
    unsafe { Response::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn about(&self) -> crate::View<'_, AboutResponse> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(&self) -> Option<crate::View<'_, AboutResponse>> {
    if !unsafe { Response::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<AboutResponse as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.about } as *mut _ as *mut u8)) }
  }
  pub fn about_mut(&mut self) -> crate::Mut<'_, AboutResponse> {
    self.about_mut_or().into_mut()
  }
  pub fn about_mut_or(&mut self) -> crate::value::OptMut<'_, AboutResponse> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Response::__tdp_info().field(0),
      )
    }
  }

  pub fn codegen(&self) -> crate::View<'_, CodegenResponse> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(&self) -> Option<crate::View<'_, CodegenResponse>> {
    if !unsafe { Response::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<CodegenResponse as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.codegen } as *mut _ as *mut u8)) }
  }
  pub fn codegen_mut(&mut self) -> crate::Mut<'_, CodegenResponse> {
    self.codegen_mut_or().into_mut()
  }
  pub fn codegen_mut_or(&mut self) -> crate::value::OptMut<'_, CodegenResponse> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Response::__tdp_info().field(1),
      )
    }
  }

  #[doc(hidden)]
  pub const __LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<__priv_Response::Storage>();
  #[doc(hidden)]
  pub unsafe fn __raw_clear(raw: *mut u8) {
    (&mut *raw.cast::<__priv_Response::Storage>()).which = 0;
  }
  #[doc(hidden)]
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_Response::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

pub enum ResponseCases<'proto, Which: crate::ptr::select::Select> {
  Unset(std::marker::PhantomData<&'proto Which>),
  About(crate::ptr::Proxy<'proto, AboutResponse, Which>),
  Codegen(crate::ptr::Proxy<'proto, CodegenResponse, Which>),
}

impl Default for Response {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for Response {
  type View<'proto> = __priv_Response::View<'proto>;
  type Mut<'proto> = __priv_Response::Mut<'proto>;
}

impl<'proto> __priv_Response::View<'proto> {
  pub fn as_view(&self) -> crate::View<Response> {
    __priv_Response::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn cases(self) -> ResponseCases<'proto, crate::ptr::select::View> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      let raw = self.ptr.as_ptr().add(__priv_Response::UNION_OFFSET);
      match number {
        0 => ResponseCases::Unset(std::marker::PhantomData),
        1 => ResponseCases::About(<AboutResponse as crate::value::Type>::__make_view(raw)),
        2 => ResponseCases::Codegen(<CodegenResponse as crate::value::Type>::__make_view(raw)),
        _ => unreachable!(),
      }
    }
  }

  pub fn about(self) -> crate::View<'proto, AboutResponse> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> Option<crate::View<'proto, AboutResponse>> {
    if !unsafe { Response::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<AboutResponse as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.about } as *mut _ as *mut u8)) }
  }

  pub fn codegen(self) -> crate::View<'proto, CodegenResponse> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> Option<crate::View<'proto, CodegenResponse>> {
    if !unsafe { Response::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<CodegenResponse as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.codegen } as *mut _ as *mut u8)) }
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.about_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("about")?;
      value.__debug(debug)?;
      count += 1;
    }
    if let Some(value) = self.codegen_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("codegen")?;
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

impl Default for __priv_Response::View<'_> {
  fn default() -> Self {
    Response::DEFAULT
  }
}

impl<'proto> __priv_Response::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<Response> {
    __priv_Response::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, Response> {
    __priv_Response::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Response> {
    __priv_Response::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn cases(self) -> ResponseCases<'proto, crate::ptr::select::View> {
    self.into_view().cases()
  }

  pub fn cases_mut(self) -> ResponseCases<'proto, crate::ptr::select::Mut> {
    unsafe {
      let number = self.ptr.as_ptr().cast::<u32>().read();
      let raw = self.ptr.as_ptr().add(__priv_Response::UNION_OFFSET);
      match number {
        0 => ResponseCases::Unset(std::marker::PhantomData),
        1 => ResponseCases::About(<AboutResponse as crate::value::Type>::__make_mut(raw, self.arena)),
        2 => ResponseCases::Codegen(<CodegenResponse as crate::value::Type>::__make_mut(raw, self.arena)),
        _ => unreachable!(),
      }
    }
  }

  pub fn clear(self) {
    unsafe { Response::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Response::__tdp_info())
  }

  pub fn about(self) -> crate::View<'proto, AboutResponse> {
    self.about_or().unwrap_or_default()
  }
  pub fn about_or(self) -> Option<crate::View<'proto, AboutResponse>> {
    if !unsafe { Response::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<AboutResponse as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.about } as *mut _ as *mut u8)) }
  }
  pub fn about_mut(self) -> crate::Mut<'proto, AboutResponse> {
    self.about_mut_or().into_mut()
  }
  pub fn about_mut_or(self) -> crate::value::OptMut<'proto, AboutResponse> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Response::__tdp_info().field(0),
      )
    }
  }

  pub fn codegen(self) -> crate::View<'proto, CodegenResponse> {
    self.codegen_or().unwrap_or_default()
  }
  pub fn codegen_or(self) -> Option<crate::View<'proto, CodegenResponse>> {
    if !unsafe { Response::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<CodegenResponse as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().union.codegen } as *mut _ as *mut u8)) }
  }
  pub fn codegen_mut(self) -> crate::Mut<'proto, CodegenResponse> {
    self.codegen_mut_or().into_mut()
  }
  pub fn codegen_mut_or(self) -> crate::value::OptMut<'proto, CodegenResponse> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Response::__tdp_info().field(1),
      )
    }
  }

}

impl Drop for Response {
  fn drop(&mut self) {
    unsafe { self.arena.destroy() }
  }
}

impl std::fmt::Debug for __priv_Response::View<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    fmt.write_str("pz.plugin.Response ")?;
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Response::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl std::fmt::Debug for Response {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.as_view(), fmt)
  }
}

impl crate::value::Type for Response {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_Response::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_Response::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
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
    let align = std::mem::align_of::<__priv_Response::Union>();
    if align < 4 { 4 } else { align }
  };

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{2 + 1}> =
    crate::__z::tdp::DescStorage::<{2 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = Response::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
            AboutResponse::__tdp_info,
            CodegenResponse::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 0,
        kind: crate::__z::tdp::DescKind::Choice,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: __priv_Response::UNION_OFFSET as u32,
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: __priv_Response::UNION_OFFSET as u32,
          desc: 1,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Response::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto Response>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Response> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Response::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut Response>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Response> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::Response> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.AboutRequest`
pub struct AboutRequest {
  ptr: crate::__z::ABox<__priv_AboutRequest::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_AboutRequest::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl AboutRequest {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_AboutRequest::Storage = __priv_AboutRequest::Storage {
      __hasbits: [0; 0],
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_AboutRequest::Storage as *mut __priv_AboutRequest::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_AboutRequest::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_AboutRequest::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for AboutRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for AboutRequest {
  type View<'proto> = __priv_AboutRequest::View<'proto>;
  type Mut<'proto> = __priv_AboutRequest::Mut<'proto>;
}

impl crate::value::Type for AboutRequest {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_AboutRequest::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_AboutRequest::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_AboutRequest::View<'proto> {
  pub fn as_view(&self) -> crate::View<AboutRequest> {
    __priv_AboutRequest::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_AboutRequest::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<AboutRequest> {
    __priv_AboutRequest::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, AboutRequest> {
    __priv_AboutRequest::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<AboutRequest> {
    __priv_AboutRequest::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { AboutRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_AboutRequest::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{0 + 1}> =
    crate::__z::tdp::DescStorage::<{0 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = AboutRequest::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 0,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_AboutRequest::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto AboutRequest>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::AboutRequest> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_AboutRequest::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut AboutRequest>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::AboutRequest> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::AboutRequest> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.AboutResponse`
pub struct AboutResponse {
  ptr: crate::__z::ABox<__priv_AboutResponse::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_AboutResponse::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl AboutResponse {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_AboutResponse::Storage = __priv_AboutResponse::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      version: (0 as *mut u8, 0),
      options: crate::__z::AVec::new(),
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_AboutResponse::Storage as *mut __priv_AboutResponse::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_AboutResponse::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_AboutResponse::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { AboutResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::View<'_, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { AboutResponse::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        AboutResponse::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn version(&self) -> crate::View<'_, crate::Str> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { AboutResponse::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().version };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn version_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.version_mut_or().into_mut()
  }
  pub fn version_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        AboutResponse::__tdp_info().field(1),
      )
    }
  }
  pub fn version_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.version_mut().set(value);
  }

  pub fn options(&self) -> crate::Slice<'_, AboutResponse_Option> {
    if !unsafe { AboutResponse::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().options };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(&self, idx: usize) -> crate::View<'_, AboutResponse_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(&mut self) -> crate::Repeated<'_, AboutResponse_Option> {
    unsafe {
      AboutResponse::__tdp_info().field(2).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().options } as *mut _ as *mut u8,
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_AboutResponse::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for AboutResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for AboutResponse {
  type View<'proto> = __priv_AboutResponse::View<'proto>;
  type Mut<'proto> = __priv_AboutResponse::Mut<'proto>;
}

impl crate::value::Type for AboutResponse {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_AboutResponse::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_AboutResponse::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_AboutResponse::View<'proto> {
  pub fn as_view(&self) -> crate::View<AboutResponse> {
    __priv_AboutResponse::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { AboutResponse::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn version(self) -> crate::View<'proto, crate::Str> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { AboutResponse::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().version };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn options(self) -> crate::Slice<'proto, AboutResponse_Option> {
    if !unsafe { AboutResponse::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().options };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(self, idx: usize) -> crate::View<'proto, AboutResponse_Option> {
    self.options().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_AboutResponse::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<AboutResponse> {
    __priv_AboutResponse::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, AboutResponse> {
    __priv_AboutResponse::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<AboutResponse> {
    __priv_AboutResponse::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { AboutResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, AboutResponse::__tdp_info())
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { AboutResponse::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        AboutResponse::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn version(self) -> crate::View<'proto, crate::Str> {
    self.version_or().unwrap_or_default()
  }
  pub fn version_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { AboutResponse::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().version };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn version_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.version_mut_or().into_mut()
  }
  pub fn version_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        AboutResponse::__tdp_info().field(1),
      )
    }
  }
  pub fn version_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.version_mut().set(value);
  }

  pub fn options(self) -> crate::Slice<'proto, AboutResponse_Option> {
    if !unsafe { AboutResponse::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().options };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(self, idx: usize) -> crate::View<'proto, AboutResponse_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(self) -> crate::Repeated<'proto, AboutResponse_Option> {
    unsafe {
      AboutResponse::__tdp_info().field(2).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().options } as *mut _ as *mut u8,
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_AboutResponse::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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
    pub(in super) options: crate::__z::AVec<*mut u8>,
  }

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{3 + 1}> =
    crate::__z::tdp::DescStorage::<{3 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = AboutResponse::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
            AboutResponse_Option::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = AboutResponse::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = AboutResponse::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().version as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage {
          number: 10,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = AboutResponse::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().options as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_AboutResponse::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto AboutResponse>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::AboutResponse> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_AboutResponse::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut AboutResponse>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::AboutResponse> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::AboutResponse> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.AboutResponse.Option`
pub struct AboutResponse_Option {
  ptr: crate::__z::ABox<__priv_AboutResponse_Option::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_AboutResponse_Option::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl AboutResponse_Option {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_AboutResponse_Option::Storage = __priv_AboutResponse_Option::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      help: (0 as *mut u8, 0),
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_AboutResponse_Option::Storage as *mut __priv_AboutResponse_Option::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_AboutResponse_Option::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_AboutResponse_Option::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { AboutResponse_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::View<'_, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { AboutResponse_Option::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        AboutResponse_Option::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn help(&self) -> crate::View<'_, crate::Str> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { AboutResponse_Option::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().help };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn help_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.help_mut_or().into_mut()
  }
  pub fn help_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        AboutResponse_Option::__tdp_info().field(1),
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_AboutResponse_Option::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for AboutResponse_Option {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for AboutResponse_Option {
  type View<'proto> = __priv_AboutResponse_Option::View<'proto>;
  type Mut<'proto> = __priv_AboutResponse_Option::Mut<'proto>;
}

impl crate::value::Type for AboutResponse_Option {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_AboutResponse_Option::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_AboutResponse_Option::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_AboutResponse_Option::View<'proto> {
  pub fn as_view(&self) -> crate::View<AboutResponse_Option> {
    __priv_AboutResponse_Option::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { AboutResponse_Option::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn help(self) -> crate::View<'proto, crate::Str> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { AboutResponse_Option::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().help };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_AboutResponse_Option::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<AboutResponse_Option> {
    __priv_AboutResponse_Option::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, AboutResponse_Option> {
    __priv_AboutResponse_Option::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<AboutResponse_Option> {
    __priv_AboutResponse_Option::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { AboutResponse_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, AboutResponse_Option::__tdp_info())
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { AboutResponse_Option::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        AboutResponse_Option::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn help(self) -> crate::View<'proto, crate::Str> {
    self.help_or().unwrap_or_default()
  }
  pub fn help_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { AboutResponse_Option::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().help };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn help_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.help_mut_or().into_mut()
  }
  pub fn help_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        AboutResponse_Option::__tdp_info().field(1),
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_AboutResponse_Option::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{2 + 1}> =
    crate::__z::tdp::DescStorage::<{2 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = AboutResponse_Option::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = AboutResponse_Option::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = AboutResponse_Option::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().help as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_AboutResponse_Option::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto AboutResponse_Option>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::AboutResponse_Option> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_AboutResponse_Option::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut AboutResponse_Option>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::AboutResponse_Option> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::AboutResponse_Option> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenRequest`
pub struct CodegenRequest {
  ptr: crate::__z::ABox<__priv_CodegenRequest::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_CodegenRequest::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl CodegenRequest {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenRequest::Storage = __priv_CodegenRequest::Storage {
      __hasbits: [0; 1],
      bundle: 0 as *mut u8,
      requested_indices: crate::__z::AVec::new(),
      options: crate::__z::AVec::new(),
      debug: false,
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_CodegenRequest::Storage as *mut __priv_CodegenRequest::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_CodegenRequest::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_CodegenRequest::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { CodegenRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn bundle(&self) -> crate::View<'_, Bundle> {
    self.bundle_or().unwrap_or_default()
  }
  pub fn bundle_or(&self) -> Option<crate::View<'_, Bundle>> {
    if !unsafe { CodegenRequest::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<Bundle as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().bundle } as *mut _ as *mut u8)) }
  }
  pub fn bundle_mut(&mut self) -> crate::Mut<'_, Bundle> {
    self.bundle_mut_or().into_mut()
  }
  pub fn bundle_mut_or(&mut self) -> crate::value::OptMut<'_, Bundle> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenRequest::__tdp_info().field(0),
      )
    }
  }

  pub fn requested_indices(&self) -> crate::Slice<'_, u32> {
    if !unsafe { CodegenRequest::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().requested_indices };
      crate::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn requested_indices_at(&self, idx: usize) -> crate::View<'_, u32> {
    self.requested_indices().at(idx)
  }
  pub fn requested_indices_mut(&mut self) -> crate::Repeated<'_, u32> {
    unsafe {
      CodegenRequest::__tdp_info().field(1).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().requested_indices } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn options(&self) -> crate::Slice<'_, CodegenRequest_Option> {
    if !unsafe { CodegenRequest::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().options };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(&self, idx: usize) -> crate::View<'_, CodegenRequest_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(&mut self) -> crate::Repeated<'_, CodegenRequest_Option> {
    unsafe {
      CodegenRequest::__tdp_info().field(2).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().options } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn debug(&self) -> crate::View<'_, bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(&self) -> Option<crate::View<'_, bool>> {
    if !unsafe { CodegenRequest::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().debug }) })
  }
  pub fn debug_mut(&mut self) -> crate::Mut<'_, bool> {
    self.debug_mut_or().into_mut()
  }
  pub fn debug_mut_or(&mut self) -> crate::value::OptMut<'_, bool> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenRequest::__tdp_info().field(3),
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_CodegenRequest::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for CodegenRequest {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for CodegenRequest {
  type View<'proto> = __priv_CodegenRequest::View<'proto>;
  type Mut<'proto> = __priv_CodegenRequest::Mut<'proto>;
}

impl crate::value::Type for CodegenRequest {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_CodegenRequest::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_CodegenRequest::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_CodegenRequest::View<'proto> {
  pub fn as_view(&self) -> crate::View<CodegenRequest> {
    __priv_CodegenRequest::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn bundle(self) -> crate::View<'proto, Bundle> {
    self.bundle_or().unwrap_or_default()
  }
  pub fn bundle_or(self) -> Option<crate::View<'proto, Bundle>> {
    if !unsafe { CodegenRequest::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<Bundle as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().bundle } as *mut _ as *mut u8)) }
  }

  pub fn requested_indices(self) -> crate::Slice<'proto, u32> {
    if !unsafe { CodegenRequest::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().requested_indices };
      crate::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn requested_indices_at(self, idx: usize) -> crate::View<'proto, u32> {
    self.requested_indices().at(idx)
  }

  pub fn options(self) -> crate::Slice<'proto, CodegenRequest_Option> {
    if !unsafe { CodegenRequest::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().options };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(self, idx: usize) -> crate::View<'proto, CodegenRequest_Option> {
    self.options().at(idx)
  }

  pub fn debug(self) -> crate::View<'proto, bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(self) -> Option<crate::View<'proto, bool>> {
    if !unsafe { CodegenRequest::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().debug }) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_CodegenRequest::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<CodegenRequest> {
    __priv_CodegenRequest::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, CodegenRequest> {
    __priv_CodegenRequest::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<CodegenRequest> {
    __priv_CodegenRequest::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { CodegenRequest::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, CodegenRequest::__tdp_info())
  }

  pub fn bundle(self) -> crate::View<'proto, Bundle> {
    self.bundle_or().unwrap_or_default()
  }
  pub fn bundle_or(self) -> Option<crate::View<'proto, Bundle>> {
    if !unsafe { CodegenRequest::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<Bundle as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().bundle } as *mut _ as *mut u8)) }
  }
  pub fn bundle_mut(self) -> crate::Mut<'proto, Bundle> {
    self.bundle_mut_or().into_mut()
  }
  pub fn bundle_mut_or(self) -> crate::value::OptMut<'proto, Bundle> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenRequest::__tdp_info().field(0),
      )
    }
  }

  pub fn requested_indices(self) -> crate::Slice<'proto, u32> {
    if !unsafe { CodegenRequest::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().requested_indices };
      crate::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn requested_indices_at(self, idx: usize) -> crate::View<'proto, u32> {
    self.requested_indices().at(idx)
  }
  pub fn requested_indices_mut(self) -> crate::Repeated<'proto, u32> {
    unsafe {
      CodegenRequest::__tdp_info().field(1).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().requested_indices } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn options(self) -> crate::Slice<'proto, CodegenRequest_Option> {
    if !unsafe { CodegenRequest::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().options };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn options_at(self, idx: usize) -> crate::View<'proto, CodegenRequest_Option> {
    self.options().at(idx)
  }
  pub fn options_mut(self) -> crate::Repeated<'proto, CodegenRequest_Option> {
    unsafe {
      CodegenRequest::__tdp_info().field(2).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().options } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn debug(self) -> crate::View<'proto, bool> {
    self.debug_or().unwrap_or_default()
  }
  pub fn debug_or(self) -> Option<crate::View<'proto, bool>> {
    if !unsafe { CodegenRequest::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().debug }) })
  }
  pub fn debug_mut(self) -> crate::Mut<'proto, bool> {
    self.debug_mut_or().into_mut()
  }
  pub fn debug_mut_or(self) -> crate::value::OptMut<'proto, bool> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenRequest::__tdp_info().field(3),
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_CodegenRequest::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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
    pub (in super) requested_indices: crate::__z::AVec<u32>,
    pub(in super) options: crate::__z::AVec<*mut u8>,
    pub(in super) debug: bool,
  }

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{4 + 1}> =
    crate::__z::tdp::DescStorage::<{4 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = CodegenRequest::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
            Bundle::__tdp_info,
            CodegenRequest_Option::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = CodegenRequest::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().bundle as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = CodegenRequest::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().requested_indices as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage {
          number: 3,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = CodegenRequest::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().options as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 1,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage {
          number: 4,
          flags:
            crate::__z::tdp::Kind::Bool.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = CodegenRequest::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().debug as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_CodegenRequest::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto CodegenRequest>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::CodegenRequest> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_CodegenRequest::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut CodegenRequest>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::CodegenRequest> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::CodegenRequest> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenRequest.Option`
pub struct CodegenRequest_Option {
  ptr: crate::__z::ABox<__priv_CodegenRequest_Option::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_CodegenRequest_Option::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl CodegenRequest_Option {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenRequest_Option::Storage = __priv_CodegenRequest_Option::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      value: (0 as *mut u8, 0),
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_CodegenRequest_Option::Storage as *mut __priv_CodegenRequest_Option::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_CodegenRequest_Option::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_CodegenRequest_Option::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { CodegenRequest_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::View<'_, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { CodegenRequest_Option::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenRequest_Option::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn value(&self) -> crate::View<'_, crate::Str> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { CodegenRequest_Option::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().value };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn value_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.value_mut_or().into_mut()
  }
  pub fn value_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenRequest_Option::__tdp_info().field(1),
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_CodegenRequest_Option::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for CodegenRequest_Option {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for CodegenRequest_Option {
  type View<'proto> = __priv_CodegenRequest_Option::View<'proto>;
  type Mut<'proto> = __priv_CodegenRequest_Option::Mut<'proto>;
}

impl crate::value::Type for CodegenRequest_Option {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_CodegenRequest_Option::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_CodegenRequest_Option::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_CodegenRequest_Option::View<'proto> {
  pub fn as_view(&self) -> crate::View<CodegenRequest_Option> {
    __priv_CodegenRequest_Option::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { CodegenRequest_Option::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn value(self) -> crate::View<'proto, crate::Str> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { CodegenRequest_Option::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().value };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_CodegenRequest_Option::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<CodegenRequest_Option> {
    __priv_CodegenRequest_Option::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, CodegenRequest_Option> {
    __priv_CodegenRequest_Option::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<CodegenRequest_Option> {
    __priv_CodegenRequest_Option::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { CodegenRequest_Option::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, CodegenRequest_Option::__tdp_info())
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { CodegenRequest_Option::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenRequest_Option::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn value(self) -> crate::View<'proto, crate::Str> {
    self.value_or().unwrap_or_default()
  }
  pub fn value_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { CodegenRequest_Option::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().value };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn value_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.value_mut_or().into_mut()
  }
  pub fn value_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenRequest_Option::__tdp_info().field(1),
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_CodegenRequest_Option::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{2 + 1}> =
    crate::__z::tdp::DescStorage::<{2 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = CodegenRequest_Option::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = CodegenRequest_Option::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = CodegenRequest_Option::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().value as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_CodegenRequest_Option::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto CodegenRequest_Option>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::CodegenRequest_Option> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_CodegenRequest_Option::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut CodegenRequest_Option>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::CodegenRequest_Option> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::CodegenRequest_Option> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenResponse`
pub struct CodegenResponse {
  ptr: crate::__z::ABox<__priv_CodegenResponse::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_CodegenResponse::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl CodegenResponse {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenResponse::Storage = __priv_CodegenResponse::Storage {
      __hasbits: [0; 0],
      files: crate::__z::AVec::new(),
      report: crate::__z::AVec::new(),
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_CodegenResponse::Storage as *mut __priv_CodegenResponse::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_CodegenResponse::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_CodegenResponse::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { CodegenResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn files(&self) -> crate::Slice<'_, CodegenResponse_File> {
    if !unsafe { CodegenResponse::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().files };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn files_at(&self, idx: usize) -> crate::View<'_, CodegenResponse_File> {
    self.files().at(idx)
  }
  pub fn files_mut(&mut self) -> crate::Repeated<'_, CodegenResponse_File> {
    unsafe {
      CodegenResponse::__tdp_info().field(0).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().files } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn report(&self) -> crate::Slice<'_, Diagnostic> {
    if !unsafe { CodegenResponse::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().report };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn report_at(&self, idx: usize) -> crate::View<'_, Diagnostic> {
    self.report().at(idx)
  }
  pub fn report_mut(&mut self) -> crate::Repeated<'_, Diagnostic> {
    unsafe {
      CodegenResponse::__tdp_info().field(1).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().report } as *mut _ as *mut u8,
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_CodegenResponse::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for CodegenResponse {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for CodegenResponse {
  type View<'proto> = __priv_CodegenResponse::View<'proto>;
  type Mut<'proto> = __priv_CodegenResponse::Mut<'proto>;
}

impl crate::value::Type for CodegenResponse {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_CodegenResponse::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_CodegenResponse::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_CodegenResponse::View<'proto> {
  pub fn as_view(&self) -> crate::View<CodegenResponse> {
    __priv_CodegenResponse::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn files(self) -> crate::Slice<'proto, CodegenResponse_File> {
    if !unsafe { CodegenResponse::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().files };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn files_at(self, idx: usize) -> crate::View<'proto, CodegenResponse_File> {
    self.files().at(idx)
  }

  pub fn report(self) -> crate::Slice<'proto, Diagnostic> {
    if !unsafe { CodegenResponse::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().report };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn report_at(self, idx: usize) -> crate::View<'proto, Diagnostic> {
    self.report().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_CodegenResponse::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<CodegenResponse> {
    __priv_CodegenResponse::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, CodegenResponse> {
    __priv_CodegenResponse::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<CodegenResponse> {
    __priv_CodegenResponse::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { CodegenResponse::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, CodegenResponse::__tdp_info())
  }

  pub fn files(self) -> crate::Slice<'proto, CodegenResponse_File> {
    if !unsafe { CodegenResponse::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().files };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn files_at(self, idx: usize) -> crate::View<'proto, CodegenResponse_File> {
    self.files().at(idx)
  }
  pub fn files_mut(self) -> crate::Repeated<'proto, CodegenResponse_File> {
    unsafe {
      CodegenResponse::__tdp_info().field(0).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().files } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn report(self) -> crate::Slice<'proto, Diagnostic> {
    if !unsafe { CodegenResponse::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().report };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn report_at(self, idx: usize) -> crate::View<'proto, Diagnostic> {
    self.report().at(idx)
  }
  pub fn report_mut(self) -> crate::Repeated<'proto, Diagnostic> {
    unsafe {
      CodegenResponse::__tdp_info().field(1).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().report } as *mut _ as *mut u8,
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_CodegenResponse::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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
    pub(in super) files: crate::__z::AVec<*mut u8>,
    pub(in super) report: crate::__z::AVec<*mut u8>,
  }

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{2 + 1}> =
    crate::__z::tdp::DescStorage::<{2 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = CodegenResponse::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
            CodegenResponse_File::__tdp_info,
            Diagnostic::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 0,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = CodegenResponse::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().files as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = CodegenResponse::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().report as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 1,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_CodegenResponse::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto CodegenResponse>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::CodegenResponse> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_CodegenResponse::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut CodegenResponse>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::CodegenResponse> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::CodegenResponse> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.CodegenResponse.File`
pub struct CodegenResponse_File {
  ptr: crate::__z::ABox<__priv_CodegenResponse_File::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_CodegenResponse_File::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl CodegenResponse_File {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_CodegenResponse_File::Storage = __priv_CodegenResponse_File::Storage {
      __hasbits: [0; 1],
      path: (0 as *mut u8, 0),
      content: (0 as *mut u8, 0),
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_CodegenResponse_File::Storage as *mut __priv_CodegenResponse_File::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_CodegenResponse_File::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_CodegenResponse_File::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { CodegenResponse_File::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn path(&self) -> crate::View<'_, crate::Str> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { CodegenResponse_File::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().path };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn path_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.path_mut_or().into_mut()
  }
  pub fn path_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenResponse_File::__tdp_info().field(0),
      )
    }
  }
  pub fn path_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.path_mut().set(value);
  }

  pub fn content(&self) -> crate::View<'_, crate::Str> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { CodegenResponse_File::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().content };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn content_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.content_mut_or().into_mut()
  }
  pub fn content_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenResponse_File::__tdp_info().field(1),
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_CodegenResponse_File::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for CodegenResponse_File {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for CodegenResponse_File {
  type View<'proto> = __priv_CodegenResponse_File::View<'proto>;
  type Mut<'proto> = __priv_CodegenResponse_File::Mut<'proto>;
}

impl crate::value::Type for CodegenResponse_File {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_CodegenResponse_File::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_CodegenResponse_File::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_CodegenResponse_File::View<'proto> {
  pub fn as_view(&self) -> crate::View<CodegenResponse_File> {
    __priv_CodegenResponse_File::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn path(self) -> crate::View<'proto, crate::Str> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { CodegenResponse_File::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().path };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn content(self) -> crate::View<'proto, crate::Str> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { CodegenResponse_File::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().content };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_CodegenResponse_File::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<CodegenResponse_File> {
    __priv_CodegenResponse_File::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, CodegenResponse_File> {
    __priv_CodegenResponse_File::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<CodegenResponse_File> {
    __priv_CodegenResponse_File::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { CodegenResponse_File::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, CodegenResponse_File::__tdp_info())
  }

  pub fn path(self) -> crate::View<'proto, crate::Str> {
    self.path_or().unwrap_or_default()
  }
  pub fn path_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { CodegenResponse_File::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().path };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn path_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.path_mut_or().into_mut()
  }
  pub fn path_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenResponse_File::__tdp_info().field(0),
      )
    }
  }
  pub fn path_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.path_mut().set(value);
  }

  pub fn content(self) -> crate::View<'proto, crate::Str> {
    self.content_or().unwrap_or_default()
  }
  pub fn content_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { CodegenResponse_File::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().content };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn content_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.content_mut_or().into_mut()
  }
  pub fn content_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        CodegenResponse_File::__tdp_info().field(1),
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_CodegenResponse_File::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{2 + 1}> =
    crate::__z::tdp::DescStorage::<{2 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = CodegenResponse_File::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = CodegenResponse_File::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().path as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = CodegenResponse_File::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().content as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_CodegenResponse_File::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto CodegenResponse_File>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::CodegenResponse_File> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_CodegenResponse_File::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut CodegenResponse_File>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::CodegenResponse_File> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::CodegenResponse_File> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.plugin.Diagnostic`
pub struct Diagnostic {
  ptr: crate::__z::ABox<__priv_Diagnostic::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_Diagnostic::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl Diagnostic {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_Diagnostic::Storage = __priv_Diagnostic::Storage {
      __hasbits: [0; 1],
      kind: Diagnostic_Kind::new().0 as u32,
      msg: (0 as *mut u8, 0),
      snippets: crate::__z::AVec::new(),
      notes: crate::__z::AVec::new(),
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_Diagnostic::Storage as *mut __priv_Diagnostic::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Diagnostic::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Diagnostic::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Diagnostic::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn kind(&self) -> crate::View<'_, Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(&self) -> Option<crate::View<'_, Diagnostic_Kind>> {
    if !unsafe { Diagnostic::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, Diagnostic_Kind>(*unsafe { &self.ptr.as_ref().kind }) })
  }
  pub fn kind_mut(&mut self) -> crate::Mut<'_, Diagnostic_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(&mut self) -> crate::value::OptMut<'_, Diagnostic_Kind> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Diagnostic::__tdp_info().field(0),
      )
    }
  }
  pub fn kind_set(&mut self, value: Diagnostic_Kind) {
    self.kind_mut().set(value);
  }

  pub fn msg(&self) -> crate::View<'_, crate::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { Diagnostic::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().msg };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn msg_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.msg_mut_or().into_mut()
  }
  pub fn msg_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Diagnostic::__tdp_info().field(1),
      )
    }
  }
  pub fn msg_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.msg_mut().set(value);
  }

  pub fn snippets(&self) -> crate::Slice<'_, Diagnostic_Snippet> {
    if !unsafe { Diagnostic::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().snippets };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn snippets_at(&self, idx: usize) -> crate::View<'_, Diagnostic_Snippet> {
    self.snippets().at(idx)
  }
  pub fn snippets_mut(&mut self) -> crate::Repeated<'_, Diagnostic_Snippet> {
    unsafe {
      Diagnostic::__tdp_info().field(2).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().snippets } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn notes(&self) -> crate::Slice<'_, crate::Str> {
    if !unsafe { Diagnostic::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().notes };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn notes_at(&self, idx: usize) -> crate::View<'_, crate::Str> {
    self.notes().at(idx)
  }
  pub fn notes_mut(&mut self) -> crate::Repeated<'_, crate::Str> {
    unsafe {
      Diagnostic::__tdp_info().field(3).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().notes } as *mut _ as *mut u8,
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_Diagnostic::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for Diagnostic {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for Diagnostic {
  type View<'proto> = __priv_Diagnostic::View<'proto>;
  type Mut<'proto> = __priv_Diagnostic::Mut<'proto>;
}

impl crate::value::Type for Diagnostic {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_Diagnostic::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_Diagnostic::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Diagnostic::View<'proto> {
  pub fn as_view(&self) -> crate::View<Diagnostic> {
    __priv_Diagnostic::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn kind(self) -> crate::View<'proto, Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> Option<crate::View<'proto, Diagnostic_Kind>> {
    if !unsafe { Diagnostic::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, Diagnostic_Kind>(*unsafe { &self.ptr.as_ref().kind }) })
  }

  pub fn msg(self) -> crate::View<'proto, crate::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Diagnostic::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().msg };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn snippets(self) -> crate::Slice<'proto, Diagnostic_Snippet> {
    if !unsafe { Diagnostic::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().snippets };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn snippets_at(self, idx: usize) -> crate::View<'proto, Diagnostic_Snippet> {
    self.snippets().at(idx)
  }

  pub fn notes(self) -> crate::Slice<'proto, crate::Str> {
    if !unsafe { Diagnostic::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().notes };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn notes_at(self, idx: usize) -> crate::View<'proto, crate::Str> {
    self.notes().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_Diagnostic::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<Diagnostic> {
    __priv_Diagnostic::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, Diagnostic> {
    __priv_Diagnostic::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Diagnostic> {
    __priv_Diagnostic::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { Diagnostic::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Diagnostic::__tdp_info())
  }

  pub fn kind(self) -> crate::View<'proto, Diagnostic_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> Option<crate::View<'proto, Diagnostic_Kind>> {
    if !unsafe { Diagnostic::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, Diagnostic_Kind>(*unsafe { &self.ptr.as_ref().kind }) })
  }
  pub fn kind_mut(self) -> crate::Mut<'proto, Diagnostic_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(self) -> crate::value::OptMut<'proto, Diagnostic_Kind> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Diagnostic::__tdp_info().field(0),
      )
    }
  }
  pub fn kind_set(self, value: Diagnostic_Kind) {
    self.kind_mut().set(value);
  }

  pub fn msg(self) -> crate::View<'proto, crate::Str> {
    self.msg_or().unwrap_or_default()
  }
  pub fn msg_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Diagnostic::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().msg };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn msg_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.msg_mut_or().into_mut()
  }
  pub fn msg_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Diagnostic::__tdp_info().field(1),
      )
    }
  }
  pub fn msg_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.msg_mut().set(value);
  }

  pub fn snippets(self) -> crate::Slice<'proto, Diagnostic_Snippet> {
    if !unsafe { Diagnostic::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().snippets };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn snippets_at(self, idx: usize) -> crate::View<'proto, Diagnostic_Snippet> {
    self.snippets().at(idx)
  }
  pub fn snippets_mut(self) -> crate::Repeated<'proto, Diagnostic_Snippet> {
    unsafe {
      Diagnostic::__tdp_info().field(2).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().snippets } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn notes(self) -> crate::Slice<'proto, crate::Str> {
    if !unsafe { Diagnostic::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().notes };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn notes_at(self, idx: usize) -> crate::View<'proto, crate::Str> {
    self.notes().at(idx)
  }
  pub fn notes_mut(self) -> crate::Repeated<'proto, crate::Str> {
    unsafe {
      Diagnostic::__tdp_info().field(3).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().notes } as *mut _ as *mut u8,
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Diagnostic::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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
    pub(in super) snippets: crate::__z::AVec<*mut u8>,
    pub(crate) notes: crate::__z::AVec<(*mut u8, usize)>,
  }

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{4 + 1}> =
    crate::__z::tdp::DescStorage::<{4 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = Diagnostic::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
            Diagnostic_Snippet::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Diagnostic::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().kind as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Diagnostic::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().msg as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage {
          number: 3,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Diagnostic::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().snippets as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
        },
        crate::__z::tdp::FieldStorage {
          number: 4,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Diagnostic::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().notes as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Diagnostic::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto Diagnostic>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Diagnostic> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Diagnostic::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut Diagnostic>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Diagnostic> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::Diagnostic> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
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

impl crate::ptr::Proxied for Diagnostic_Kind {
  type View<'a> = Self;
  type Mut<'a> = crate::ptr::ScalarMut<'a, Self>;
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
  ptr: crate::__z::ABox<__priv_Diagnostic_Snippet::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_Diagnostic_Snippet::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl Diagnostic_Snippet {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_Diagnostic_Snippet::Storage = __priv_Diagnostic_Snippet::Storage {
      __hasbits: [0; 1],
      span: 0,
      message: (0 as *mut u8, 0),
      is_remark: false,
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_Diagnostic_Snippet::Storage as *mut __priv_Diagnostic_Snippet::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Diagnostic_Snippet::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Diagnostic_Snippet::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Diagnostic_Snippet::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn span(&self) -> crate::View<'_, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> Option<crate::View<'_, u32>> {
    if !unsafe { Diagnostic_Snippet::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().span }) })
  }
  pub fn span_mut(&mut self) -> crate::Mut<'_, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(&mut self) -> crate::value::OptMut<'_, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Diagnostic_Snippet::__tdp_info().field(0),
      )
    }
  }
  pub fn span_set(&mut self, value: u32) {
    self.span_mut().set(value);
  }

  pub fn message(&self) -> crate::View<'_, crate::Str> {
    self.message_or().unwrap_or_default()
  }
  pub fn message_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { Diagnostic_Snippet::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().message };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn message_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.message_mut_or().into_mut()
  }
  pub fn message_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Diagnostic_Snippet::__tdp_info().field(1),
      )
    }
  }
  pub fn message_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.message_mut().set(value);
  }

  pub fn is_remark(&self) -> crate::View<'_, bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(&self) -> Option<crate::View<'_, bool>> {
    if !unsafe { Diagnostic_Snippet::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().is_remark }) })
  }
  pub fn is_remark_mut(&mut self) -> crate::Mut<'_, bool> {
    self.is_remark_mut_or().into_mut()
  }
  pub fn is_remark_mut_or(&mut self) -> crate::value::OptMut<'_, bool> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Diagnostic_Snippet::__tdp_info().field(2),
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_Diagnostic_Snippet::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for Diagnostic_Snippet {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for Diagnostic_Snippet {
  type View<'proto> = __priv_Diagnostic_Snippet::View<'proto>;
  type Mut<'proto> = __priv_Diagnostic_Snippet::Mut<'proto>;
}

impl crate::value::Type for Diagnostic_Snippet {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_Diagnostic_Snippet::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_Diagnostic_Snippet::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Diagnostic_Snippet::View<'proto> {
  pub fn as_view(&self) -> crate::View<Diagnostic_Snippet> {
    __priv_Diagnostic_Snippet::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn span(self) -> crate::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Diagnostic_Snippet::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().span }) })
  }

  pub fn message(self) -> crate::View<'proto, crate::Str> {
    self.message_or().unwrap_or_default()
  }
  pub fn message_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Diagnostic_Snippet::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().message };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn is_remark(self) -> crate::View<'proto, bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(self) -> Option<crate::View<'proto, bool>> {
    if !unsafe { Diagnostic_Snippet::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().is_remark }) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
    let mut count = 0;
    debug.start_block()?;
    if let Some(value) = self.span_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("span")?;
      debug.write_debug(value);
      count += 1;
    }
    if let Some(value) = self.message_or() {
      if count != 0 { debug.comma(false)?; }
      debug.field("message")?;
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

impl<'proto> __priv_Diagnostic_Snippet::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<Diagnostic_Snippet> {
    __priv_Diagnostic_Snippet::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, Diagnostic_Snippet> {
    __priv_Diagnostic_Snippet::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Diagnostic_Snippet> {
    __priv_Diagnostic_Snippet::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { Diagnostic_Snippet::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Diagnostic_Snippet::__tdp_info())
  }

  pub fn span(self) -> crate::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Diagnostic_Snippet::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().span }) })
  }
  pub fn span_mut(self) -> crate::Mut<'proto, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(self) -> crate::value::OptMut<'proto, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Diagnostic_Snippet::__tdp_info().field(0),
      )
    }
  }
  pub fn span_set(self, value: u32) {
    self.span_mut().set(value);
  }

  pub fn message(self) -> crate::View<'proto, crate::Str> {
    self.message_or().unwrap_or_default()
  }
  pub fn message_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Diagnostic_Snippet::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().message };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn message_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.message_mut_or().into_mut()
  }
  pub fn message_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Diagnostic_Snippet::__tdp_info().field(1),
      )
    }
  }
  pub fn message_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.message_mut().set(value);
  }

  pub fn is_remark(self) -> crate::View<'proto, bool> {
    self.is_remark_or().unwrap_or_default()
  }
  pub fn is_remark_or(self) -> Option<crate::View<'proto, bool>> {
    if !unsafe { Diagnostic_Snippet::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().is_remark }) })
  }
  pub fn is_remark_mut(self) -> crate::Mut<'proto, bool> {
    self.is_remark_mut_or().into_mut()
  }
  pub fn is_remark_mut_or(self) -> crate::value::OptMut<'proto, bool> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Diagnostic_Snippet::__tdp_info().field(2),
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Diagnostic_Snippet::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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
    pub(in super) message: (*mut u8, usize),
    pub(in super) is_remark: bool,
  }

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{3 + 1}> =
    crate::__z::tdp::DescStorage::<{3 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = Diagnostic_Snippet::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Diagnostic_Snippet::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().span as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Diagnostic_Snippet::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().message as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage {
          number: 3,
          flags:
            crate::__z::tdp::Kind::Bool.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Diagnostic_Snippet::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().is_remark as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Diagnostic_Snippet::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto Diagnostic_Snippet>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Diagnostic_Snippet> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Diagnostic_Snippet::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut Diagnostic_Snippet>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Diagnostic_Snippet> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::Diagnostic_Snippet> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.Bundle`
pub struct Bundle {
  ptr: crate::__z::ABox<__priv_Bundle::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_Bundle::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl Bundle {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_Bundle::Storage = __priv_Bundle::Storage {
      __hasbits: [0; 0],
      types: crate::__z::AVec::new(),
      packages: crate::__z::AVec::new(),
      foreign_types: crate::__z::AVec::new(),
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_Bundle::Storage as *mut __priv_Bundle::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Bundle::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Bundle::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Bundle::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn types(&self) -> crate::Slice<'_, Type> {
    if !unsafe { Bundle::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().types };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn types_at(&self, idx: usize) -> crate::View<'_, Type> {
    self.types().at(idx)
  }
  pub fn types_mut(&mut self) -> crate::Repeated<'_, Type> {
    unsafe {
      Bundle::__tdp_info().field(0).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().types } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn packages(&self) -> crate::Slice<'_, crate::Str> {
    if !unsafe { Bundle::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().packages };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn packages_at(&self, idx: usize) -> crate::View<'_, crate::Str> {
    self.packages().at(idx)
  }
  pub fn packages_mut(&mut self) -> crate::Repeated<'_, crate::Str> {
    unsafe {
      Bundle::__tdp_info().field(1).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().packages } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn foreign_types(&self) -> crate::Slice<'_, Bundle_ForeignType> {
    if !unsafe { Bundle::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().foreign_types };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn foreign_types_at(&self, idx: usize) -> crate::View<'_, Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }
  pub fn foreign_types_mut(&mut self) -> crate::Repeated<'_, Bundle_ForeignType> {
    unsafe {
      Bundle::__tdp_info().field(2).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().foreign_types } as *mut _ as *mut u8,
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_Bundle::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for Bundle {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for Bundle {
  type View<'proto> = __priv_Bundle::View<'proto>;
  type Mut<'proto> = __priv_Bundle::Mut<'proto>;
}

impl crate::value::Type for Bundle {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_Bundle::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_Bundle::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Bundle::View<'proto> {
  pub fn as_view(&self) -> crate::View<Bundle> {
    __priv_Bundle::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn types(self) -> crate::Slice<'proto, Type> {
    if !unsafe { Bundle::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().types };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn types_at(self, idx: usize) -> crate::View<'proto, Type> {
    self.types().at(idx)
  }

  pub fn packages(self) -> crate::Slice<'proto, crate::Str> {
    if !unsafe { Bundle::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().packages };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn packages_at(self, idx: usize) -> crate::View<'proto, crate::Str> {
    self.packages().at(idx)
  }

  pub fn foreign_types(self) -> crate::Slice<'proto, Bundle_ForeignType> {
    if !unsafe { Bundle::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().foreign_types };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn foreign_types_at(self, idx: usize) -> crate::View<'proto, Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_Bundle::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<Bundle> {
    __priv_Bundle::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, Bundle> {
    __priv_Bundle::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Bundle> {
    __priv_Bundle::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { Bundle::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Bundle::__tdp_info())
  }

  pub fn types(self) -> crate::Slice<'proto, Type> {
    if !unsafe { Bundle::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().types };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn types_at(self, idx: usize) -> crate::View<'proto, Type> {
    self.types().at(idx)
  }
  pub fn types_mut(self) -> crate::Repeated<'proto, Type> {
    unsafe {
      Bundle::__tdp_info().field(0).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().types } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn packages(self) -> crate::Slice<'proto, crate::Str> {
    if !unsafe { Bundle::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().packages };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn packages_at(self, idx: usize) -> crate::View<'proto, crate::Str> {
    self.packages().at(idx)
  }
  pub fn packages_mut(self) -> crate::Repeated<'proto, crate::Str> {
    unsafe {
      Bundle::__tdp_info().field(1).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().packages } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn foreign_types(self) -> crate::Slice<'proto, Bundle_ForeignType> {
    if !unsafe { Bundle::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().foreign_types };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn foreign_types_at(self, idx: usize) -> crate::View<'proto, Bundle_ForeignType> {
    self.foreign_types().at(idx)
  }
  pub fn foreign_types_mut(self) -> crate::Repeated<'proto, Bundle_ForeignType> {
    unsafe {
      Bundle::__tdp_info().field(2).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().foreign_types } as *mut _ as *mut u8,
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Bundle::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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
    pub(in super) types: crate::__z::AVec<*mut u8>,
    pub(crate) packages: crate::__z::AVec<(*mut u8, usize)>,
    pub(in super) foreign_types: crate::__z::AVec<*mut u8>,
  }

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{3 + 1}> =
    crate::__z::tdp::DescStorage::<{3 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = Bundle::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
            Bundle_ForeignType::__tdp_info,
            Type::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 0,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Bundle::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().types as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 1,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Bundle::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().packages as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 3,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Bundle::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().foreign_types as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Bundle::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto Bundle>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Bundle> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Bundle::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut Bundle>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Bundle> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::Bundle> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.Bundle.ForeignType`
pub struct Bundle_ForeignType {
  ptr: crate::__z::ABox<__priv_Bundle_ForeignType::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_Bundle_ForeignType::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl Bundle_ForeignType {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_Bundle_ForeignType::Storage = __priv_Bundle_ForeignType::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      package: 0,
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_Bundle_ForeignType::Storage as *mut __priv_Bundle_ForeignType::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Bundle_ForeignType::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Bundle_ForeignType::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Bundle_ForeignType::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::View<'_, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { Bundle_ForeignType::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Bundle_ForeignType::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(&self) -> crate::View<'_, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(&self) -> Option<crate::View<'_, u32>> {
    if !unsafe { Bundle_ForeignType::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().package }) })
  }
  pub fn package_mut(&mut self) -> crate::Mut<'_, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(&mut self) -> crate::value::OptMut<'_, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Bundle_ForeignType::__tdp_info().field(1),
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_Bundle_ForeignType::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for Bundle_ForeignType {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for Bundle_ForeignType {
  type View<'proto> = __priv_Bundle_ForeignType::View<'proto>;
  type Mut<'proto> = __priv_Bundle_ForeignType::Mut<'proto>;
}

impl crate::value::Type for Bundle_ForeignType {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_Bundle_ForeignType::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_Bundle_ForeignType::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Bundle_ForeignType::View<'proto> {
  pub fn as_view(&self) -> crate::View<Bundle_ForeignType> {
    __priv_Bundle_ForeignType::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Bundle_ForeignType::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn package(self) -> crate::View<'proto, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Bundle_ForeignType::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().package }) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_Bundle_ForeignType::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<Bundle_ForeignType> {
    __priv_Bundle_ForeignType::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, Bundle_ForeignType> {
    __priv_Bundle_ForeignType::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Bundle_ForeignType> {
    __priv_Bundle_ForeignType::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { Bundle_ForeignType::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Bundle_ForeignType::__tdp_info())
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Bundle_ForeignType::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Bundle_ForeignType::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(self) -> crate::View<'proto, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Bundle_ForeignType::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().package }) })
  }
  pub fn package_mut(self) -> crate::Mut<'proto, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(self) -> crate::value::OptMut<'proto, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Bundle_ForeignType::__tdp_info().field(1),
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Bundle_ForeignType::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{2 + 1}> =
    crate::__z::tdp::DescStorage::<{2 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = Bundle_ForeignType::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Bundle_ForeignType::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Bundle_ForeignType::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().package as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Bundle_ForeignType::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto Bundle_ForeignType>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Bundle_ForeignType> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Bundle_ForeignType::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut Bundle_ForeignType>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Bundle_ForeignType> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::Bundle_ForeignType> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.Type`
pub struct Type {
  ptr: crate::__z::ABox<__priv_Type::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_Type::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl Type {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_Type::Storage = __priv_Type::Storage {
      __hasbits: [0; 1],
      name: (0 as *mut u8, 0),
      package: 0,
      kind: Type_Kind::new().0 as u32,
      declared_in: 0,
      fields: crate::__z::AVec::new(),
      nesteds: crate::__z::AVec::new(),
      attrs: 0 as *mut u8,
      span: 0,
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_Type::Storage as *mut __priv_Type::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Type::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Type::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Type::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::View<'_, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { Type::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(&self) -> crate::View<'_, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(&self) -> Option<crate::View<'_, u32>> {
    if !unsafe { Type::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().package }) })
  }
  pub fn package_mut(&mut self) -> crate::Mut<'_, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(&mut self) -> crate::value::OptMut<'_, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(1),
      )
    }
  }
  pub fn package_set(&mut self, value: u32) {
    self.package_mut().set(value);
  }

  pub fn kind(&self) -> crate::View<'_, Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(&self) -> Option<crate::View<'_, Type_Kind>> {
    if !unsafe { Type::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, Type_Kind>(*unsafe { &self.ptr.as_ref().kind }) })
  }
  pub fn kind_mut(&mut self) -> crate::Mut<'_, Type_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(&mut self) -> crate::value::OptMut<'_, Type_Kind> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(2),
      )
    }
  }
  pub fn kind_set(&mut self, value: Type_Kind) {
    self.kind_mut().set(value);
  }

  pub fn declared_in(&self) -> crate::View<'_, u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(&self) -> Option<crate::View<'_, u32>> {
    if !unsafe { Type::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().declared_in }) })
  }
  pub fn declared_in_mut(&mut self) -> crate::Mut<'_, u32> {
    self.declared_in_mut_or().into_mut()
  }
  pub fn declared_in_mut_or(&mut self) -> crate::value::OptMut<'_, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(3),
      )
    }
  }
  pub fn declared_in_set(&mut self, value: u32) {
    self.declared_in_mut().set(value);
  }

  pub fn fields(&self) -> crate::Slice<'_, Field> {
    if !unsafe { Type::__tdp_info().field(4).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().fields };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn fields_at(&self, idx: usize) -> crate::View<'_, Field> {
    self.fields().at(idx)
  }
  pub fn fields_mut(&mut self) -> crate::Repeated<'_, Field> {
    unsafe {
      Type::__tdp_info().field(4).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().fields } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn nesteds(&self) -> crate::Slice<'_, u32> {
    if !unsafe { Type::__tdp_info().field(5).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().nesteds };
      crate::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn nesteds_at(&self, idx: usize) -> crate::View<'_, u32> {
    self.nesteds().at(idx)
  }
  pub fn nesteds_mut(&mut self) -> crate::Repeated<'_, u32> {
    unsafe {
      Type::__tdp_info().field(5).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().nesteds } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn attrs(&self) -> crate::View<'_, Type_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(&self) -> Option<crate::View<'_, Type_Attrs>> {
    if !unsafe { Type::__tdp_info().field(6).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<Type_Attrs as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().attrs } as *mut _ as *mut u8)) }
  }
  pub fn attrs_mut(&mut self) -> crate::Mut<'_, Type_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(&mut self) -> crate::value::OptMut<'_, Type_Attrs> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(6),
      )
    }
  }

  pub fn span(&self) -> crate::View<'_, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> Option<crate::View<'_, u32>> {
    if !unsafe { Type::__tdp_info().field(7).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().span }) })
  }
  pub fn span_mut(&mut self) -> crate::Mut<'_, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(&mut self) -> crate::value::OptMut<'_, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(7),
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_Type::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for Type {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for Type {
  type View<'proto> = __priv_Type::View<'proto>;
  type Mut<'proto> = __priv_Type::Mut<'proto>;
}

impl crate::value::Type for Type {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_Type::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_Type::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Type::View<'proto> {
  pub fn as_view(&self) -> crate::View<Type> {
    __priv_Type::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Type::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn package(self) -> crate::View<'proto, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Type::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().package }) })
  }

  pub fn kind(self) -> crate::View<'proto, Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> Option<crate::View<'proto, Type_Kind>> {
    if !unsafe { Type::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, Type_Kind>(*unsafe { &self.ptr.as_ref().kind }) })
  }

  pub fn declared_in(self) -> crate::View<'proto, u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Type::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().declared_in }) })
  }

  pub fn fields(self) -> crate::Slice<'proto, Field> {
    if !unsafe { Type::__tdp_info().field(4).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().fields };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn fields_at(self, idx: usize) -> crate::View<'proto, Field> {
    self.fields().at(idx)
  }

  pub fn nesteds(self) -> crate::Slice<'proto, u32> {
    if !unsafe { Type::__tdp_info().field(5).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().nesteds };
      crate::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn nesteds_at(self, idx: usize) -> crate::View<'proto, u32> {
    self.nesteds().at(idx)
  }

  pub fn attrs(self) -> crate::View<'proto, Type_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> Option<crate::View<'proto, Type_Attrs>> {
    if !unsafe { Type::__tdp_info().field(6).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<Type_Attrs as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().attrs } as *mut _ as *mut u8)) }
  }

  pub fn span(self) -> crate::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Type::__tdp_info().field(7).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().span }) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_Type::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<Type> {
    __priv_Type::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, Type> {
    __priv_Type::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Type> {
    __priv_Type::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { Type::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Type::__tdp_info())
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Type::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn package(self) -> crate::View<'proto, u32> {
    self.package_or().unwrap_or_default()
  }
  pub fn package_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Type::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().package }) })
  }
  pub fn package_mut(self) -> crate::Mut<'proto, u32> {
    self.package_mut_or().into_mut()
  }
  pub fn package_mut_or(self) -> crate::value::OptMut<'proto, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(1),
      )
    }
  }
  pub fn package_set(self, value: u32) {
    self.package_mut().set(value);
  }

  pub fn kind(self) -> crate::View<'proto, Type_Kind> {
    self.kind_or().unwrap_or_default()
  }
  pub fn kind_or(self) -> Option<crate::View<'proto, Type_Kind>> {
    if !unsafe { Type::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, Type_Kind>(*unsafe { &self.ptr.as_ref().kind }) })
  }
  pub fn kind_mut(self) -> crate::Mut<'proto, Type_Kind> {
    self.kind_mut_or().into_mut()
  }
  pub fn kind_mut_or(self) -> crate::value::OptMut<'proto, Type_Kind> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(2),
      )
    }
  }
  pub fn kind_set(self, value: Type_Kind) {
    self.kind_mut().set(value);
  }

  pub fn declared_in(self) -> crate::View<'proto, u32> {
    self.declared_in_or().unwrap_or_default()
  }
  pub fn declared_in_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Type::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().declared_in }) })
  }
  pub fn declared_in_mut(self) -> crate::Mut<'proto, u32> {
    self.declared_in_mut_or().into_mut()
  }
  pub fn declared_in_mut_or(self) -> crate::value::OptMut<'proto, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(3),
      )
    }
  }
  pub fn declared_in_set(self, value: u32) {
    self.declared_in_mut().set(value);
  }

  pub fn fields(self) -> crate::Slice<'proto, Field> {
    if !unsafe { Type::__tdp_info().field(4).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().fields };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn fields_at(self, idx: usize) -> crate::View<'proto, Field> {
    self.fields().at(idx)
  }
  pub fn fields_mut(self) -> crate::Repeated<'proto, Field> {
    unsafe {
      Type::__tdp_info().field(4).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().fields } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn nesteds(self) -> crate::Slice<'proto, u32> {
    if !unsafe { Type::__tdp_info().field(5).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().nesteds };
      crate::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
    }
  }
  pub fn nesteds_at(self, idx: usize) -> crate::View<'proto, u32> {
    self.nesteds().at(idx)
  }
  pub fn nesteds_mut(self) -> crate::Repeated<'proto, u32> {
    unsafe {
      Type::__tdp_info().field(5).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().nesteds } as *mut _ as *mut u8,
        self.arena,
      )
    }
  }

  pub fn attrs(self) -> crate::View<'proto, Type_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> Option<crate::View<'proto, Type_Attrs>> {
    if !unsafe { Type::__tdp_info().field(6).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<Type_Attrs as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().attrs } as *mut _ as *mut u8)) }
  }
  pub fn attrs_mut(self) -> crate::Mut<'proto, Type_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(self) -> crate::value::OptMut<'proto, Type_Attrs> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(6),
      )
    }
  }

  pub fn span(self) -> crate::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Type::__tdp_info().field(7).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().span }) })
  }
  pub fn span_mut(self) -> crate::Mut<'proto, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(self) -> crate::value::OptMut<'proto, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type::__tdp_info().field(7),
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Type::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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
    pub(in super) fields: crate::__z::AVec<*mut u8>,
    pub (in super) nesteds: crate::__z::AVec<u32>,
    pub(in super) attrs: *mut u8,
    pub(in super) span: u32,
  }

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{8 + 1}> =
    crate::__z::tdp::DescStorage::<{8 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = Type::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
            Field::__tdp_info,
            Type_Attrs::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().package as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage {
          number: 3,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().kind as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
        },
        crate::__z::tdp::FieldStorage {
          number: 4,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().declared_in as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 3,
        },
        crate::__z::tdp::FieldStorage {
          number: 10,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().fields as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 4,
        },
        crate::__z::tdp::FieldStorage {
          number: 11,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().nesteds as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 4,
        },
        crate::__z::tdp::FieldStorage {
          number: 12,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().attrs as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 1,
          hasbit: 4,
        },
        crate::__z::tdp::FieldStorage {
          number: 20,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Type::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().span as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 5,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Type::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto Type>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Type> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Type::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut Type>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Type> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::Type> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
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

impl crate::ptr::Proxied for Type_Kind {
  type View<'a> = Self;
  type Mut<'a> = crate::ptr::ScalarMut<'a, Self>;
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
  ptr: crate::__z::ABox<__priv_Type_Attrs::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_Type_Attrs::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl Type_Attrs {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_Type_Attrs::Storage = __priv_Type_Attrs::Storage {
      __hasbits: [0; 1],
      deprecated: (0 as *mut u8, 0),
      docs: crate::__z::AVec::new(),
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_Type_Attrs::Storage as *mut __priv_Type_Attrs::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Type_Attrs::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Type_Attrs::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Type_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn deprecated(&self) -> crate::View<'_, crate::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { Type_Attrs::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().deprecated };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn deprecated_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type_Attrs::__tdp_info().field(0),
      )
    }
  }
  pub fn deprecated_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(&self) -> crate::Slice<'_, crate::Str> {
    if !unsafe { Type_Attrs::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().docs };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(&self, idx: usize) -> crate::View<'_, crate::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(&mut self) -> crate::Repeated<'_, crate::Str> {
    unsafe {
      Type_Attrs::__tdp_info().field(1).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().docs } as *mut _ as *mut u8,
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_Type_Attrs::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for Type_Attrs {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for Type_Attrs {
  type View<'proto> = __priv_Type_Attrs::View<'proto>;
  type Mut<'proto> = __priv_Type_Attrs::Mut<'proto>;
}

impl crate::value::Type for Type_Attrs {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_Type_Attrs::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_Type_Attrs::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Type_Attrs::View<'proto> {
  pub fn as_view(&self) -> crate::View<Type_Attrs> {
    __priv_Type_Attrs::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn deprecated(self) -> crate::View<'proto, crate::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Type_Attrs::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().deprecated };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn docs(self) -> crate::Slice<'proto, crate::Str> {
    if !unsafe { Type_Attrs::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().docs };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(self, idx: usize) -> crate::View<'proto, crate::Str> {
    self.docs().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_Type_Attrs::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<Type_Attrs> {
    __priv_Type_Attrs::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, Type_Attrs> {
    __priv_Type_Attrs::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Type_Attrs> {
    __priv_Type_Attrs::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { Type_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Type_Attrs::__tdp_info())
  }

  pub fn deprecated(self) -> crate::View<'proto, crate::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Type_Attrs::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().deprecated };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn deprecated_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Type_Attrs::__tdp_info().field(0),
      )
    }
  }
  pub fn deprecated_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(self) -> crate::Slice<'proto, crate::Str> {
    if !unsafe { Type_Attrs::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().docs };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(self, idx: usize) -> crate::View<'proto, crate::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(self) -> crate::Repeated<'proto, crate::Str> {
    unsafe {
      Type_Attrs::__tdp_info().field(1).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().docs } as *mut _ as *mut u8,
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Type_Attrs::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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
    pub(crate) docs: crate::__z::AVec<(*mut u8, usize)>,
  }

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{2 + 1}> =
    crate::__z::tdp::DescStorage::<{2 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = Type_Attrs::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Type_Attrs::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().deprecated as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 100,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Type_Attrs::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().docs as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Type_Attrs::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto Type_Attrs>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Type_Attrs> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Type_Attrs::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut Type_Attrs>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Type_Attrs> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::Type_Attrs> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

/// message `pz.Field`
pub struct Field {
  ptr: crate::__z::ABox<__priv_Field::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_Field::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl Field {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
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
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_Field::Storage as *mut __priv_Field::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Field::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Field::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Field::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn name(&self) -> crate::View<'_, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { Field::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn number(&self) -> crate::View<'_, i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(&self) -> Option<crate::View<'_, i32>> {
    if !unsafe { Field::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().number }) })
  }
  pub fn number_mut(&mut self) -> crate::Mut<'_, i32> {
    self.number_mut_or().into_mut()
  }
  pub fn number_mut_or(&mut self) -> crate::value::OptMut<'_, i32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(1),
      )
    }
  }
  pub fn number_set(&mut self, value: i32) {
    self.number_mut().set(value);
  }

  pub fn is_repeated(&self) -> crate::View<'_, bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(&self) -> Option<crate::View<'_, bool>> {
    if !unsafe { Field::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().is_repeated }) })
  }
  pub fn is_repeated_mut(&mut self) -> crate::Mut<'_, bool> {
    self.is_repeated_mut_or().into_mut()
  }
  pub fn is_repeated_mut_or(&mut self) -> crate::value::OptMut<'_, bool> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(2),
      )
    }
  }
  pub fn is_repeated_set(&mut self, value: bool) {
    self.is_repeated_mut().set(value);
  }

  pub fn r#type(&self) -> crate::View<'_, Field_Type> {
    self.r#type_or().unwrap_or_default()
  }
  pub fn r#type_or(&self) -> Option<crate::View<'_, Field_Type>> {
    if !unsafe { Field::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, Field_Type>(*unsafe { &self.ptr.as_ref().r#type }) })
  }
  pub fn r#type_mut(&mut self) -> crate::Mut<'_, Field_Type> {
    self.r#type_mut_or().into_mut()
  }
  pub fn r#type_mut_or(&mut self) -> crate::value::OptMut<'_, Field_Type> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(3),
      )
    }
  }
  pub fn r#type_set(&mut self, value: Field_Type) {
    self.r#type_mut().set(value);
  }

  pub fn type_index(&self) -> crate::View<'_, u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(&self) -> Option<crate::View<'_, u32>> {
    if !unsafe { Field::__tdp_info().field(4).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().type_index }) })
  }
  pub fn type_index_mut(&mut self) -> crate::Mut<'_, u32> {
    self.type_index_mut_or().into_mut()
  }
  pub fn type_index_mut_or(&mut self) -> crate::value::OptMut<'_, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(4),
      )
    }
  }
  pub fn type_index_set(&mut self, value: u32) {
    self.type_index_mut().set(value);
  }

  pub fn attrs(&self) -> crate::View<'_, Field_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(&self) -> Option<crate::View<'_, Field_Attrs>> {
    if !unsafe { Field::__tdp_info().field(5).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<Field_Attrs as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().attrs } as *mut _ as *mut u8)) }
  }
  pub fn attrs_mut(&mut self) -> crate::Mut<'_, Field_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(&mut self) -> crate::value::OptMut<'_, Field_Attrs> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(5),
      )
    }
  }

  pub fn span(&self) -> crate::View<'_, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(&self) -> Option<crate::View<'_, u32>> {
    if !unsafe { Field::__tdp_info().field(6).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().span }) })
  }
  pub fn span_mut(&mut self) -> crate::Mut<'_, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(&mut self) -> crate::value::OptMut<'_, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(6),
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_Field::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for Field {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for Field {
  type View<'proto> = __priv_Field::View<'proto>;
  type Mut<'proto> = __priv_Field::Mut<'proto>;
}

impl crate::value::Type for Field {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_Field::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_Field::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Field::View<'proto> {
  pub fn as_view(&self) -> crate::View<Field> {
    __priv_Field::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Field::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn number(self) -> crate::View<'proto, i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(self) -> Option<crate::View<'proto, i32>> {
    if !unsafe { Field::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().number }) })
  }

  pub fn is_repeated(self) -> crate::View<'proto, bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(self) -> Option<crate::View<'proto, bool>> {
    if !unsafe { Field::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().is_repeated }) })
  }

  pub fn r#type(self) -> crate::View<'proto, Field_Type> {
    self.r#type_or().unwrap_or_default()
  }
  pub fn r#type_or(self) -> Option<crate::View<'proto, Field_Type>> {
    if !unsafe { Field::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, Field_Type>(*unsafe { &self.ptr.as_ref().r#type }) })
  }

  pub fn type_index(self) -> crate::View<'proto, u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Field::__tdp_info().field(4).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().type_index }) })
  }

  pub fn attrs(self) -> crate::View<'proto, Field_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> Option<crate::View<'proto, Field_Attrs>> {
    if !unsafe { Field::__tdp_info().field(5).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<Field_Attrs as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().attrs } as *mut _ as *mut u8)) }
  }

  pub fn span(self) -> crate::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Field::__tdp_info().field(6).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().span }) })
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_Field::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<Field> {
    __priv_Field::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, Field> {
    __priv_Field::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Field> {
    __priv_Field::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { Field::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Field::__tdp_info())
  }

  pub fn name(self) -> crate::View<'proto, crate::Str> {
    self.name_or().unwrap_or_default()
  }
  pub fn name_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Field::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().name };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn name_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.name_mut_or().into_mut()
  }
  pub fn name_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(0),
      )
    }
  }
  pub fn name_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.name_mut().set(value);
  }

  pub fn number(self) -> crate::View<'proto, i32> {
    self.number_or().unwrap_or_default()
  }
  pub fn number_or(self) -> Option<crate::View<'proto, i32>> {
    if !unsafe { Field::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, i32>(*unsafe { &self.ptr.as_ref().number }) })
  }
  pub fn number_mut(self) -> crate::Mut<'proto, i32> {
    self.number_mut_or().into_mut()
  }
  pub fn number_mut_or(self) -> crate::value::OptMut<'proto, i32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(1),
      )
    }
  }
  pub fn number_set(self, value: i32) {
    self.number_mut().set(value);
  }

  pub fn is_repeated(self) -> crate::View<'proto, bool> {
    self.is_repeated_or().unwrap_or_default()
  }
  pub fn is_repeated_or(self) -> Option<crate::View<'proto, bool>> {
    if !unsafe { Field::__tdp_info().field(2).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<bool, bool>(*unsafe { &self.ptr.as_ref().is_repeated }) })
  }
  pub fn is_repeated_mut(self) -> crate::Mut<'proto, bool> {
    self.is_repeated_mut_or().into_mut()
  }
  pub fn is_repeated_mut_or(self) -> crate::value::OptMut<'proto, bool> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(2),
      )
    }
  }
  pub fn is_repeated_set(self, value: bool) {
    self.is_repeated_mut().set(value);
  }

  pub fn r#type(self) -> crate::View<'proto, Field_Type> {
    self.r#type_or().unwrap_or_default()
  }
  pub fn r#type_or(self) -> Option<crate::View<'proto, Field_Type>> {
    if !unsafe { Field::__tdp_info().field(3).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, Field_Type>(*unsafe { &self.ptr.as_ref().r#type }) })
  }
  pub fn r#type_mut(self) -> crate::Mut<'proto, Field_Type> {
    self.r#type_mut_or().into_mut()
  }
  pub fn r#type_mut_or(self) -> crate::value::OptMut<'proto, Field_Type> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(3),
      )
    }
  }
  pub fn r#type_set(self, value: Field_Type) {
    self.r#type_mut().set(value);
  }

  pub fn type_index(self) -> crate::View<'proto, u32> {
    self.type_index_or().unwrap_or_default()
  }
  pub fn type_index_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Field::__tdp_info().field(4).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().type_index }) })
  }
  pub fn type_index_mut(self) -> crate::Mut<'proto, u32> {
    self.type_index_mut_or().into_mut()
  }
  pub fn type_index_mut_or(self) -> crate::value::OptMut<'proto, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(4),
      )
    }
  }
  pub fn type_index_set(self, value: u32) {
    self.type_index_mut().set(value);
  }

  pub fn attrs(self) -> crate::View<'proto, Field_Attrs> {
    self.attrs_or().unwrap_or_default()
  }
  pub fn attrs_or(self) -> Option<crate::View<'proto, Field_Attrs>> {
    if !unsafe { Field::__tdp_info().field(5).has(self.ptr.as_ptr()) } { return None }
    unsafe { Some(<Field_Attrs as crate::value::Type>::__make_view(unsafe { &mut self.ptr.as_mut().attrs } as *mut _ as *mut u8)) }
  }
  pub fn attrs_mut(self) -> crate::Mut<'proto, Field_Attrs> {
    self.attrs_mut_or().into_mut()
  }
  pub fn attrs_mut_or(self) -> crate::value::OptMut<'proto, Field_Attrs> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(5),
      )
    }
  }

  pub fn span(self) -> crate::View<'proto, u32> {
    self.span_or().unwrap_or_default()
  }
  pub fn span_or(self) -> Option<crate::View<'proto, u32>> {
    if !unsafe { Field::__tdp_info().field(6).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe { std::mem::transmute::<u32, u32>(*unsafe { &self.ptr.as_ref().span }) })
  }
  pub fn span_mut(self) -> crate::Mut<'proto, u32> {
    self.span_mut_or().into_mut()
  }
  pub fn span_mut_or(self) -> crate::value::OptMut<'proto, u32> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field::__tdp_info().field(6),
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Field::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{7 + 1}> =
    crate::__z::tdp::DescStorage::<{7 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = Field::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
            Field_Attrs::__tdp_info,
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().name as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 2,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().number as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage {
          number: 3,
          flags:
            crate::__z::tdp::Kind::Bool.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().is_repeated as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 2,
        },
        crate::__z::tdp::FieldStorage {
          number: 4,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().r#type as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 3,
        },
        crate::__z::tdp::FieldStorage {
          number: 5,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().type_index as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 4,
        },
        crate::__z::tdp::FieldStorage {
          number: 10,
          flags:
            crate::__z::tdp::Kind::Type.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().attrs as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 5,
        },
        crate::__z::tdp::FieldStorage {
          number: 20,
          flags:
            crate::__z::tdp::Kind::I32.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Field::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().span as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 6,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Field::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto Field>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Field> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Field::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut Field>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Field> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::Field> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
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

impl crate::ptr::Proxied for Field_Type {
  type View<'a> = Self;
  type Mut<'a> = crate::ptr::ScalarMut<'a, Self>;
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
  ptr: crate::__z::ABox<__priv_Field_Attrs::Storage>,
  arena: crate::__z::RawArena,
}

const _: () = {
  assert!(
    std::mem::size_of::<__priv_Field_Attrs::Storage>() < (u32::MAX as usize),
    "storage size excees 4GB",
  );
};

impl Field_Attrs {
  pub const DEFAULT: crate::View<'static, Self> = unsafe {
    const VALUE: __priv_Field_Attrs::Storage = __priv_Field_Attrs::Storage {
      __hasbits: [0; 1],
      deprecated: (0 as *mut u8, 0),
      docs: crate::__z::AVec::new(),
    };
    crate::View::<Self> {
      ptr: crate::__z::ABox::from_ptr(&VALUE as *const __priv_Field_Attrs::Storage as *mut __priv_Field_Attrs::Storage as *mut u8),
      _ph: std::marker::PhantomData,
    }
  };

  pub fn new() -> Self {
    let arena = crate::__z::RawArena::new();
    let ptr = arena.alloc(Self::__LAYOUT).as_ptr();
    unsafe {
      ptr.write_bytes(0, Self::__LAYOUT.size());
      Self { ptr: crate::__z::ABox::from_ptr(ptr), arena }
    }
  }

  pub fn from_pb(input: &mut dyn std::io::Read) -> Result<Self, crate::Error> {
    let mut new = Self::new();
    new.parse_pb(input)?;
    Ok(new)
  }

  pub fn parse_pb(&mut self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    self.as_mut().parse_pb(input)
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Field_Attrs::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Field_Attrs::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(&mut self) {
    unsafe { Field_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn into_raw(self) -> *mut u8 {
    self.ptr.as_ptr()
  }

  pub fn deprecated(&self) -> crate::View<'_, crate::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(&self) -> Option<crate::View<'_, crate::Str>> {
    if !unsafe { Field_Attrs::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().deprecated };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn deprecated_mut(&mut self) -> crate::Mut<'_, crate::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(&mut self) -> crate::value::OptMut<'_, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field_Attrs::__tdp_info().field(0),
      )
    }
  }
  pub fn deprecated_set(&mut self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(&self) -> crate::Slice<'_, crate::Str> {
    if !unsafe { Field_Attrs::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().docs };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(&self, idx: usize) -> crate::View<'_, crate::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(&mut self) -> crate::Repeated<'_, crate::Str> {
    unsafe {
      Field_Attrs::__tdp_info().field(1).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().docs } as *mut _ as *mut u8,
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
  pub fn __tdp_info() -> crate::__z::tdp::Desc {
    unsafe { __priv_Field_Attrs::TDP_INFO.get() }
  }
  #[doc(hidden)]
  pub unsafe fn __raw_data(&self) -> &[u8] {
    std::slice::from_raw_parts(self.ptr.as_ptr(), Self::__LAYOUT.size())
  }
}

impl Default for Field_Attrs {
  fn default() -> Self {
    Self::new()
  }
}

impl crate::ptr::Proxied for Field_Attrs {
  type View<'proto> = __priv_Field_Attrs::View<'proto>;
  type Mut<'proto> = __priv_Field_Attrs::Mut<'proto>;
}

impl crate::value::Type for Field_Attrs {
  type __Storage = *mut u8;

  unsafe fn __make_view<'a>(ptr: *mut u8) -> crate::View<'a, Self> {
    __priv_Field_Attrs::View {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      _ph: std::marker::PhantomData,
    }
  }
  unsafe fn __make_mut<'a>(ptr: *mut u8, arena: crate::__z::RawArena) -> crate::Mut<'a, Self> {
    __priv_Field_Attrs::Mut {
      ptr: crate::__z::ABox::from_ptr(ptr.cast::<*mut u8>().read()),
      arena,
      _ph: std::marker::PhantomData,
    }
  }

  unsafe fn __resize(ptr: *mut u8, new_len: usize, arena: crate::__z::RawArena) {
    (&mut *ptr.cast::<crate::__z::AVec<*mut u8>>()).resize_msg(new_len, arena, Self::__LAYOUT)
  }
}

impl<'proto> __priv_Field_Attrs::View<'proto> {
  pub fn as_view(&self) -> crate::View<Field_Attrs> {
    __priv_Field_Attrs::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn deprecated(self) -> crate::View<'proto, crate::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Field_Attrs::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().deprecated };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }

  pub fn docs(self) -> crate::Slice<'proto, crate::Str> {
    if !unsafe { Field_Attrs::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().docs };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(self, idx: usize) -> crate::View<'proto, crate::Str> {
    self.docs().at(idx)
  }

  #[doc(hidden)]
  pub fn __debug(self, debug: &mut crate::__z::Debug) -> std::fmt::Result {
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

impl<'proto> __priv_Field_Attrs::Mut<'proto>  {
  pub fn as_view(&self) -> crate::View<Field_Attrs> {
    __priv_Field_Attrs::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn into_view(self) -> crate::View<'proto, Field_Attrs> {
    __priv_Field_Attrs::View { ptr: self.ptr, _ph: std::marker::PhantomData }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Field_Attrs> {
    __priv_Field_Attrs::Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
  }

  pub fn clear(self) {
    unsafe { Field_Attrs::__raw_clear(self.ptr.as_ptr()) }
  }

  pub fn parse_pb(self, input: &mut dyn std::io::Read) -> Result<(), crate::Error> {
    let mut ctx = crate::__z::tdp::parse::Context::new(input, self.arena);
    ctx.parse(self.ptr.as_ptr() as *mut u8, Field_Attrs::__tdp_info())
  }

  pub fn deprecated(self) -> crate::View<'proto, crate::Str> {
    self.deprecated_or().unwrap_or_default()
  }
  pub fn deprecated_or(self) -> Option<crate::View<'proto, crate::Str>> {
    if !unsafe { Field_Attrs::__tdp_info().field(0).has(self.ptr.as_ptr()) } { return None }
    Some(unsafe {
      let (mut ptr, len) = *unsafe { &self.ptr.as_ref().deprecated };
      if ptr.is_null() { ptr = 1 as *mut u8; }
      crate::Str::from_raw_parts(ptr, len)
    })
  }
  pub fn deprecated_mut(self) -> crate::Mut<'proto, crate::Str> {
    self.deprecated_mut_or().into_mut()
  }
  pub fn deprecated_mut_or(self) -> crate::value::OptMut<'proto, crate::Str> {
    unsafe {
      crate::value::OptMut::__wrap(
        self.ptr.as_ptr(),
        self.arena,
        Field_Attrs::__tdp_info().field(0),
      )
    }
  }
  pub fn deprecated_set(self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
    self.deprecated_mut().set(value);
  }

  pub fn docs(self) -> crate::Slice<'proto, crate::Str> {
    if !unsafe { Field_Attrs::__tdp_info().field(1).has(self.ptr.as_ptr()) } { return crate::Slice::default() }
    unsafe {
      let vec = unsafe { &self.ptr.as_ref().docs };
      crate::Slice::__wrap(vec.as_ptr(), vec.len())
    }
  }
  pub fn docs_at(self, idx: usize) -> crate::View<'proto, crate::Str> {
    self.docs().at(idx)
  }
  pub fn docs_mut(self) -> crate::Repeated<'proto, crate::Str> {
    unsafe {
      Field_Attrs::__tdp_info().field(1).init(self.ptr.as_ptr(), self.arena);
      crate::Repeated::__wrap(
        unsafe { &mut self.ptr.as_mut().docs } as *mut _ as *mut u8,
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
    let mut debug = crate::__z::Debug::new(fmt);
    self.__debug(&mut debug)
  }
}

impl std::fmt::Debug for __priv_Field_Attrs::Mut<'_> {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    use crate::ptr::ViewFor;
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
    pub(crate) docs: crate::__z::AVec<(*mut u8, usize)>,
  }

  pub static TDP_INFO: crate::__z::tdp::DescStorage<{2 + 1}> =
    crate::__z::tdp::DescStorage::<{2 + 1}> {
      header: crate::__z::tdp::DescHeader {
        size: {
          let size = Field_Attrs::__LAYOUT.size();
          assert!(size <= (u32::MAX as usize));
          size as u32
        },
        descs: {
          const DESCS: &[fn() -> crate::__z::tdp::Desc] = &[
          ];
          DESCS.as_ptr()
        },
        num_hasbit_words: 1,
        kind: crate::__z::tdp::DescKind::Message,
      },
      fields: [
        crate::__z::tdp::FieldStorage {
          number: 1,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            0 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Field_Attrs::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().deprecated as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 0,
        },
        crate::__z::tdp::FieldStorage {
          number: 100,
          flags:
            crate::__z::tdp::Kind::Str.raw() << crate::__z::tdp::Field::KIND_SHIFT |
            1 << crate::__z::tdp::Field::REP_SHIFT,
          offset: unsafe {
            let msg = Field_Attrs::DEFAULT;
            let top = msg.ptr.as_ptr().cast::<u8>();
            let field = &msg.ptr.as_ref().docs as *const _ as *const u8;
            field.offset_from(top) as u32
          },
          desc: 0,
          hasbit: 1,
        },
        crate::__z::tdp::FieldStorage { number: 0, flags: 0, offset: 0, desc: 0, hasbit: 0 },
      ],
    };

  #[derive(Copy, Clone)]
  pub struct View<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Field_Attrs::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto Field_Attrs>,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Field_Attrs> for View<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  pub struct Mut<'proto> {
    pub(in super) ptr: crate::__z::ABox<__priv_Field_Attrs::Storage>,
    pub(in super) _ph: std::marker::PhantomData<&'proto mut Field_Attrs>,
    pub(in super) arena: crate::__z::RawArena,
  }

  impl<'proto> crate::ptr::ViewFor<'proto, super::Field_Attrs> for Mut<'proto> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }
  }

  impl<'proto> crate::ptr::MutFor<'proto, super::Field_Attrs> for Mut<'proto> {
    fn into_view(self) -> View<'proto> {
      View { ptr: self.ptr, _ph: std::marker::PhantomData }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr, _ph: std::marker::PhantomData, arena: self.arena }
    }
  }
}

