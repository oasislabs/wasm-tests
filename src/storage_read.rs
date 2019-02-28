#![no_std]

extern crate owasm_std;
extern crate owasm_ethereum;

use owasm_std::write_u32;
use owasm_std::hash::H256;
use owasm_ethereum::{ret, read};

fn get_value_from_key(key: u32) -> [u8; 32] {
	let mut full_key = [0u8; 32];
	write_u32(&mut full_key[0..4], key);
	read(&H256::from(full_key))
}

#[no_mangle]
pub fn call() {
	let val: [u8; 32] = get_value_from_key(1);
	ret(&val[..])
}
