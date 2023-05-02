// ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

/// message `pz.Bundle`
#[derive(Clone)]
pub struct Bundle {
  ptr: Box<__priv_Bundle::Storage>,
}

impl Bundle {
  pub const DEFAULT: crate::View<'static, Self> = {
    const VALUE: __priv_Bundle::Storage = __priv_Bundle::Storage {
      __hasbits: [0; 0],
      types: Vec::new(),
      packages: Vec::new(),
      foreign_types: Vec::new(),
    };
    crate::View::<Self> { ptr: &VALUE }
  };

  pub fn new() -> Self {
    Self {
      ptr: Box::new(__priv_Bundle::Storage {
        __hasbits: [0; 0],
        types: Vec::new(),
        packages: Vec::new(),
        foreign_types: Vec::new(),
      }),
    }
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Bundle::View { ptr: &self.ptr }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Bundle::Mut { ptr: &mut self.ptr }
  }

  pub fn clear(&mut self) {
    self.as_mut().clear();
  }

  pub fn types_len(&self) -> usize {
    self.ptr.types.len()
  }
  pub fn types(&self, n: usize) -> Option<crate::View<'_, Type>> {
    self.ptr.types.get(n).map(|ptr| crate::View::<Type> { ptr: &ptr.ptr })
  }
  pub fn types_iter(&self) -> impl Iterator<Item = crate::View<'_, Type>> + '_ {
    self.ptr.types.iter().map(|ptr| crate::View::<Type> { ptr: &ptr.ptr })
  }
  pub fn types_mut(&mut self, n: usize) -> Option<crate::Mut<'_, Type>> {
    self.ptr.types.get_mut(n).map(|ptr| crate::Mut::<Type> { ptr: &mut ptr.ptr })
  }
  pub fn types_add(&mut self) -> crate::Mut<'_, Type> {
    self.ptr.types.push(Type::new());
    self.ptr.types.last_mut().map(|ptr| crate::Mut::<Type> { ptr: &mut ptr.ptr }).unwrap()
  }
  pub fn types_resize(&mut self, n: usize) {
    self.ptr.types.resize_with(n, Type::new)
  }

  pub fn packages_len(&self) -> usize {
    self.ptr.packages.len()
  }
  pub fn packages(&self, n: usize) -> Option<&'_ crate::Str> {
    self.ptr.packages.get(n).map(crate::Str::new)
  }
  pub fn packages_iter(&self) -> impl Iterator<Item = &'_ crate::Str> + '_ {
    self.ptr.packages.iter().map(crate::Str::new)
  }
  pub fn packages_mut(&mut self, n: usize) -> Option<crate::StrBuf<'_>> {
    self.ptr.packages.get_mut(n).map(crate::StrBuf::__wrap)
  }
  pub fn packages_add(&mut self) -> crate::StrBuf<'_> {
    self.ptr.packages.push(Vec::new());
    self.ptr.packages.last_mut().map(crate::StrBuf::__wrap).unwrap()
  }
  pub fn packages_resize(&mut self, n: usize) {
    self.ptr.packages.resize(n, Vec::new())
  }

  pub fn foreign_types_len(&self) -> usize {
    self.ptr.foreign_types.len()
  }
  pub fn foreign_types(&self, n: usize) -> Option<&'_ crate::Str> {
    self.ptr.foreign_types.get(n).map(crate::Str::new)
  }
  pub fn foreign_types_iter(&self) -> impl Iterator<Item = &'_ crate::Str> + '_ {
    self.ptr.foreign_types.iter().map(crate::Str::new)
  }
  pub fn foreign_types_mut(&mut self, n: usize) -> Option<crate::StrBuf<'_>> {
    self.ptr.foreign_types.get_mut(n).map(crate::StrBuf::__wrap)
  }
  pub fn foreign_types_add(&mut self) -> crate::StrBuf<'_> {
    self.ptr.foreign_types.push(Vec::new());
    self.ptr.foreign_types.last_mut().map(crate::StrBuf::__wrap).unwrap()
  }
  pub fn foreign_types_resize(&mut self, n: usize) {
    self.ptr.foreign_types.resize(n, Vec::new())
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
    self.ptr.types.len()
  }
  pub fn types(self, n: usize) -> Option<crate::View<'msg, Type>> {
    self.ptr.types.get(n).map(|ptr| crate::View::<Type> { ptr: &ptr.ptr })
  }
  pub fn types_iter(self) -> impl Iterator<Item = crate::View<'msg, Type>> + 'msg {
    self.ptr.types.iter().map(|ptr| crate::View::<Type> { ptr: &ptr.ptr })
  }

  pub fn packages_len(self) -> usize {
    self.ptr.packages.len()
  }
  pub fn packages(self, n: usize) -> Option<&'msg crate::Str> {
    self.ptr.packages.get(n).map(crate::Str::new)
  }
  pub fn packages_iter(self) -> impl Iterator<Item = &'msg crate::Str> + 'msg {
    self.ptr.packages.iter().map(crate::Str::new)
  }

  pub fn foreign_types_len(self) -> usize {
    self.ptr.foreign_types.len()
  }
  pub fn foreign_types(self, n: usize) -> Option<&'msg crate::Str> {
    self.ptr.foreign_types.get(n).map(crate::Str::new)
  }
  pub fn foreign_types_iter(self) -> impl Iterator<Item = &'msg crate::Str> + 'msg {
    self.ptr.foreign_types.iter().map(crate::Str::new)
  }

}

