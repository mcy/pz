//! Rust codegen backend.

use crate::plugin::emit;
use crate::plugin::exec_plugin;
use crate::plugin::rust::names::deprecated;
use crate::proto::plugin;

mod r#enum;
mod message;
mod names;

pub fn rust_plugin() -> ! {
  exec_plugin(
    |_| plugin::AboutResponse {
      name: Some("rust".into()),
      version: Some(env!("CARGO_PKG_VERSION").into()),
      options: vec![plugin::about_response::Option {
        name: Some("rt-crate".into()),
        help: Some(
          "Rust crate name to use for importing the pz runtime".into(),
        ),
      }],
    },
    |ctx| {
      let mut w = emit::SourceWriter::new(emit::Options::default());
      w.emit(
        [],
        r"
        // ! ! ! GENERATED CODE, DO NOT EDIT ! ! !
        #![cfg_attr(rustfmt, rustfmt_skip)]
        #![allow(non_camel_case_types)]
        #![allow(non_upper_case_globals)]
      ",
      );

      let rt = ctx.option("rt-crate").unwrap_or("pz");
      if rt == "crate" {
        w.emit(
          vars! {},
          r"
          use crate as __rt;
        ",
        );
      } else {
        w.emit(
          vars! { rt },
          r"
          extern $rt as __rt;
        ",
        );
      }
      w.new_line();

      for ty in ctx.types_to_generate() {
        w.with_vars(
          vars! {
            deprecated: deprecated(
              ty.proto().attrs.as_ref().and_then(|a| a.deprecated.as_deref())),
          },
          |w| match ty.kind() {
            crate::proto::r#type::Kind::Message => message::emit(ty, w),
            crate::proto::r#type::Kind::Struct => {
              ctx
                .warn("sorry: can't emit this kind of type yet")
                .at(ty.span().unwrap());
            }
            crate::proto::r#type::Kind::Choice => {
              ctx
                .warn("sorry: can't emit this kind of type yet")
                .at(ty.span().unwrap());
            }
            crate::proto::r#type::Kind::Enum => r#enum::emit(ty, w),
          },
        )
      }

      ctx.add_file(plugin::codegen_response::File {
        path: Some("lib.pz.rs".into()),
        content: Some(w.to_string().into_bytes()),
      })
    },
  )
}
