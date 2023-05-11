//! Field codegen (accessors, storage, etc).

use std::fmt;

use pz::proto::field::Type as TypeEnum;
use pz::proto::r#type::Kind;

use crate::emit;
use crate::emit::SourceWriter;
use crate::rust::names::ident;
use crate::rust::names::type_name;
use crate::Field;
use crate::Type;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Where {
  TypeImpl,
  ViewImpl,
  MutImpl,
}

pub struct GenField<'ccx> {
  gen: Box<dyn GenFieldImpl + 'ccx>,
  pub field: Field<'ccx>,
  pub hasbit: Option<u32>,
}

impl<'ccx> GenField<'ccx> {
  fn common_vars(
    &self,
    w: &mut SourceWriter,
    cb: impl FnOnce(&mut SourceWriter),
  ) {
    w.with_vars(
      vars! {
        name: ident(self.field.name()),
        Name: ident(heck::AsPascalCase(self.field.name())),
        raw_name: self.field.name(),
        field: |w| {
          if self.field.parent().kind() == Kind::Choice {
            w.emit(vars! {}, "unsafe { &self.ptr.as_ref().union.$name }")
          } else {
            w.emit(vars! {}, "unsafe { &self.ptr.as_ref().$name }")
          }
        },
        field_mut: |w| {
          if self.field.parent().kind() == Kind::Choice {
            w.emit(vars! {}, "unsafe { &mut self.ptr.as_mut().union.$name }")
          } else {
            w.emit(vars! {}, "unsafe { &mut self.ptr.as_mut().$name }")
          }
        },
      },
      cb,
    )
  }

  pub fn in_storage(&self, w: &mut SourceWriter) {
    self.common_vars(w, |w| self.gen.in_storage(self.field, w));
  }
  pub fn in_variants(&self, w: &mut SourceWriter) {
    self.common_vars(w, |w| self.gen.in_variants(self.field, w));
  }
  pub fn in_storage_init(&self, w: &mut SourceWriter) {
    self.common_vars(w, |w| self.gen.in_storage_init(self.field, w));
  }
  pub fn in_ref_methods(&self, at: Where, w: &mut SourceWriter) {
    self.common_vars(w, |w| self.gen.in_ref_methods(self.field, at, w));
  }
  pub fn in_mut_methods(&self, at: Where, w: &mut SourceWriter) {
    self.common_vars(w, |w| self.gen.in_mut_methods(self.field, at, w))
  }
  pub fn in_debug(&self, w: &mut SourceWriter) {
    self.common_vars(w, |w| self.gen.in_debug(self.field, w))
  }
}

#[allow(unused_variables)]
trait GenFieldImpl {
  fn in_storage(&self, field: Field, w: &mut SourceWriter) {}
  fn in_variants(&self, field: Field, w: &mut SourceWriter) {}
  fn in_storage_init(&self, field: Field, w: &mut SourceWriter) {}
  fn in_ref_methods(&self, field: Field, at: Where, w: &mut SourceWriter) {}
  fn in_adt_ref_methods(&self, field: Field, w: &mut SourceWriter) {}
  fn in_mut_methods(&self, field: Field, at: Where, w: &mut SourceWriter) {}
  fn in_debug(&self, field: Field, w: &mut SourceWriter) {}
}

pub struct FieldGenerators<'ccx> {
  pub num_hasbits: u32,
  pub fields: Vec<GenField<'ccx>>,
}

impl<'ccx> FieldGenerators<'ccx> {
  pub fn build(fields: impl IntoIterator<Item = Field<'ccx>>) -> Self {
    let mut generators = Self {
      num_hasbits: 0,
      fields: Vec::new(),
    };

    for field in fields {
      let gen: Box<dyn GenFieldImpl> = match (field.ty(), field.is_repeated()) {
        ((TypeEnum::String, _), false) => Box::new(SingularString),
        ((TypeEnum::String, _), true) => Box::new(RepeatedString),
        ((TypeEnum::Type, Some(submsg)), false) => match submsg.kind() {
          Kind::Message | Kind::Choice => Box::new(SingularMessage { submsg }),
          Kind::Enum => Box::new(SingularScalar),
          _ => {
            field
              .ccx()
              .warn("sorry: cannot generate code for this kind of field yet")
              .at(field.span().unwrap());
            continue;
          }
        },
        ((TypeEnum::Type, Some(submsg)), true) => match submsg.kind() {
          Kind::Message | Kind::Choice => Box::new(RepeatedMessage { submsg }),
          Kind::Enum => Box::new(RepeatedScalar),
          _ => {
            field
              .ccx()
              .warn("sorry: cannot generate code for this kind of field yet")
              .at(field.span().unwrap());
            continue;
          }
        },
        ((TypeEnum::Foreign, _), _) => unreachable!(),
        (_, false) => Box::new(SingularScalar),
        (_, true) => Box::new(RepeatedScalar),
      };

      let needs_hasbit =
        field.parent().kind() != Kind::Choice && !field.is_repeated();
      generators.fields.push(GenField {
        gen,
        field,
        hasbit: needs_hasbit.then(|| {
          generators.num_hasbits += 1;
          generators.num_hasbits - 1
        }),
      });
    }

    generators
  }
}

