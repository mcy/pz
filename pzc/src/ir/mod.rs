//! Intermediate representation for semantic analysis.

use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

use bumpalo::collections::Vec as AVec;

use pz::proto;

use crate::syn;

mod resolve;
mod to_proto;

pub use resolve::ResolveCtx;

pub struct Bundle<'ast, 'rcx> {
  types: RefCell<Vec<&'rcx Type<'ast, 'rcx>>>,
}

impl Bundle<'_, '_> {
  pub fn to_proto(
    &self,
    icx: &ilex::Context,
  ) -> (proto::Bundle, HashMap<u32, ilex::Span>) {
    to_proto::to_proto(self, icx)
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

pub struct Type<'ast, 'rcx> {
  name: Cell<TypeName<'rcx>>,
  kind: Cell<syn::DeclKind>,
  decl: Option<&'ast syn::Decl<'ast>>,
  fields: RefCell<AVec<'rcx, Field<'ast, 'rcx>>>,
  nesteds: RefCell<AVec<'rcx, &'rcx Type<'ast, 'rcx>>>,
  parent: Cell<Option<&'rcx Type<'ast, 'rcx>>>,
  attrs: proto::r#type::Attrs,
}

impl<'ast, 'rcx> Type<'ast, 'rcx> {
  pub fn name(&self) -> TypeName<'rcx> {
    self.name.get()
  }

  pub fn kind(&self) -> syn::DeclKind {
    self.kind.get()
  }

  pub fn decl(&self) -> Option<&'ast syn::Decl<'ast>> {
    self.decl
  }

  pub fn parent(&self) -> Option<&'rcx Type<'ast, 'rcx>> {
    self.parent.get()
  }

  pub fn field<R>(
    &self,
    index: usize,
    body: impl FnOnce(&Field<'ast, 'rcx>) -> R,
  ) -> Option<R> {
    self.fields.borrow().get(index).map(body)
  }

  pub fn fields<R>(&self, body: impl FnOnce(&[Field<'ast, 'rcx>]) -> R) -> R {
    body(&self.fields.borrow())
  }

  pub fn nested<R>(
    &self,
    index: usize,
    body: impl FnOnce(&'rcx Type<'ast, 'rcx>) -> R,
  ) -> Option<R> {
    self.nesteds.borrow().get(index).map(|x| body(x))
  }

  pub fn nesteds<R>(
    &self,
    body: impl FnOnce(&[&'rcx Type<'ast, 'rcx>]) -> R,
  ) -> R {
    body(&self.nesteds.borrow())
  }
}

pub struct Field<'ast, 'rcx> {
  name: Cell<&'rcx str>,
  parent: &'rcx Type<'ast, 'rcx>,

  decl: Option<&'ast syn::Field<'ast>>,
  ty: Cell<Option<FieldType<'ast, 'rcx>>>,
  number: Cell<Option<i32>>,
  attrs: proto::field::Attrs,
}

impl<'ast, 'rcx> Field<'ast, 'rcx> {
  pub fn name(&self) -> &'rcx str {
    self.name.get()
  }

  pub fn decl(&self) -> Option<&'ast syn::Field<'ast>> {
    self.decl
  }

  pub fn ty(&self) -> Option<FieldType<'ast, 'rcx>> {
    self.ty.get()
  }

  pub fn number(&self) -> Option<i32> {
    self.number.get()
  }

  pub fn parent(&self) -> &'rcx Type<'ast, 'rcx> {
    self.parent
  }
}

#[derive(Copy, Clone)]
pub struct FieldType<'ast, 'rcx> {
  is_repeated: bool,
  kind: FieldTypeKind<'ast, 'rcx>,
}

impl<'ast, 'rcx> FieldType<'ast, 'rcx> {
  pub fn is_repeated(&self) -> bool {
    self.is_repeated
  }

  pub fn kind(&self) -> FieldTypeKind<'ast, 'rcx> {
    self.kind
  }
}

#[derive(Copy, Clone)]
pub enum FieldTypeKind<'ast, 'rcx> {
  I32,
  U32,
  F32,
  I64,
  U64,
  F64,
  Bool,
  String,
  Type(&'rcx Type<'ast, 'rcx>),
}
