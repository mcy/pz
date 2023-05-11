//! The resolver (primarily name resolution, among other things).

use std::cell::Cell;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::mem;

use pz::proto;
use pz::proto::field::Type as TypeEnum;
use pz::proto::r#type::Kind;

use crate::ir;
use crate::report::Report;
use crate::syn;
use crate::syn::Spanned;

use bumpalo::collections::String as AString;
use bumpalo::collections::Vec as AVec;

use super::TypeName;

pub struct ResolveCtx<'src, 'scx> {
  arena: bumpalo::Bump,
  scx: &'scx syn::SourceCtx,

  types: RefCell<
    // Let 'rcx represent the lifetime of `arena`. Then, the "real" type of this
    // map is `HashMap<String, &'rcx Type<'src, 'rcx>>`. We're forced to erase
    // both lifetimes, because ir::Type requires 'src: 'rcx, because there are
    // 'rcx references that point to 'src references.
    HashMap<
      ir::TypeName<'static>,
      (*const ir::Type<'static, 'static>, Option<syn::Span>),
    >,
  >,
  bundles_by_package:
    RefCell<HashMap<&'src str, (&'src str, &'src proto::Bundle)>>,
  _ph: PhantomData<&'src syn::PzFile>,
}

fn redef_error(
  report: &mut Report,
  ty: impl fmt::Display,
  at: Option<syn::Span>,
  orig: Option<syn::Span>,
) {
  let diagnostic = report.error(format_args!("symbol `{ty}` already exists"));

  if let Some(span) = at {
    diagnostic.saying(span, "symbol already exists with this name");
  }
  if let Some(span) = orig {
    diagnostic.remark(span, "previous definition is here");
  }
}

