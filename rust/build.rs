use std::io;

fn main() -> io::Result<()> {
  prost_build::compile_protos(
    &["src/proto/pz.proto", "src/proto/plugin.proto"],
    &["src/proto"],
  )?;
  Ok(())
}
