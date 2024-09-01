//! Field codegen (accessors, storage, etc).

use pz::proto::field::Type as TypeEnum;
use pz::proto::r#type::Kind;

use crate::emit::SourceWriter;
use crate::rust::names::ident;
use crate::Field;

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
        name: ident(self.field.name()),
        Name: ident(heck::AsPascalCase(self.field.name())),
        idx: self.field.index(),
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

#[allow(unused)]
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
        ((TypeEnum::String, _), false) => Box::new(str::Singular),
        ((TypeEnum::String, _), true) => Box::new(str::Repeated),
        ((TypeEnum::Type, Some(submsg)), false) => match submsg.kind() {
          Kind::Message | Kind::Choice => Box::new(submsg::Singular { submsg }),
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
          Kind::Message | Kind::Choice => Box::new(submsg::Repeated { submsg }),
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
