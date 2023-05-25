#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

# Single test
RUST_LOG=info ~/zkWasm/target/release/cli  -k 18 --function zkmain --output ./output --wasm ./pkg/output.wasm setup

RUST_LOG=info ~/zkWasm/target/release/cli  -k 18 --function zkmain --output ./output --wasm ./pkg/output.wasm single-prove --public 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001d58da95562c3d482f78bb86de18bae100000000000000000000000000000000d9aa2c9fa71e6ea73d8e65b51043fd8a00000000000000000000000000000000:bytes-packed --private 0x010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000d91a86b4d8551290655caced21856ef6e532f2d4:bytes-packed

RUST_LOG=info ~/zkWasm/target/release/cli  -k 18 --function zkmain --output ./output --wasm ./pkg/output.wasm single-verify --proof output/zkwasm.0.transcript.data
