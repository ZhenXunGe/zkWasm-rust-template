build:
	wasm-pack build --release --out-name image.wasm --out-dir pkg --features local
	wasm-opt -Oz -o pkg/output.wasm pkg/image.wasm

clean:
	rm -rf pkg
