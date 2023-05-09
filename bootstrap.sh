#!/bin/bash
set -x
set -e
cd $(dirname $0)

build() {
  rt=$1; shift
  out=$1; shift

  cp $out $out.bck
  
  set -e
  cargo run -- \
    --plugin=rust \
    --output-dir=$(dirname $out) \
    --rust.rt-crate $rt \
    $@

  set +e
  cargo build
  if [[ $? != 0 ]]; then
    mv $out.bck $out 
  else
    rm $out.bck
  fi
}

build crate rust/src/proto/lib.pz.rs rust/src/proto/*.pz
build pz rust/tests/proto/lib.pz.rs rust/tests/proto/*.pz 