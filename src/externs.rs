#![no_std]

extern crate owasm_std;
extern crate owasm_ethereum;
extern crate uint;

use owasm_std::{Vec, write_u64};
use owasm_ethereum::{self as ext};
use uint::U256;

fn push_u64(buf: &mut Vec<u8>, val: u64) {
	let mut slc = [0u8; 8];
	write_u64(&mut slc, val);
	buf.extend(&slc[..]);
}

fn push_u256(buf: &mut Vec<u8>, val: U256) {
	let mut slc = [0u8; 32];
	val.to_big_endian(&mut slc);
	buf.extend(&slc[..]);
}

#[no_mangle]
pub fn call() {
	let mut output: Vec<u8> = Vec::with_capacity(64 + 20 + 8 + 8 + 32 + 32);

	output.extend(&ext::block_hash(0)[..]);
	output.extend(&ext::block_hash(1)[..]);
	output.extend(&ext::coinbase()[..]);
	push_u64(&mut output, ext::timestamp());
	push_u64(&mut output, ext::block_number());
	push_u256(&mut output, ext::difficulty());
	push_u256(&mut output, ext::gas_limit());

	ext::ret(&output);
}
