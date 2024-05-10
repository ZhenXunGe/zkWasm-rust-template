#!/bin/bash

set -e
set -x

rm -rf output
mkdir output

#RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params zkmain dry-run --wasm ./pkg/output.wasm --output ./output

# Single test
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params zkmain setup -k 18 --wasm ./pkg/output.wasm --host standard
RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params zkmain prove --wasm ./pkg/output.wasm --output ./output \
--public 14789582351289948625:i64 \
--public 10919489180071018470:i64 \
--public 10309858136294505219:i64 \
--public 2839580074036780766:i64 \
--private 4:i64 \
--private 1:i64 \
--private 1:i64 \
--private 1:i64 \
--private 1:i64 \
--private 0x7137da164bacaa9332b307e25c1abd906c5c240dcb27e520b84522a1674aab01:bytes-packed \
--private 0x33b51854d1cde428aa0379606752a341b85cf1d47803e22330a0c9d41ce59c2b:bytes-packed \
--private 0x58d3d6174882989c8c8d01cd1306f75489a83354a408883361089c1008fbcd21:bytes-packed \
--private 0xb6ad36f0ae8814ec6e6172ea77fa7d7803469e81a9b5721b057b40847024d810:bytes-packed \
--private 0xf0d425213e068b3c69e275416ad48f4d58b6b51e125700877a1f1e5b762f2803:bytes-packed


RUST_LOG=info ~/zkWasm/target/release/delphinus-cli --params params zkmain verify --output ./output
