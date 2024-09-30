#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

ZKWASM_DIR="/home/xgao/zkWasm";
ZKWASM_CLI=$ZKWASM_DIR/target/release/zkwasm-cli
ZKWASM_PARAMS=$ZKWASM_DIR/params

$ZKWASM_CLI --params $ZKWASM_PARAMS flp setup
$ZKWASM_CLI --params $ZKWASM_PARAMS flp dry-run --wasm ./pkg/output.wasm --output ./output
$ZKWASM_CLI --params $ZKWASM_PARAMS flp prove --wasm ./pkg/output.wasm --output ./output
$ZKWASM_CLI --params $ZKWASM_PARAMS flp verify --output ./output

#RUST_LOG=info cargo run --release $Features -- --params params testwasm prove --wasm $Path/pkg/output.wasm --output $Path/output --ctxout ctxout
