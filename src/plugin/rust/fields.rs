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
  fn in_debug(&self, w: &mut SourceWriter) {}
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

fn scalar_storage_type<'ccx>(field: Field<'ccx>) -> impl fmt::Display + 'ccx {
  emit::display(move |f| match field.ty() {
    (TypeEnum::I32, _) => f.write_str("u32"),
    (TypeEnum::U32, _) => f.write_str("u32"),
    (TypeEnum::F32, _) => f.write_str("u32"),
    (TypeEnum::I64, _) => f.write_str("u64"),
    (TypeEnum::U64, _) => f.write_str("u64"),
    (TypeEnum::F64, _) => f.write_str("u64"),
    (TypeEnum::Bool, _) => f.write_str("bool"),
    (TypeEnum::Type, _) => f.write_str("u32"),
    (t, _) => panic!("non-scalar type: {t:?}"),
  })
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
    (TypeEnum::F32, _) => f.write_str("0"),
    (TypeEnum::I64, _) => f.write_str("0"),
    (TypeEnum::U64, _) => f.write_str("0"),
    (TypeEnum::F64, _) => f.write_str("0"),
    (TypeEnum::Bool, _) => f.write_str("false"),
    (TypeEnum::Type, Some(e)) => write!(f, "{}::new().0 as u32", type_name(e)),
    (t, _) => panic!("non-scalar type: {t:?}"),
  })
}

