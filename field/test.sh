#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

# Single test
#RUST_LOG=info ~/zkWasm/target/release/delphinus-cli  -k 19 --function zkmain --param params --output ./output --wasm ./pkg/output.wasm setup
#RUST_LOG=info ~/zkWasm/target/release/delphinus-cli  --host standard -k 19 --function zkmain --param params --output ./output --wasm ./pkg/output.wasm dry-run
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli  --host standard -k 19 --function zkmain --param params --output ./output --wasm ./pkg/output.wasm dry-run
#RUST_LOG=info ~/zkWasm/target/release/delphinus-cli  --host standard -k 19 --function zkmain --param params --output ./output --wasm ./pkg/output.wasm single-prove 
#RUST_LOG=info ~/zkWasm/target/release/delphinus-cli  -k 19 --function zkmain --param params --output ./output --wasm ./pkg/output.wasm single-verify
