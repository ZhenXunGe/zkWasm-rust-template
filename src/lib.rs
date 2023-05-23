use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    Merkle,
};

pub fn get_account(account: &[u64;4]) -> [u64; 4] {
    Merkle::get(account[0])
}

struct VerifyInfo {
    oldroot: [u64;4],
    newroot: [u64;4],
    hash: [u64;4]
}

impl VerifyInfo {
    fn read() -> Self {
        VerifyInfo {
            oldroot: [
                unsafe {wasm_input(1)},
                unsafe {wasm_input(1)},
                unsafe {wasm_input(1)},
                unsafe {wasm_input(1)},
            ],
            newroot: [
                unsafe {wasm_input(1)},
                unsafe {wasm_input(1)},
                unsafe {wasm_input(1)},
                unsafe {wasm_input(1)},
            ],
            hash : [
                unsafe {wasm_input(1)},
                unsafe {wasm_input(1)},
                unsafe {wasm_input(1)},
                unsafe {wasm_input(1)},
            ],

        }
    }
}

struct TxInfo {
    data: [u64;10],
}

impl TxInfo {
    fn read() -> Self {
        TxInfo {
            data: [
                unsafe {wasm_input(0)},
                unsafe {wasm_input(0)},
                unsafe {wasm_input(0)},
                unsafe {wasm_input(0)},
                unsafe {wasm_input(0)},
                unsafe {wasm_input(0)},
                unsafe {wasm_input(0)},
                unsafe {wasm_input(0)},
                unsafe {wasm_input(0)},
                unsafe {wasm_input(0)},
            ]
        }
    }
}

#[wasm_bindgen]
pub fn test() {
    Merkle::new();
    let verify_info = VerifyInfo::read();
    let tx_info = TxInfo::read();
    /*
    let address = [
        unsafe {wasm_input(1)},
        unsafe {wasm_input(1)},
        unsafe {wasm_input(1)},
        unsafe {wasm_input(1)},
    ];
    let account_info = get_account(&address);
    */

    // Check hash of tx_info == verify_info.hash
    //
}