impl<'msg> __priv_Bundle::Mut<'msg>  {
  pub fn clear(&mut self) {
    self.ptr.__hasbits = [0; 0];
    self.ptr.types.clear();
    self.ptr.packages.clear();
    self.ptr.foreign_types.clear();
  }

  pub fn types_len(self) -> usize {
    self.ptr.types.len()
  }
  pub fn types(self, n: usize) -> Option<crate::View<'msg, Type>> {
    self.ptr.types.get(n).map(|ptr| crate::View::<Type> { ptr: &ptr.ptr })
  }
  pub fn types_iter(self) -> impl Iterator<Item = crate::View<'msg, Type>> + 'msg {
    self.ptr.types.iter().map(|ptr| crate::View::<Type> { ptr: &ptr.ptr })
  }
  pub fn types_mut(self, n: usize) -> Option<crate::Mut<'msg, Type>> {
    self.ptr.types.get_mut(n).map(|ptr| crate::Mut::<Type> { ptr: &mut ptr.ptr })
  }
  pub fn types_add(self) -> crate::Mut<'msg, Type> {
    self.ptr.types.push(Type::new());
    self.ptr.types.last_mut().map(|ptr| crate::Mut::<Type> { ptr: &mut ptr.ptr }).unwrap()
  }
  pub fn types_resize(self, n: usize) {
    self.ptr.types.resize_with(n, Type::new)
  }

  pub fn packages_len(self) -> usize {
    self.ptr.packages.len()
  }
  pub fn packages(self, n: usize) -> Option<&'msg crate::Str> {
    self.ptr.packages.get(n).map(crate::Str::new)
  }
  pub fn packages_iter(self) -> impl Iterator<Item = &'msg crate::Str> + 'msg {
    self.ptr.packages.iter().map(crate::Str::new)
  }
  pub fn packages_mut(self, n: usize) -> Option<crate::StrBuf<'msg>> {
    self.ptr.packages.get_mut(n).map(crate::StrBuf::__wrap)
  }
  pub fn packages_add(self) -> crate::StrBuf<'msg> {
    self.ptr.packages.push(Vec::new());
    self.ptr.packages.last_mut().map(crate::StrBuf::__wrap).unwrap()
  }
  pub fn packages_resize(self, n: usize) {
    self.ptr.packages.resize(n, Vec::new())
  }

  pub fn foreign_types_len(self) -> usize {
    self.ptr.foreign_types.len()
  }
  pub fn foreign_types(self, n: usize) -> Option<&'msg crate::Str> {
    self.ptr.foreign_types.get(n).map(crate::Str::new)
  }
  pub fn foreign_types_iter(self) -> impl Iterator<Item = &'msg crate::Str> + 'msg {
    self.ptr.foreign_types.iter().map(crate::Str::new)
  }
  pub fn foreign_types_mut(self, n: usize) -> Option<crate::StrBuf<'msg>> {
    self.ptr.foreign_types.get_mut(n).map(crate::StrBuf::__wrap)
  }
  pub fn foreign_types_add(self) -> crate::StrBuf<'msg> {
    self.ptr.foreign_types.push(Vec::new());
    self.ptr.foreign_types.last_mut().map(crate::StrBuf::__wrap).unwrap()
  }
  pub fn foreign_types_resize(self, n: usize) {
    self.ptr.foreign_types.resize(n, Vec::new())
  }

}

