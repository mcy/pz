// ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
use crate as __rt;

/// message `pz.Bundle`
#[derive(Clone)]
pub struct Bundle {
  __hasbits: [u32; 0],
  types: Vec<Type>,
  packages: Vec<Vec<u8>>,
  foreign_types: Vec<Vec<u8>>,
}

impl Bundle {
  pub const fn new() -> Self {
    Self {
      __hasbits: [0; 0],
      types: Vec::new(),
      packages: Vec::new(),
      foreign_types: Vec::new(),
    }
  }

  pub fn types_len(&self) -> usize {
    self.types.len()
  }
  pub fn types(&self, n: usize) -> Option<&Type> {
    self.types.get(n)
  }
  pub fn types_mut(&mut self, n: usize) -> Option<&mut Type> {
    self.types.get_mut(n)
  }
  pub fn types_add(&mut self) -> &mut Type {
    self.types.push(Default::default());
    self.types.last_mut().unwrap()
  }
  pub fn types_iter(&self) -> impl Iterator<Item = &Type> + '_ {
    self.types.iter()
  }
  pub fn types_resize(&mut self, n: usize) {
    self.types.resize(n, Default::default())
  }

  pub fn packages_len(&self) -> usize {
    self.packages.len()
  }
  pub fn packages(&self, n: usize) -> Option<&__rt::Str> {
    self.packages.get(n).map(__rt::Str::new)
  }
  pub fn packages_mut(&mut self, n: usize) -> Option<__rt::StrBuf> {
    self.packages.get_mut(n).map(__rt::StrBuf::__wrap)
  }
  pub fn packages_add(&mut self) -> __rt::StrBuf {
    self.packages.push(Default::default());
    self.packages.last_mut().map(__rt::StrBuf::__wrap).unwrap()
  }
  pub fn packages_iter(&self) -> impl Iterator<Item = &__rt::Str> + '_ {
    self.packages.iter().map(__rt::Str::new)
  }
  pub fn packages_resize(&mut self, n: usize) {
    self.packages.resize(n, Default::default())
  }

  pub fn foreign_types_len(&self) -> usize {
    self.foreign_types.len()
  }
  pub fn foreign_types(&self, n: usize) -> Option<&__rt::Str> {
    self.foreign_types.get(n).map(__rt::Str::new)
  }
  pub fn foreign_types_mut(&mut self, n: usize) -> Option<__rt::StrBuf> {
    self.foreign_types.get_mut(n).map(__rt::StrBuf::__wrap)
  }
  pub fn foreign_types_add(&mut self) -> __rt::StrBuf {
    self.foreign_types.push(Default::default());
    self.foreign_types.last_mut().map(__rt::StrBuf::__wrap).unwrap()
  }
  pub fn foreign_types_iter(&self) -> impl Iterator<Item = &__rt::Str> + '_ {
    self.foreign_types.iter().map(__rt::Str::new)
  }
  pub fn foreign_types_resize(&mut self, n: usize) {
    self.foreign_types.resize(n, Default::default())
  }

}

impl Default for Bundle {
  fn default() -> Self {
    Self::new()
  }
}

/// message `pz.Type`
#[derive(Clone)]
pub struct Type {
  __hasbits: [u32; 1],
  name: Vec<u8>,
  package: u32,
  kind: Type_Kind,
  declared_in: u32,
  fields: Vec<Field>,
  nesteds: Vec<u32>,
  span: u32,
}

impl Type {
  pub const fn new() -> Self {
    Self {
      __hasbits: [0; 1],
      name: Vec::new(),
      package: 0,
      kind: Type_Kind::new(),
      declared_in: 0,
      fields: Vec::new(),
      nesteds: Vec::new(),
      span: 0,
    }
  }

