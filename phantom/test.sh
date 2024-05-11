#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

K=19
Path=~/zkWasm-rust-template/phantom
Features="--features cuda"

# Single test
cd ~/zkWasm
RUST_LOG=info cargo run --release $Features -- --params params testwasm setup --host standard -k $K --wasm $Path/pkg/output.wasm
RUST_LOG=info cargo run --release $Features -- --params params testwasm prove --wasm $Path/pkg/output.wasm --output $Path/output --ctxout ctxout --private 1:i64
RUST_LOG=info cargo run --release $Features -- --params params testwasm verify --output $Path/output
