#![no_std]

extern crate owasm_std;
extern crate owasm_ethereum;
extern crate uint;

extern crate parity_hash as hash;

use owasm_std::logger;
use owasm_ethereum::{input, ret, create, value};

#[no_mangle]
pub fn call() {
	if let Ok(addr) = create(value() >> 1, &input()) {
		logger::debug("Created contract with code");
	} else {
		logger::debug("Error creating contract");
	}

	// if let Ok(addr) = create2(value() / U256::from(2), H256::from([5u8].as_ref()), &input()) {
	// 	logger::debug("Created contract with code and salt");
	// 	(&mut r[0..20]).copy_from_slice(&addr[..]);
	// } else {
	// 	logger::debug("Error creating contract");
	// }
}
