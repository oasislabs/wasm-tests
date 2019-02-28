#![no_std]

extern crate owasm_std;
extern crate owasm_ethereum as ext;

use owasm_std::keccak;
use owasm_std::hash::H256;

#[no_mangle]
pub fn call() {
	let input = ext::input();

	let hash = keccak(&input);
	let mut reverse_hash = hash.to_vec();
	reverse_hash.reverse();

	let mut reverse_input = input.to_vec();
	reverse_input.reverse();

	ext::log(&[hash, H256::from(&reverse_hash[..])], &reverse_input);

	ext::ret(&reverse_input);
}
