use wasm_bindgen::prelude::*;
use sha2::{Sha256, Digest};

#[wasm_bindgen]
pub fn zkmain() {
    let mut hasher = Sha256::new();
    let mut data = vec![];
    for i in 0..100000u32 {
        data.push((i & 0xff) as u8);
    }
    hasher.update(&data);
    hasher.finalize();
    zkwasm_rust_sdk::dbg!("done!\n");
}
