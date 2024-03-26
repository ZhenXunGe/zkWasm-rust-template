#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

K=22

# Single test
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --host default -k $K --function zkmain --param params --output ./output --wasm ./pkg/output.wasm setup
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --host default -k $K --function zkmain --param params --output ./output --wasm ./pkg/output.wasm single-prove
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --host default -k $K --function zkmain --param params --output ./output --wasm ./pkg/output.wasm single-verify
