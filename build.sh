#!/bin/bash
set -e

cd $1
cargo build --release
cargo build
cd - >/dev/null
mv $1/target/release/$1 ./bin/$1.release
mv $1/target/debug/$1 ./bin/$1.debug
