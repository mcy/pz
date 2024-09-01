// Field codegen for scalar fields.

use std::fmt;

use crate::emit;
use crate::emit::SourceWriter;
use crate::rust::fields::GenFieldImpl;
use crate::rust::fields::TypeEnum;
use crate::rust::fields::Where;
use crate::rust::names;
use crate::Field;

fn scalar_storage_type(field: Field) -> impl fmt::Display + '_ {
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

fn scalar_type(field: Field) -> impl fmt::Display + '_ {
  emit::display(move |f| match field.ty() {
    (TypeEnum::I32, _) => f.write_str("i32"),
    (TypeEnum::U32, _) => f.write_str("u32"),
    (TypeEnum::F32, _) => f.write_str("f32"),
    (TypeEnum::I64, _) => f.write_str("i64"),
    (TypeEnum::U64, _) => f.write_str("u64"),
    (TypeEnum::F64, _) => f.write_str("f64"),
    (TypeEnum::Bool, _) => f.write_str("bool"),
    (TypeEnum::Type, Some(e)) => write!(f, "{}", names::type_name(e)),
    (t, _) => panic!("non-scalar type: {t:?}"),
  })
}

fn scalar_default(field: Field) -> impl fmt::Display + '_ {
  emit::display(move |f| match field.ty() {
    (TypeEnum::I32, _) => f.write_str("0"),
    (TypeEnum::U32, _) => f.write_str("0"),
    (TypeEnum::F32, _) => f.write_str("0"),
    (TypeEnum::I64, _) => f.write_str("0"),
    (TypeEnum::U64, _) => f.write_str("0"),
    (TypeEnum::F64, _) => f.write_str("0"),
    (TypeEnum::Bool, _) => f.write_str("false"),
    (TypeEnum::Type, Some(e)) => {
      write!(f, "{}::new().0 as u32", names::type_name(e))
    }
    (t, _) => panic!("non-scalar type: {t:?}"),
  })
}

pub struct Singular;
impl GenFieldImpl for Singular {
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
          unsafe {
            let field = $Type::__tdp_info().field($idx);
            if field.has(self.ptr.as_ptr()) { return None }
            Some(field.make_view::<$Scalar>(self.ptr.as_ptr()))
          }
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
              $Type::__tdp_info().field($idx),
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

pub struct Repeated;
impl GenFieldImpl for Repeated {
  fn in_storage(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Storage: scalar_storage_type(field) },
      "
        pub(in super) $name: $z::AVec<$Storage>,
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
      vars! { name: names::ident(field.name()) },
      "
        $name: $z::AVec::new(),
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
        pub fn $name($self) -> $rt::Slice<'$lt, $Scalar> {
          unsafe {
            let field = $Type::__tdp_info().field($idx);
            if field.has(self.ptr.as_ptr()) { return $rt::Slice::default() }
            field.make_slice::<$Scalar>(self.ptr.as_ptr())
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
        self: if at == Where::TypeImpl { "&mut self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $rt::Repeated<'$lt, $Scalar> {
          unsafe {
            let field = $Type::__tdp_info().field($idx);
            field.init(self.ptr.as_ptr(), self.arena);
            field.make_rep::<$Scalar>(self.ptr.as_ptr(), self.arena)
          }
        }
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.write(
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
