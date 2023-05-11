//! Plugins!

use std::cell;
use std::cell::RefCell;
use std::env;
use std::fmt;
use std::io;
use std::io::Read;
use std::io::Write;

use prost::Message;

use pz::proto;
use pz::proto::plugin;

#[macro_use]
mod emit;
mod rust;

pub use rust::rust_plugin;

/// The context for the current codegen operation.
pub struct CodegenCtx {
  req: plugin::CodegenRequest,
  resp: RefCell<plugin::CodegenResponse>,
}

impl CodegenCtx {
  /// Returns the bundle that was sent as part of the request.
  pub fn bundle(&self) -> &proto::Bundle {
    self.req.bundle.as_ref().unwrap()
  }

  /// Returns an iterator over types that we need to generate.
  pub fn types_to_generate<'ccx>(
    &'ccx self,
  ) -> impl Iterator<Item = Type<'ccx>> + 'ccx {
    self.req.requested_indices.iter().map(|&idx| Type {
      ctx: self,
      bundle: self.bundle(),
      proto: &self.bundle().types[idx as usize],
    })
  }

  /// Looks up an option in the options array.
  pub fn option(&self, name: &str) -> Option<&str> {
    self.req.options.get(name).map(|x| &**x)
  }

  /// Returns whether the `pz` driver has requested we operate in "debug mode".
  pub fn is_debug_mode(&self) -> bool {
    self.req.debug()
  }

  /// Adds a file to the response. The file paths are relative to the output
  /// directory of the driver process.
  pub fn add_file(&self, file: plugin::codegen_response::File) {
    self.resp.borrow_mut().files.push(file);
  }

  /// Adds a new error to the response.
  pub fn error(&self, msg: impl fmt::Display) -> Diagnostic<'_> {
    Diagnostic {
      proto: cell::RefMut::map(self.resp.borrow_mut(), |resp| {
        resp.report.push(plugin::Diagnostic {
          message: Some(msg.to_string()),
          kind: Some(plugin::diagnostic::Kind::Error.into()),
          ..Default::default()
        });
        resp.report.last_mut().unwrap()
      }),
    }
  }

  pub fn warn(&self, msg: impl fmt::Display) -> Diagnostic<'_> {
    Diagnostic {
      proto: cell::RefMut::map(self.resp.borrow_mut(), |resp| {
        resp.report.push(plugin::Diagnostic {
          message: Some(msg.to_string()),
          kind: Some(plugin::diagnostic::Kind::Warning.into()),
          ..Default::default()
        });
        resp.report.last_mut().unwrap()
      }),
    }
  }
}

/// A diagnostic that is being built up.
pub struct Diagnostic<'ccx> {
  proto: cell::RefMut<'ccx, plugin::Diagnostic>,
}

impl Diagnostic<'_> {
  /// Adds a new relevant snippet at the given location.
  pub fn at(self, span: Span) -> Self {
    self.saying(span, "")
  }

  /// Adds a new relevant snippet at the given location, with the given message
  /// attached to it.
  pub fn saying(mut self, span: Span, message: impl fmt::Display) -> Self {
    self.proto.snippets.push(plugin::diagnostic::Snippet {
      span: Some(span.0),
      message: Some(message.to_string()),
      is_remark: Some(false),
    });
    self
  }

  /// Like `saying`, but the underline is as for a "note" rather than the
  /// overall diagnostic.
  pub fn remark(mut self, span: Span, message: impl fmt::Display) -> Self {
    self.proto.snippets.push(plugin::diagnostic::Snippet {
      span: Some(span.0),
      message: Some(message.to_string()),
      is_remark: Some(true),
    });
    self
  }

  /// Appends a note to the bottom of the diagnostic.
  pub fn note(&mut self, message: impl fmt::Display) -> &mut Self {
    self.proto.notes.push(message.to_string());
    self
  }
}

#[derive(Copy, Clone)]
pub struct Span(u32);

#[derive(Copy, Clone)]
pub struct Type<'ccx> {
  ctx: &'ccx CodegenCtx,
  bundle: &'ccx proto::Bundle,
  proto: &'ccx proto::Type,
}

