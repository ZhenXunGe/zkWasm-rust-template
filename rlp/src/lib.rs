use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    require,
};

#[wasm_bindgen]
pub fn zkmain() {
    let data = vec![0x83, b'c', b'a', b't'];
    let _animal: String = rlp::decode(&data).unwrap();
    // assert_eq!(animal, "cat".to_owned());
    //
}
