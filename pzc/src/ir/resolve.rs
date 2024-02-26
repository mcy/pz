//! The resolver (primarily name resolution, among other things).

use std::cell::Cell;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::mem;

use ilex::Spanned;
use pz::proto;
use pz::proto::field::Type as TypeEnum;
use pz::proto::r#type::Kind;

use crate::ir;
use crate::syn;

use bumpalo::collections::String as AString;
use bumpalo::collections::Vec as AVec;

use super::TypeName;

pub struct ResolveCtx<'ast> {
  arena: bumpalo::Bump,
  icx: &'ast ilex::Context,
  report: &'ast ilex::Report,

  types: RefCell<
    // Let 'rcx represent the lifetime of `arena`. Then, the "real" type of this
    // map is `HashMap<String, &'rcx Type<'ast, 'rcx>>`. We're forced to erase
    // both lifetimes, because ir::Type requires 'ast: 'rcx, because there are
    // 'rcx references that point to 'ast references.
    HashMap<ir::TypeName<'static>, *const ir::Type<'static, 'static>>,
  >,
  bundles_by_package:
    RefCell<HashMap<&'ast str, (&'ast str, &'ast proto::Bundle)>>,
  _ph: PhantomData<&'ast syn::PzFile<'ast>>,
}

impl<'ast> ResolveCtx<'ast> {
  pub fn new(icx: &'ast ilex::Context, report: &'ast ilex::Report) -> Self {
    Self {
      icx,
      report,
      arena: bumpalo::Bump::new(),
      types: Default::default(),
      bundles_by_package: Default::default(),
      _ph: PhantomData,
    }
  }

  fn redef_error(
    &self,
    ty: impl fmt::Display,
    at: Option<impl ilex::Spanned>,
    orig: Option<impl ilex::Spanned>,
  ) {
    let mut d = self
      .report
      .error(format_args!("symbol `{ty}` already exists"));

    if let Some(span) = at {
      d = d.saying(span, "symbol already exists with this name");
    }
    if let Some(span) = orig {
      d.remark(span, "previous definition is here");
    }
  }

  pub fn type_by_name(
    &self,
    name: ir::TypeName,
  ) -> Option<&ir::Type<'ast, '_>> {
    let ptr = self.types.borrow().get(&name).copied()?;
    unsafe {
      // SAFETY: this pointer is allocated on `self.arena`.
      Some(&*ptr.cast::<ir::Type<'ast, '_>>())
    }
  }

