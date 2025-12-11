#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

../sh/cargo_install.sh volo-cli volo

SRV=../srv

[ -d "$SRV/gen" ] &&
  find $SRV/gen -mindepth 1 -not -path "$SRV/gen/volo/lib.rs" -delete

# rs2proto $SRV

~/js0/grpc/rs2proto/build.sh $SRV
~/js0/grpc/rs2proto/lib/cli.js $SRV

rm -rf volo-gen
volo init --includes=$SRV/gen js0_volo $SRV/gen/api.proto
cd src
rm *.rs
ln -s ../../srv/gen/volo/*.rs .
cd ..
cargo fmt
