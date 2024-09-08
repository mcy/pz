//! Compiler driver binary.

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use clap::Arg;
use clap::ArgAction;
use clap::CommandFactory;
use clap::FromArgMatches;
use ilex::report::Report;
use pz::prost::Message;

use pz::proto::plugin;

use pzc::ir;
use pzc::syn;

#[derive(clap::Parser, Debug)]
#[command(about, disable_help_flag(true), arg_required_else_help(true))]
struct Pzc {
  /// Prints this help.
  #[arg(long, short)]
  help: bool,

  /// Prints the version.
  #[arg(long, short = 'V')]
  version: bool,

  /// What plugin to execute; this can either be the name of a built-in plugin,
  /// or the path to a plugin binary.
  #[arg(long)]
  plugin: Option<String>,

  /// Where to output the generated files to; defaults to the current directory.
  #[arg(long)]
  output_dir: Option<PathBuf>,

  /// Bundle files to use for resolving imports.
  #[arg(long = "extern")]
  bundles: Vec<PathBuf>,

  /// The `.pz` files to pass to the plugin.
  files: Vec<String>,
}

fn expect<T, E: fmt::Display>(
  report: &Report,
  msg: impl fmt::Display,
  res: Result<T, E>,
) -> Result<T, ilex::Fatal> {
  match res {
    Ok(x) => Ok(x),
    Err(e) => {
      report.error(format_args!("{msg}: {e}"));
      report.fatal()
    }
  }
}

fn run_plugin(
  plugin: &Path,
  req: &plugin::Request,
  report: &Report,
) -> Result<plugin::Response, ilex::Fatal> {
  let mut child = expect(
    report,
    format_args!("could not spawn plugin {}", plugin.display()),
    Command::new(plugin)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .stderr(Stdio::inherit())
      .env("RUST_BACKTRACE", "1")
      .spawn(),
  )?;

  expect(
    report,
    "could not send request to plugin",
    child.stdin.take().unwrap().write_all(&req.encode_to_vec()),
  )?;

  let exit = expect(
    report,
    "plugin did not exit successfully",
    child.wait_with_output(),
  )?;

  if exit.status.code() != Some(0) {
    expect(
      report,
      "plugin returned abnormally",
      Err(format_args!("$? = {:?}", exit.status)),
    )?;
  }

  expect(
    report,
    "plugin returned malformed response",
    plugin::Response::decode(exit.stdout.as_slice()),
  )
}

fn main() {
  match env::var("_PZ_SELF_EXEC").as_deref() {
    Ok("bundle") => pz_plugins::bundle_plugin(),
    Ok("rust") => pz_plugins::rust_plugin(),
    _ => {}
  }

  let mut ctx = ilex::Context::new();
  let report = ctx.new_report();
  let options = ilex::ice::Options {
    show_backtrace: env::var_os("PZ_BRACKTRACE").map(|val| val != "0"),
    what_panicked: Some("the compiler".into()),
    report_bugs_at: Some("https://github.com/mcy/pz/issues".into()),
    extra_notes: Vec::new(),
  };

  let result =
    ilex::ice::handle(&mut ctx, &report, options, |ctx| pzc(ctx, &report));
  if let Err(e) = result {
    e.terminate();
  }
}

