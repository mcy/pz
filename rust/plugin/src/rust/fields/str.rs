// Field codegen for string fields.

use crate::emit::SourceWriter;
use crate::rust::fields::GenFieldImpl;
use crate::Field;

pub struct Singular;
impl GenFieldImpl for Singular {
  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $name: __z::RawStr::new(),
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
