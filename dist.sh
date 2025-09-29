#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex
rm -rf rustc-ice-*.txt
touch Cargo.lock
git add -u || true
git commit -m. || true
cargo v patch -y
git add -u || true
git commit -m. || true
git pull || true
git push || true
cargo publish
