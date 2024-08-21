/// To communicate with the console, we need to implement a port to the
/// system. However, creating ports is a very complex task. Therefore, 
/// we are going to use UART, which is no longer used in the modern 
/// systems. To use UART we need a serial port. 

use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

/// Like the isa-debug-exit device, the UART is programmed using port I/O. 
/// Since the UART is more complex, it uses multiple I/O ports for programming 
/// different device registers. The unsafe SerialPort::new function expects 
/// the address of the first I/O port of the UART as an argument, from which 
/// it can calculate the addresses of all needed ports. We’re passing the port 
/// address 0x3F8, which is the standard port number for the first serial interface.
lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/// To make the serial port easily usable, we add serial_print! and serial_println! macros
#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}