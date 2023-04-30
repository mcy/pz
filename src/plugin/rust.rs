//! Rust codegen backend.

use crate::plugin::exec_plugin;
use crate::plugin::CodegenCtx;
use crate::proto::plugin;

pub fn rust_plugin() -> ! {
  exec_plugin(
    |_| plugin::AboutResponse {
      name: Some("rust".into()),
      version: Some(env!("CARGO_PKG_VERSION").into()),
      options: vec![],
    },
    |ctx| {
      for ty in ctx.types_to_generate() {
        ctx
          .warn(format_args!("skipping type {}.{}", ty.package(), ty.name()))
          .at(ty.span().unwrap());
      }
    },
  )
}
