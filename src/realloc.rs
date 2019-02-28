#![no_std]

extern crate owasm_std;
extern crate owasm_ethereum as ext;

use owasm_std::Vec;

#[no_mangle]
pub fn call() {
	ext::ret(&{
		let mut data = Vec::with_capacity(1);
		data.push(0u8);
		for arg in ext::input().iter() {
			data.push(*arg);
		}
		data
	});
}
