/// message `pz.Bundle`
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
      types: todo!(),
      packages: todo!(),
      foreign_types: todo!(),
    }
  }

}

/// message `pz.Type`
pub struct Type {
  __hasbits: [u32; 1],
  name: Vec<u8>,
  package: u32,
  kind: Type_Kind,
  declared_in: u32,
  fields: Vec<Field>,
  nesteds: Vec<u32>,
}

impl Type {
  pub const fn new() -> Self {
    Self {
      __hasbits: [0; 1],
      name: todo!(),
      package: todo!(),
      kind: todo!(),
      declared_in: todo!(),
      fields: todo!(),
      nesteds: todo!(),
    }
  }

  fn package(&self) -> u32 {
    self.package_opt().unwrap_or_default()
  }
  fn package_opt(&self) -> Option<u32> {
    if self.__hasbits[0] & 2 != 0 {
      Some(self.package)
    } else {
      None
    }
  }
  fn package_set(&mut self, value: impl Into<Option<u32>>) {
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

  fn kind(&self) -> Type_Kind {
    self.kind_opt().unwrap_or_default()
  }
  fn kind_opt(&self) -> Option<Type_Kind> {
    if self.__hasbits[0] & 4 != 0 {
      Some(self.kind)
    } else {
      None
    }
  }
  fn kind_set(&mut self, value: impl Into<Option<Type_Kind>>) {
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

  fn declared_in(&self) -> u32 {
    self.declared_in_opt().unwrap_or_default()
  }
  fn declared_in_opt(&self) -> Option<u32> {
    if self.__hasbits[0] & 8 != 0 {
      Some(self.declared_in)
    } else {
      None
    }
  }
  fn declared_in_set(&mut self, value: impl Into<Option<u32>>) {
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

  fn nesteds(&self) -> &[Vec<u32>] {
    &self.nesteds
  }
  fn nesteds_mut(&mut self) -> &mut [Vec<u32>] {
    &mut self.nesteds
  }
  fn nesteds_set(&mut self, that: &[Vec<u32>]) {
    self.nesteds.clear();
    self.nesteds_extend(that)
  }
  fn nesteds_extend(&mut self, that: &[Vec<u32>]) {
    self.nesteds.extend_from_slice(that)
  }

}

/// enum `pz.Type.Kind`
pub struct Type_Kind(pub i32);

impl Type_Kind {
  pub const MESSAGE: Self = Self(0);
  pub const STRUCT: Self = Self(1);
  pub const CHOICE: Self = Self(2);
  pub const ENUM: Self = Self(3);
}

impl Default for Type_Kind {
  fn default() -> Self {
    Self(0)
  }
}

/// message `pz.Field`
pub struct Field {
  __hasbits: [u32; 1],
  name: Vec<u8>,
  number: i32,
  is_repeated: bool,
  r#type: Field_Type,
  type_index: u32,
}

impl Field {
  pub const fn new() -> Self {
    Self {
      __hasbits: [0; 1],
      name: todo!(),
      number: todo!(),
      is_repeated: todo!(),
      r#type: todo!(),
      type_index: todo!(),
    }
  }

  fn number(&self) -> i32 {
    self.number_opt().unwrap_or_default()
  }
  fn number_opt(&self) -> Option<i32> {
    if self.__hasbits[0] & 2 != 0 {
      Some(self.number)
    } else {
      None
    }
  }
  fn number_set(&mut self, value: impl Into<Option<i32>>) {
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

  fn is_repeated(&self) -> bool {
    self.is_repeated_opt().unwrap_or_default()
  }
  fn is_repeated_opt(&self) -> Option<bool> {
    if self.__hasbits[0] & 4 != 0 {
      Some(self.is_repeated)
    } else {
      None
    }
  }
  fn is_repeated_set(&mut self, value: impl Into<Option<bool>>) {
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

  fn r#type(&self) -> Field_Type {
    self.r#type_opt().unwrap_or_default()
  }
  fn r#type_opt(&self) -> Option<Field_Type> {
    if self.__hasbits[0] & 8 != 0 {
      Some(self.r#type)
    } else {
      None
    }
  }
  fn r#type_set(&mut self, value: impl Into<Option<Field_Type>>) {
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

  fn type_index(&self) -> u32 {
    self.type_index_opt().unwrap_or_default()
  }
  fn type_index_opt(&self) -> Option<u32> {
    if self.__hasbits[0] & 16 != 0 {
      Some(self.type_index)
    } else {
      None
    }
  }
  fn type_index_set(&mut self, value: impl Into<Option<u32>>) {
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

}

/// enum `pz.Field.Type`
pub struct Field_Type(pub i32);

impl Field_Type {
  pub const NONE: Self = Self(0);
  pub const I32: Self = Self(1);
  pub const U32: Self = Self(2);
  pub const F32: Self = Self(3);
  pub const I64: Self = Self(4);
  pub const U64: Self = Self(5);
  pub const F64: Self = Self(6);
  pub const BOOL: Self = Self(7);
  pub const STRING: Self = Self(8);
  pub const TYPE: Self = Self(9);
  pub const FOREIGN: Self = Self(10);
}

impl Default for Field_Type {
  fn default() -> Self {
    Self(0)
  }
}

