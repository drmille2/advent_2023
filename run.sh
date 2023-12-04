#!/bin/bash
set -e

cd $1 
cargo run -- --input $2 
cd - >/dev/null
