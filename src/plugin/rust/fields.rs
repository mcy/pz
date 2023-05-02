//! Field codegen (accessors, storage, etc).

use std::fmt;

use crate::plugin::emit;
use crate::plugin::emit::SourceWriter;
use crate::plugin::Field;
use crate::plugin::Type;
use crate::proto::field::Type as TypeEnum;

use crate::plugin::rust::names::ident;
use crate::plugin::rust::names::type_name;
use crate::proto::r#type::Kind;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Where {
  MsgImpl,
  ViewImpl,
  MutImpl,
}

#[allow(unused_variables)]
pub trait GenField {
  fn in_storage(&self, w: &mut SourceWriter) {}
  fn in_storage_init(&self, w: &mut SourceWriter) {}
  fn in_ref_methods(&self, at: Where, w: &mut SourceWriter) {}
  fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {}
  fn in_clear(&self, w: &mut SourceWriter) {}
  fn in_drop(&self, w: &mut SourceWriter) {}
}

pub struct FieldGenerators<'ccx> {
  pub num_hasbits: u32,
  pub fields: Vec<Box<dyn GenField + 'ccx>>,
}

impl<'ccx> FieldGenerators<'ccx> {
  pub fn build(fields: impl IntoIterator<Item = Field<'ccx>>) -> Self {
    let mut generators = Self {
      num_hasbits: 0,
      fields: Vec::new(),
    };

    for field in fields {
      let gen: Box<dyn GenField> = match (field.ty(), field.is_repeated()) {
        ((TypeEnum::String, _), false) => Box::new(SingularString {
          field,
          hasbit_index: generators.num_hasbits,
        }),
        ((TypeEnum::String, _), true) => Box::new(RepeatedString { field }),
        ((TypeEnum::Type, Some(submsg)), false) => match submsg.kind() {
          Kind::Message => Box::new(SingularMessage {
            field,
            submsg,
            hasbit_index: generators.num_hasbits,
          }),
          Kind::Enum => Box::new(SingularScalar {
            field,
            hasbit_index: generators.num_hasbits,
          }),
          _ => {
            field
              .ccx()
              .warn("sorry: cannot generate code for this kind of field yet")
              .at(field.span().unwrap());
            continue;
          }
        },
        ((TypeEnum::Type, Some(submsg)), true) => match submsg.kind() {
          Kind::Message => Box::new(RepeatedMessage { field, submsg }),
          Kind::Enum => Box::new(RepeatedScalar { field }),
          _ => {
            field
              .ccx()
              .warn("sorry: cannot generate code for this kind of field yet")
              .at(field.span().unwrap());
            continue;
          }
        },
        ((TypeEnum::Foreign, _), _) => unreachable!(),
        (_, false) => Box::new(SingularScalar {
          field,
          hasbit_index: generators.num_hasbits,
        }),
        (_, true) => Box::new(RepeatedScalar { field }),
      };

      if !field.is_repeated() {
        generators.num_hasbits += 1;
      }
      generators.fields.push(gen);
    }

    generators
  }
}

struct SingularScalar<'ccx> {
  field: Field<'ccx>,
  hasbit_index: u32,
}

fn scalar_type<'ccx>(field: Field<'ccx>) -> impl fmt::Display + 'ccx {
  emit::display(move |f| match field.ty() {
    (TypeEnum::I32, _) => f.write_str("i32"),
    (TypeEnum::U32, _) => f.write_str("u32"),
    (TypeEnum::F32, _) => f.write_str("f32"),
    (TypeEnum::I64, _) => f.write_str("i64"),
    (TypeEnum::U64, _) => f.write_str("u64"),
    (TypeEnum::F64, _) => f.write_str("f64"),
    (TypeEnum::Bool, _) => f.write_str("bool"),
    (TypeEnum::Type, Some(e)) => write!(f, "{}", type_name(e)),
    (t, _) => panic!("non-scalar type: {t:?}"),
  })
}

