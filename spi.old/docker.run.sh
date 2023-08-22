#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

./build.sh

name=spi

docker stop $name || true
docker build . -t $name
docker run -it -e PG_URI=$PG_URI -p 8080:8080 --name $name --rm $name
