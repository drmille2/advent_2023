#!/bin/bash
set -e

cd $1
cargo build --release
cd - >/dev/null
mv $1/target/release/$1 ./$1.release