impl Drop for Bundle {
  fn drop(&mut self) {
  }
}

mod __priv_Bundle {
  pub use super::*;

  #[derive(Clone)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 0],
    pub(crate) types: Vec<Type>,
    pub(crate) packages: Vec<Vec<u8>>,
    pub(crate) foreign_types: Vec<Vec<u8>>,
  }

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(crate) ptr: &'msg Storage,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Bundle> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr }
    }
  }

  pub struct Mut<'msg> {
    pub(crate) ptr: &'msg mut Storage,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Bundle> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: &self.ptr }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Bundle> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr }
    }
  }
}

/// message `pz.Type`
#[derive(Clone)]
pub struct Type {
  ptr: Box<__priv_Type::Storage>,
}

impl Type {
  pub const DEFAULT: crate::View<'static, Self> = {
    const VALUE: __priv_Type::Storage = __priv_Type::Storage {
      __hasbits: [0; 1],
      name: Vec::new(),
      package: 0,
      kind: Type_Kind::new(),
      declared_in: 0,
      fields: Vec::new(),
      nesteds: Vec::new(),
      span: 0,
    };
    crate::View::<Self> { ptr: &VALUE }
  };

  pub fn new() -> Self {
    Self {
      ptr: Box::new(__priv_Type::Storage {
        __hasbits: [0; 1],
        name: Vec::new(),
        package: 0,
        kind: Type_Kind::new(),
        declared_in: 0,
        fields: Vec::new(),
        nesteds: Vec::new(),
        span: 0,
      }),
    }
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Type::View { ptr: &self.ptr }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Type::Mut { ptr: &mut self.ptr }
  }

  pub fn clear(&mut self) {
    self.as_mut().clear();
  }

  pub fn name(&self) -> &'_ crate::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(&self) -> Option<&'_ crate::Str> {
    if self.ptr.__hasbits[0] & 1 == 0 { return None }
    Some(crate::Str::new(&*self.ptr.name))
  }
  pub fn name_mut(&mut self) -> crate::StrBuf<'_> {
    self.ptr.__hasbits[0] |= 1;
    crate::StrBuf::__wrap(&mut self.ptr.name)
  }
  pub fn name_opt_mut(&mut self) -> Option<crate::StrBuf<'_>> {
    if self.ptr.__hasbits[0] & 1 == 0 { return None }
    Some(crate::StrBuf::__wrap(&mut self.ptr.name))
  }
  pub fn name_set<'a>(&mut self, value: impl crate::rt::str::IntoStrOpt<'a>) {
    match value.into_str_opt() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 1;
        self.ptr.name.clear();
        self.ptr.name.extend_from_slice(value.as_bytes())
      }
      None => {
        self.ptr.__hasbits[0] &= !1;
        self.ptr.name.clear();
      }
    }
  }

  pub fn package(&self) -> u32 {
    self.package_opt().unwrap_or_default()
  }
  pub fn package_opt(&self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 2 == 0 { return None }
    Some(self.ptr.package)
  }
  pub fn package_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 2;
        self.ptr.package = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !2;
      }
    }
  }

  pub fn kind(&self) -> Type_Kind {
    self.kind_opt().unwrap_or_default()
  }
  pub fn kind_opt(&self) -> Option<Type_Kind> {
    if self.ptr.__hasbits[0] & 4 == 0 { return None }
    Some(self.ptr.kind)
  }
  pub fn kind_set(&mut self, value: impl Into<Option<Type_Kind>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 4;
        self.ptr.kind = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !4;
      }
    }
  }

  pub fn declared_in(&self) -> u32 {
    self.declared_in_opt().unwrap_or_default()
  }
  pub fn declared_in_opt(&self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 8 == 0 { return None }
    Some(self.ptr.declared_in)
  }
  pub fn declared_in_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 8;
        self.ptr.declared_in = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !8;
      }
    }
  }

  pub fn fields_len(&self) -> usize {
    self.ptr.fields.len()
  }
  pub fn fields(&self, n: usize) -> Option<crate::View<'_, Field>> {
    self.ptr.fields.get(n).map(|ptr| crate::View::<Field> { ptr: &ptr.ptr })
  }
  pub fn fields_iter(&self) -> impl Iterator<Item = crate::View<'_, Field>> + '_ {
    self.ptr.fields.iter().map(|ptr| crate::View::<Field> { ptr: &ptr.ptr })
  }
  pub fn fields_mut(&mut self, n: usize) -> Option<crate::Mut<'_, Field>> {
    self.ptr.fields.get_mut(n).map(|ptr| crate::Mut::<Field> { ptr: &mut ptr.ptr })
  }
  pub fn fields_add(&mut self) -> crate::Mut<'_, Field> {
    self.ptr.fields.push(Field::new());
    self.ptr.fields.last_mut().map(|ptr| crate::Mut::<Field> { ptr: &mut ptr.ptr }).unwrap()
  }
  pub fn fields_resize(&mut self, n: usize) {
    self.ptr.fields.resize_with(n, Field::new)
  }

  pub fn nesteds(&self) -> &'_ [u32] {
    &self.ptr.nesteds
  }
  pub fn nesteds_mut(&mut self) -> &'_ mut [u32] {
    &mut self.ptr.nesteds
  }
  pub fn nesteds_set(&mut self, that: &[u32]) {
    self.ptr.nesteds.clear();
    self.nesteds_extend(that)
  }
  pub fn nesteds_extend(&mut self, that: &[u32]) {
    self.ptr.nesteds.extend_from_slice(that)
  }

  pub fn span(&self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(&self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 16 == 0 { return None }
    Some(self.ptr.span)
  }
  pub fn span_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 16;
        self.ptr.span = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !16;
      }
    }
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
  pub fn name(self) -> &'msg crate::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(self) -> Option<&'msg crate::Str> {
    if self.ptr.__hasbits[0] & 1 == 0 { return None }
    Some(crate::Str::new(&*self.ptr.name))
  }

  pub fn package(self) -> u32 {
    self.package_opt().unwrap_or_default()
  }
  pub fn package_opt(self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 2 == 0 { return None }
    Some(self.ptr.package)
  }

  pub fn kind(self) -> Type_Kind {
    self.kind_opt().unwrap_or_default()
  }
  pub fn kind_opt(self) -> Option<Type_Kind> {
    if self.ptr.__hasbits[0] & 4 == 0 { return None }
    Some(self.ptr.kind)
  }

  pub fn declared_in(self) -> u32 {
    self.declared_in_opt().unwrap_or_default()
  }
  pub fn declared_in_opt(self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 8 == 0 { return None }
    Some(self.ptr.declared_in)
  }

  pub fn fields_len(self) -> usize {
    self.ptr.fields.len()
  }
  pub fn fields(self, n: usize) -> Option<crate::View<'msg, Field>> {
    self.ptr.fields.get(n).map(|ptr| crate::View::<Field> { ptr: &ptr.ptr })
  }
  pub fn fields_iter(self) -> impl Iterator<Item = crate::View<'msg, Field>> + 'msg {
    self.ptr.fields.iter().map(|ptr| crate::View::<Field> { ptr: &ptr.ptr })
  }

  pub fn nesteds(self) -> &'msg [u32] {
    &self.ptr.nesteds
  }

  pub fn span(self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 16 == 0 { return None }
    Some(self.ptr.span)
  }

}

