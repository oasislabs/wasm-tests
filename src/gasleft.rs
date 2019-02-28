#![no_std]

extern crate owasm_std;
extern crate owasm_ethereum;

use owasm_std::{Vec, write_u64};
use owasm_ethereum::{self as ext};

fn push_u64(buf: &mut Vec<u8>, val: u64) {
	let mut slc = [0u8; 8];
	write_u64(&mut slc, val);
	buf.extend(&slc[..]);
}

#[no_mangle]
pub fn call() {
	let mut output: Vec<u8> = Vec::with_capacity(8);
	push_u64(&mut output, ext::gas_left());
	ext::ret(&output);
}
