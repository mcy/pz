//! Rust codegen backend.

use pz::proto::plugin;

use crate::emit;
use crate::exec_plugin;
use crate::rust::names::deprecated;

mod choice;
mod r#enum;
mod fields;
mod message;
mod names;

pub fn rust_plugin() -> ! {
  exec_plugin(
    |_| plugin::AboutResponse {
      name: Some("rust".into()),
      version: Some(env!("CARGO_PKG_VERSION").into()),
      options: vec![
        plugin::about_response::Option {
          name: Some("rt-crate".into()),
          help: Some(
            "Rust crate name to use for importing the pz runtime".into(),
          ),
        },
        plugin::about_response::Option {
          name: Some("package-prefix".into()),
          help: Some(
            "comma-separated packages to strip from module emits".into(),
          ),
        },
      ],
    },
    |ctx| {
      let mut w = emit::SourceWriter::new(emit::Options::default());
      let rt = ctx.option("rt-crate").unwrap_or("pz");
      w.emit(
        vars! {
          use_rt: |w| match rt {
            "crate" => w.write("use crate"),
            rt => w.emit(vars! { rt }, "extern crate $rt"),
          },
        },
        r"
        // ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
        #![cfg_attr(rustfmt, rustfmt_skip)]
        
        #![allow(non_camel_case_types)]
        #![allow(non_upper_case_globals)]
        #![allow(non_snake_case)]
        #![allow(unused)]

        #![allow(clippy::derivable_impls)]
        #![allow(clippy::identity_op)]
        #![allow(clippy::needless_borrow)]
        #![allow(clippy::transmute_float_to_int)]
        #![allow(clippy::transmute_int_to_float)]
        #![allow(clippy::useless_transmute)]
        #![allow(clippy::unnecessary_cast)]
        #![allow(clippy::wrong_self_convention)]
        
        #![no_implicit_prelude]

        $use_rt as __rt;
        use __rt::__z;
        use __z::std as __s;
        
        use __s::default::Default as _;
      ",
      );
      w.new_line();

      let mut prefixes: Vec<Vec<&str>> = ctx
        .option("package-prefix")
        .map(|opt| {
          opt
            .split(",")
            .map(|s| s.split(".").map(str::trim).collect())
            .collect()
        })
        .unwrap_or_default();
      prefixes.sort();

      // Extract the module set of each type.
      let types = ctx.types_to_generate().collect::<Vec<_>>();
      let mut types = types
        .iter()
        .map(|ty| {
          let mut pkg =
            ty.package().split(".").map(str::trim).collect::<Vec<_>>();

          // Want to search the longest prefixes first.
          for prefix in prefixes.iter().rev() {
            if let Some(suf) = pkg.strip_prefix(prefix.as_slice()) {
              pkg = suf.to_vec();
              break;
            }
          }

          (pkg, *ty)
        })
        .collect::<Vec<_>>();

      // Sort by modules.
      types.sort_by(|(a, _), (b, _)| Ord::cmp(a, b));

      let mut stack = Vec::new();
      for &(ref mods, ty) in &types {
        // Quadratic, oops.
        let common_prefix =
          stack.iter().zip(mods).filter(|(a, b)| a == b).count();
        while stack.len() > common_prefix {
          w.emit(
            vars! { mod: stack.pop().unwrap() },
            "
              } // mod $mod
            ",
          );
        }
        for &m in &mods[common_prefix..] {
          w.emit(
            vars! { mod: m },
            "
              pub mod $mod {
              use super::{__, __rt, __z, __s};
              use __s::default::Default as _;
            ",
          );
          stack.push(m);
        }

        w.with_vars(
          vars! {
            deprecated: deprecated(
              ty.proto().attrs.as_ref().and_then(|a| a.deprecated.as_deref())),

            // Common standard library types and names.
            fmt: "__s::fmt",
            size_of: "__s::mem::size_of",
            AsRef: "__s::convert::AsRef",
            Default: "__s::default::Default",
            Into: "__s::convert::Into",
            NonNull: "__s::ptr::NonNull",
            Layout: "__s::alloc::Layout",
            PhantomData: "__s::marker::PhantomData",
            Sized: "__s::marker::Sized",

            Read: "__s::io::Read",
            Write: "__s::io::Write",

            Option: "__s::option::Option",
            Some: "__s::option::Option::Some",
            None: "__s::option::Option::None",

            Result: "__s::result::Result",
            Ok: "__s::result::Result::Ok",
            Err: "__s::result::Result::Err",

            // Common runtime types and names.
            View: "__rt::View",
            Mut: "__rt::Mut",
            Slice: "__rt::Slice",
            Repeated: "__rt::Repeated",

            package: names::ident(ty.package()),
            Name: names::ident(ty.name()),
            Ident: names::type_ident(ty),
            Type: names::type_name(ty),
            TDP: format_args!("{}::__tdp_info()", names::type_ident(ty)),
            priv: format_args!("__priv_{}", names::type_ident(ty)),

            NUM_FIELDS: ty.fields().count(),
          },
          |w| match ty.kind() {
            crate::proto::r#type::Kind::Message => message::emit(ty, w),
            crate::proto::r#type::Kind::Struct => {
              ctx
                .warn("sorry: can't emit this kind of type yet")
                .at(ty.span().unwrap());
            }
            crate::proto::r#type::Kind::Choice => choice::emit(ty, w),
            crate::proto::r#type::Kind::Enum => r#enum::emit(ty, w),
          },
        )
      }

      while let Some(m) = stack.pop() {
        w.emit(
          vars! { mod: m },
          "
            } // mod $mod
          ",
        );
      }
      w.new_line();

      w.write(
        "
        // The __ module exports the package universe needed for this module, to
        // simplify cross-type references.
        mod __f { pub use super::*; }
        mod __ {
        use super::__f;
        pub use __f::*;
        ",
      );
      let mut stack = Vec::new();
      for prefix in &prefixes {
        // Quadratic, oops.
        let common_prefix =
          stack.iter().zip(prefix).filter(|(a, b)| a == b).count();
        while stack.len() > common_prefix {
          w.emit(
            vars! { mod: stack.pop().unwrap() },
            "
              } // mod $mod
            ",
          );
        }
        for &m in &prefix[common_prefix..] {
          w.emit(
            vars! { mod: m },
            "
              pub mod $mod {
              use super::__f;
              pub use __f::*;
            ",
          );
          stack.push(m);
        }
      }
      while let Some(m) = stack.pop() {
        w.emit(
          vars! { mod: m },
          "
            } // mod $mod
          ",
        );
      }
      w.write(
        "
        } // mod __
        ",
      );

      ctx.add_file(plugin::codegen_response::File {
        path: Some("lib.pz.rs".into()),
        content: Some(w.to_string().into_bytes()),
      })
    },
  )
}
