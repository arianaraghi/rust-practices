// main.rs

#![no_std]// don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"] // handling test functions errors
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]


use core::panic::PanicInfo;
mod vga_buffer;
mod serial;

/// This function is called on panic.
/// The PanicInfo parameter contains the file and line where 
/// the panic happened and the optional panic message. 
/// The function should never return, so it is marked as a 
/// diverging function by returning the “never” type !.
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
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

/// Exiting QEMU without the need of user interacting with it
/// Not entering QEMU on tests
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// To implement a custom test framework for our kernel, we add the following.
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) { // new
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run(); // new
    }
    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// First test function
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


