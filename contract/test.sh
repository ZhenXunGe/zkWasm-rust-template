#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

ZKWASM_DIR="/home/xgao/zkWasm";
ZKWASM_CLI=$ZKWASM_DIR/target/release/zkwasm-cli
ZKWASM_PARAMS=$ZKWASM_DIR/params

$ZKWASM_CLI --params $ZKWASM_PARAMS flp setup
$ZKWASM_CLI --params $ZKWASM_PARAMS flp dry-run --wasm ./pkg/output.wasm --output ./output --private 1:i64,1:i64,1:i64,1:i64,1:i64
$ZKWASM_CLI --params $ZKWASM_PARAMS flp prove --wasm ./pkg/output.wasm --output ./output --private 1:i64,1:i64,1:i64,1:i64,1:i64
$ZKWASM_CLI --params $ZKWASM_PARAMS flp verify --wasm ./pkg/output.wasm --output ./output
