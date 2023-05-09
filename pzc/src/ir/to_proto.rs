//! Serializes an IR bundle into a proto.

use std::collections::HashMap;

use pz::proto;

use crate::ir;
use crate::syn;

pub fn to_proto(bundle: &ir::Bundle) -> proto::Bundle {
  let mut proto = proto::Bundle::default();

  let mut pkg_to_idx = HashMap::new();
  let mut ty_to_idx = HashMap::new();
  let mut foreign_to_idx = HashMap::new();

  // First, materialize all of the types in the type array.
  for (i, &ty) in bundle.types.borrow().iter().enumerate() {
    ty_to_idx.insert(ty as *const ir::Type, i);

    let &mut pkg = pkg_to_idx.entry(ty.name().package).or_insert_with(|| {
      proto.packages.push(ty.name().package.to_string());
      proto.packages.len() - 1
    });

    proto.types.push(proto::Type {
      name: Some(ty.name().name.to_string()),
      package: Some(pkg as u32),
      kind: match ty.kind() {
        syn::DeclKind::Message => Some(proto::r#type::Kind::Message as i32),
        syn::DeclKind::Struct => Some(proto::r#type::Kind::Struct as i32),
        syn::DeclKind::Choice => Some(proto::r#type::Kind::Choice as i32),
        syn::DeclKind::Enum => Some(proto::r#type::Kind::Enum as i32),
      },
      span: ty.decl().map(|t| t.span.id()),
      attrs: Some(ty.attrs.clone()),

      ..Default::default()
    });
  }

  // Now, resolve fields and parent relations.
  for (&ty, proto_ty) in bundle.types.borrow().iter().zip(&mut proto.types) {
    if let Some(parent) = ty.parent() {
      proto_ty.declared_in =
        Some(ty_to_idx[&(parent as *const ir::Type)] as u32);
    }

    ty.nesteds(|tys| {
      for &ty in tys {
        proto_ty
          .nesteds
          .push(ty_to_idx[&(ty as *const ir::Type)] as u32)
      }
    });

    ty.fields(|fs| {
      for field in fs {
        let kind = field.ty().map(|t| match t.kind {
          ir::FieldTypeKind::I32 => proto::field::Type::I32,
          ir::FieldTypeKind::U32 => proto::field::Type::U32,
          ir::FieldTypeKind::F32 => proto::field::Type::F32,
          ir::FieldTypeKind::I64 => proto::field::Type::I64,
          ir::FieldTypeKind::U64 => proto::field::Type::U64,
          ir::FieldTypeKind::F64 => proto::field::Type::F64,
          ir::FieldTypeKind::Bool => proto::field::Type::Bool,
          ir::FieldTypeKind::String => proto::field::Type::String,
          ir::FieldTypeKind::Type(ty) if ty.decl().is_some() => {
            proto::field::Type::Type
          }
          ir::FieldTypeKind::Type(..) => proto::field::Type::Foreign,
        } as i32);

        let type_index = field.ty().and_then(|t| match t.kind {
          ir::FieldTypeKind::Type(ty) if ty.decl().is_some() => {
            Some(ty_to_idx[&(ty as *const ir::Type)] as u32)
          }
          ir::FieldTypeKind::Type(ty) => {
            let &mut pkg =
              pkg_to_idx.entry(ty.name().package).or_insert_with(|| {
                proto.packages.push(ty.name().package.to_string());
                proto.packages.len() - 1
              });

            let &mut idx =
              foreign_to_idx.entry(ty.name()).or_insert_with(|| {
                proto.foreign_types.push(proto::bundle::ForeignType {
                  name: Some(ty.name().name.to_string()),
                  package: Some(pkg as u32),
                });
                proto.foreign_types.len() - 1
              });
            Some(idx as u32)
          }
          _ => None,
        });

        proto_ty.fields.push(proto::Field {
          name: Some(field.name().to_string()),
          number: field.number(),
          is_repeated: field.ty().map(|t| t.is_repeated()),
          r#type: kind,
          type_index,
          span: field.decl().map(|f| f.span.id()),
          attrs: Some(field.attrs.clone()),
        })
      }
    });
  }

  proto
}