  pub fn name(&self) -> &__rt::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(&self) -> Option<&__rt::Str> {
    if self.__hasbits[0] & 1 != 0 {
      Some(__rt::Str::new(&*self.name))
    } else {
      None
    }
  }
  pub fn name_mut(&mut self) -> __rt::StrBuf {
    self.__hasbits[0] |= 1;
    __rt::StrBuf::__wrap(&mut self.name)
  }
  pub fn name_opt_mut(&mut self) -> Option<__rt::StrBuf> {
    if self.__hasbits[0] & 1 != 0 {
      Some(__rt::StrBuf::__wrap(&mut self.name))
    } else {
      None
    }
  }
  pub fn name_set<'a>(&mut self, value: impl __rt::rt::str::IntoStrOpt<'a>) {
    match value.into_str_opt() {
      Some(value) => {
        self.__hasbits[0] |= 1;
        self.name.clear();
        self.name.extend_from_slice(value.as_bytes())
      }
      None => {
        self.__hasbits[0] &= !1;
        self.name.clear();
      }
    }
  }

  pub fn package(&self) -> u32 {
    self.package_opt().unwrap_or_default()
  }
  pub fn package_opt(&self) -> Option<u32> {
    if self.__hasbits[0] & 2 != 0 {
      Some(self.package)
    } else {
      None
    }
  }
  pub fn package_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.__hasbits[0] |= 2;
        self.package = value;
      }
      None => {
        self.__hasbits[0] &= !2;
      }
    }
  }

  pub fn kind(&self) -> Type_Kind {
    self.kind_opt().unwrap_or_default()
  }
  pub fn kind_opt(&self) -> Option<Type_Kind> {
    if self.__hasbits[0] & 4 != 0 {
      Some(self.kind)
    } else {
      None
    }
  }
  pub fn kind_set(&mut self, value: impl Into<Option<Type_Kind>>) {
    match value.into() {
      Some(value) => {
        self.__hasbits[0] |= 4;
        self.kind = value;
      }
      None => {
        self.__hasbits[0] &= !4;
      }
    }
  }

  pub fn declared_in(&self) -> u32 {
    self.declared_in_opt().unwrap_or_default()
  }
  pub fn declared_in_opt(&self) -> Option<u32> {
    if self.__hasbits[0] & 8 != 0 {
      Some(self.declared_in)
    } else {
      None
    }
  }
  pub fn declared_in_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.__hasbits[0] |= 8;
        self.declared_in = value;
      }
      None => {
        self.__hasbits[0] &= !8;
      }
    }
  }

  pub fn fields_len(&self) -> usize {
    self.fields.len()
  }
  pub fn fields(&self, n: usize) -> Option<&Field> {
    self.fields.get(n)
  }
  pub fn fields_mut(&mut self, n: usize) -> Option<&mut Field> {
    self.fields.get_mut(n)
  }
  pub fn fields_add(&mut self) -> &mut Field {
    self.fields.push(Default::default());
    self.fields.last_mut().unwrap()
  }
  pub fn fields_iter(&self) -> impl Iterator<Item = &Field> + '_ {
    self.fields.iter()
  }
  pub fn fields_resize(&mut self, n: usize) {
    self.fields.resize(n, Default::default())
  }

  pub fn nesteds(&self) -> &[u32] {
    &self.nesteds
  }
  pub fn nesteds_mut(&mut self) -> &mut [u32] {
    &mut self.nesteds
  }
  pub fn nesteds_set(&mut self, that: &[u32]) {
    self.nesteds.clear();
    self.nesteds_extend(that)
  }
  pub fn nesteds_extend(&mut self, that: &[u32]) {
    self.nesteds.extend_from_slice(that)
  }

  pub fn span(&self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(&self) -> Option<u32> {
    if self.__hasbits[0] & 16 != 0 {
      Some(self.span)
    } else {
      None
    }
  }
  pub fn span_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.__hasbits[0] |= 16;
        self.span = value;
      }
      None => {
        self.__hasbits[0] &= !16;
      }
    }
  }

}

impl Default for Type {
  fn default() -> Self {
    Self::new()
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
  __hasbits: [u32; 1],
  name: Vec<u8>,
  number: i32,
  is_repeated: bool,
  r#type: Field_Type,
  type_index: u32,
  span: u32,
}

impl Field {
  pub const fn new() -> Self {
    Self {
      __hasbits: [0; 1],
      name: Vec::new(),
      number: 0,
      is_repeated: false,
      r#type: Field_Type::new(),
      type_index: 0,
      span: 0,
    }
  }

