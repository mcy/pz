//! The resolver (primarily name resolution, among other things).

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;

use crate::ir;
use crate::report::Report;
use crate::syn;
use crate::syn::Spanned;

use bumpalo::collections::String as AString;
use bumpalo::collections::Vec as AVec;

pub struct ResolveCtx<'ast, 'scx> {
  arena: bumpalo::Bump,
  scx: &'scx syn::SourceCtx,
  _ph: PhantomData<&'ast syn::PzFile>,

  types: RefCell<
    // Let 'rcx represent the lifetime of `arena`. Then, the "real" type of this
    // map is `HashMap<String, &'rcx Type<'ast, 'rcx>>`. We're forced to erase
    // both lifetimes, because ir::Type requires 'ast: 'rcx, because there are
    // 'rcx references that point to 'ast references.
    HashMap<String, (*const ir::Type<'static, 'static>, Option<syn::Span>)>,
  >,
}

impl<'ast, 'scx: 'ast> ResolveCtx<'ast, 'scx> {
  pub fn new(scx: &'scx syn::SourceCtx) -> Self {
    Self {
      arena: bumpalo::Bump::new(),
      scx,
      types: Default::default(),
      _ph: PhantomData,
    }
  }

  pub fn type_by_name(&self, name: &str) -> Option<&ir::Type<'ast, '_>> {
    let (ptr, _) = self.types.borrow().get(name).copied()?;
    unsafe {
      // SAFETY: this pointer is allocated on `self.arena`.
      Some(&*ptr.cast::<ir::Type<'ast, '_>>())
    }
  }

  pub fn resolve<'rcx>(
    &'rcx self,
    file: &'ast syn::PzFile,
    report: &mut Report,
  ) -> ir::Bundle<'ast, 'rcx> {
    let mut bundle = ir::Bundle::<'ast, 'rcx> {
      types: Vec::new().into(),
    };

    let mut package = AString::new_in(&self.arena);
    for (i, name) in file.package.path.components.iter().enumerate() {
      let text = name.name(self.scx);
      if i != 0 {
        package.push('.');
      }
      package.push_str(text);
    }
    let package = package.into_bump_str();

    // First, un-nest all the declarations.
    let mut roots = AVec::with_capacity_in(file.items.len(), &self.arena);
    for item in &file.items {
      match item {
        syn::Item::Decl(decl) => {
          self.extract_types(
            &mut bundle,
            package,
            None,
            decl,
            &mut roots,
            report,
          );
        }
        _ => unreachable!(),
      }
    }

    // Resolve the types of all the fields.
    for &ty in &*bundle.types.borrow() {
      let decl = ty.decl().unwrap();

      let field_count = decl
        .items
        .iter()
        .filter(|i| matches!(i, syn::Item::Field(..)))
        .count();

      let mut fields = ty.fields.borrow_mut();
      fields.reserve(field_count);

      let mut numbers = HashMap::<i32, syn::Span>::new();
      for field in &decl.items {
        let syn::Item::Field(field) = field else { continue };
        let field_ty = field.ty.as_ref().and_then(|t| {
          let kind = match &t.kind {
            syn::TypeKind::I32 => ir::FieldTypeKind::I32,
            syn::TypeKind::U32 => ir::FieldTypeKind::U32,
            syn::TypeKind::F32 => ir::FieldTypeKind::F32,
            syn::TypeKind::I64 => ir::FieldTypeKind::I64,
            syn::TypeKind::U64 => ir::FieldTypeKind::U64,
            syn::TypeKind::F64 => ir::FieldTypeKind::F64,
            syn::TypeKind::Bool => ir::FieldTypeKind::Bool,
            syn::TypeKind::String => ir::FieldTypeKind::String,

            syn::TypeKind::Path(path) => {
              assert!(path.components.len() > 0);

              // Three cases:
              // - A single-element path referring to an item in this or a
              //   parent's body.
              // - A multi-element, fully-qualified path that resolves to a type
              //   we know.
              // - A multi-element, fully-qualified path that we can't resolve
              //   yet.
              if let [name] = path.components.as_slice() {
                let name = name.name(self.scx);
                let ty = self.resolve_relative_type(name, &roots, ty);
                if ty.is_none() {
                  report
                    .error(format_args!("could not resolve type `{name}`"))
                    .at(path);
                }

                ty.map(ir::FieldTypeKind::Type)?
              } else {
                let name = path.join(self.scx);
                match self.type_by_name(&name) {
                  Some(ty) => ir::FieldTypeKind::Type(ty),
                  None => {
                    ir::FieldTypeKind::Unresolved(self.arena.alloc_str(&name))
                  }
                }
              }
            }
          };

          Some(ir::FieldType {
            is_repeated: t.repeated.is_some(),
            kind,
          })
        });

        let field_number = field.number.and_then(|lit| {
          let number: Option<i32> = lit.value().and_then(|v| v.try_into().ok());

          if decl.kind == syn::DeclKind::Enum {
            if number.is_none() {
              report.error("enum value out of range").at(lit).note(
                format_args!(
                  "enum values signed, 32-bit integers ({} to {}, inclusive)",
                  i32::MIN,
                  i32::MAX
                ),
              );
            }

            return number;
          }

          match number {
            Some(n) if n > 0 && n < 1 << 29 => {
              match numbers.entry(n) {
                Entry::Occupied(e) => {
                  report
                    .error(format_args!("field number `{n}` used twice"))
                    .saying(lit, "re-used here")
                    .remark(e.get(), "previously used here")
                    .note("all field numbers must be unique");
                }
                Entry::Vacant(e) => {
                  e.insert(lit.span());
                }
              }

              Some(n)
            }
            _ => {
              report.error("field number out of range").at(lit).note(
                format_args!(
                  "field numbers are positive, 29-bit integers (1 to {}, inclusive)",
                  (1 << 29) - 1,
                ),
              );
              None
            }
          }
        });

        fields.push(ir::Field {
          name: field.name.name(self.scx).into(),
          parent: ty,
          decl: Some(field),
          ty: field_ty.into(),
          number: field_number.into(),
        });
      }
    }

    bundle
  }

  fn extract_types<'rcx>(
    &'rcx self,
    bundle: &mut ir::Bundle<'ast, 'rcx>,
    package: &'rcx str,
    parent: Option<&'rcx ir::Type<'ast, 'rcx>>,
    decl: &'ast syn::Decl,
    types_out: &mut AVec<&'rcx ir::Type<'ast, 'rcx>>,
    report: &mut Report,
  ) {
    let declared_name = decl.name.name(self.scx);
    let name = match parent {
      Some(parent) => {
        let mut name = AString::with_capacity_in(
          parent.name().len() + declared_name.len() + 1,
          &self.arena,
        );
        name.push_str(parent.name());
        name.push('.');
        name.push_str(declared_name);
        name.into_bump_str()
      }
      None => declared_name,
    };

    let mut decls_count = 0;
    let mut fields_count = 0;
    for item in &decl.items {
      match item {
        syn::Item::Decl(..) => decls_count += 1,
        syn::Item::Field(..) => fields_count += 1,
      }
    }

    let ty = self.arena.alloc(ir::Type {
      package: package.into(),
      name: name.into(),
      kind: decl.kind.into(),
      decl: Some(decl),
      fields: AVec::with_capacity_in(fields_count, &self.arena).into(),
      nesteds: AVec::with_capacity_in(decls_count, &self.arena).into(),
      parent,
    });
    bundle.types.borrow_mut().push(ty);
    types_out.push(ty);

    {
      let key = match package {
        "" => name.to_string(),
        pkg => format!("{pkg}.{name}"),
      };

      // Register the type in our big table of types. We need to extend some
      // lifetimes, since they refer to the arena (or data borrowed by the
      // ResolveCtx that outlives it).
      let value = (ty as *const ir::Type).cast::<ir::Type>();
      let mut types = self.types.borrow_mut();
      match types.entry(key) {
        Entry::Occupied(e) => {
          let diagnostic = report
            .error(format_args!("type named `{package}.{name}` already exists"))
            .saying(decl.name, "type already exists with this name");

          if let (_, Some(span)) = e.get() {
            diagnostic.remark(span, "previous definition is here");
          }
        }
        Entry::Vacant(e) => {
          e.insert((value, Some(decl.span())));
        }
      }
    }

    for item in &decl.items {
      let syn::Item::Decl(decl) = item else { continue };
      self.extract_types(
        bundle,
        package,
        Some(ty),
        decl,
        &mut ty.nesteds.borrow_mut(),
        report,
      );
    }
  }

  fn resolve_relative_type<'rcx>(
    &'rcx self,
    name: &str,
    root_types: &[&'rcx ir::Type<'ast, 'rcx>],
    referenced_in: &'rcx ir::Type<'ast, 'rcx>,
  ) -> Option<&'rcx ir::Type<'ast, 'rcx>> {
    let mut search_in = referenced_in;
    loop {
      let resolved = search_in.nesteds(|tys| {
        for ty in tys {
          let suffix = ty.name().split(".").last().unwrap();
          if suffix == name {
            // Rust tries to infer an over-short lifetime here?
            let ty: &'rcx ir::Type<'ast, 'rcx> = ty;
            return Some(ty);
          }
        }

        None
      });

      match (resolved, search_in.parent()) {
        (Some(ty), _) => return Some(ty),
        (None, Some(parent)) => search_in = parent,
        (None, None) => break,
      }
    }

    // Search in the top-level directly.
    for ty in root_types {
      let suffix = ty.name().split(".").last().unwrap();
      if suffix == name {
        return Some(ty);
      }
    }

    None
  }
}
