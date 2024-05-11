use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::kvpair::KeyValueMap;
use zkwasm_rust_sdk::{
    wasm_input,
    wasm_output, Merkle,
};
use crate::DepositInfo;
use crate::output_tx_info;

struct State {
    balance: u64
}

#[wasm_bindgen]
pub fn zkmain() {
    let merkle = unsafe {
        Merkle::load([
            wasm_input(1),
            wasm_input(1),
            wasm_input(1),
            wasm_input(1),
        ])
    };
    let user_address = unsafe {
        [
            wasm_input(0),
            wasm_input(0),
            wasm_input(0),
            wasm_input(0)
        ]
    };

    let mut kvpair = KeyValueMap::new(merkle);

    //let state_buf = kvpair.get([0; 4]); // store the global state at [0; 4]
    let user_data = kvpair.get(&user_address);

    let mut state = if user_data.len() == 0 {
        State {
            balance: 0
        }
    } else {
        State {
            balance: user_data[0]
        }
    };

    state.balance += 1000; // charge
    
    kvpair.set(&user_address, &[state.balance]);

    let root = kvpair.merkle.root;
    unsafe {
            wasm_output(root[0]);
            wasm_output(root[1]);
            wasm_output(root[2]);
            wasm_output(root[3]);
    }

    let deposit = DepositInfo::new(
        0,
        0,
        0,
        [1000, 0, 0, 0],
        [0; 32]
    );

    output_tx_info(&deposit.to_be_bytes());
}
