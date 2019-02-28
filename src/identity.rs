#![no_std]

extern crate owasm_ethereum as ext;

#[no_mangle]
pub fn call() {
	ext::ret(&ext::sender()[..]);
}
