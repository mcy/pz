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

build crate src/lib.pz.rs src/pz.pz src/plugin.pz
build pz tests/proto/lib.pz.rs tests/test.pz 