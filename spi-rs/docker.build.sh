#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

cat .env >.docker.env
name=$(basename $DIR)
docker build -t $name .
docker tag $name $name:latest
