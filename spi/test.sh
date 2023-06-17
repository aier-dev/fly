#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

if ! [ -x "$(command -v msgpack-cli)" ]; then
  go install github.com/jakm/msgpack-cli@master
fi

curl http://127.0.0.1:8080/sampler --output - | msgpack-cli decode
