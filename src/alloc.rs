#![no_std]

extern crate owasm_std;
extern crate owasm_ethereum as ext;

use owasm_std::Vec;

#[no_mangle]
pub fn call() {
	ext::ret(&{
		let mut data = Vec::with_capacity(400 * 1024);
		data.extend_from_slice(&[5u8; 400*1024][..]);
		data
	});
}