fn scalar_default<'ccx>(field: Field<'ccx>) -> impl fmt::Display + 'ccx {
  emit::display(move |f| match field.ty() {
    (TypeEnum::I32, _) => f.write_str("0"),
    (TypeEnum::U32, _) => f.write_str("0"),
    (TypeEnum::F32, _) => f.write_str("0.0"),
    (TypeEnum::I64, _) => f.write_str("0"),
    (TypeEnum::U64, _) => f.write_str("0"),
    (TypeEnum::F64, _) => f.write_str("0.0"),
    (TypeEnum::Bool, _) => f.write_str("false"),
    (TypeEnum::Type, Some(e)) => write!(f, "{}::new()", type_name(e)),
    (t, _) => panic!("non-scalar type: {t:?}"),
  })
}

impl GenField for SingularScalar<'_> {
  fn in_storage(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
      },
      "
        pub(crate) $name: $Type,
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        default: scalar_default(self.field),
      },
      "
        $name: $default,
      ",
    );
  }

  fn in_ref_methods(&self, at: Where, w: &mut SourceWriter) {
    let hasbit_word = self.hasbit_index / 32;
    let hasbit_bit = 1 << (self.hasbit_index % 32);
    w.emit(
      vars! {
        hasbit_word,
        hasbit_bit,
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
        self: if at == Where::MsgImpl { "&self" } else { "self" }
      },
      r"
        $deprecated
        pub fn $name($self) -> $Type {
          self.${name}_opt().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_opt($self) -> Option<$Type> {
          if self.ptr.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          Some(self.ptr.$name)
        }
      ",
    );
  }

  fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {
    let hasbit_word = self.hasbit_index / 32;
    let hasbit_bit = 1 << (self.hasbit_index % 32);
    w.emit(
      vars! {
        hasbit_word,
        hasbit_bit,
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" }
      },
      r"
        $deprecated
        pub fn ${name}_set($self, value: impl Into<Option<$Type>>) {
          match value.into() {
            Some(value) => {
              self.ptr.__hasbits[$hasbit_word] |= $hasbit_bit;
              self.ptr.$name = value;
            }
            None => {
              self.ptr.__hasbits[$hasbit_word] &= !$hasbit_bit;
            }
          }
        }
      ",
    );
  }
}

struct RepeatedScalar<'ccx> {
  field: Field<'ccx>,
}

impl GenField for RepeatedScalar<'_> {
  fn in_storage(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
      },
      "
        pub(crate) $name: Vec<$Type>,
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
      },
      "
        $name: Vec::new(),
      ",
    );
  }

  fn in_ref_methods(&self, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
        self: if at == Where::MsgImpl { "&self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn $name($self) -> &'$lt [$Type] {
          &self.ptr.$name
        }
      ",
    );
  }

  fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> &'$lt mut [$Type] {
          &mut self.ptr.$name
        }
        $deprecated
        pub fn ${name}_set($self, that: &[$Type]) {
          self.ptr.$name.clear();
          self.${name}_extend(that)
        }
        $deprecated
        pub fn ${name}_extend($self, that: &[$Type]) {
          self.ptr.$name.extend_from_slice(that)
        }
      ",
    );
  }
}

struct SingularString<'ccx> {
  field: Field<'ccx>,
  hasbit_index: u32,
}

