use std::io;

fn main() -> io::Result<()> {
  prost_build::compile_protos(
    &["src/pz.proto", "src/plugin.proto"],
    &["src/"],
  )?;
  Ok(())
}
