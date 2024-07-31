// main.rs

#![no_std]

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

fn main() {
}
