#!/bin/bash

#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

ZKWASM_DIR="/home/xgao/zkWasm";
ZKWASM_CLI=$ZKWASM_DIR/target/release/zkwasm-cli
ZKWASM_PARAMS=$ZKWASM_DIR/params

$ZKWASM_CLI --params $ZKWASM_PARAMS keccak setup
$ZKWASM_CLI --params $ZKWASM_PARAMS keccak dry-run --wasm ./pkg/output.wasm --output ./output
$ZKWASM_CLI --params $ZKWASM_PARAMS keccak prove --wasm ./pkg/output.wasm --output ./output
$ZKWASM_CLI --params $ZKWASM_PARAMS keccak verify --wasm ./pkg/output.wasm --output ./output
