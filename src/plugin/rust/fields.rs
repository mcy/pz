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
  fn in_init(&self, w: &mut SourceWriter) {}
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
        self: if at == Where::MsgImpl { "&self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn $name($self) -> $rt::View<'$lt, $Type> {
          self.${name}_or().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_or($self) -> Option<$rt::View<'$lt, $Type>> {
          if unsafe { self.ptr.as_ref() }.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          Some(unsafe { $transmute::<$Storage, $Type>(self.ptr.as_ref().$name) })
        }
      ",
    );
  }

  fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        raw_name: self.field.name(),
        Type: scalar_type(self.field),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $rt::Mut<'$lt, $Type> {
          self.${name}_mut_or().into_mut()
        }
        $deprecated
        pub fn ${name}_mut_or($self) -> $rt::value::OptMut<'$lt, $Type> {
          unsafe {
            $rt::value::OptMut::__wrap(
              self.ptr.as_ptr().add($priv::FIELD_OFFSET_$raw_name as usize),
              self.arena,
              $Msg::__hazzer_$raw_name,
            )
          }
        }
        $deprecated
        pub fn ${name}_set($self, value: $Type) {
          self.${name}_mut().set(value);
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
        if let Some(value) = self.${name}_or() {
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
        pub fn $name($self) -> $rt::Slice<'$lt, $Type> {
          unsafe {
            let vec = &self.ptr.as_ref().$name;
            $rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
          }
        }
        $deprecated
        pub fn ${name}_at($self, idx: usize) -> $rt::View<'$lt, $Type> {
          self.$name().at(idx)
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
        pub fn ${name}_mut($self) -> $rt::Repeated<'$lt, $Type> {
          unsafe {
            $rt::Repeated::__wrap(
              (&mut self.ptr.as_mut().$name) as *mut _ as *mut u8,
              self.arena,
            )
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
        if !self.$name().is_empty() {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          debug.iter(self.$name())?;
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
        pub fn $name($self) -> $rt::View<'$lt, $rt::Str> {
          self.${name}_or().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_or($self) -> Option<$rt::View<'$lt, $rt::Str>> {
          if unsafe { self.ptr.as_ref() }.__hasbits[$hasbit_word] & $hasbit_bit == 0 { return None }
          Some(unsafe {
            let (mut ptr, len) = self.ptr.as_ref().$name;
            if ptr.is_null() { ptr = 1 as *mut u8; }
            $rt::Str::from_raw_parts(ptr, len)
          })
        }
      ",
    );
  }

  fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        raw_name: self.field.name(),
        Type: scalar_type(self.field),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $rt::Mut<'$lt, $rt::Str> {
          self.${name}_mut_or().into_mut()
        }
        $deprecated
        pub fn ${name}_mut_or($self) -> $rt::value::OptMut<'$lt, $rt::Str> {
          unsafe {
            $rt::value::OptMut::__wrap(
              self.ptr.as_ptr().add($priv::FIELD_OFFSET_$raw_name as usize),
              self.arena,
              $Msg::__hazzer_$raw_name,
            )
          }
        }
        $deprecated
        pub fn ${name}_set($self, value: &(impl std::convert::AsRef<[u8]> + ?Sized)) {
          self.${name}_mut().set(value);
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
        if let Some(value) = self.${name}_or() {
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
        pub fn $name($self) -> $rt::Slice<'$lt, $rt::Str> {
          unsafe {
            let vec = &self.ptr.as_ref().$name;
            $rt::Slice::__wrap(vec.as_ptr(), vec.len())
          }
        }
        $deprecated
        pub fn ${name}_at($self, idx: usize) -> $rt::View<'$lt, $rt::Str> {
          self.$name().at(idx)
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
        pub fn ${name}_mut($self) -> $rt::Repeated<'$lt, $rt::Str> {
          unsafe {
            $rt::Repeated::__wrap(
              (&mut self.ptr.as_mut().$name) as *mut _ as *mut u8,
              self.arena,
            )
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
        if !self.$name().is_empty() {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          debug.iter(self.$name())?;
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
          self.${name}_or().unwrap_or($Submsg::DEFAULT)
        }
        $deprecated
        pub fn ${name}_or($self) -> Option<$rt::View<'$lt, $Submsg>> {
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
    w.emit(
      vars! {
        name: ident(self.field.name()),
        raw_name: self.field.name(),
        Submsg: type_name(self.submsg),
        self: if at == Where::MsgImpl { "&mut self" } else { "self" },
        lt: if at == Where::MsgImpl { "_" } else { "msg" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $rt::Mut<'$lt, $Submsg> {
          self.${name}_mut_or().into_mut()
        }
        $deprecated
        pub fn ${name}_mut_or($self) -> $rt::value::OptMut<'$lt, $Submsg> {
          unsafe {
            $rt::value::OptMut::__wrap(
              self.ptr.as_ptr().add($priv::FIELD_OFFSET_$raw_name as usize),
              self.arena,
              $Msg::__hazzer_$raw_name,
            )
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
        if let Some(value) = self.${name}_or() {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          value.__debug(debug)?;
          count += 1;
        }
      "#,
    );
  }

  fn in_init(&self, w: &mut SourceWriter) {
    w.emit(
      vars! {
        name: ident(self.field.name()),
        Submsg: type_name(self.submsg),
      },
      r"
        let storage = &mut *raw.cast::<$priv::Storage>();
        if storage.$name.is_null() {
          storage.$name = arena.alloc($Submsg::__LAYOUT).as_ptr();
          $Submsg::__raw_init(storage.$name);
        }
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
        pub fn $name($self) -> $rt::Slice<'$lt, $Submsg> {
          unsafe {
            let vec = &self.ptr.as_ref().$name;
            $rt::Slice::__wrap(vec.as_ptr(), vec.len())
          }
        }
        $deprecated
        pub fn ${name}_at($self, idx: usize) -> $rt::View<'$lt, $Submsg> {
          self.$name().at(idx)
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
        pub fn ${name}_mut($self) -> $rt::Repeated<'$lt, $Submsg> {
          unsafe {
            $rt::Repeated::__wrap(
              (&mut self.ptr.as_mut().$name) as *mut _ as *mut u8,
              self.arena,
            )
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
        for value in self.$name() {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          value.__debug(debug)?;
          count += 1;
        }
      "#,
    );
  }
}