fn pzc(icx: &ilex::Context, report: &Report) -> Result<(), ilex::Fatal> {
  let _dbg = icx.use_for_debugging_spans();

  match env::var("_PZ_SELF_EXEC").as_deref() {
    Ok("bundle") => pz_plugins::bundle_plugin(),
    Ok("rust") => pz_plugins::rust_plugin(),
    _ => {}
  }

  let args = env::args_os().collect::<Vec<_>>();
  let mut plugin_name = None;
  for (i, arg) in args.iter().enumerate() {
    plugin_name = if arg == "--plugin" {
      args.get(i + 1).and_then(|s| s.to_str())
    } else if let Some(arg) =
      arg.to_str().filter(|s| s.starts_with("--plugin="))
    {
      arg.strip_prefix("--plugin=")
    } else {
      continue;
    };

    break;
  }

  let plugin = match plugin_name {
    plugin @ (None | Some("bundle") | Some("rust")) => {
      env::set_var("_PZ_SELF_EXEC", plugin.unwrap_or("bundle"));
      env::current_exe().unwrap()
    }
    Some(plugin) => plugin.into(),
  };

  let req = plugin::Request {
    value: Some(plugin::request::Value::About(plugin::AboutRequest {})),
  };
  let resp = match run_plugin(&plugin, &req, report)?.value {
    Some(plugin::response::Value::About(a)) => a,
    _ => {
      report
        .error("plugin returned malformed response: expected AboutResponse");
      return report.fatal();
    }
  };

  let mut plugin_command = Pzc::command().help_template(format!(
    "\
Usage: {{name}} {plugin}[OPTIONS] <FILES>

Options:
{{options}}
",
    plugin = plugin_name
      .map(|p| format!("--plugin {p:?} "))
      .unwrap_or_default()
  ));
  for opt in &resp.options {
    let name = format!("{}.{}", resp.name(), opt.name());
    plugin_command = plugin_command.arg(
      Arg::new(name.clone())
        .long(name)
        .action(ArgAction::Set)
        .value_parser(clap::value_parser!(String))
        .value_name("ARG")
        .help(opt.help().to_string()),
    );
  }

  let opts = plugin_command
    .clone()
    .try_get_matches()
    .and_then(|opts| Ok((Pzc::from_arg_matches(&opts)?, opts)));
  let (opts, plugin_opts) = match opts {
    Ok(opts) => opts,
    Err(e) => {
      use clap::error::ErrorKind::*;
      if matches!(
        e.kind(),
        DisplayHelp | DisplayVersion | DisplayHelpOnMissingArgumentOrSubcommand
      ) {
        e.exit();
      }

      let text = e.to_string();
      let message = text.trim_start_matches("error: ");
      let message =
        message[..message.find("Usage: pz").unwrap_or(message.len())].trim();

      report.error(message);
      return report.fatal();
    }
  };

  if opts.help {
    plugin_command.print_help().unwrap();
    return Ok(());
  }

  if opts.version {
    eprintln!(
      "pz v{} / {} v{}",
      env!("CARGO_PKG_VERSION"),
      resp.name(),
      resp.version()
    );
    return Ok(());
  }

  let mut options = HashMap::new();
  for opt in &resp.options {
    let name = format!("{}.{}", resp.name(), opt.name());
    if let Some(val) = plugin_opts.get_one::<String>(&name) {
      options.insert(opt.name().to_string(), val.to_string());
    }
  }

  if opts.files.is_empty() {
    report.error("missing input filename");
    return report.fatal();
  }

  let contents = opts
    .files
    .iter()
    .filter_map(|path| icx.open_file(path, report).ok())
    .collect::<Vec<_>>();
  report.fatal_or(())?;

  let bundles = opts
    .bundles
    .iter()
    .filter_map(|path| match fs::read(path) {
      Ok(data) => match pz::proto::Bundle::decode(&*data) {
        Ok(b) => Some((path.to_string_lossy().into_owned(), b)),
        Err(e) => {
          report
            .error(format_args!("could not process --extern argument: {e}"));
          None
        }
      },

      Err(e) => {
        report.error(format_args!("could not process --extern argument: {e}"));
        None
      }
    })
    .collect::<Vec<_>>();
  report.fatal_or(())?;

  let tokens = contents
    .iter()
    .filter_map(|file| syn::lex(*file, report).ok())
    .collect::<Vec<_>>();
  report.fatal_or(())?;

  let files = tokens
    .iter()
    .filter_map(|file| syn::PzFile::parse(file, report).ok())
    .collect::<Vec<_>>();
  report.fatal_or(())?;

  let rcx = ir::ResolveCtx::new(icx, report);
  let bundle = rcx.resolve(&bundles, &files);
  report.fatal_or(())?;

  let (bundle_proto, spans) = bundle.unwrap().to_proto(icx);
  let req = plugin::Request {
    value: Some(plugin::request::Value::Codegen(plugin::CodegenRequest {
      requested_indices: (0..bundle_proto.types.len() as u32).collect(),
      bundle: Some(bundle_proto),
      options,
      debug: Some(env::var_os("PZ_DEBUG").is_some()),
    })),
  };

  let resp = match run_plugin(&plugin, &req, report)?.value {
    Some(plugin::response::Value::Codegen(a)) => a,
    _ => {
      report
        .error("plugin returned malformed response: expected CodegenResponse");
      return report.fatal();
    }
  };

  for diagnostic in &resp.report {
    let mut d = match diagnostic.kind() {
      plugin::diagnostic::Kind::Error => report.error(diagnostic.message()),
      plugin::diagnostic::Kind::Warning => report.warn(diagnostic.message()),
    };
    for snippet in &diagnostic.snippets {
      if snippet.is_remark() {
        d = d.remark(spans[&snippet.span()], snippet.message());
      } else {
        d = d.saying(spans[&snippet.span()], snippet.message());
      }
    }
    for note in &diagnostic.notes {
      d = d.note(note);
    }
  }
  report.fatal_or(())?;

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

  report.fatal_or(())
}
