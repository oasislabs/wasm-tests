#![no_std]

extern crate owasm_std;

use owasm_std::logger;

#[no_mangle]
pub fn call() {
	logger::debug("Exception will occur here:");
	unreachable!();
}
