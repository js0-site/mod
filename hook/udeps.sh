#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*/*}
cd $DIR
set -x

./volo/gen.sh
