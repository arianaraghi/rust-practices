// main.rs

#![no_std]
#![no_main]
#[no_mangle]

use core::panic::PanicInfo;

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
pub extern "C" fn _start() -> ! {
    loop {}
}


