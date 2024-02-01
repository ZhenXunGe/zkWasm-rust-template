use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    require,
};

#[wasm_bindgen]
pub fn phantom(s: u64) -> u64 {
    let mut b = s;
    for _ in 0..s {
        b = b*s;
    }
    return b
    //return test;
}

pub fn phantom_wrapper(f: &dyn Fn(u64)-> u64, t:u64) -> u64 {
    f(t)
}

#[wasm_bindgen]
pub fn zkmain() {
    let a = unsafe {
        wasm_input(0)
    };
    let v = phantom_wrapper(&phantom, a);
    unsafe {require(v == 3)};
}