impl GenField for SingularString<'_> {
  fn in_storage(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
      },
      "
        pub(crate) $name: Vec<u8>,
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! { name: ident(self.field.name()), },
      "
        $name: Vec::new(),
      ",
    );
  }

  fn in_ref_methods(&self, at: Where, w: &mut SourceWriter) {
    let hasbit_word = self.hasbit_index / 32;
    let hasbit_bit = 1 << (self.hasbit_index % 32);
    w.emit(
      vars! {
        hasbit_word,
        hasbit_bit,
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
        self: if at == Where::MsgImpl { "&self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn $name($self) -> &'$lt $rt::Str {
          self.${name}_opt().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_opt($self) -> Option<&'$lt $rt::Str> {
          if self.ptr.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          Some($rt::Str::new(&*self.ptr.$name))
        }
      ",
    );
  }

  fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {
    let hasbit_word = self.hasbit_index / 32;
    let hasbit_bit = 1 << (self.hasbit_index % 32);
    w.emit(
      vars! {
        hasbit_word,
        hasbit_bit,
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $rt::StrBuf<'$lt> {
          self.ptr.__hasbits[$hasbit_word] |= $hasbit_bit;
          $rt::StrBuf::__wrap(&mut self.ptr.$name)
        }
        $deprecated
        pub fn ${name}_opt_mut($self) -> Option<$rt::StrBuf<'$lt>> {
          if self.ptr.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          Some($rt::StrBuf::__wrap(&mut self.ptr.$name))
        }
        $deprecated
        pub fn ${name}_set<'a>($self, value: impl $rt::rt::str::IntoStrOpt<'a>) {
          match value.into_str_opt() {
            Some(value) => {
              self.ptr.__hasbits[$hasbit_word] |= $hasbit_bit;
              self.ptr.$name.clear();
              self.ptr.$name.extend_from_slice(value.as_bytes())
            }
            None => {
              self.ptr.__hasbits[$hasbit_word] &= !$hasbit_bit;
              self.ptr.$name.clear();
            }
          }
        }
      ",
    );
  }

  fn in_clear(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
      },
      "
        self.ptr.$name.clear();
      ",
    );
  }
}

struct RepeatedString<'ccx> {
  field: Field<'ccx>,
}

impl GenField for RepeatedString<'_> {
  fn in_storage(&self, w: &mut SourceWriter) {
    w.emit(
      vars! { name: ident(self.field.name()) },
      "
        pub(crate) $name: Vec<Vec<u8>>,
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! { name: ident(self.field.name()) },
      "
        $name: Vec::new(),
      ",
    );
  }

  fn in_ref_methods(&self, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        self: if at == Where::MsgImpl { "&self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_len($self) -> usize {
          self.ptr.$name.len()
        }
        $deprecated
        pub fn $name($self, n: usize) -> Option<&'$lt $rt::Str> {
          self.ptr.$name.get(n).map($rt::Str::new)
        }
        $deprecated
        pub fn ${name}_iter($self) -> impl Iterator<Item = &'$lt $rt::Str> + '$lt {
          self.ptr.$name.iter().map($rt::Str::new)
        }
      ",
    );
  }

  fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self, n: usize) -> Option<$rt::StrBuf<'$lt>> {
          self.ptr.$name.get_mut(n).map($rt::StrBuf::__wrap)
        }
        $deprecated
        pub fn ${name}_add($self) -> $rt::StrBuf<'$lt> {
          self.ptr.$name.push(Vec::new());
          self.ptr.$name.last_mut().map($rt::StrBuf::__wrap).unwrap()
        }
        $deprecated
        pub fn ${name}_resize($self, n: usize) {
          self.ptr.$name.resize(n, Vec::new())
        }
      ",
    );
  }
  fn in_clear(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
      },
      "
        self.ptr.$name.clear();
      ",
    );
  }
}

struct SingularMessage<'ccx> {
  field: Field<'ccx>,
  submsg: Type<'ccx>,
  hasbit_index: u32,
}

