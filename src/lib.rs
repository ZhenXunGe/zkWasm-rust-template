use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::Merkle;

pub fn get_account(account: &[u64;4]) -> [u64; 4] {
    Merkle::get(account[0])
}

#[wasm_bindgen]
pub fn zkmain() {
    Merkle::new();
    let account = get_account(&[0u64; 4]);
}
