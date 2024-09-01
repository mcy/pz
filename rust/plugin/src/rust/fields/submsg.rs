// Field codegen for submessage fields.

use crate::emit::SourceWriter;
use crate::rust::fields::GenFieldImpl;
use crate::rust::fields::Where;
use crate::rust::names;
use crate::Field;
use crate::Type;

pub struct Singular<'ccx> {
  pub submsg: Type<'ccx>,
}

impl GenFieldImpl for Singular<'_> {
  fn in_storage(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        pub(in super) $name: *mut u8,
      ",
    );
  }

  fn in_variants(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Submsg: names::type_name(self.submsg) },
      "
        $Name(__rt::ptr::Proxy<'proto, $Submsg, Which>),
      ",
    );
  }

  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $name: 0 as *mut u8,
      ",
    );
  }

  fn in_ref_methods(&self, _: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Submsg: names::type_name(self.submsg),
        self: if at == Where::TypeImpl { "&self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn $name($self) -> $View<'$lt, $Submsg> {
          self.${name}_or().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_or($self) -> $Option<$View<'$lt, $Submsg>> {
          unsafe {
            let field = $TDP.field($idx);
            if !field.has(self.ptr.as_ptr()) { return $None }
            $Some(field.make_view::<$Submsg>(self.ptr.as_ptr()))
          }
        }
      ",
    );
  }

  fn in_mut_methods(&self, _: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Submsg: names::type_name(self.submsg),
        self: if at == Where::TypeImpl { "&mut self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $Mut<'$lt, $Submsg> {
          self.${name}_mut_or().into_mut()
        }
        $deprecated
        pub fn ${name}_mut_or($self) -> __rt::OptMut<'$lt, $Submsg> {
          unsafe {
            __rt::OptMut::__wrap(
              self.ptr.as_ptr(),
              self.arena,
              $TDP.field($idx),
            )
          }
        }
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

pub struct Repeated<'ccx> {
  pub submsg: Type<'ccx>,
}

impl GenFieldImpl for Repeated<'_> {
  fn in_storage(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        pub(in super) $name: __z::AVec<*mut u8>,
      ",
    );
  }

  fn in_variants(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { Submsg: names::type_name(self.submsg) },
      "
        $Name(__rt::ptr::Proxy<'proto, __rt::ptr::Rep<$Submsg>, Which>),
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
        Submsg: names::type_name(self.submsg),
        self: if at == Where::TypeImpl { "&self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn $name($self) -> $Slice<'$lt, $Submsg> {
          unsafe {
            let field = $TDP.field($idx);
            if !field.has(self.ptr.as_ptr()) { return $Slice::default() }
            field.make_slice::<$Submsg>(self.ptr.as_ptr())
          }
        }
        $deprecated
        pub fn ${name}_at($self, idx: usize) -> $View<'$lt, $Submsg> {
          self.$name().at(idx)
        }
      ",
    );
  }

  fn in_mut_methods(&self, _: Field, at: Where, w: &mut SourceWriter) {
    w.emit(
      vars! {
        Submsg: names::type_name(self.submsg),
        self: if at == Where::TypeImpl { "&mut self" } else { "self" },
        lt: if at == Where::TypeImpl { "_" } else { "proto" },
      },
      r"
        $deprecated
        pub fn ${name}_mut($self) -> $Repeated<'$lt, $Submsg> {
          unsafe {
            let field = $TDP.field($idx);
            field.init(self.ptr.as_ptr(), self.arena);
            field.make_rep::<$Submsg>(self.ptr.as_ptr(), self.arena)
          }
        }
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
