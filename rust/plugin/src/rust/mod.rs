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
      options: vec![plugin::about_response::Option {
        name: Some("rt-crate".into()),
        help: Some(
          "Rust crate name to use for importing the pz runtime".into(),
        ),
      }],
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

      for ty in ctx.types_to_generate() {
        w.with_vars(
          vars! {
            deprecated: deprecated(
              ty.proto().attrs.as_ref().and_then(|a| a.deprecated.as_deref())),
            NonNull: "__s::ptr::NonNull",
            Layout: "__s::alloc::Layout",
            PhantomData: "__s::marker::PhantomData",
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

      ctx.add_file(plugin::codegen_response::File {
        path: Some("lib.pz.rs".into()),
        content: Some(w.to_string().into_bytes()),
      })
    },
  )
}
