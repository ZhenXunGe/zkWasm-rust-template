use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use zkwasm_rust_sdk::allocator::alloc_witness_memory;
use zkwasm_rust_sdk::kvpair::KeyValueMap;
use zkwasm_rust_sdk::{
    wasm_input,
    wasm_output, Merkle,
};
use sha2::{Sha256, Digest};


pub const DEPOSIT: u8 = 0x0;
pub const WITHDRAW: u8 = 0x1;

pub struct TxInfo {
    pub opinfo: u64,
    pub account_index: u32,
    pub object_index: u32,
    pub args: [u64; 8],
}

pub struct DepositInfo {
    pub opinfo: u64,
    pub account_index: u32,
    pub object_index: u32,
    pub amount: [u64; 4],
    pub sender: [u8; 32],
}

pub struct WithdrawInfo {
    pub opinfo: u64,
    pub account_index: u32,
    pub object_index: u32,
    pub amount: [u64; 4],
    pub sender: [u8; 32],
}

/// encode bytes into wasm output
pub fn output_tx_info(data: &[u8; 80]) {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    for c in result.chunks_exact(8) {
        zkwasm_rust_sdk::dbg!("c is {:?}", c);
        unsafe { wasm_output(u64::from_le_bytes(c.try_into().unwrap())) }
    }
}

impl DepositInfo {
    pub fn new(
        nounce: u64,
        account_index: u32,
        object_index: u32,
        amount: [u64; 4],
        sender: [u8; 32],
    ) -> Self {
        DepositInfo {
            opinfo: (DEPOSIT as u64) + (nounce << 8),
            account_index,
            object_index,
            amount,
            sender,
        }
    }
    /// change everything to big endian that should fits solidity's format
    pub fn to_be_bytes(&self) -> [u8; 80] {
        let mut bytes = vec![];
        bytes.append(&mut self.opinfo.to_be_bytes().to_vec());
        bytes.append(&mut self.account_index.to_be_bytes().to_vec());
        bytes.append(&mut self.object_index.to_be_bytes().to_vec());
        for i in 0..4 {
            bytes.append(&mut self.amount[3-i].to_be_bytes().to_vec());
        }
        bytes.append(&mut self.sender.to_vec());
        bytes.try_into().unwrap()
    }
}

impl WithdrawInfo {
    pub fn new(
        nounce: u64,
        account_index: u32,
        object_index: u32,
        amount: [u64; 4],
        sender: [u8; 32],
    ) -> Self {
        WithdrawInfo {
            opinfo: (DEPOSIT as u64) + (nounce << 8),
            account_index,
            object_index,
            amount,
            sender,
        }
    }
    /// change everything to big endian that should fits solidity's format
    pub fn to_be_bytes(&self) -> [u8; 80] {
        let mut bytes = vec![];
        bytes.append(&mut self.opinfo.to_be_bytes().to_vec());
        bytes.append(&mut self.account_index.to_be_bytes().to_vec());
        bytes.append(&mut self.object_index.to_be_bytes().to_vec());
        for i in 0..4 {
            bytes.append(&mut self.amount[3-i].to_be_bytes().to_vec());
        }
        bytes.append(&mut self.sender.to_vec());
        bytes.try_into().unwrap()

    }
}

#[wasm_bindgen]
pub fn zkmain() {
    let mut merkle = unsafe {
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
    let mut data_buf = [0; 16];
    /*
    let mut kvpair = KeyValueMap::new(merkle);
    let len = kvpair.get(&user_address, &mut data_buf);
    */
    let root = merkle.root;
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
