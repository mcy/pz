#!/bin/bash
set -x
set -e

cp src/lib.pz.rs src/lib.pz.rs.bck

cd $(dirname $0)
cargo run -- \
  --plugin=rust \
  --output-dir=src \
  --rust.rt-crate crate \
  src/pz.pz

set +e
cargo build
if [[ $? != 0 ]]; then
  mv src/lib.pz.rs.bck src/lib.pz.rs 
else
  rm src/lib.pz.rs.bck
fi