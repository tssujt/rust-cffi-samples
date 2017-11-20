extern crate libc;

extern {
    fn double_input(input: libc::c_int) -> libc::c_int;
}

pub fn ffi_double_input(input: i32) -> i32 {
    unsafe {
        double_input(input)
    }
}
