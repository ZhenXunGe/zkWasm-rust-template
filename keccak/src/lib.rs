use wasm_bindgen::prelude::*;
use sha2::{Sha256, Digest};

#[wasm_bindgen]
pub fn zkmain() {
    let mut hasher = Sha256::new();
    let data = vec![0x83; 9182];
    hasher.update(&data);
    hasher.finalize();
    zkwasm_rust_sdk::dbg!("done!\n");
}
