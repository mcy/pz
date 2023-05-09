//! Integration tests for the frontend, which consist of executing the pz
//! compiler as a subprocess.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use std::sync::atomic::AtomicU32;

use pz::proto::z::Bundle;

#[test]
fn empty_file() {
  let output = run_pz(
    &[(
      "test.pz",
      &text(
        "
          @edition = 2023
          package test
        ",
      ),
    )],
    &["test.pz"],
  );

  assert_eq!(output.exit_code, 0);
  let bundle = dbg!(Bundle::from_pb(&mut output.bundle.as_slice()).unwrap());
  assert!(bundle.types().is_empty());
}

#[test]
fn intra_dependency() {
  let output = run_pz(
    &[
      (
        "dep.pz",
        &text(
          "
            @edition = 2023
            package test.dep

            message Foo {}
          ",
        ),
      ),
      (
        "test.pz",
        &text(
          "
            @edition = 2023
            package test

            import test.dep { Foo as Rename }

            message Bar {
              1. foo1: Rename
              2. foo2: test.dep.Foo
            }
          ",
        ),
      ),
    ],
    &["test.pz", "dep.pz"],
  );

  assert_eq!(output.exit_code, 0);
  let bundle = dbg!(Bundle::from_pb(&mut output.bundle.as_slice()).unwrap());
  assert_eq!(bundle.types_at(0).name(), "Bar");
  assert_eq!(bundle.types_at(1).name(), "Foo");
}

#[test]
fn extra_dependency() {
  let output = run_pz(
    &[(
      "dep.pz",
      &text(
        "
          @edition = 2023
          package test.dep
          
          message Foo {}
        ",
      ),
    )],
    &["dep.pz"],
  );
  assert_eq!(output.exit_code, 0);

  let output = run_pz(
    &[
      ("dep.pb", &output.bundle),
      (
        "test.pz",
        &text(
          "
            @edition = 2023
            package test

            import test.dep { Foo as Rename }
            
            message Bar {
              1. foo1: Rename
              2. foo2: test.dep.Foo
            }
          ",
        ),
      ),
    ],
    &["--extern=dep.pb", "test.pz"],
  );

  assert_eq!(output.exit_code, 0);
  let bundle = dbg!(Bundle::from_pb(&mut output.bundle.as_slice()).unwrap());
  assert_eq!(bundle.types_at(0).name(), "Bar");
}

#[allow(unused)]
struct PzExit {
  bundle: Vec<u8>,
  stderr: String,
  exit_code: i32,
}

fn text(lit: &str) -> Vec<u8> {
  unindent::unindent(lit).to_string().into()
}

fn run_pz(filesystem: &[(&str, &[u8])], cli: &[&str]) -> PzExit {
  static RUN_COUNT: AtomicU32 = AtomicU32::new(0);
  let tmpdir = PathBuf::from(format!(
    "/tmp/pz.frontend.{}.{}",
    process::id(),
    RUN_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
  ));

  struct Erase(PathBuf);
  impl Drop for Erase {
    fn drop(&mut self) {
      fs::remove_dir_all(&self.0).unwrap();
    }
  }
  let _erase = Erase(tmpdir.clone());

  for (name, contents) in filesystem {
    let path = tmpdir.join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, contents).unwrap()
  }

  let mut exe = env::current_exe().unwrap();
  exe.pop();
  if exe.ends_with("deps") {
    exe.pop();
  }
  exe.push("pzc");

  let output = dbg!(Command::new(exe)
    .arg("--plugin=bundle")
    .arg("--output-dir")
    .arg(&tmpdir)
    .arg("--bundle.out=bundle.pb")
    .args(cli)
    .current_dir(&tmpdir))
  .output()
  .unwrap();

  let stderr = String::from_utf8(output.stderr).unwrap();
  if !output.status.success() {
    eprintln!("{stderr}");
    return PzExit {
      bundle: Vec::new(),
      stderr,
      exit_code: output.status.code().unwrap_or(-1),
    };
  }

  PzExit {
    bundle: fs::read(&tmpdir.join("bundle.pb")).unwrap(),
    stderr,
    exit_code: 0,
  }
}