impl GenField for SingularScalar<'_> {
  fn in_storage(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Storage: scalar_storage_type(self.field),
      },
      "
        pub(in super) $name: $Storage,
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
        Storage: scalar_storage_type(self.field),
        self: if at == Where::MsgImpl { "&self" } else { "self" }
      },
      r"
        $deprecated
        pub fn $name($self) -> $Type {
          self.${name}_opt().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_opt($self) -> Option<$Type> {
          if unsafe { self.ptr.as_ref() }.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          Some(unsafe { $transmute::<$Storage, $Type>(self.ptr.as_ref().$name) })
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
        Storage: scalar_storage_type(self.field),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" }
      },
      r"
        $deprecated
        pub fn ${name}_set($self, value: impl Into<Option<$Type>>) {
          match value.into() {
            Some(value) => unsafe {
              self.ptr.as_mut().__hasbits[$hasbit_word] |= $hasbit_bit;
              self.ptr.as_mut().$name = $transmute::<$Type, $Storage>(value);
            }
            None => {
              unsafe { self.ptr.as_mut() }.__hasbits[$hasbit_word] &= !$hasbit_bit;
            }
          }
        }
      ",
    );
  }

  fn in_debug(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        raw_name: self.field.name(),
      },
      r#"
        if let Some(value) = self.${name}_opt() {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          debug.write_debug(value);
          count += 1;
        }
      "#,
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
        Storage: scalar_storage_type(self.field),
      },
      "
        pub (in super) $name: $z::AVec<$Storage>,
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
      },
      "
        $name: $z::AVec::new(),
      ",
    );
  }

  fn in_ref_methods(&self, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
        Storage: scalar_storage_type(self.field),
        self: if at == Where::MsgImpl { "&self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn $name($self) -> &'$lt [$Type] {
          unsafe {
            let slice = self.ptr.as_ref().$name.as_slice();
            $transmute::<&'$lt [$Storage], &'$lt [$Type]>(slice)
          }
        }
      ",
    );
  }

  fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Type: scalar_type(self.field),
        Storage: scalar_storage_type(self.field),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> &'$lt mut [$Type] {
          unsafe {
            let slice = self.ptr.as_mut().$name.as_mut_slice();
            $transmute::<&'$lt mut [$Storage], &'$lt mut [$Type]>(slice)
          }
        }
        $deprecated
        pub fn ${name}_set($self, that: &[$Type]) {
          unsafe {
            let vec = &mut self.ptr.as_mut().$name;
            vec.resize(that.len(), self.arena);
            let ptr = vec.as_mut_slice().as_mut_ptr();
            ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
          }
        }
        $deprecated
        pub fn ${name}_extend($self, that: &[$Type]) {
          unsafe {
            let vec = &mut self.ptr.as_mut().$name;
            let old_len = vec.len();
            let new_len = old_len + that.len();
            vec.resize(new_len, self.arena);
            let ptr = vec.as_mut_slice().as_mut_ptr().add(old_len);
            ptr.copy_from_nonoverlapping(that.as_ptr() as *const _, that.len());
          }
        }
      ",
    );
  }

  fn in_debug(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        raw_name: self.field.name(),
      },
      r#"
        let slice = self.$name();
        if !slice.is_empty() {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          debug.iter(slice)?;
          count += 1;
        }
      "#,
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
        pub(in super) $name: (*mut u8, usize),
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! { name: ident(self.field.name()), },
      "
        $name: (0 as *mut u8, 0),
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
          if unsafe { self.ptr.as_ref() }.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          Some(unsafe {
            let (ptr, len) = self.ptr.as_ref().$name;
            $rt::Str::from_raw_parts(ptr, len)
          })
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
          unsafe {
            let mut buf = $rt::StrBuf::__wrap(&mut self.ptr.as_mut().$name, self.arena);
            if self.ptr.as_ref().__hasbits[$hasbit_word] & $hasbit_bit == 0 {
              buf.clear();
            }
            self.ptr.as_mut().__hasbits[$hasbit_word] |= $hasbit_bit;
            buf
          }
        }
        $deprecated
        pub fn ${name}_opt_mut($self) -> Option<$rt::StrBuf<'$lt>> {
          if unsafe { self.ptr.as_ref() }.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          Some(unsafe {
            $rt::StrBuf::__wrap(&mut self.ptr.as_mut().$name, self.arena)
          })
        }
        $deprecated
        pub fn ${name}_set<'a>($self, value: impl $rt::str::IntoStrOpt<'a>) {
          match value.into_str_opt() {
            Some(value) => self.${name}_mut().set(value.as_bytes()),
            None => unsafe {
              self.ptr.as_mut().__hasbits[$hasbit_word] &= !$hasbit_bit;
            }
          }
        }
      ",
    );
  }

  fn in_debug(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        raw_name: self.field.name(),
      },
      r#"
        if let Some(value) = self.${name}_opt() {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          debug.write_debug(value);
          count += 1;
        }
      "#,
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
        pub(crate) $name: $z::AVec<(*mut u8, usize)>,
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! { name: ident(self.field.name()) },
      "
        $name: $z::AVec::new(),
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
          unsafe { self.ptr.as_ref() }.$name.len()
        }
        $deprecated
        pub fn $name($self, n: usize) -> Option<&'$lt $rt::Str> {
          unsafe { self.ptr.as_ref().$name.as_slice() }.get(n).map(|&(p, n)| unsafe {
            $rt::Str::from_raw_parts(p, n)
          })
        }
        $deprecated
        pub fn ${name}_iter($self) -> impl Iterator<Item = &'$lt $rt::Str> + '$lt {
          unsafe { self.ptr.as_ref().$name.as_slice() }.iter().map(|&(p, n)| unsafe {
            $rt::Str::from_raw_parts(p, n)
          })
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
          unsafe { self.ptr.as_mut().$name.as_mut_slice() }.get_mut(n)
            .map(|data| $rt::StrBuf::__wrap(data, self.arena))
        }
        $deprecated
        pub fn ${name}_add($self) -> $rt::StrBuf<'$lt> {
          unsafe {
            let vec = &mut self.ptr.as_mut().$name;
            let new_len = vec.len() + 1;
            vec.resize(new_len, self.arena);
            self.${name}_mut(new_len - 1).unwrap_unchecked()
          }
        }
        $deprecated
        pub fn ${name}_resize($self, n: usize) {
          unsafe {
            self.ptr.as_mut().$name.resize(n, self.arena);
          }
        }
      ",
    );
  }

  fn in_debug(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        raw_name: self.field.name(),
      },
      r#"
        if self.${name}_len() != 0 {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          debug.iter(self.${name}_iter())?;
          count += 1;
        }
      "#,
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
        priv: format!("__priv_{}", type_name(self.submsg)),
      },
      "
        pub(in super) $name: *mut u8,
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! { name: ident(self.field.name()), },
      "
        $name: 0 as *mut u8,
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
          if unsafe { self.ptr.as_ref() }.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          Some($rt::View::<$Submsg> {
            ptr: unsafe { $z::ABox::from_ptr(self.ptr.as_ref().$name) },
            _ph: $PhantomData,
          })
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
          unsafe {
            if self.ptr.as_ref().$name.is_null() {
              self.ptr.as_mut().$name = self.arena.alloc($Submsg::__LAYOUT).as_ptr();
              self.ptr.as_mut().$name.write_bytes(0, $Msg::__LAYOUT.size());
            } else if self.ptr.as_ref().__hasbits[$hasbit_word] & $hasbit_bit == 0 {
              $Submsg::__raw_clear(self.ptr.as_ref().$name);
            }

            unsafe { self.ptr.as_mut() }.__hasbits[$hasbit_word] |= $hasbit_bit;
            $rt::Mut::<$Submsg> {
              ptr: $z::ABox::from_ptr(self.ptr.as_ref().$name),
              _ph: $PhantomData,
              arena: self.arena,
            }
          }
        }
        $deprecated
        pub fn ${name}_opt_mut($self) -> Option<$rt::Mut<'$lt, $Submsg>> {
          if unsafe { self.ptr.as_ref() }.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          unsafe {
            Some($rt::Mut::<$Submsg> {
              ptr: $z::ABox::from_ptr(self.ptr.as_ref().$name),
              _ph: $PhantomData,
              arena: self.arena,
            })
          }
        }
        $deprecated
        pub fn ${name}_clear($self) {
          unsafe { self.ptr.as_mut() }.__hasbits[$hasbit_word] &= !$hasbit_bit;
        }
      ",
    );
  }

  fn in_debug(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        raw_name: self.field.name(),
      },
      r#"
        if let Some(value) = self.${name}_opt() {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          value.__debug(debug)?;
          count += 1;
        }
      "#,
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
        pub(in super) $name: $z::AVec<*mut u8>,
      ",
    );
  }

  fn in_storage_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! { name: ident(self.field.name()) },
      "
        $name: $z::AVec::new(),
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
          unsafe { self.ptr.as_ref() }.$name.len()
        }
        $deprecated
        pub fn $name($self, n: usize) -> Option<$rt::View<'$lt, $Submsg>> {
          unsafe { self.ptr.as_ref().$name.as_slice() }.get(n)
            .map(|&ptr| $rt::View::<$Submsg> {
              ptr: unsafe { $z::ABox::from_ptr(ptr) },
              _ph: $PhantomData,
            })
        }
        $deprecated
        pub fn ${name}_iter($self) -> impl Iterator<Item = $rt::View<'$lt, $Submsg>> + '$lt {
          unsafe { self.ptr.as_ref().$name.as_slice() }.iter()
            .map(|&ptr| $rt::View::<$Submsg> {
              ptr: unsafe { $z::ABox::from_ptr(ptr) },
              _ph: $PhantomData,
            })
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
          unsafe { self.ptr.as_mut().$name.as_mut_slice() }.get_mut(n)
            .map(|&mut ptr| $rt::Mut::<$Submsg> {
              ptr: unsafe { $z::ABox::from_ptr(ptr) },
              _ph: $PhantomData,
              arena: self.arena,
            })
        }
        $deprecated
        pub fn ${name}_add($self) -> $rt::Mut<'$lt, $Submsg> {
          unsafe {
            let vec = &mut self.ptr.as_mut().$name;
            let new_len = vec.len() + 1;
            vec.resize_msg(new_len, self.arena,
              $Submsg::__LAYOUT, $Submsg::__raw_clear);
            self.${name}_mut(new_len - 1).unwrap_unchecked()
          }
        }
        $deprecated
        pub fn ${name}_resize($self, n: usize) {
          unsafe {
            self.ptr.as_mut().$name.resize_msg(
              n, self.arena,
              $Submsg::__LAYOUT, $Submsg::__raw_clear);
          }
        }
      ",
    );
  }

  fn in_debug(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        raw_name: self.field.name(),
      },
      r#"
        for value in self.${name}_iter() {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          value.__debug(debug)?;
          count += 1;
        }
      "#,
    );
  }
}
