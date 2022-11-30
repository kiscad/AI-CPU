use std::os::raw::c_char;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

use std::ffi::CString;

fn main() {
    let rust_str = "I'll be back";
    let null_terminated = CString::new(rust_str).unwrap();

    unsafe {
        println!("length: {:?}", strlen(null_terminated.as_ptr()));
    }
}
