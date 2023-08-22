#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex

rm -f .env
echo "APG_URI=$APG_URI" >>.env
