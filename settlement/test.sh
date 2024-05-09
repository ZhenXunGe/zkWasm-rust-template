#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

#RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params zkmain dry-run --wasm ./pkg/output.wasm --output ./output

# Single test
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params zkmain setup -k 18 --wasm ./pkg/output.wasm
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params zkmain prove --wasm ./pkg/output.wasm --output ./output \
--public 14789582351289948625:i64 \
--public 10919489180071018470:i64 \
--public 10309858136294505219:i64 \
--public 2839580074036780766:i64

RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params zkmain verify --output ./output
