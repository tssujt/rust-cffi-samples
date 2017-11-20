mod ffi;

fn main() {
    let input = 4;
    let output = ffi::ffi_double_input(2);
    println!("{} * 2 = {}", input, output);
}
