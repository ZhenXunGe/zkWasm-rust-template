use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    require,
    Merkle,
};

static mut root:[u64; 4] = [0,0,0,0];

mod phantom {
use zkwasm_rust_sdk::Merkle;
use wasm_bindgen::prelude::*;
use crate::root;
#[inline(never)]
#[wasm_bindgen]
    pub  fn pp(s: u64) -> u64 { // does not work
        let b = s;
        return b as u64
        //return test;
    }

#[inline(never)]
#[wasm_bindgen]
    pub  fn pp2() -> u64 { // does not work
        let merkle = unsafe {Merkle::load(root.clone())};
        let mut leaf = [0, 0, 0, 0];
        //return test;
        let len = merkle.get(0, &mut leaf, &mut [0; 4], false);
        return len
    }
}


#[wasm_bindgen]
pub fn zkmain() {
    let a = unsafe { wasm_input(0) };
    let v = phantom::pp(a);
    let mut merkle = Merkle::new();
    merkle.set(0, &[1, 1, 2, 2], false, None);
    unsafe {
        root = merkle.root.clone();
    }
    //phantom::pp2();
}
