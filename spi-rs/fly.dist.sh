#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

cat .env >.docker.env
fly deploy
