// main.rs

#![no_std]// don't link the Rust standard library
#![no_main] // disable all Rust-level entry points


use core::panic::PanicInfo;
mod vga_buffer;

/// This function is called on panic.
/// The PanicInfo parameter contains the file and line where 
/// the panic happened and the optional panic message. 
/// The function should never return, so it is marked as a 
/// diverging function by returning the “never” type !.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Reqriting the `crt0` (C RunTime Zero) to start at a 
/// point other than the main() function. We should initialize
/// the program somewhere other than main, since we don't have 
/// standard libraries. _start is the starting point of 
/// compile process, which will be implemented later.
/// extern "C" is to tell the compiler that it should use the
/// C calling convention for this function (instead of the 
/// unspecified Rust calling convention).
static HELLO: &[u8] = b"Hello World!"; //Byte version of the string "Hello World!"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

    loop {}
}




