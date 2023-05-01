#!/bin/bash
set -x
set -e

cd $(dirname $0)
cargo run -- \
  --plugin=rust \
  --output-dir=src \
  --rust.rt-crate crate \
  src/pz.pz

set +e
cargo build
if [[ $? != 0 ]]; then
  git restore src/lib.pz.rs
fi