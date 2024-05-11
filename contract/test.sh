#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

K=18
Path=~/zkWasm-rust-template/contract
Features="--features cuda"

# Single test
cd ~/zkWasm
RUST_LOG=info cargo run --release $Features -- --params params testwasm setup -k $K --wasm $Path/pkg/output.wasm
RUST_LOG=info cargo run --release $Features -- --params params testwasm prove --wasm $Path/pkg/output.wasm --output $Path/output --ctxout ctxout --private 1:i64,1:i64,1:i64,1:i64,1:i64
RUST_LOG=info cargo run --release $Features -- --params params testwasm verify --output $Path/output
