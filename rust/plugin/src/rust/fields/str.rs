// Field codegen for string fields.

use crate::emit::SourceWriter;
use crate::rust::fields::GenFieldImpl;
use crate::rust::fields::Where;
use crate::Field;

pub struct Singular;
impl GenFieldImpl for Singular {
  fn in_storage(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        pub(in super) $name: __z::RawStr,
      ",
    );
  }

  fn in_variants(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $Name(__rt::ptr::Proxy<'proto, __rt::Str, Which>),
      ",
    );
  }

  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $name: __z::RawStr::new(),
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
        pub fn $name($self) -> $View<'$lt, __rt::Str> {
          self.${name}_or().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_or($self) -> $Option<$View<'$lt, __rt::Str>> {
          unsafe {
            let field = $TDP.field($idx);
            if !field.has(self.ptr.as_ptr()) { return $None }
            $Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
          }
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
        pub fn ${name}_mut($self) -> $Mut<'$lt, __rt::Str> {
          self.${name}_mut_or().into_mut()
        }
        $deprecated
        pub fn ${name}_mut_or($self) -> __rt::OptMut<'$lt, __rt::Str> {
          unsafe {
            __rt::OptMut::__wrap(
              self.ptr.as_ptr(),
              self.arena,
              $TDP.field($idx),
            )
          }
        }
        $deprecated
        pub fn ${name}_set($self, value: &(impl $AsRef<[u8]> + ?$Sized)) {
          self.${name}_mut().set(value);
        }
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
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
  fn in_storage(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        pub(crate) $name: __z::AVec<(*mut u8, usize)>,
      ",
    );
  }

  fn in_variants(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $Name(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<__rt::Str>, Which>),
      ",
    );
  }

  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $name: __z::AVec::new(),
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
        pub fn $name($self) -> $Slice<'$lt, __rt::Str> {
          unsafe {
            let field = $TDP.field($idx);
            if !field.has(self.ptr.as_ptr()) { return $Slice::default() }
            field.make_slice::<__rt::Str>(self.ptr.as_ptr())
          }
        }
        $deprecated
        pub fn ${name}_at($self, idx: usize) -> $View<'$lt, __rt::Str> {
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
        pub fn ${name}_mut($self) -> $Repeated<'$lt, __rt::Str> {
          unsafe {
            let field = $TDP.field($idx);
            field.init(self.ptr.as_ptr(), self.arena);
            field.make_rep::<__rt::Str>(self.ptr.as_ptr(), self.arena)
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
