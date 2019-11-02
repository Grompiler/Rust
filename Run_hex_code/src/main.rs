use std::mem;

#[no_mangle]
#[link_section = ".text"]
static hello_world: [u8; 38] =
    *b"\x6a\x01\x5f\x89\xf8\x6a\x0d\x5a\xeb\x0a\x5e\x0f\x05\x6a\x3c\x58\xff\xcf\x0f\x05\xe8\xf1\xff\xff\xff\x48\x65\x6c\x6c\x6f\x20\x77\x6f\x72\x6c\x64\x21\x0a";

fn main() {
	let exec_data: extern "C" fn () -> ! = unsafe {  mem::transmute(&hello_world as *const _ as *const ()) };
	exec_data();
}
