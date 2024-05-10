use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    Merkle,
};


static mut root:[u64; 4] = [0,0,0,0];

mod phantom {
use zkwasm_rust_sdk::Merkle;
use zkwasm_rust_sdk::wasm_dbg;
use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::allocator::start_alloc_witness;
use zkwasm_rust_sdk::allocator::stop_alloc_witness;
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
    pub  fn pp2(a:u64) -> u64 { // does not work
        unsafe {
            start_alloc_witness();
        }
        unsafe {wasm_dbg(1044)};

        let merkle = unsafe {Merkle::load(root.clone())};
        //return test;
        let (_, leaf) = merkle.get(0, false);
        if a>0 {
            return pp2(a-1);
        }
        unsafe {
            stop_alloc_witness();
        }
        return leaf.len() as u64
    }
}


#[wasm_bindgen]
pub fn zkmain() {
    let a = unsafe { wasm_input(0) };
    let mut merkle = Merkle::new();
    merkle.set(0, &[1, 1, 2, 2], false, None);
    let _ = unsafe { root };
    unsafe {
        root = merkle.root.clone();
    }
    let _ = phantom::pp2(a);
    let _ = merkle.get(0, false);
}
