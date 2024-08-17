// main.rs

#![no_std]// don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"] // handling test functions errors
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]


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
//static HELLO: &[u8] = b"Hello World!"; //Byte version of the string "Hello World!"
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// To implement a custom test framework for our kernel, we add the following.
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

/// First test function
#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