impl GenField for SingularMessage<'_> {
  fn in_storage(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Submsg: type_name(self.submsg),
      },
      "
        pub(crate) $name: Option<$Submsg>,
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! { name: ident(self.field.name()), },
      "
        $name: None,
      ",
    );
  }

  fn in_ref_methods(&self, at: Where, w: &mut SourceWriter) {
    let hasbit_word = self.hasbit_index / 32;
    let hasbit_bit = 1 << (self.hasbit_index % 32);
    w.emit(
      vars! {
        hasbit_word,
        hasbit_bit,
        name: ident(self.field.name()),
        Submsg: type_name(self.submsg),
        self: if at == Where::MsgImpl { "&self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn $name($self) -> $rt::View<'$lt, $Submsg> {
          self.${name}_opt().unwrap_or($Submsg::DEFAULT)
        }
        $deprecated
        pub fn ${name}_opt($self) -> Option<$rt::View<'$lt, $Submsg>> {
          if self.ptr.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          unsafe {
            Some($rt::View {
              ptr: self.ptr.$name.as_ref().unwrap_unchecked()
            })
          }
        }
      ",
    );
  }

  fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {
    let hasbit_word = self.hasbit_index / 32;
    let hasbit_bit = 1 << (self.hasbit_index % 32);
    w.emit(
      vars! {
        hasbit_word,
        hasbit_bit,
        name: ident(self.field.name()),
        Submsg: type_name(self.submsg),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $rt::Mut<'$lt, $Submsg> {
          if self.ptr.$name.is_none() {
            self.ptr.$name = Some($Submsg::new());
          } else if self.ptr.__hasbits[$hasbit_word] & $hasbit_bit == 0 {
            self.ptr.$name.clear();
          }

          self.ptr.__hasbits[$hasbit_word] |= $hasbit_bit;
          unsafe {
            $rt::Mut {
              ptr: self.ptr.$name.as_mut().unwrap_unchecked()
            }
          }
        }
        $deprecated
        pub fn ${name}_opt_mut($self) -> Option<$rt::Mut<'$lt, $Submsg>> {
          if self.ptr.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          unsafe {
            Some($rt::Mut {
              ptr: self.ptr.$name.as_mut().unwrap_unchecked()
            })
          }
        }
        $deprecated
        pub fn ${name}_clear($self) {
          self.ptr.__hasbits[$hasbit_word] &= !$hasbit_bit;
        }
      ",
    );
  }

  fn in_clear(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
      },
      "
        self.ptr.$name.clear();
      ",
    );
  }
}

struct RepeatedMessage<'ccx> {
  field: Field<'ccx>,
  submsg: Type<'ccx>,
}

impl GenField for RepeatedMessage<'_> {
  fn in_storage(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Submsg: type_name(self.submsg),
      },
      "
        pub(crate) $name: Vec<$Submsg>,
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! { name: ident(self.field.name()) },
      "
        $name: Vec::new(),
      ",
    );
  }

  fn in_ref_methods(&self, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Submsg: type_name(self.submsg),
        self: if at == Where::MsgImpl { "&self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_len($self) -> usize {
          self.ptr.$name.len()
        }
        $deprecated
        pub fn $name($self, n: usize) -> Option<$rt::View<'$lt, $Submsg>> {
          self.ptr.$name.get(n).map(|ptr| $rt::View::<$Submsg> { ptr: &ptr.ptr })
        }
        $deprecated
        pub fn ${name}_iter($self) -> impl Iterator<Item = $rt::View<'$lt, $Submsg>> + '$lt {
          self.ptr.$name.iter().map(|ptr| $rt::View::<$Submsg> { ptr: &ptr.ptr })
        }
      ",
    );
  }

  fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Submsg: type_name(self.submsg),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self, n: usize) -> Option<$rt::Mut<'$lt, $Submsg>> {
          self.ptr.$name.get_mut(n).map(|ptr| $rt::Mut::<$Submsg> { ptr: &mut ptr.ptr })
        }
        $deprecated
        pub fn ${name}_add($self) -> $rt::Mut<'$lt, $Submsg> {
          self.ptr.$name.push($Submsg::new());
          self.ptr.$name.last_mut().map(|ptr| $rt::Mut::<$Submsg> { ptr: &mut ptr.ptr }).unwrap()
        }
        $deprecated
        pub fn ${name}_resize($self, n: usize) {
          self.ptr.$name.resize_with(n, $Submsg::new)
        }
      ",
    );
  }
  fn in_clear(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
      },
      "
        self.ptr.$name.clear();
      ",
    );
  }
}
