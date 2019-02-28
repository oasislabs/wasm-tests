#![no_std]

extern crate owasm_std;
extern crate owasm_ethereum;

use owasm_std::keccak;
use owasm_ethereum::{ret, input};

#[no_mangle]
pub fn call() {
	ret(&keccak(&input()));
}
