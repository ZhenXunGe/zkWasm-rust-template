use wasm_bindgen::prelude::*;
use secp256k1::{Secp256k1, Message, SecretKey, PublicKey};

extern "C" {
    //pub fn wasm_input(is_public: u32) -> u64;
    //pub fn wasm_dbg(v:u64);
    pub fn wasm_trace_size() -> u64;
}


#[wasm_bindgen]
pub fn zkmain() {
    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    // This is unsafe unless the supplied byte slice is the output of a cryptographic hash
    // function.
    // See the above example for how to use this library together with `hashes-std`.
    let message = Message::from_digest_slice(&[0xab; 32]).expect("32 bytes");
    let sig = secp.sign_ecdsa(&message, &secret_key);
    let trace_size = unsafe { wasm_trace_size() };
    assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());
    let delta_size = unsafe { wasm_trace_size() - trace_size };
    zkwasm_rust_sdk::dbg!("delta size is {}\n", delta_size);
}
