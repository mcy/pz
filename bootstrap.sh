#!/bin/bash
set -x
set -e
cd $(dirname $0)

build() {
  path=$1
  out=$2
  rt=$3

  cp $out $out.bck
  
  set -e
  cargo run -- \
    --plugin=rust \
    --output-dir=$(dirname $out) \
    --rust.rt-crate $rt \
    $path

  set +e
  cargo build
  if [[ $? != 0 ]]; then
    mv $out.bck $out 
  else
    rm $out.bck
  fi
}

build src/pz.pz src/lib.pz.rs crate
build tests/test.pz tests/proto/lib.pz.rs pz