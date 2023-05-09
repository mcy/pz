//! Intermediate representation for semantic analysis.

use std::cell::Cell;
use std::cell::RefCell;
use std::fmt;

use bumpalo::collections::Vec as AVec;

use pz::proto;

use crate::syn;

mod resolve;
mod to_proto;

pub use resolve::ResolveCtx;

pub struct Bundle<'syn, 'rcx> {
  types: RefCell<Vec<&'rcx Type<'syn, 'rcx>>>,
}

impl Bundle<'_, '_> {
  pub fn to_proto(&self) -> proto::Bundle {
    to_proto::to_proto(self)
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct TypeName<'rcx> {
  pub package: &'rcx str,
  pub name: &'rcx str,
}

impl fmt::Display for TypeName<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.package.is_empty() {
      f.write_str(self.name)
    } else {
      write!(f, "{}.{}", self.package, self.name)
    }
  }
}

pub struct Type<'syn, 'rcx> {
  name: Cell<TypeName<'rcx>>,
  kind: Cell<syn::DeclKind>,
  decl: Option<&'syn syn::Decl>,
  fields: RefCell<AVec<'rcx, Field<'syn, 'rcx>>>,
  nesteds: RefCell<AVec<'rcx, &'rcx Type<'syn, 'rcx>>>,
  parent: Cell<Option<&'rcx Type<'syn, 'rcx>>>,
  attrs: proto::r#type::Attrs,
}

impl<'syn, 'rcx> Type<'syn, 'rcx> {
  pub fn name(&self) -> TypeName<'rcx> {
    self.name.get()
  }

  pub fn kind(&self) -> syn::DeclKind {
    self.kind.get()
  }

  pub fn decl(&self) -> Option<&'syn syn::Decl> {
    self.decl
  }

  pub fn parent(&self) -> Option<&'rcx Type<'syn, 'rcx>> {
    self.parent.get()
  }

  pub fn field<R>(
    &self,
    index: usize,
    body: impl FnOnce(&Field<'syn, 'rcx>) -> R,
  ) -> Option<R> {
    self.fields.borrow().get(index).map(|x| body(&x))
  }

  pub fn fields<R>(&self, body: impl FnOnce(&[Field<'syn, 'rcx>]) -> R) -> R {
    body(&self.fields.borrow())
  }

  pub fn nested<R>(
    &self,
    index: usize,
    body: impl FnOnce(&Type<'syn, 'rcx>) -> R,
  ) -> Option<R> {
    self.nesteds.borrow().get(index).map(|x| body(&x))
  }

  pub fn nesteds<R>(
    &self,
    body: impl FnOnce(&[&'rcx Type<'syn, 'rcx>]) -> R,
  ) -> R {
    body(&self.nesteds.borrow())
  }
}

pub struct Field<'syn, 'rcx> {
  name: Cell<&'rcx str>,
  parent: &'rcx Type<'syn, 'rcx>,

  decl: Option<&'syn syn::Field>,
  ty: Cell<Option<FieldType<'syn, 'rcx>>>,
  number: Cell<Option<i32>>,
  attrs: proto::field::Attrs,
}

impl<'syn, 'rcx> Field<'syn, 'rcx> {
  pub fn name(&self) -> &'rcx str {
    self.name.get()
  }

  pub fn decl(&self) -> Option<&'syn syn::Field> {
    self.decl
  }

  pub fn ty(&self) -> Option<FieldType<'syn, 'rcx>> {
    self.ty.get()
  }

  pub fn number(&self) -> Option<i32> {
    self.number.get()
  }

  pub fn parent(&self) -> &'rcx Type<'syn, 'rcx> {
    self.parent
  }
}

#[derive(Copy, Clone)]
pub struct FieldType<'syn, 'rcx> {
  is_repeated: bool,
  kind: FieldTypeKind<'syn, 'rcx>,
}

impl<'syn, 'rcx> FieldType<'syn, 'rcx> {
  pub fn is_repeated(&self) -> bool {
    self.is_repeated
  }

  pub fn kind(&self) -> FieldTypeKind<'syn, 'rcx> {
    self.kind
  }
}

#[derive(Copy, Clone)]
pub enum FieldTypeKind<'syn, 'rcx> {
  I32,
  U32,
  F32,
  I64,
  U64,
  F64,
  Bool,
  String,
  Type(&'rcx Type<'syn, 'rcx>),
}