struct SingularScalar;

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

impl GenFieldImpl for SingularScalar {
  fn in_storage(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Storage: scalar_storage_type(field) },
      "
        pub(in super) $name: $Storage,
      ",
    );
  }

  fn in_variants(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Scalar: scalar_type(field) },
      "
        $Name($rt::ptr::Proxy<'proto, $Scalar, Which>),
      ",
    );
  }

  fn in_storage_init(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { default: scalar_default(field) },
      "
        $name: $default,
      ",
    );
  }

  fn in_ref_methods(&self, field: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Scalar: scalar_type(field),
        Storage: scalar_storage_type(field),
        self: if at == Where::TypeImpl { "&self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn $name($self) -> $rt::View<'$lt, $Scalar> {
          self.${name}_or().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_or($self) -> Option<$rt::View<'$lt, $Scalar>> {
          if !unsafe { $Type::__HAZZER_$raw_name.has(self.ptr.as_ptr()) } { return None }
          Some(unsafe { $transmute::<$Storage, $Scalar>(*$field) })
        }
      ",
    );
  }

  fn in_mut_methods(&self, field: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Scalar: scalar_type(field),
        self: if at == Where::TypeImpl { "&mut self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $rt::Mut<'$lt, $Scalar> {
          self.${name}_mut_or().into_mut()
        }
        $deprecated
        pub fn ${name}_mut_or($self) -> $rt::value::OptMut<'$lt, $Scalar> {
          unsafe {
            $rt::value::OptMut::__wrap(
              self.ptr.as_ptr(),
              self.arena,
              $Type::__HAZZER_$raw_name,
            )
          }
        }
        $deprecated
        pub fn ${name}_set($self, value: $Scalar) {
          self.${name}_mut().set(value);
        }
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
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