impl<'ccx> Type<'ccx> {
  pub fn ccx(&self) -> &'ccx CodegenCtx {
    self.ctx
  }

  pub fn package(&self) -> &str {
    &self.bundle.packages[self.proto.package() as usize]
  }

  pub fn name(&self) -> &str {
    self.proto.name()
  }

  pub fn kind(&self) -> proto::r#type::Kind {
    self.proto.kind()
  }

  pub fn parent(&self) -> Option<Type<'ccx>> {
    self.proto.declared_in.map(|idx| Type {
      ctx: self.ctx,
      bundle: self.bundle,
      proto: &self.bundle.types[idx as usize],
    })
  }

  pub fn nesteds(&self) -> impl Iterator<Item = Type<'ccx>> + '_ {
    self.proto.nesteds.iter().map(|&idx| Type {
      ctx: self.ctx,
      bundle: self.bundle,
      proto: &self.bundle.types[idx as usize],
    })
  }

  pub fn fields(&self) -> impl Iterator<Item = Field<'ccx>> + '_ {
    self.proto.fields.iter().enumerate().map(|(i, f)| Field {
      ctx: self.ctx,
      bundle: self.bundle,
      proto: f,
      parent: self.proto,
      index: i as u32,
    })
  }

  pub fn span(&self) -> Option<Span> {
    self.proto.span.map(Span)
  }

  pub fn proto(&self) -> &'ccx proto::Type {
    self.proto
  }
}

#[derive(Copy, Clone)]
pub struct Field<'ccx> {
  ctx: &'ccx CodegenCtx,
  bundle: &'ccx proto::Bundle,
  proto: &'ccx proto::Field,
  parent: &'ccx proto::Type,
  index: u32,
}

impl<'ccx> Field<'ccx> {
  pub fn ccx(&self) -> &'ccx CodegenCtx {
    self.ctx
  }

  pub fn name(&self) -> &str {
    self.proto.name()
  }

  pub fn number(&self) -> Option<i32> {
    self.proto.number
  }

  pub fn index(&self) -> u32 {
    self.index
  }

  pub fn is_repeated(&self) -> bool {
    self.proto.is_repeated()
  }

  pub fn parent(&self) -> Type<'ccx> {
    Type {
      ctx: self.ctx,
      bundle: self.bundle,
      proto: self.parent,
    }
  }

  pub fn ty(&self) -> (proto::field::Type, Option<Type<'ccx>>) {
    let kind = self.proto.r#type();
    if kind == proto::field::Type::Type {
      return (
        kind,
        Some(Type {
          ctx: self.ctx,
          bundle: self.bundle,
          proto: &self.bundle.types[self.proto.type_index() as usize],
        }),
      );
    }

    (kind, None)
  }

  pub fn span(&self) -> Option<Span> {
    self.proto.span.map(Span)
  }

  pub fn proto(&self) -> &'ccx proto::Field {
    self.proto
  }
}

/// Executes a plugin main function.
///
/// This function should be called in the `main` function of a program that
/// implements a codegen backend.
pub fn exec_plugin(
  about: impl FnOnce(&plugin::AboutRequest) -> plugin::AboutResponse,
  codegen: impl FnOnce(&CodegenCtx),
) -> ! {
  let mut input = Vec::new();
  io::stdin()
    .read_to_end(&mut input)
    .expect("failed to read request proto");

  let mut req = plugin::Request::decode(input.as_slice())
    .expect("failed to parse request proto");
  let mut resp = plugin::Response::default();

  match req.value.take() {
    Some(plugin::request::Value::About(req)) => {
      resp.value = Some(plugin::response::Value::About(about(&req)));
    }
    Some(plugin::request::Value::Codegen(req)) => {
      let ctx = CodegenCtx {
        req,
        resp: Default::default(),
      };
      codegen(&ctx);
      resp.value = Some(plugin::response::Value::Codegen(RefCell::into_inner(
        ctx.resp,
      )));
    }
    None => panic!("unknown request proto"),
  }

  io::stdout()
    .write(&resp.encode_to_vec())
    .expect("failed to write response proto");

  std::process::exit(0);
}

/// Runs the "trivial" bundle plugin that simply echoes the request bundle.
pub fn bundle_plugin() -> ! {
  exec_plugin(
    |_| plugin::AboutResponse {
      name: Some("bundle".into()),
      version: Some(env!("CARGO_PKG_VERSION").into()),
      options: vec![plugin::about_response::Option {
        name: Some("out".into()),
        help: Some(
          "The file to write the bundle proto to; defaults to \"bundle.pb\""
            .into(),
        ),
      }],
    },
    |ctx| {
      ctx.add_file(plugin::codegen_response::File {
        path: Some(ctx.option("out").unwrap_or("bundle.pb").into()),
        content: Some(ctx.bundle().encode_to_vec()),
      });
    },
  )
}
