extern crate serde;

#[derive(serde::Deserialize)]
struct TestInput {
	syscall_index: usize,
	payload: Vec<u32>,
	data: Vec<u8>,
	relocations: Vec<usize>,
}

extern "C" {
	fn rust_wasm_syscall(index: usize, data: *mut u8) -> usize;
}

#[no_mangle]
pub unsafe fn call() {
	let test_input: TestInput = serde_cbor::from_slice(&owasm_ethereum::input()).unwrap();
	let mut payload = test_input.payload;
	for reloc in test_input.relocations.iter() {
		payload[*reloc] += test_input.data.as_ptr() as u32;
	}
	rust_wasm_syscall(test_input.syscall_index, payload.as_mut_ptr() as *mut u8);
	let ret_data = std::slice::from_raw_parts(
		payload.as_ptr() as *const u8,
		payload.len() * std::mem::size_of::<u32>()
	);
	owasm_ethereum::ret(ret_data);
}