impl<'msg> __priv_Type::Mut<'msg>  {
  pub fn clear(&mut self) {
    self.ptr.__hasbits = [0; 1];
    self.ptr.name.clear();
    self.ptr.fields.clear();
  }

  pub fn name(self) -> &'msg crate::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(self) -> Option<&'msg crate::Str> {
    if self.ptr.__hasbits[0] & 1 == 0 { return None }
    Some(crate::Str::new(&*self.ptr.name))
  }
  pub fn name_mut(self) -> crate::StrBuf<'msg> {
    self.ptr.__hasbits[0] |= 1;
    crate::StrBuf::__wrap(&mut self.ptr.name)
  }
  pub fn name_opt_mut(self) -> Option<crate::StrBuf<'msg>> {
    if self.ptr.__hasbits[0] & 1 == 0 { return None }
    Some(crate::StrBuf::__wrap(&mut self.ptr.name))
  }
  pub fn name_set<'a>(self, value: impl crate::rt::str::IntoStrOpt<'a>) {
    match value.into_str_opt() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 1;
        self.ptr.name.clear();
        self.ptr.name.extend_from_slice(value.as_bytes())
      }
      None => {
        self.ptr.__hasbits[0] &= !1;
        self.ptr.name.clear();
      }
    }
  }

  pub fn package(self) -> u32 {
    self.package_opt().unwrap_or_default()
  }
  pub fn package_opt(self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 2 == 0 { return None }
    Some(self.ptr.package)
  }
  pub fn package_set(self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 2;
        self.ptr.package = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !2;
      }
    }
  }

  pub fn kind(self) -> Type_Kind {
    self.kind_opt().unwrap_or_default()
  }
  pub fn kind_opt(self) -> Option<Type_Kind> {
    if self.ptr.__hasbits[0] & 4 == 0 { return None }
    Some(self.ptr.kind)
  }
  pub fn kind_set(self, value: impl Into<Option<Type_Kind>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 4;
        self.ptr.kind = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !4;
      }
    }
  }

  pub fn declared_in(self) -> u32 {
    self.declared_in_opt().unwrap_or_default()
  }
  pub fn declared_in_opt(self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 8 == 0 { return None }
    Some(self.ptr.declared_in)
  }
  pub fn declared_in_set(self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 8;
        self.ptr.declared_in = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !8;
      }
    }
  }

  pub fn fields_len(self) -> usize {
    self.ptr.fields.len()
  }
  pub fn fields(self, n: usize) -> Option<crate::View<'msg, Field>> {
    self.ptr.fields.get(n).map(|ptr| crate::View::<Field> { ptr: &ptr.ptr })
  }
  pub fn fields_iter(self) -> impl Iterator<Item = crate::View<'msg, Field>> + 'msg {
    self.ptr.fields.iter().map(|ptr| crate::View::<Field> { ptr: &ptr.ptr })
  }
  pub fn fields_mut(self, n: usize) -> Option<crate::Mut<'msg, Field>> {
    self.ptr.fields.get_mut(n).map(|ptr| crate::Mut::<Field> { ptr: &mut ptr.ptr })
  }
  pub fn fields_add(self) -> crate::Mut<'msg, Field> {
    self.ptr.fields.push(Field::new());
    self.ptr.fields.last_mut().map(|ptr| crate::Mut::<Field> { ptr: &mut ptr.ptr }).unwrap()
  }
  pub fn fields_resize(self, n: usize) {
    self.ptr.fields.resize_with(n, Field::new)
  }

  pub fn nesteds(self) -> &'msg [u32] {
    &self.ptr.nesteds
  }
  pub fn nesteds_mut(self) -> &'msg mut [u32] {
    &mut self.ptr.nesteds
  }
  pub fn nesteds_set(self, that: &[u32]) {
    self.ptr.nesteds.clear();
    self.nesteds_extend(that)
  }
  pub fn nesteds_extend(self, that: &[u32]) {
    self.ptr.nesteds.extend_from_slice(that)
  }

  pub fn span(self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 16 == 0 { return None }
    Some(self.ptr.span)
  }
  pub fn span_set(self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 16;
        self.ptr.span = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !16;
      }
    }
  }

}

