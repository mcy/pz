//! Compiler driver binary.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use prost::Message;

use pz::ir;
use pz::report::Report;
use pz::syn;

/// Protobuf but with pizzazz
#[derive(clap::Parser, Debug)]
#[command(version, about)]
struct Pz {
  /// Where to output the generated bundle to; defaults to `bundle.pb`.
  #[arg(short, long)]
  output: Option<PathBuf>,

  input: PathBuf,
}

fn main() {
  let opts = Pz::parse();

  let mut report = Report::new();

  let mut scx = syn::SourceCtx::new();
  let file = scx.open_file(&opts.input, &mut report);
  report.dump_and_die(&scx, 2);

  let file = syn::PzFile::parse(file.unwrap(), &mut scx, &mut report);
  report.dump_and_die(&scx, 2);
  let file = file.unwrap();

  let rcx = ir::ResolveCtx::new(&scx);
  let bundle = rcx.resolve(&file, &mut report);
  report.dump_and_die(&scx, 2);

  let pz_bundle = bundle.to_pz();
  let binary = pz_bundle.encode_to_vec();

  let out = opts.output.as_deref().unwrap_or(Path::new("bundle.pb"));
  if let Err(e) = fs::write(out, binary) {
    report.error(format_args!(
      "could not write output {}: {e}",
      out.display()
    ));
  }
  report.dump_and_die(&scx, 2);
}
