extern "C" {
	fn ret(ptr: *const u8, len: u32);
	fn return_length() -> usize;
	fn fetch_return(buf: *mut u8);
}

#[no_mangle]
pub unsafe fn call() {
	let data = vec![1u8; 257];
	ret(data.as_ptr(), data.len() as u32);

	let length: usize = return_length();
	assert_eq!(length, data.len());

	let mut buf = Vec::with_capacity(length);
	buf.set_len(length);
	fetch_return(buf.as_mut_ptr());
	owasm_ethereum::ret(&buf);
}