impl Drop for Type {
  fn drop(&mut self) {
  }
}

mod __priv_Type {
  pub use super::*;

  #[derive(Clone)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(crate) name: Vec<u8>,
    pub(crate) package: u32,
    pub(crate) kind: Type_Kind,
    pub(crate) declared_in: u32,
    pub(crate) fields: Vec<Field>,
    pub(crate) nesteds: Vec<u32>,
    pub(crate) span: u32,
  }

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(crate) ptr: &'msg Storage,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Type> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr }
    }
  }

  pub struct Mut<'msg> {
    pub(crate) ptr: &'msg mut Storage,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Type> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: &self.ptr }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Type> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr }
    }
  }
}

/// enum `pz.Type.Kind`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

/// message `pz.Field`
#[derive(Clone)]
pub struct Field {
  ptr: Box<__priv_Field::Storage>,
}

impl Field {
  pub const DEFAULT: crate::View<'static, Self> = {
    const VALUE: __priv_Field::Storage = __priv_Field::Storage {
      __hasbits: [0; 1],
      name: Vec::new(),
      number: 0,
      is_repeated: false,
      r#type: Field_Type::new(),
      type_index: 0,
      span: 0,
    };
    crate::View::<Self> { ptr: &VALUE }
  };

  pub fn new() -> Self {
    Self {
      ptr: Box::new(__priv_Field::Storage {
        __hasbits: [0; 1],
        name: Vec::new(),
        number: 0,
        is_repeated: false,
        r#type: Field_Type::new(),
        type_index: 0,
        span: 0,
      }),
    }
  }

  pub fn as_view(&self) -> crate::View<Self> {
    __priv_Field::View { ptr: &self.ptr }
  }

  pub fn as_mut(&mut self) -> crate::Mut<Self> {
    __priv_Field::Mut { ptr: &mut self.ptr }
  }

  pub fn clear(&mut self) {
    self.as_mut().clear();
  }

  pub fn name(&self) -> &'_ crate::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(&self) -> Option<&'_ crate::Str> {
    if self.ptr.__hasbits[0] & 1 == 0 { return None }
    Some(crate::Str::new(&*self.ptr.name))
  }
  pub fn name_mut(&mut self) -> crate::StrBuf<'_> {
    self.ptr.__hasbits[0] |= 1;
    crate::StrBuf::__wrap(&mut self.ptr.name)
  }
  pub fn name_opt_mut(&mut self) -> Option<crate::StrBuf<'_>> {
    if self.ptr.__hasbits[0] & 1 == 0 { return None }
    Some(crate::StrBuf::__wrap(&mut self.ptr.name))
  }
  pub fn name_set<'a>(&mut self, value: impl crate::rt::str::IntoStrOpt<'a>) {
    match value.into_str_opt() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 1;
        self.ptr.name.clear();
        self.ptr.name.extend_from_slice(value.as_bytes())
      }
      None => {
        self.ptr.__hasbits[0] &= !1;
        self.ptr.name.clear();
      }
    }
  }

  pub fn number(&self) -> i32 {
    self.number_opt().unwrap_or_default()
  }
  pub fn number_opt(&self) -> Option<i32> {
    if self.ptr.__hasbits[0] & 2 == 0 { return None }
    Some(self.ptr.number)
  }
  pub fn number_set(&mut self, value: impl Into<Option<i32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 2;
        self.ptr.number = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !2;
      }
    }
  }

  pub fn is_repeated(&self) -> bool {
    self.is_repeated_opt().unwrap_or_default()
  }
  pub fn is_repeated_opt(&self) -> Option<bool> {
    if self.ptr.__hasbits[0] & 4 == 0 { return None }
    Some(self.ptr.is_repeated)
  }
  pub fn is_repeated_set(&mut self, value: impl Into<Option<bool>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 4;
        self.ptr.is_repeated = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !4;
      }
    }
  }

  pub fn r#type(&self) -> Field_Type {
    self.r#type_opt().unwrap_or_default()
  }
  pub fn r#type_opt(&self) -> Option<Field_Type> {
    if self.ptr.__hasbits[0] & 8 == 0 { return None }
    Some(self.ptr.r#type)
  }
  pub fn r#type_set(&mut self, value: impl Into<Option<Field_Type>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 8;
        self.ptr.r#type = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !8;
      }
    }
  }

  pub fn type_index(&self) -> u32 {
    self.type_index_opt().unwrap_or_default()
  }
  pub fn type_index_opt(&self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 16 == 0 { return None }
    Some(self.ptr.type_index)
  }
  pub fn type_index_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 16;
        self.ptr.type_index = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !16;
      }
    }
  }

  pub fn span(&self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(&self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 32 == 0 { return None }
    Some(self.ptr.span)
  }
  pub fn span_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 32;
        self.ptr.span = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !32;
      }
    }
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
  pub fn name(self) -> &'msg crate::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(self) -> Option<&'msg crate::Str> {
    if self.ptr.__hasbits[0] & 1 == 0 { return None }
    Some(crate::Str::new(&*self.ptr.name))
  }

  pub fn number(self) -> i32 {
    self.number_opt().unwrap_or_default()
  }
  pub fn number_opt(self) -> Option<i32> {
    if self.ptr.__hasbits[0] & 2 == 0 { return None }
    Some(self.ptr.number)
  }

  pub fn is_repeated(self) -> bool {
    self.is_repeated_opt().unwrap_or_default()
  }
  pub fn is_repeated_opt(self) -> Option<bool> {
    if self.ptr.__hasbits[0] & 4 == 0 { return None }
    Some(self.ptr.is_repeated)
  }

  pub fn r#type(self) -> Field_Type {
    self.r#type_opt().unwrap_or_default()
  }
  pub fn r#type_opt(self) -> Option<Field_Type> {
    if self.ptr.__hasbits[0] & 8 == 0 { return None }
    Some(self.ptr.r#type)
  }

  pub fn type_index(self) -> u32 {
    self.type_index_opt().unwrap_or_default()
  }
  pub fn type_index_opt(self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 16 == 0 { return None }
    Some(self.ptr.type_index)
  }

  pub fn span(self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 32 == 0 { return None }
    Some(self.ptr.span)
  }

}

