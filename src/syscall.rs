extern crate serde;

#[derive(serde::Deserialize)]
struct TestInput {
	syscall_index: usize,
	input: Vec<InputItem>,
}

#[derive(serde::Deserialize)]
enum InputItem {
	Prim(u32),
	Blob(Vec<u8>),
}

extern "C" {
	fn rust_wasm_syscall(index: usize, data: *mut u8) -> usize;
}

#[no_mangle]
pub unsafe fn call() {
	let test_input: TestInput = serde_cbor::from_slice(&owasm_ethereum::input()).unwrap();
	let mut payload: Vec<u32> = test_input.input.iter().map(|inp| match inp {
		InputItem::Prim(val) => *val,
		InputItem::Blob(ref data) => data.as_ptr() as u32,
	}).collect();
	rust_wasm_syscall(test_input.syscall_index, payload.as_mut_ptr() as *mut u8);
	let ret_data = std::slice::from_raw_parts(
		payload.as_ptr() as *const u8,
		payload.len() * std::mem::size_of::<u32>()
	);
	owasm_ethereum::ret(ret_data);
}

