// Field codegen for submessage fields.

use crate::emit::SourceWriter;
use crate::rust::fields::GenFieldImpl;
use crate::Field;

pub struct Singular;
impl GenFieldImpl for Singular {
  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $name: $None,
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      r#"
        if let $Some(value) = self.${name}_or() {
          if count != 0 { debug.comma(false)?; }
          debug.field("$raw_name")?;
          value.__debug(debug)?;
          count += 1;
        }
      "#,
    );
  }
}

pub struct Repeated;
impl GenFieldImpl for Repeated {
  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $name: __z::AVec::new(),
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.write(
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