  pub fn name(&self) -> &__rt::Str {
    self.name_opt().unwrap_or_default()
  }
  pub fn name_opt(&self) -> Option<&__rt::Str> {
    if self.__hasbits[0] & 1 != 0 {
      Some(__rt::Str::new(&*self.name))
    } else {
      None
    }
  }
  pub fn name_mut(&mut self) -> __rt::StrBuf {
    self.__hasbits[0] |= 1;
    __rt::StrBuf::__wrap(&mut self.name)
  }
  pub fn name_opt_mut(&mut self) -> Option<__rt::StrBuf> {
    if self.__hasbits[0] & 1 != 0 {
      Some(__rt::StrBuf::__wrap(&mut self.name))
    } else {
      None
    }
  }
  pub fn name_set<'a>(&mut self, value: impl __rt::rt::str::IntoStrOpt<'a>) {
    match value.into_str_opt() {
      Some(value) => {
        self.__hasbits[0] |= 1;
        self.name.clear();
        self.name.extend_from_slice(value.as_bytes())
      }
      None => {
        self.__hasbits[0] &= !1;
        self.name.clear();
      }
    }
  }

  pub fn number(&self) -> i32 {
    self.number_opt().unwrap_or_default()
  }
  pub fn number_opt(&self) -> Option<i32> {
    if self.__hasbits[0] & 2 != 0 {
      Some(self.number)
    } else {
      None
    }
  }
  pub fn number_set(&mut self, value: impl Into<Option<i32>>) {
    match value.into() {
      Some(value) => {
        self.__hasbits[0] |= 2;
        self.number = value;
      }
      None => {
        self.__hasbits[0] &= !2;
      }
    }
  }

  pub fn is_repeated(&self) -> bool {
    self.is_repeated_opt().unwrap_or_default()
  }
  pub fn is_repeated_opt(&self) -> Option<bool> {
    if self.__hasbits[0] & 4 != 0 {
      Some(self.is_repeated)
    } else {
      None
    }
  }
  pub fn is_repeated_set(&mut self, value: impl Into<Option<bool>>) {
    match value.into() {
      Some(value) => {
        self.__hasbits[0] |= 4;
        self.is_repeated = value;
      }
      None => {
        self.__hasbits[0] &= !4;
      }
    }
  }

  pub fn r#type(&self) -> Field_Type {
    self.r#type_opt().unwrap_or_default()
  }
  pub fn r#type_opt(&self) -> Option<Field_Type> {
    if self.__hasbits[0] & 8 != 0 {
      Some(self.r#type)
    } else {
      None
    }
  }
  pub fn r#type_set(&mut self, value: impl Into<Option<Field_Type>>) {
    match value.into() {
      Some(value) => {
        self.__hasbits[0] |= 8;
        self.r#type = value;
      }
      None => {
        self.__hasbits[0] &= !8;
      }
    }
  }

  pub fn type_index(&self) -> u32 {
    self.type_index_opt().unwrap_or_default()
  }
  pub fn type_index_opt(&self) -> Option<u32> {
    if self.__hasbits[0] & 16 != 0 {
      Some(self.type_index)
    } else {
      None
    }
  }
  pub fn type_index_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.__hasbits[0] |= 16;
        self.type_index = value;
      }
      None => {
        self.__hasbits[0] &= !16;
      }
    }
  }

  pub fn span(&self) -> u32 {
    self.span_opt().unwrap_or_default()
  }
  pub fn span_opt(&self) -> Option<u32> {
    if self.__hasbits[0] & 32 != 0 {
      Some(self.span)
    } else {
      None
    }
  }
  pub fn span_set(&mut self, value: impl Into<Option<u32>>) {
    match value.into() {
      Some(value) => {
        self.__hasbits[0] |= 32;
        self.span = value;
      }
      None => {
        self.__hasbits[0] &= !32;
      }
    }
  }

}

impl Default for Field {
  fn default() -> Self {
    Self::new()
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

