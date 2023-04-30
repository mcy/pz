//! Compiler driver binary.

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use clap::Parser;
use prost::Message;

use pz::ir;
use pz::proto::plugin;
use pz::report::Report;
use pz::syn;

/// Protobuf but with pizzazz
#[derive(clap::Parser, Debug)]
#[command(version, about)]
struct Pz {
  /// Where to output the generated files to; defaults to the current director.
  #[arg(short, long)]
  output_dir: Option<PathBuf>,

  /// What plugin to execute. This can either be the name of a built-in plugin,
  /// or the path to a plugin binary.
  #[arg(long)]
  plugin: Option<String>,

  input: PathBuf,
}

fn main() {
  match env::var("_PZ_SELF_EXEC").as_ref().map(|x| x.as_str()) {
    Ok("bundle") => pz::plugin::bundle_plugin(),
    _ => {}
  }

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

  let bundle_proto = bundle.to_proto();
  let req = plugin::Request {
    requested_indices: (0..bundle_proto.types.len() as u32).collect(),
    bundle: Some(bundle_proto),
    options: HashMap::new(), // TODO
    debug: Some(env::var_os("PZ_DEBUG").is_some()),
  };
  let req_bytes = req.encode_to_vec();

  let plugin = match opts.plugin.as_deref().unwrap_or("bundle") {
    plugin @ "bundle" => {
      env::set_var("_PZ_SELF_EXEC", plugin);
      env::current_exe().unwrap()
    }
    plugin => plugin.into(),
  };

  let spawn_result = Command::new(&plugin)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::inherit())
    .spawn();

  let mut child = match spawn_result {
    Ok(c) => c,
    Err(e) => report.fatal(
      &scx,
      2,
      format_args!("could not spawn plugin {}: {e}", plugin.display()),
    ),
  };

  if let Err(e) = child.stdin.take().unwrap().write_all(&req_bytes) {
    report.fatal(
      &scx,
      2,
      format_args!("could not send request to plugin: {e}"),
    );
  }

  let exit = match child.wait_with_output() {
    Ok(x) => x,
    Err(e) => report.fatal(
      &scx,
      2,
      format_args!("plugin did not exit successfully: {e}"),
    ),
  };

  if exit.status.code() != Some(0) {
    report.fatal(
      &scx,
      2,
      format_args!("plugin returned abnormally: $? = {exit:?}"),
    );
  }

  let resp = match plugin::Response::decode(exit.stdout.as_slice()) {
    Ok(resp) => resp,
    Err(e) => report.fatal(
      &scx,
      2,
      format_args!("plugin returned malformed response: {e}"),
    ),
  };

  for file in &resp.files {
    let mut path = opts.output_dir.clone().unwrap_or(PathBuf::new());
    path.push(file.path());

    if let Err(e) = fs::write(&path, file.content()) {
      report.error(format_args!(
        "could not write output {}: {e}",
        path.display()
      ));
    }
  }

  report.dump_and_die(&scx, 2);
}
