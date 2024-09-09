// Field codegen for scalar fields.

use std::fmt;

use crate::emit;
use crate::emit::SourceWriter;
use crate::rust::fields::GenFieldImpl;
use crate::rust::fields::TypeEnum;
use crate::rust::names;
use crate::Field;

fn scalar_default(field: Field) -> impl fmt::Display + '_ {
  emit::display(move |f| match field.ty() {
    (TypeEnum::I32, _) => f.write_str("0"),
    (TypeEnum::U32, _) => f.write_str("0"),
    (TypeEnum::F32, _) => f.write_str("0.0"),
    (TypeEnum::I64, _) => f.write_str("0"),
    (TypeEnum::U64, _) => f.write_str("0"),
    (TypeEnum::F64, _) => f.write_str("0.0"),
    (TypeEnum::Bool, _) => f.write_str("false"),
    (TypeEnum::Type, Some(e)) => {
      write!(f, "{}::new()", names::type_name(e))
    }
    (t, _) => panic!("non-scalar type: {t:?}"),
  })
}

pub struct Singular;
impl GenFieldImpl for Singular {
  fn in_storage_init(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { default: scalar_default(field) },
      "
        ${#name}: $default,
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      r#"
        if let $Some(value) = self.${name}_or() {
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
  fn in_storage_init(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { name: names::ident(field.name()) },
      "
        $name: __z::AVec::new(),
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
