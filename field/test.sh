#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

ZKWASM_DIR="/home/xgao/debug/zkWasm";
ZKWASM_CLI=$ZKWASM_DIR/target/release/zkwasm-cli
ZKWASM_PARAMS=$ZKWASM_DIR/params

$ZKWASM_CLI --params $ZKWASM_PARAMS field setup --host standard
$ZKWASM_CLI --params $ZKWASM_PARAMS field dry-run --wasm ./pkg/output.wasm --output ./output
$ZKWASM_CLI --params $ZKWASM_PARAMS field prove --wasm ./pkg/output.wasm --output ./output
$ZKWASM_CLI --params $ZKWASM_PARAMS field verify --output ./output

# TODO: FIX BATCH
#cd ~/continuation-batcher
#cargo run --release --features cuda -- --params $Path/params --output $Path/output batch -k $K --openschema shplonk --challenge sha --info $Path/output/testwasm.loadinfo.json --name testwasm_agg --commits $Path/batchinfo_empty.json --cont
#cd -

#RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params ./params testwasm verify --output ./output
