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
        pub(in super) $name: $z::RawStr,
      ",
    );
  }

  fn in_variants(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $Name($rt::ptr::Proxy<'proto, $rt::Str, Which>),
      ",
    );
  }

  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $name: (0 as *mut u8, 0),
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
        pub fn $name($self) -> $rt::View<'$lt, $rt::Str> {
          self.${name}_or().unwrap_or_default()
        }
        $deprecated
        pub fn ${name}_or($self) -> Option<$rt::View<'$lt, $rt::Str>> {
          unsafe {
            let field = $Type::__tdp_info().field($idx);
            if field.has(self.ptr.as_ptr()) { return None }
            Some(field.make_view::<$rt::Str>(self.ptr.as_ptr()))
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
        pub fn ${name}_mut($self) -> $rt::Mut<'$lt, $rt::Str> {
          self.${name}_mut_or().into_mut()
        }
        $deprecated
        pub fn ${name}_mut_or($self) -> $rt::value::OptMut<'$lt, $rt::Str> {
          unsafe {
            $rt::value::OptMut::__wrap(
              self.ptr.as_ptr(),
              self.arena,
              $Type::__tdp_info().field($idx),
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

pub struct Repeated;
impl GenFieldImpl for Repeated {
  fn in_storage(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        pub(crate) $name: $z::AVec<(*mut u8, usize)>,
      ",
    );
  }

  fn in_variants(&self, _: Field, w: &mut SourceWriter) {
    w.write(
      "
        $Name($rt::ptr::Proxy<'proto, $rt::ptr::Rep<$rt::Str>, Which>),
      ",
    );
  }

  fn in_storage_init(&self, _: Field, w: &mut SourceWriter) {
    w.write(
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
          unsafe {
            let field = $Type::__tdp_info().field($idx);
            if field.has(self.ptr.as_ptr()) { return $rt::Slice::default() }
            field.make_slice::<$rt::Str>(self.ptr.as_ptr())
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
            let field = $Type::__tdp_info().field($idx);
            field.init(self.ptr.as_ptr(), self.arena);
            field.make_rep::<$rt::Str>(self.ptr.as_ptr(), self.arena)
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
