#![no_std]

#[macro_use] extern crate owasm_std;

extern "C" {
	fn ret(ptr: *const u8, len: u32);
	fn return_length() -> usize;
	fn fetch_return(buf: *mut u8);
}

#[no_mangle]
pub unsafe fn call() {
	let data = vec![1, 2, 3];
	ret(data.as_ptr(), data.len() as u32);

	let length: usize = return_length();
	if length != 3 {
		panic!("Incorrect return_length");
	}

	let mut buf = owasm_std::Vec::with_capacity(length);
	fetch_return(buf.as_mut_ptr());
	owasm_ethereum::ret(&buf);
}
