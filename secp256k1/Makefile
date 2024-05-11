build:
	wasm-pack build --release --out-name image.wasm --out-dir pkg
	wasm-opt -O3 -o pkg/output.wasm pkg/image.wasm

clean:
	rm -rf pkg output params
