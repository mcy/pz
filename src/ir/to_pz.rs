//! Serializes an IR bundle into a proto.

use std::collections::HashMap;

use crate::ir;
use crate::pz;
use crate::syn;

pub fn to_pz(bundle: &ir::Bundle) -> pz::Bundle {
  let mut pz = pz::Bundle::default();

  let mut pkg_to_idx = HashMap::new();
  let mut ty_to_idx = HashMap::new();
  let mut foreign_to_idx = HashMap::new();

  // First, materialize all of the types in the type array.
  for (i, &ty) in bundle.types.borrow().iter().enumerate() {
    ty_to_idx.insert(ty as *const ir::Type, i);

    let &mut pkg = pkg_to_idx.entry(ty.package()).or_insert_with(|| {
      pz.packages.push(ty.package().to_string());
      pz.packages.len() - 1
    });

    pz.types.push(pz::Type {
      name: Some(ty.name().to_string()),
      package: Some(pkg as u32),
      kind: match ty.kind() {
        syn::DeclKind::Message => Some(pz::r#type::Kind::Message as i32),
        syn::DeclKind::Struct => Some(pz::r#type::Kind::Struct as i32),
        syn::DeclKind::Choice => Some(pz::r#type::Kind::Choice as i32),
        syn::DeclKind::Enum => Some(pz::r#type::Kind::Enum as i32),
      },

      ..Default::default()
    });
  }

  // Now, resolve fields and parent relations.
  for (&ty, pz_ty) in bundle.types.borrow().iter().zip(&mut pz.types) {
    if let Some(parent) = ty.parent() {
      pz_ty.declated_in = Some(ty_to_idx[&(parent as *const ir::Type)] as u32);
    }

    ty.nesteds(|tys| {
      for &ty in tys {
        pz_ty
          .nesteds
          .push(ty_to_idx[&(ty as *const ir::Type)] as u32)
      }
    });

    ty.fields(|fs| {
      for field in fs {
        let kind = field.ty().map(|t| match t.kind {
          ir::FieldTypeKind::I32 => pz::field::Type::I32,
          ir::FieldTypeKind::U32 => pz::field::Type::U32,
          ir::FieldTypeKind::F32 => pz::field::Type::F32,
          ir::FieldTypeKind::I64 => pz::field::Type::I64,
          ir::FieldTypeKind::U64 => pz::field::Type::U64,
          ir::FieldTypeKind::F64 => pz::field::Type::F64,
          ir::FieldTypeKind::Bool => pz::field::Type::Bool,
          ir::FieldTypeKind::String => pz::field::Type::String,
          ir::FieldTypeKind::Type(_) => pz::field::Type::Type,
          ir::FieldTypeKind::Unresolved(_) => pz::field::Type::Foreign,
        } as i32);

        let type_index = field.ty().and_then(|t| match t.kind {
          ir::FieldTypeKind::Type(ty) => {
            Some(ty_to_idx[&(ty as *const ir::Type)] as u32)
          }
          ir::FieldTypeKind::Unresolved(name) => {
            let &mut idx = foreign_to_idx.entry(name).or_insert_with(|| {
              pz.foreign_types.push(name.to_string());
              pz.foreign_types.len() - 1
            });
            Some(idx as u32)
          }
          _ => None,
        });

        pz_ty.fields.push(pz::Field {
          name: Some(field.name().to_string()),
          number: field.number(),
          is_repeated: field.ty().map(|t| t.is_repeated()),
          r#type: kind,
          type_index,
        })
      }
    });
  }

  pz
}