impl<'msg> __priv_Field::Mut<'msg>  {
  pub fn clear(&mut self) {
    self.ptr.__hasbits = [0; 1];
    self.ptr.name.clear();
  }

  pub fn name(self) -> &'msg crate::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(self) -> Option<&'msg crate::Str> {
    if self.ptr.__hasbits[0] & 1 == 0 { return None }
    Some(crate::Str::new(&*self.ptr.name))
  }
  pub fn name_mut(self) -> crate::StrBuf<'msg> {
    self.ptr.__hasbits[0] |= 1;
    crate::StrBuf::__wrap(&mut self.ptr.name)
  }
  pub fn name_opt_mut(self) -> Option<crate::StrBuf<'msg>> {
    if self.ptr.__hasbits[0] & 1 == 0 { return None }
    Some(crate::StrBuf::__wrap(&mut self.ptr.name))
  }
  pub fn name_set<'a>(self, value: impl crate::rt::str::IntoStrOpt<'a>) {
    match value.into_str_opt() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 1;
        self.ptr.name.clear();
        self.ptr.name.extend_from_slice(value.as_bytes())
      }
      None => {
        self.ptr.__hasbits[0] &= !1;
        self.ptr.name.clear();
      }
    }
  }

  pub fn number(self) -> i32 {
    self.number_opt().unwrap_or_default()
  }
  pub fn number_opt(self) -> Option<i32> {
    if self.ptr.__hasbits[0] & 2 == 0 { return None }
    Some(self.ptr.number)
  }
  pub fn number_set(self, value: impl Into<Option<i32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 2;
        self.ptr.number = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !2;
      }
    }
  }

  pub fn is_repeated(self) -> bool {
    self.is_repeated_opt().unwrap_or_default()
  }
  pub fn is_repeated_opt(self) -> Option<bool> {
    if self.ptr.__hasbits[0] & 4 == 0 { return None }
    Some(self.ptr.is_repeated)
  }
  pub fn is_repeated_set(self, value: impl Into<Option<bool>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 4;
        self.ptr.is_repeated = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !4;
      }
    }
  }

  pub fn r#type(self) -> Field_Type {
    self.r#type_opt().unwrap_or_default()
  }
  pub fn r#type_opt(self) -> Option<Field_Type> {
    if self.ptr.__hasbits[0] & 8 == 0 { return None }
    Some(self.ptr.r#type)
  }
  pub fn r#type_set(self, value: impl Into<Option<Field_Type>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 8;
        self.ptr.r#type = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !8;
      }
    }
  }

  pub fn type_index(self) -> u32 {
    self.type_index_opt().unwrap_or_default()
  }
  pub fn type_index_opt(self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 16 == 0 { return None }
    Some(self.ptr.type_index)
  }
  pub fn type_index_set(self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 16;
        self.ptr.type_index = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !16;
      }
    }
  }

  pub fn span(self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(self) -> Option<u32> {
    if self.ptr.__hasbits[0] & 32 == 0 { return None }
    Some(self.ptr.span)
  }
  pub fn span_set(self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.ptr.__hasbits[0] |= 32;
        self.ptr.span = value;
      }
      None => {
        self.ptr.__hasbits[0] &= !32;
      }
    }
  }

}

