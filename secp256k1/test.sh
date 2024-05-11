#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

K=19
Path=~/zkWasm-rust-template/keccak
Features="--features continuation,cuda"

# Single test
# Generate continuation proof
cd ~/zkWasm
RUST_LOG=info cargo run --release $Features -- --params $Path/params testwasm setup --host standard -k $K
RUST_LOG=info cargo run --release $Features -- --params $Path/params testwasm dry-run --wasm $Path/pkg/output.wasm --output $Path/output --ctxout ctxout
RUST_LOG=info cargo run --release $Features -- --params $Path/params testwasm prove --wasm $Path/pkg/output.wasm --output $Path/output --ctxout ctxout

## todo
#cd ~/continuation-batcher
#cargo run --release --features cuda -- --params $Path/params --output $Path/output batch -k $K --openschema shplonk --challenge sha --info $Path/output/testwasm.loadinfo.json --name testwasm_agg --commits $Path/batchinfo_empty.json --cont
#cd -

#RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params ./params testwasm verify --output ./output
