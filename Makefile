build:
	wasm-pack build --release --out-name zkwasm-account.wasm --out-dir pkg
	wasm-opt -Oz -o pkg/output.wasm pkg/zkwasm-account.wasm

trace:
	wasm-interp pkg/output.wasm --run-all-exports  --trace > trace.log
	wc -l trace.log

clean:
	rm -rf pkg
