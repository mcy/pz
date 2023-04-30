//! Compiler driver binary.

use std::fs;

use clap::Parser;

use prost::Message;
use pz::ir::resolve::ResolveCtx;
use pz::pz as proto;
use pz::report;
use pz::report::Report;
use pz::syn;

/// Protobuf but with pizzazz
#[derive(clap::Parser, Debug)]
#[command(version, about)]
struct Pz {
  /// Where to output the generated bundle to; defaults to `bundle.pb`.
  #[arg(short, long)]
  output: Option<String>,

  input: String,
}

fn main() {
  let opts = Pz::parse();

  let text = fs::read_to_string(&opts.input).unwrap();
  let file = proto::File {
    path: Some(opts.input),
    text: Some(text),
  };

  let mut report = Report::new();
  let report_opts = report::RenderOptions {
    color: true,
    show_report_locations: std::env::var_os("PZ_DEBUG").is_some(),
  };

  let mut ctx = syn::Context::new(&file);
  let file = syn::PzFile::parse(&mut ctx, &mut report);
  report.dump_and_die(&ctx, &report_opts, 2);
  let file = file.unwrap();

  let rcx = ResolveCtx::new(&ctx);
  let bundle = rcx.resolve(&file, &mut report);
  report.dump_and_die(&ctx, &report_opts, 2);

  let pz_bundle = bundle.to_pz();
  let binary = pz_bundle.encode_to_vec();
  fs::write(opts.output.as_deref().unwrap_or("bundle.pb"), binary).unwrap();
}