impl<'src, 'scx: 'src> ResolveCtx<'src, 'scx> {
  pub fn new(scx: &'scx syn::SourceCtx) -> Self {
    Self {
      arena: bumpalo::Bump::new(),
      scx,
      types: Default::default(),
      bundles_by_package: Default::default(),
      _ph: PhantomData,
    }
  }

  pub fn type_by_name(
    &self,
    name: ir::TypeName,
  ) -> Option<&ir::Type<'src, '_>> {
    let (ptr, _) = self.types.borrow().get(&name).copied()?;
    unsafe {
      // SAFETY: this pointer is allocated on `self.arena`.
      Some(&*ptr.cast::<ir::Type<'src, '_>>())
    }
  }

  fn register_type<'rcx>(
    &'rcx self,
    ty: &'rcx ir::Type<'src, 'rcx>,
    report: &mut Report,
  ) {
    // Register the type in our big table of types. We need to extend some
    // lifetimes, since they refer to the arena (or data borrowed by the
    // ResolveCtx that outlives it).
    let value = (ty as *const ir::Type).cast::<ir::Type>();
    let mut types = self.types.borrow_mut();
    match types.entry(unsafe {
      mem::transmute::<ir::TypeName<'_>, ir::TypeName<'static>>(ty.name())
    }) {
      Entry::Occupied(e) => {
        redef_error(
          report,
          ty.name(),
          ty.decl().map(|d| d.name.span()),
          e.get().1,
        );
      }
      Entry::Vacant(e) => {
        e.insert((value, ty.decl().map(|d| d.name.span())));
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
    bundles: &'src [(String, proto::Bundle)],
    files: &'src [syn::PzFile],
    report: &mut Report,
  ) -> Option<ir::Bundle<'src, 'rcx>> {
    // First, start by loading all of the dependency bundles.
    let mut packages = self.bundles_by_package.borrow_mut();
    let mut all_bundle_tys = Vec::new();
    for (path, bundle) in bundles {
      let mut tys_in_this_bundle = Vec::new();
      for package in &bundle.packages {
        match packages.entry(package) {
          Entry::Occupied(e) => {
            report.error(format_args!(
              "package `{package}` defined by both {} and {}",
              path,
              e.get().0,
            ));

            return None;
          }
          Entry::Vacant(e) => {
            e.insert((path, bundle));
          }
        }
      }

      // Create objects for all the types. We need to do this before we try
      // to resolve anything within the bundle protos.
      for ty_proto in &bundle.types {
        let Some(package) = bundle.packages.get(ty_proto.package() as usize) else {
          report.error(format_args!(
            "type `<unknown>.{}` had an invalid package pointer",
            ty_proto.name(),
          )).note(format_args!("defined in {path}"));

          return None;
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
        self.register_type(ty, report);
      }
      all_bundle_tys.push((path.as_str(), bundle, tys_in_this_bundle));
    }
    drop(packages);

    if report.has_errors() {
      return None;
    }

    // Now resolve all the deps' fields and types. We do not require acyclicity;
    // we assume each bundle was constructed by a previous pz invokation, which
    // does require that the input fields depend only on the input bundles and
    // not vice-versa. Under this assumption, the input bundles are acyclic by
    // construction.
    for (path, bundle_proto, bundle_tys) in &all_bundle_tys {
      for (ty_proto, ty) in bundle_tys {
        if let Some(parent) = ty_proto.declared_in {
          let Some((_, parent)) = bundle_tys.get(parent as usize) else {
            report.error(format_args!(
              "type `{}` had an invalid parent pointer", ty.name()
            )).note(format_args!("defined in {path}"));
            return None;
          };
          ty.parent.set(Some(parent));
        }

        for &nested in &ty_proto.nesteds {
          let Some((_, nested)) = bundle_tys.get(nested as usize) else {
            report.error(format_args!(
              "type `{}` had an invalid nested pointer", ty.name(),
            )).note(format_args!("defined in {path}"));
            return None;
          };
          ty.nesteds.borrow_mut().push(nested);
        }

        for field_proto in &ty_proto.fields {
          let (needs_number, needs_type) = match ty.kind() {
            syn::DeclKind::Message => (true, true),
            syn::DeclKind::Struct => (false, true),
            syn::DeclKind::Choice => (true, true),
            syn::DeclKind::Enum => (true, false),
          };

          if needs_number != field_proto.number.is_some() {
            report
              .error(format_args!(
                "field `{}.{}` is missing its number",
                ty.name(),
                field_proto.name()
              ))
              .note(format_args!("defined in {path}"));
            return None;
          }

          let field_ty = match field_proto.r#type() {
            TypeEnum::None if needs_type => {
              report.error(format_args!(
                "field `{}.{}` is missing its type",
                ty.name(),
                field_proto.name()
              ));
              return None;
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
                bundle_tys.get(field_proto.type_index() as usize) else {
                  report
                  .error(format_args!(
                    "field `{}.{}` had an invalid type pointer",
                    ty.name(),
                    field_proto.name()
                  ))
                  .note(format_args!("defined in {path}"));
                return None;
              };
              Some(ir::FieldTypeKind::Type(ty))
            }

            TypeEnum::Foreign => {
              let Some(key) =
                bundle_proto.foreign_types.get(field_proto.type_index() as usize) else {
                  report
                  .error(format_args!(
                    "field `{}.{}` had an invalid type pointer",
                    ty.name(),
                    field_proto.name()
                  ))
                  .note(format_args!("defined in {path}"));
                return None;
              };

              let Some(package) = bundle_proto.packages.get(key.package() as usize) else {
                report.error(format_args!(
                  "field `{}.{}`'s type `<unknown>.{}` had an invalid package pointer",
                  ty.name(),
                  field_proto.name(),
                  ty_proto.name(),
                )).note(format_args!("defined in {path}"));

                return None;
              };

              let key = TypeName {
                package,
                name: key.name(),
              };

              match self.type_by_name(key) {
                Some(ty) => Some(ir::FieldTypeKind::Type(ty)),
                None => {
                  report
                    .error(format_args!(
                    "field `{}.{}` refers to a type `{key}` that does not exist",
                    ty.name(),
                    field_proto.name()
                  ))
                    .note(format_args!("defined in {path}"));
                  return None;
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

    if report.has_errors() {
      return None;
    }

    // At this point, we've loaded all of the bundles, so we can proceed with
    // creating the new "resolved" bundle.

    // First, un-nest all the declarations. This needs to happen before we
    // resolve imports, because intra-compilation-unit imports can point to
    // any file.
    let mut all_symbols = Vec::new();

    let insert_symbol = |symbols: &mut HashMap<_, _>,
                         key: &str,
                         ty: &'rcx ir::Type<'src, 'rcx>,
                         span: syn::Span,
                         report: &mut Report| {
      match symbols.entry(key.to_string()) {
        Entry::Occupied(e) => {
          let (_, orig) = e.get();
          redef_error(report, ty.name(), Some(span), Some(*orig));
        }
        Entry::Vacant(e) => {
          e.insert((ty, span));
        }
      }
    };

    for file in files {
      let mut package = AString::new_in(&self.arena);
      for (i, name) in file.package.path.components.iter().enumerate() {
        let text = name.name(self.scx);
        if i != 0 {
          package.push('.');
        }
        package.push_str(text);
      }
      let package = package.into_bump_str();

      let mut symbols = HashMap::new();

      let mut roots = AVec::new_in(&self.arena);
      let mut types = Vec::new();
      for item in &file.items {
        match item {
          syn::Item::Decl(decl) => {
            self.extract_types(
              &mut types, package, None, decl, &mut roots, report,
            );
          }
          _ => unreachable!(),
        }
      }

      for root in roots {
        insert_symbol(
          &mut symbols,
          root.name().name,
          root,
          root.decl().unwrap().name.span(),
          report,
        );
      }

      for ty in &types {
        insert_symbol(
          &mut symbols,
          &ty.name().to_string(),
          ty,
          ty.decl().unwrap().name.span(),
          report,
        );
      }

      all_symbols.push((file, types, symbols));
    }

    for (file, _, symbols) in &mut all_symbols {
      for import in &file.imports {
        let package = import.package.join(self.scx);
        for (name, rename) in &import.symbols {
          let key = TypeName {
            package: &package,
            name: &name.join(self.scx),
          };
          match self.type_by_name(key) {
            Some(ty) => {
              insert_symbol(symbols, &key.to_string(), ty, name.span(), report);
              match rename {
                Some(id) => insert_symbol(
                  symbols,
                  id.name(self.scx),
                  ty,
                  name.span(),
                  report,
                ),
                None => insert_symbol(
                  symbols,
                  name.components.last().unwrap().name(self.scx),
                  ty,
                  name.span(),
                  report,
                ),
              }
            }
            None => {
              report
                .error(format_args!("cannot find imported symbol {key}"))
                .saying(&import.package, "package specified here")
                .saying(name, "symbol specified here");
            }
          }
        }
      }
    }

    if report.has_errors() {
      return None;
    }

    let bundle = ir::Bundle::<'src, 'rcx> {
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

              syn::TypeKind::Path(path) => 'name_resolve: {
                assert!(path.components.len() > 0);

                // Two cases:
                // - A single-element path referring to an item in this or a
                //   parent's body.
                // - A path that refers to a top-level element (either an import,
                //   a fully-qualified type, or a top-levle type.)
                if let [name] = path.components.as_slice() {
                  let name = name.name(self.scx);
                  let mut search_in = ty;
                  loop {
                    let resolved = search_in.nesteds(|tys| {
                      for ty in tys {
                        let suffix = ty.name().name.split(".").last().unwrap();
                        if suffix == name {
                          // Rust tries to infer an over-short lifetime here?
                          let ty: &'rcx ir::Type<'src, 'rcx> = ty;
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

                let name = path.join(self.scx);
                match top_level_symbols.get(&name) {
                  Some((ty, _)) => ir::FieldTypeKind::Type(ty),
                  None => {
                    report
                      .error(format_args!("cannot resolve type `{name}`"))
                      .at(path);
                    ir::FieldTypeKind::I32
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
            attrs: self.field_attrs(field, report),
          });
        }
      }
    }

    Some(bundle)
  }

  fn extract_types<'rcx>(
    &'rcx self,
    types: &mut Vec<&'rcx ir::Type<'src, 'rcx>>,
    package: &'rcx str,
    parent: Option<&'rcx ir::Type<'src, 'rcx>>,
    decl: &'src syn::Decl,
    types_out: &mut AVec<&'rcx ir::Type<'src, 'rcx>>,
    report: &mut Report,
  ) {
    let declared_name = decl.name.name(self.scx);
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
      attrs: self.type_attrs(decl, report),
    });
    types.push(ty);
    types_out.push(ty);
    self.register_type(ty, report);

    for item in &decl.items {
      let syn::Item::Decl(decl) = item else { continue };
      self.extract_types(
        types,
        package,
        Some(ty),
        decl,
        &mut ty.nesteds.borrow_mut(),
        report,
      );
    }
  }

  fn type_attrs<'rcx>(
    &'rcx self,
    decl: &syn::Decl,
    report: &mut Report,
  ) -> proto::r#type::Attrs {
    let mut attrs = proto::r#type::Attrs::default();
    for attr in &decl.attrs {
      match &attr.kind {
        syn::AttrKind::Doc => {
          let text = attr.span().text(&self.scx);
          let doc = text.trim_start_matches("///").trim();
          attrs.docs.push(doc.to_string());
        }
        syn::AttrKind::At(path) => {
          if path.is_exactly(&self.scx, &["deprecated"]) {
            if attrs.deprecated.is_some() {
              report
                .error("cannot specify `@deprecated` multiple times")
                .at(attr);
            }

            attrs.deprecated = match &attr.value {
              syn::AttrValue::None => Some(String::new()),
              syn::AttrValue::Str(str) => {
                Some(str.unescape_utf8(&self.scx, report))
              }
              _ => {
                report
                  .error("`@deprecated` expects a string value")
                  .at(attr);
                None
              }
            };
          } else {
            report
              .error(format_args!(
                "unknown attribute `{}`",
                path.join(&self.scx)
              ))
              .at(attr);
          }
        }
      }
    }

    attrs
  }

  fn field_attrs<'rcx>(
    &'rcx self,
    field: &syn::Field,
    report: &mut Report,
  ) -> proto::field::Attrs {
    let mut attrs = proto::field::Attrs::default();
    for attr in &field.attrs {
      match &attr.kind {
        syn::AttrKind::Doc => {
          let text = attr.span().text(&self.scx);
          let doc = text.trim_start_matches("///").trim();
          attrs.docs.push(doc.to_string());
        }
        syn::AttrKind::At(path) => {
          if path.is_exactly(&self.scx, &["deprecated"]) {
            if attrs.deprecated.is_some() {
              report
                .error("cannot specify `@deprecated` multiple times")
                .at(attr);
            }

            attrs.deprecated = match &attr.value {
              syn::AttrValue::None => Some(String::new()),
              syn::AttrValue::Str(str) => {
                Some(str.unescape_utf8(&self.scx, report))
              }
              _ => {
                report
                  .error("`@deprecated` expects a string value")
                  .at(attr);
                None
              }
            };
          } else {
            report
              .error(format_args!(
                "unknown attribute `{}`",
                path.join(&self.scx)
              ))
              .at(attr);
          }
        }
      }
    }

    attrs
  }
}
