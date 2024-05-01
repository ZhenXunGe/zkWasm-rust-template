#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

# Single test
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params testwasm setup -k 18 --wasm ./pkg/output.wasm
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params testwasm prove --wasm ./pkg/output.wasm --output ./output --ctxout ctxout
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params testwasm verify --output ./output