impl Drop for Field {
  fn drop(&mut self) {
  }
}

mod __priv_Field {
  pub use super::*;

  #[derive(Clone)]
  pub struct Storage {
    pub(crate) __hasbits: [u32; 1],
    pub(crate) name: Vec<u8>,
    pub(crate) number: i32,
    pub(crate) is_repeated: bool,
    pub(crate) r#type: Field_Type,
    pub(crate) type_index: u32,
    pub(crate) span: u32,
  }

  #[derive(Copy, Clone)]
  pub struct View<'msg> {
    pub(crate) ptr: &'msg Storage,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Field> for View<'msg> {
    fn as_view(&self) -> View {
      View { ptr: self.ptr }
    }
  }

  pub struct Mut<'msg> {
    pub(crate) ptr: &'msg mut Storage,
  }

  impl<'msg> crate::rt::ptr::ViewFor<'msg, super::Field> for Mut<'msg> {
    fn as_view(&self) -> View {
      View { ptr: &self.ptr }
    }
  }

  impl<'msg> crate::rt::ptr::MutFor<'msg, super::Field> for Mut<'msg> {
    fn into_view(self) -> View<'msg> {
      View { ptr: self.ptr }
    }

    fn as_mut(&mut self) -> Mut {
      Mut { ptr: self.ptr }
    }
  }
}

/// enum `pz.Field.Type`
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

