#!/usr/bin/env bash

DIR=$(realpath ${0%/*})
cd $DIR
set -ex

if ! command -v fly &>/dev/null; then
  curl -L https://fly.io/install.sh | sh
fi
./env.sh
flyctl machine destroy $(flyctl machine list | grep -v '^[[:space:]]*$' | tail -n 1 | awk '{print $1}') --force || true
fly deploy
