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
        pub fn $name($self) -> __rt::View<'$lt, __rt::Str> {
          self.${name}_or().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_or($self) -> __s::option::Option<__rt::View<'$lt, __rt::Str>> {
          unsafe {
            let field = $Type::__tdp_info().field($idx);
            if !field.has(self.ptr.as_ptr()) { return __s::option::Option::None }
            __s::option::Option::Some(field.make_view::<__rt::Str>(self.ptr.as_ptr()))
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
        pub fn ${name}_mut($self) -> __rt::Mut<'$lt, __rt::Str> {
          self.${name}_mut_or().into_mut()
        }
        $deprecated
        pub fn ${name}_mut_or($self) -> __rt::value::OptMut<'$lt, __rt::Str> {
          unsafe {
            __rt::value::OptMut::__wrap(
              self.ptr.as_ptr(),
              self.arena,
              $Type::__tdp_info().field($idx),
            )
          }
        }
        $deprecated
        pub fn ${name}_set($self, value: &(impl __s::convert::AsRef<[u8]> + ?__s::marker::Sized)) {
          self.${name}_mut().set(value);
        }
      ",
    );
  }

  fn in_debug(&self, _: Field, w: &mut SourceWriter) {
    w.emit(
      vars! {},
      r#"
        if let __s::option::Option::Some(value) = self.${name}_or() {
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
        pub fn $name($self) -> __rt::Slice<'$lt, __rt::Str> {
          unsafe {
            let field = $Type::__tdp_info().field($idx);
            if !field.has(self.ptr.as_ptr()) { return __rt::Slice::default() }
            field.make_slice::<__rt::Str>(self.ptr.as_ptr())
          }
        }
        $deprecated
        pub fn ${name}_at($self, idx: usize) -> __rt::View<'$lt, __rt::Str> {
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
        pub fn ${name}_mut($self) -> __rt::Repeated<'$lt, __rt::Str> {
          unsafe {
            let field = $Type::__tdp_info().field($idx);
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
