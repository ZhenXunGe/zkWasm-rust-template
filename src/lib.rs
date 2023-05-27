use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::{
    wasm_input,
    Merkle,
    require,
};

pub fn get_account(account: u32) -> [u64; 4] {
    Merkle::get(account as u64)
}

pub fn set_account(account: u32, data:&[u64; 4]) {
    Merkle::set(account as u64, data)
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
                unsafe {wasm_input(0)}, // opcode and id
                unsafe {wasm_input(0)}, // id:[u32; 2]
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
pub fn zkmain() {
    let verify_info = VerifyInfo::read();
    Merkle::load(&verify_info.oldroot);
    let tx_info = TxInfo::read();
    let opcode = tx_info.data[0] >> 56;
    if opcode == 0 {
        let address_index = (tx_info.data[1] >> 32) as u32;
        let mut account_info = get_account(address_index);
        let amount = tx_info.data[2].to_be();
        account_info[0] += amount; 
        set_account(address_index, &account_info);
    }
    if opcode == 1 {
        let address_index = (tx_info.data[1] >> 32) as u32;
        let mut account_info = get_account(address_index);
        let amount = tx_info.data[2].to_be();
        unsafe {require((amount <= account_info[0]) as i32)};
        account_info[0] -= amount; 
        set_account(address_index, &account_info);
    }


    // Check hash of tx_info == verify_info.hash
    //
}