  fn register_type<'rcx>(&'rcx self, ty: &'rcx ir::Type<'ast, 'rcx>) {
    // Register the type in our big table of types. We need to extend some
    // lifetimes, since they refer to the arena (or data borrowed by the
    // ResolveCtx that outlives it).
    let value = (ty as *const ir::Type).cast::<ir::Type<'static, 'static>>();
    let mut types = self.types.borrow_mut();
    match types.entry(unsafe {
      mem::transmute::<ir::TypeName<'_>, ir::TypeName<'static>>(ty.name())
    }) {
      Entry::Occupied(e) => {
        self.redef_error(ty.name(), ty.decl().map(|d| d.name), unsafe {
          (**e.get()).decl().map(|d| d.name)
        });
      }
      Entry::Vacant(e) => {
        e.insert(value);
      }
    }
  }

  /*pub fn load_bundle(&self, bundle: proto::Bundle) -> Result<(), impl Display> {
    self.protos.borrow_mut().push(Box::new(bundle));
    let protos = self.protos.borrow();
    let proto = protos.last().unwrap();

    // First, verify that all of the foreign types we need to load are already
    // loaded.
    for foreign in &proto.foreign_types {
      if !self.types.borrow().contains_key(foreign) {
        return Err("")
      }
    }

    Ok(())
  }*/

  pub fn resolve<'rcx>(
    &'rcx self,
    bundles: &'ast [(String, proto::Bundle)],
    files: &'ast [syn::PzFile<'ast>],
  ) -> Result<ir::Bundle<'ast, 'rcx>, ilex::Fatal> {
    // First, start by loading all of the dependency bundles.
    let mut packages = self.bundles_by_package.borrow_mut();
    let mut all_bundle_tys = Vec::new();
    for (path, bundle) in bundles {
      let mut tys_in_this_bundle = Vec::new();
      for package in &bundle.packages {
        match packages.entry(package) {
          Entry::Occupied(e) => {
            self.report.error(format_args!(
              "package `{package}` defined by both {} and {}",
              path,
              e.get().0,
            ));

            return self.report.fatal();
          }
          Entry::Vacant(e) => {
            e.insert((path, bundle));
          }
        }
      }

      // Create objects for all the types. We need to do this before we try
      // to resolve anything within the bundle protos.
      for ty_proto in &bundle.types {
        let Some(package) = bundle.packages.get(ty_proto.package() as usize)
        else {
          self
            .report
            .error(format_args!(
              "type `<unknown>.{}` had an invalid package pointer",
              ty_proto.name(),
            ))
            .note(format_args!("defined in {path}"));

          return self.report.fatal();
        };

        let ty: &_ = self.arena.alloc(ir::Type {
          name: Cell::new(ir::TypeName {
            package,
            name: ty_proto.name(),
          }),
          kind: Cell::new(match ty_proto.kind() {
            Kind::Message => syn::DeclKind::Message,
            Kind::Struct => syn::DeclKind::Struct,
            Kind::Choice => syn::DeclKind::Choice,
            Kind::Enum => syn::DeclKind::Enum,
          }),
          decl: None,
          fields: AVec::with_capacity_in(ty_proto.fields.len(), &self.arena)
            .into(),
          nesteds: AVec::with_capacity_in(ty_proto.nesteds.len(), &self.arena)
            .into(),
          parent: Cell::new(None),
          attrs: ty_proto.attrs.as_ref().cloned().unwrap_or_default(),
        });
        tys_in_this_bundle.push((ty_proto, ty));
        self.register_type(ty);
      }
      all_bundle_tys.push((path.as_str(), bundle, tys_in_this_bundle));
    }
    drop(packages);

    // Now resolve all the deps' fields and types. We do not require acyclicity;
    // we assume each bundle was constructed by a previous pz invocation, which
    // does require that the input fields depend only on the input bundles and
    // not vice-versa. Under this assumption, the input bundles are acyclic by
    // construction.
    for (path, bundle_proto, bundle_tys) in &all_bundle_tys {
      for (ty_proto, ty) in bundle_tys {
        if let Some(parent) = ty_proto.declared_in {
          let Some((_, parent)) = bundle_tys.get(parent as usize) else {
            self
              .report
              .error(format_args!(
                "type `{}` had an invalid parent pointer",
                ty.name()
              ))
              .note(format_args!("defined in {path}"));
            return self.report.fatal();
          };
          ty.parent.set(Some(parent));
        }

        for &nested in &ty_proto.nesteds {
          let Some((_, nested)) = bundle_tys.get(nested as usize) else {
            self
              .report
              .error(format_args!(
                "type `{}` had an invalid nested pointer",
                ty.name(),
              ))
              .note(format_args!("defined in {path}"));
            return self.report.fatal();
          };
          ty.nesteds.borrow_mut().push(nested);
        }

        'fields: for field_proto in &ty_proto.fields {
          let (needs_number, needs_type) = match ty.kind() {
            syn::DeclKind::Message => (true, true),
            syn::DeclKind::Struct => (false, true),
            syn::DeclKind::Choice => (true, true),
            syn::DeclKind::Enum => (true, false),
          };

          if needs_number != field_proto.number.is_some() {
            self
              .report
              .error(format_args!(
                "field `{}.{}` is missing its number",
                ty.name(),
                field_proto.name()
              ))
              .note(format_args!("defined in {path}"));
          }

          let field_ty = match field_proto.r#type() {
            TypeEnum::None if needs_type => {
              self.report.error(format_args!(
                "field `{}.{}` is missing its type",
                ty.name(),
                field_proto.name()
              ));
              continue 'fields;
            }
            TypeEnum::None => None,

            TypeEnum::I32 => Some(ir::FieldTypeKind::I32),
            TypeEnum::U32 => Some(ir::FieldTypeKind::U32),
            TypeEnum::F32 => Some(ir::FieldTypeKind::F32),
            TypeEnum::I64 => Some(ir::FieldTypeKind::I64),
            TypeEnum::U64 => Some(ir::FieldTypeKind::U64),
            TypeEnum::F64 => Some(ir::FieldTypeKind::F64),
            TypeEnum::Bool => Some(ir::FieldTypeKind::Bool),
            TypeEnum::String => Some(ir::FieldTypeKind::String),

            TypeEnum::Type => {
              let Some((_, ty)) =
                bundle_tys.get(field_proto.type_index() as usize)
              else {
                self
                  .report
                  .error(format_args!(
                    "field `{}.{}` had an invalid type pointer",
                    ty.name(),
                    field_proto.name()
                  ))
                  .note(format_args!("defined in {path}"));
                continue;
              };
              Some(ir::FieldTypeKind::Type(ty))
            }

            TypeEnum::Foreign => {
              let Some(key) = bundle_proto
                .foreign_types
                .get(field_proto.type_index() as usize)
              else {
                self
                  .report
                  .error(format_args!(
                    "field `{}.{}` had an invalid type pointer",
                    ty.name(),
                    field_proto.name()
                  ))
                  .note(format_args!("defined in {path}"));
                continue 'fields;
              };

              let Some(package) =
                bundle_proto.packages.get(key.package() as usize)
              else {
                self.
                report.error(format_args!(
                  "field `{}.{}`'s type `<unknown>.{}` had an invalid package pointer",
                  ty.name(),
                  field_proto.name(),
                  ty_proto.name(),
                )).note(format_args!("defined in {path}"));

                continue 'fields;
              };

              let key = TypeName {
                package,
                name: key.name(),
              };

              match self.type_by_name(key) {
                Some(ty) => Some(ir::FieldTypeKind::Type(ty)),
                None => {
                  self.
                  report
                    .error(format_args!(
                    "field `{}.{}` refers to a type `{key}` that does not exist",
                    ty.name(),
                    field_proto.name()
                  ))
                    .note(format_args!("defined in {path}"));
                  continue 'fields;
                }
              }
            }
          };
          let field_ty = field_ty.map(|kind| ir::FieldType {
            kind,
            is_repeated: field_proto.is_repeated(),
          });

          ty.fields.borrow_mut().push(ir::Field {
            name: Cell::new(field_proto.name()),
            parent: ty,
            decl: None,
            ty: field_ty.into(),
            number: field_proto.number.into(),
            attrs: field_proto.attrs.as_ref().cloned().unwrap_or_default(),
          });
        }
      }
    }

    // At this point, we've loaded all of the bundles, so we can proceed with
    // creating the new "resolved" bundle.

    // First, un-nest all the declarations. This needs to happen before we
    // resolve imports, because intra-compilation-unit imports can point to
    // any file.
    let mut all_symbols = Vec::new();

    let insert_symbol = |symbols: &mut HashMap<_, _>,
                         key: &str,
                         ty: &'rcx ir::Type<'ast, 'rcx>,
                         span: ilex::Span| {
      match symbols.entry(key.to_string()) {
        Entry::Occupied(e) => {
          let (_, orig) = e.get();
          self.redef_error(ty.name(), Some(span), Some(*orig));
        }
        Entry::Vacant(e) => {
          e.insert((ty, span));
        }
      }
    };

    for file in files {
      let mut package = AString::new_in(&self.arena);
      for (name, dot) in &file.package.path.components {
        package.push_str(name.name().text(self.icx));
        if dot.is_some() {
          package.push('.');
        }
      }
      let package = package.into_bump_str();

      let mut symbols = HashMap::new();

      let mut roots = AVec::new_in(&self.arena);
      let mut types = Vec::new();
      for item in &file.items {
        match item {
          syn::Item::Decl(decl) => {
            self.extract_types(&mut types, package, None, decl, &mut roots);
          }
          _ => unreachable!(),
        }
      }

      for root in roots {
        insert_symbol(
          &mut symbols,
          root.name().name,
          root,
          root.decl().unwrap().name.span(self.icx),
        );
      }

      for ty in &types {
        insert_symbol(
          &mut symbols,
          &ty.name().to_string(),
          ty,
          ty.decl().unwrap().name.span(self.icx),
        );
      }

      all_symbols.push((file, types, symbols));
    }

    for (file, _, symbols) in &mut all_symbols {
      for import in &file.imports {
        let package = import.package.join();
        for syn::ImportedSymbol { symbol, rename } in &import.symbols {
          let key = TypeName {
            package: &package,
            name: &symbol.join(),
          };
          match self.type_by_name(key) {
            Some(ty) => {
              insert_symbol(
                symbols,
                &key.to_string(),
                ty,
                symbol.span(self.icx),
              );
              match rename {
                Some((_, id)) => insert_symbol(
                  symbols,
                  id.name().text(self.icx),
                  ty,
                  symbol.span(self.icx),
                ),
                None => insert_symbol(
                  symbols,
                  symbol.last().name().text(self.icx),
                  ty,
                  symbol.span(self.icx),
                ),
              }
            }
            None => {
              self
                .report
                .error(format_args!("cannot find imported symbol {key}"))
                .saying(&import.package, "package specified here")
                .saying(symbol, "symbol specified here");
            }
          }
        }
      }
    }

    let bundle = ir::Bundle::<'ast, 'rcx> {
      types: Vec::new().into(),
    };

    // Next, resolve the types of all the fields.
    for (_, tys, top_level_symbols) in all_symbols {
      for ty in tys {
        bundle.types.borrow_mut().push(ty);
        let decl = ty.decl().unwrap();

        let field_count = decl
          .items
          .iter()
          .filter(|i| matches!(i, syn::Item::Field(..)))
          .count();

        let mut fields = ty.fields.borrow_mut();
        fields.reserve(field_count);

        let mut numbers = HashMap::<i32, ilex::Span>::new();
        for field in &decl.items {
          let syn::Item::Field(field) = field else {
            continue;
          };
          let field_ty = field.ty.as_ref().and_then(|t| {
            let path = match t {
              syn::Type::Repeated { element, .. } => match &**element {
                syn::Type::Path(path) => path,
                other => {
                  self
                    .report
                    .error("repeated repeated is not supported yet, sorry")
                    .saying(other, "expected a path here");
                  return None;
                }
              },
              syn::Type::Path(path) => path,
            };

            let kind = 'name_resolve: {
              assert!(!path.components.is_empty());

              // Two cases:
              // - A single-element path referring to an item in this or a
              //   parent's body.
              // - A path that refers to a top-level element (either an import,
              //   a fully-qualified type, or a top-level type.)
              if let [(id, _)] = path.components.as_slice() {
                let name = id.name().text(self.icx);
                let mut search_in = ty;
                loop {
                  let resolved = search_in.nesteds(|tys| {
                    for ty in tys {
                      let suffix = ty.name().name.split('.').last().unwrap();
                      if suffix == name {
                        // Rust tries to infer an over-short lifetime here?
                        let ty: &'rcx ir::Type<'ast, 'rcx> = ty;
                        return Some(ty);
                      }
                    }

                    None
                  });

                  match (resolved, search_in.parent()) {
                    (Some(ty), _) => {
                      break 'name_resolve ir::FieldTypeKind::Type(ty)
                    }
                    (None, Some(parent)) => search_in = parent,
                    (None, None) => break,
                  }
                }
              }

              let name = path.join();
              if let Some((ty, _)) = top_level_symbols.get(&name) {
                break 'name_resolve ir::FieldTypeKind::Type(ty);
              }

              let builtins = &[
                ("i32", ir::FieldTypeKind::I32),
                ("u32", ir::FieldTypeKind::U32),
                ("f32", ir::FieldTypeKind::F32),
                ("i64", ir::FieldTypeKind::I64),
                ("u64", ir::FieldTypeKind::U64),
                ("f64", ir::FieldTypeKind::F64),
                ("bool", ir::FieldTypeKind::Bool),
                ("str", ir::FieldTypeKind::String),
              ];

              for &(builtin, ty) in builtins {
                if name == builtin {
                  break 'name_resolve ty;
                }
              }

              self
                .report
                .error(format_args!("cannot resolve type `{name}`"))
                .at(path);
              return None;
            };

            Some(ir::FieldType {
              is_repeated: matches!(t, syn::Type::Repeated { .. }),
              kind,
            })
          });

          let field_number = field.number.map(|lit| {
            let number = match decl.kind {
              syn::DeclKind::Enum => lit.to_int::<i32>(.., self.report),
              _ => lit.to_int::<i32>(1..1 << 29, self.report),
            };

            match numbers.entry(number) {
              Entry::Occupied(e) => {
                self
                  .report
                  .error(format_args!("field number `{number}` used twice"))
                  .saying(lit, "re-used here")
                  .remark(e.get(), "previously used here")
                  .note("all field numbers must be unique");
              }
              Entry::Vacant(e) => {
                e.insert(lit.span(self.icx));
              }
            }

            number
          });

          fields.push(ir::Field {
            name: field.name.name().text(self.icx).into(),
            parent: ty,
            decl: Some(field),
            ty: field_ty.into(),
            number: field_number.into(),
            attrs: self.field_attrs(field),
          });
        }
      }
    }

    Ok(bundle)
  }

  fn extract_types<'rcx>(
    &'rcx self,
    types: &mut Vec<&'rcx ir::Type<'ast, 'rcx>>,
    package: &'rcx str,
    parent: Option<&'rcx ir::Type<'ast, 'rcx>>,
    decl: &'ast syn::Decl,
    types_out: &mut AVec<&'rcx ir::Type<'ast, 'rcx>>,
  ) {
    let declared_name = decl.name.name().text(self.icx);
    let name = match parent {
      Some(parent) => {
        let mut name = AString::with_capacity_in(
          parent.name().name.len() + declared_name.len() + 1,
          &self.arena,
        );
        name.push_str(parent.name().name);
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
      name: Cell::new(ir::TypeName { package, name }),
      kind: decl.kind.into(),
      decl: Some(decl),
      fields: AVec::with_capacity_in(fields_count, &self.arena).into(),
      nesteds: AVec::with_capacity_in(decls_count, &self.arena).into(),
      parent: parent.into(),
      attrs: self.type_attrs(decl),
    });
    types.push(ty);
    types_out.push(ty);
    self.register_type(ty);

    for item in &decl.items {
      let syn::Item::Decl(decl) = item else {
        continue;
      };
      self.extract_types(
        types,
        package,
        Some(ty),
        decl,
        &mut ty.nesteds.borrow_mut(),
      );
    }
  }

  fn type_attrs(&self, decl: &syn::Decl) -> proto::r#type::Attrs {
    let mut attrs = proto::r#type::Attrs::default();
    for attr in &decl.attrs {
      match attr {
        syn::Attr::Doc(doc) => {
          let doc = doc.text(self.icx).trim_start_matches("///").trim();
          attrs.docs.push(doc.to_string());
        }
        syn::Attr::At { name, value, .. } => {
          if name.is_exactly(["deprecated"]) {
            if attrs.deprecated.is_some() {
              self
                .report
                .error("cannot specify `@deprecated` multiple times")
                .at(attr);
            }

            attrs.deprecated = match &value {
              None => Some(String::new()),
              Some((_, syn::AttrValue::Str(str))) => Some(syn::unescape(*str)),
              _ => {
                self
                  .report
                  .error("`@deprecated` expects a string value")
                  .at(attr);
                None
              }
            };
          } else {
            self
              .report
              .error(format_args!("unknown attribute `{}`", name.join()))
              .at(attr);
          }
        }
      }
    }

    attrs
  }

  fn field_attrs(&self, field: &syn::Field) -> proto::field::Attrs {
    let mut attrs = proto::field::Attrs::default();
    for attr in &field.attrs {
      match attr {
        syn::Attr::Doc(doc) => {
          let doc = doc.text(self.icx).trim_start_matches("///").trim();
          attrs.docs.push(doc.to_string());
        }
        syn::Attr::At { name, value, .. } => {
          if name.is_exactly(["deprecated"]) {
            if attrs.deprecated.is_some() {
              self
                .report
                .error("cannot specify `@deprecated` multiple times")
                .at(attr);
            }

            attrs.deprecated = match value {
              None => Some(String::new()),
              Some((_, syn::AttrValue::Str(str))) => Some(syn::unescape(*str)),
              _ => {
                self
                  .report
                  .error("`@deprecated` expects a string value")
                  .at(attr);
                None
              }
            };
          } else {
            self
              .report
              .error(format_args!("unknown attribute `{}`", name.join()))
              .at(attr);
          }
        }
      }
    }

    attrs
  }
}
