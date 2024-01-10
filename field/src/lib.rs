use wasm_bindgen::prelude::*;
use ff::Field;
use std::ops::Mul;
use std::ops::Add;
use primitive_types::U256;


extern "C" {
    //pub fn wasm_input(is_public: u32) -> u64;
    //pub fn wasm_dbg(v:u64);
    pub fn wasm_trace_size() -> u64;
}

#[macro_use]
extern crate ff;

#[derive(PrimeField)]
#[PrimeFieldModulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
struct Fp([u64; 4]);


#[wasm_bindgen]
pub fn zkmain() {
    let a = Fp::from(12);
    let b = Fp::from(7).invert().unwrap();
    let c = Fp::from(7);
    let trace_size = unsafe { wasm_trace_size() };
    let x = a.mul(&b);
    let delta_size = unsafe { wasm_trace_size() - trace_size };
    zkwasm_rust_sdk::dbg!("delta fpmul size is {}\n", delta_size);
    let trace_size = unsafe { wasm_trace_size() };
    let x = (U256 (a.0)).full_mul(U256 (c.0));
    let delta_size = unsafe { wasm_trace_size() - trace_size };
    zkwasm_rust_sdk::dbg!("delta u256mul size is {}\n", delta_size);
    let trace_size = unsafe { wasm_trace_size() };
    let x = u128::from(0xffffffffffffffffu64);
    let y = u128::from(0xfffffffffffffffeu64);
    let z = x * y;
    let delta_size = unsafe { wasm_trace_size() - trace_size };
    zkwasm_rust_sdk::dbg!("delta u128 mul size is {} {}\n", delta_size, z);
}
