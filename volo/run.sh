#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -e
. ../sh/pid.sh
set -a
. ../conf/r.env
set +a
set -x

exec cargo run $@
