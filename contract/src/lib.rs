use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::allocator::alloc_witness_memory;
use zkwasm_rust_sdk::{
    wasm_input,
    require,
};
use derive_builder::WitnessObj;
use zkwasm_rust_sdk::witness::{load_witness_obj, prepare_witness_obj, WitnessObjWriter};
use zkwasm_rust_sdk::witness::WitnessObjReader;

#[derive(Serialize, Deserialize, WitnessObj, PartialEq, Clone, Debug)]
struct A {
    x: u64
}


#[derive(Serialize, Deserialize, WitnessObj, PartialEq, Clone, Debug)]
struct B {
    x: u64
}

#[derive(Serialize, Deserialize, WitnessObj, PartialEq, Clone, Debug)]
enum Command {
    S(A),
    R(B)
}


fn handle_tx(c: &Command) {
    return;
}

#[wasm_bindgen]
pub fn zkmain() {
    let calldata_addr = alloc_witness_memory();
    let calldata = load_witness_obj::<Vec<Command>>(||
        unsafe {
            wasm_input(0)
        },
        calldata_addr
    );
    for tx in unsafe { &*calldata } {
        handle_tx(tx);
    }
}

#[wasm_bindgen]
pub fn produce_inputs(json_str: String) {
    let mut commands = vec![];
    prepare_witness_obj(
        &mut |x| commands.push(x),
        |json_str| {
            serde_json::from_str::<Vec<Command>>(json_str).unwrap()
        },
        &json_str
    )
}