struct RepeatedScalar;
impl GenFieldImpl for RepeatedScalar {
  fn in_storage(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Storage: scalar_storage_type(field) },
      "
        pub (in super) $name: $z::AVec<$Storage>,
      ",
    );
  }

  fn in_variants(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Scalar: scalar_type(field) },
      "
        $Name($rt::ptr::Proxy<'proto, $rt::ptr::Rep<$Scalar>, Which>),
      ",
    );
  }

  fn in_storage_init(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { name: ident(field.name()) },
      "
        $name: $z::AVec::new(),
      ",
    );
  }

  fn in_ref_methods(&self, field: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Scalar: scalar_type(field),
        Storage: scalar_storage_type(field),
        self: if at == Where::TypeImpl { "&self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn $name($self) -> $rt::Slice<'$lt, $Scalar> {
          if !unsafe { $Type::__HAZZER_$raw_name.has(self.ptr.as_ptr()) } { return $rt::Slice::default() }
          unsafe {
            let vec = $field;
            $rt::Slice::__wrap(vec.as_ptr() as *mut _, vec.len())
          }
        }
        $deprecated
        pub fn ${name}_at($self, idx: usize) -> $rt::View<'$lt, $Scalar> {
          self.$name().at(idx)
        }
      ",
    );
  }

  fn in_mut_methods(&self, field: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Scalar: scalar_type(field),
        Storage: scalar_storage_type(field),
        self: if at == Where::TypeImpl { "&mut self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $rt::Repeated<'$lt, $Scalar> {
          unsafe {
            $Type::__HAZZER_$raw_name.init(self.ptr.as_ptr(), self.arena);
            $rt::Repeated::__wrap(
              $field_mut as *mut _ as *mut u8,
              self.arena,
            )
          }
        }
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
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

struct SingularString;
impl GenFieldImpl for SingularString {
  fn in_storage(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Scalar: scalar_type(field) },
      "
        pub(in super) $name: (*mut u8, usize),
      ",
    );
  }

  fn in_variants(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Storage: scalar_storage_type(field) },
      "
        $Name($rt::ptr::Proxy<'proto, $rt::Str, Which>),
      ",
    );
  }

  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
      "
        $name: (0 as *mut u8, 0),
      ",
    );
  }

  fn in_ref_methods(&self, field: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Scalar: scalar_type(field),
        self: if at == Where::TypeImpl { "&self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn $name($self) -> $rt::View<'$lt, $rt::Str> {
          self.${name}_or().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_or($self) -> Option<$rt::View<'$lt, $rt::Str>> {
          if !unsafe { $Type::__HAZZER_$raw_name.has(self.ptr.as_ptr()) } { return None }
          Some(unsafe {
            let (mut ptr, len) = *$field;
            if ptr.is_null() { ptr = 1 as *mut u8; }
            $rt::Str::from_raw_parts(ptr, len)
          })
        }
      ",
    );
  }

  fn in_mut_methods(&self, field: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Scalar: scalar_type(field),
        self: if at == Where::TypeImpl { "&mut self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
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
              self.ptr.as_ptr(),
              self.arena,
              $Type::__HAZZER_$raw_name,
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

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
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

struct RepeatedString;
impl GenFieldImpl for RepeatedString {
  fn in_storage(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
      "
        pub(crate) $name: $z::AVec<(*mut u8, usize)>,
      ",
    );
  }

  fn in_variants(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
      "
        $Name($rt::ptr::Proxy<'proto, $rt::ptr::Rep<$rt::Str>, Which>),
      ",
    );
  }

  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
      "
        $name: $z::AVec::new(),
      ",
    );
  }

  fn in_ref_methods(&self, _: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        self: if at == Where::TypeImpl { "&self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn $name($self) -> $rt::Slice<'$lt, $rt::Str> {
          if !unsafe { $Type::__HAZZER_$raw_name.has(self.ptr.as_ptr()) } { return $rt::Slice::default() }
          unsafe {
            let vec = $field;
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

  fn in_mut_methods(&self, _: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        self: if at == Where::TypeImpl { "&mut self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $rt::Repeated<'$lt, $rt::Str> {
          unsafe {
            $Type::__HAZZER_$raw_name.init(self.ptr.as_ptr(), self.arena);
            $rt::Repeated::__wrap(
              $field_mut as *mut _ as *mut u8,
              self.arena,
            )
          }
        }
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
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
  submsg: Type<'ccx>,
}

impl GenFieldImpl for SingularMessage<'_> {
  fn in_storage(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Submsg: type_name(self.submsg),
        priv: format!("__priv_{}", type_name(self.submsg)),
      },
      "
        pub(in super) $name: *mut u8,
      ",
    );
  }

  fn in_variants(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Submsg: type_name(self.submsg) },
      "
        $Name($rt::ptr::Proxy<'proto, $Submsg, Which>),
      ",
    );
  }

  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
      "
        $name: 0 as *mut u8,
      ",
    );
  }

  fn in_ref_methods(&self, _: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Submsg: type_name(self.submsg),
        self: if at == Where::TypeImpl { "&self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn $name($self) -> $rt::View<'$lt, $Submsg> {
          self.${name}_or().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_or($self) -> Option<$rt::View<'$lt, $Submsg>> {
          if !unsafe { $Type::__HAZZER_$raw_name.has(self.ptr.as_ptr()) } { return None }
          unsafe { Some(<$Submsg as $rt::value::Type>::__make_view($field_mut as *mut _ as *mut u8)) }
        }
      ",
    );
  }

  fn in_mut_methods(&self, _: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Submsg: type_name(self.submsg),
        self: if at == Where::TypeImpl { "&mut self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
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
              self.ptr.as_ptr(),
              self.arena,
              $Type::__HAZZER_$raw_name,
            )
          }
        }
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
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
}

struct RepeatedMessage<'ccx> {
  submsg: Type<'ccx>,
}

impl GenFieldImpl for RepeatedMessage<'_> {
  fn in_storage(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Submsg: type_name(self.submsg) },
      "
        pub(in super) $name: $z::AVec<*mut u8>,
      ",
    );
  }

  fn in_variants(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Submsg: type_name(self.submsg) },
      "
        $Name($rt::ptr::Proxy<'proto, $rt::ptr::Rep<$Submsg>, Which>),
      ",
    );
  }

  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
      "
        $name: $z::AVec::new(),
      ",
    );
  }

  fn in_ref_methods(&self, _: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Submsg: type_name(self.submsg),
        self: if at == Where::TypeImpl { "&self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn $name($self) -> $rt::Slice<'$lt, $Submsg> {
          if !unsafe { $Type::__HAZZER_$raw_name.has(self.ptr.as_ptr()) } { return $rt::Slice::default() }
          unsafe {
            let vec = $field;
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

  fn in_mut_methods(&self, _: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Submsg: type_name(self.submsg),
        self: if at == Where::TypeImpl { "&mut self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $rt::Repeated<'$lt, $Submsg> {
          unsafe {
            $Type::__HAZZER_$raw_name.init(self.ptr.as_ptr(), self.arena);
            $rt::Repeated::__wrap(
              $field_mut as *mut _ as *mut u8,
              self.arena,
            )
          }
        }
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
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
