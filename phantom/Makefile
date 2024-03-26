build:
	wasm-pack build --release --out-name image.wasm --out-dir pkg
	wasm-opt -Os -aimfs 0 -o pkg/output.wasm pkg/image.wasm

clean:
	rm -rf pkg
