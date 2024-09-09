//! Field codegen (accessors, storage, etc).

use pz::proto::field::Type as TypeEnum;
use pz::proto::r#type::Kind;

use crate::emit::SourceWriter;
use crate::rust::names::ident;
use crate::Field;

use super::names;

mod scalar;
mod str;
mod submsg;

#[derive(Copy, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum Where {
  TypeImpl,
  ViewImpl,
  MutImpl,
}

pub struct GenField<'ccx> {
  gen: Box<dyn GenFieldImpl + 'ccx>,
  pub field: Field<'ccx>,
  #[allow(unused)]
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
        name: self.field.name(),
        "#name": ident(self.field.name()),
        Name: ident(heck::AsPascalCase(self.field.name())),
        __name: names::field_name_type_name(self.field),
        number: self.field.number().unwrap_or(0),
        idx: self.field.index(),
        raw_name: self.field.name(),
        Field: |w| match self.field.ty() {
          (TypeEnum::I32, _) => w.write("__s::primitive::i32"),
          (TypeEnum::U32, _) => w.write("__s::primitive::u32"),
          (TypeEnum::F32, _) => w.write("__s::primitive::f32"),
          (TypeEnum::I64, _) => w.write("__s::primitive::i64"),
          (TypeEnum::U64, _) => w.write("__s::primitive::u64"),
          (TypeEnum::F64, _) => w.write("__s::primitive::f64"),
          (TypeEnum::Bool, _) => w.write("__s::primitive::bool"),
          (TypeEnum::String, _) => w.write("__rt::String"),
          (TypeEnum::Type, Some(t)) => w.emit(vars!{ Type: names::type_name(t), }, "$Type"),
          (t, _) => panic!("unsupported type: {t:?}"),
        },
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

  pub fn in_impls(&self, w: &mut SourceWriter) {
    self.common_vars(w, |w| self.gen.in_impls(self.field, w));
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

#[allow(unused)]
trait GenFieldImpl {
  fn in_impls(&self, field: Field, w: &mut SourceWriter) {
    w.emit(
      vars! { How: if field.is_repeated() { "Rep" } else { "Opt" } },
      r#"
        type $__name = __rt::field!($name);
        impl __r::Field<$__name> for $Type {
          type Type = __r::$How<$Field>;
          type Name = $__name;
          const NUMBER: __s::primitive::i32 = $number;
          const INDEX: __s::primitive::usize = $idx;
          const NAME: &'static __s::primitive::str = "$name";
        }
      "#,
    )
  }

  fn in_storage(&self, field: Field, w: &mut SourceWriter) {
    match field.is_repeated() {
      false => w.write(
        "
          pub(in super) ${#name}: <$Field as __z::Type>::__Storage<__z::Seal>,
        ",
      ),
      true => w.write(
        "
          pub(in super) ${#name}: __z::AVec<<$Field as __z::Type>::__Storage<__z::Seal>>,
        ",
      ),
    }
  }

  fn in_variants(&self, field: Field, w: &mut SourceWriter) {
    match field.is_repeated() {
      false => w.write(
        "
          $Name(__r::View<'proto, $Field, Which>),
        ",
      ),
      true => w.write(
        "
          $Name(__r::View<'proto, __r::Rep<$Field>, Which>),
        ",
      ),
    }
  }

  fn in_storage_init(&self, field: Field, w: &mut SourceWriter) {}

  fn in_ref_methods(&self, field: Field, at: Where, w: &mut SourceWriter) {
    match field.is_repeated() {
      false => w.emit(
        vars! {
          self: if at == Where::TypeImpl { "&self" } else { "self" },
          lt: if at == Where::TypeImpl { "_" } else { "proto" },
        },
        r"
          $deprecated
          pub fn ${#name}($self) -> $Ref<'$lt, $Field> {
            self.${name}_or().unwrap_or_default()
          }
          $deprecated
          pub fn ${name}_or($self) -> $Option<$Ref<'$lt, $Field>> {
            self.get($__name{})
          }
        ",
      ),
      true => w.emit(
        vars! {
          self: if at == Where::TypeImpl { "&self" } else { "self" },
          lt: if at == Where::TypeImpl { "_" } else { "proto" },
        },
        r"
          $deprecated
          pub fn ${#name}($self) -> $Slice<'$lt, $Field> {
            self.get($__name{})
          }
          $deprecated
          pub fn ${name}_at($self, idx: usize) -> $Ref<'$lt, $Field> {
            self.$name().at(idx)
          }
        ",
      ),
    }
  }

  fn in_mut_methods(&self, field: Field, at: Where, w: &mut SourceWriter) {
    match field.is_repeated() {
      false => w.emit(
        vars! {
          self: if at == Where::TypeImpl { "&mut self" } else { "mut self" },
          as_mut: if at == Where::TypeImpl { "self.as_mut()" } else { "self" },
          lt: if at == Where::TypeImpl { "_" } else { "proto" },
        },
        r"
          $deprecated
          pub fn ${name}_mut($self) -> $Mut<'$lt, $Field> {
            self.${name}_mut_or().into_inner()
          }
          $deprecated
          pub fn ${name}_mut_or($self) -> __rt::OptMut<'$lt, $Field> {
            self.get_mut($__name{})
          }
          $deprecated
          pub fn set_${name}($self, value: impl __r::Set<__r::Opt<$Field>>) -> __r::Mut<'$lt, $Type> {
            value.apply_to(self.as_mut().${name}_mut_or());
            $as_mut
          }
        ",
      ),
      true => w.emit(
        vars! {
          self: if at == Where::TypeImpl { "&mut self" } else { "mut self" },
          as_mut: if at == Where::TypeImpl { "self.as_mut()" } else { "self" },
          lt: if at == Where::TypeImpl { "_" } else { "proto" },
        },
        r"
          $deprecated
          pub fn ${name}_mut($self) -> $Repeated<'$lt, $Field> {
            self.get_mut($__name{})
          }
          $deprecated
          pub fn set_${name}($self, value: impl __r::Set<__r::Rep<$Field>>) -> __r::Mut<'$lt, $Type> {
            value.apply_to(self.as_mut().${name}_mut());
            $as_mut
          }
        ",
      ),
    }
  }

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
        ((TypeEnum::String, _), false) => Box::new(str::Singular),
        ((TypeEnum::String, _), true) => Box::new(str::Repeated),
        ((TypeEnum::Type, Some(submsg)), false) => match submsg.kind() {
          Kind::Message | Kind::Choice => Box::new(submsg::Singular),
          Kind::Enum => Box::new(scalar::Singular),
          _ => {
            field
              .ccx()
              .warn("sorry: cannot generate code for this kind of field yet")
              .at(field.span().unwrap());
            continue;
          }
        },
        ((TypeEnum::Type, Some(submsg)), true) => match submsg.kind() {
          Kind::Message | Kind::Choice => Box::new(submsg::Repeated),
          Kind::Enum => Box::new(scalar::Repeated),
          _ => {
            field
              .ccx()
              .warn("sorry: cannot generate code for this kind of field yet")
              .at(field.span().unwrap());
            continue;
          }
        },
        ((TypeEnum::Foreign, _), _) => unreachable!(),
        (_, false) => Box::new(scalar::Singular),
        (_, true) => Box::new(scalar::Repeated),
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
