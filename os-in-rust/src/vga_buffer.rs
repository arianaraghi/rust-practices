/// Creating a module to use VGA text mode to print characters
/// on the screen.
/// 
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use core::fmt;
 
////////////////////////////////////////////////////////
///                                                  ///
///                       Color                      ///
///                                                  ///
////////////////////////////////////////////////////////
/// Normally the compiler would issue a warning for each unused variant. 
/// By using the #[allow(dead_code)] attribute, we disable these warnings for the Color enum.
#[allow(dead_code)]
/// By deriving the Copy, Clone, Debug, PartialEq, and Eq traits, 
/// we enable copy semantics for the type and make it printable and comparable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Because of the repr(u8) attribute, each enum variant is stored as a u8. 
/// Actually 4 bits would be sufficient, but Rust doesn’t have a u4 type.
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
///The ColorCode struct contains the full color byte, containing foreground and background color.
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}


////////////////////////////////////////////////////////
///                                                  ///
///                   Text Buffer                    ///
///                                                  ///
////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Since the field ordering in default structs is undefined in Rust, 
/// we need the repr(C) attribute. 
/// It guarantees that the struct’s fields are laid out exactly 
/// like in a C struct and thus guarantees the correct field ordering.
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

/// to ensure that it has the same memory layout as its single field.
/// The problem is that we only write to the Buffer 
/// and never read from it again. The compiler doesn’t 
/// know that we really access VGA buffer memory 
/// (instead of normal RAM) and knows nothing about 
/// the side effect that some characters appear on the screen. 
/// So it might decide that these writes are unnecessary 
/// and can be omitted. To avoid this erroneous optimization, 
/// we need to specify these writes as volatile. 
/// This tells the compiler that the write has side effects 
/// and should not be optimized away.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// The writer will always write to the last line and shift 
/// lines up when a line is full (or on \n)
pub struct Writer {
    /// Keeps track of the current position in the last row.
    column_position: usize,
    /// The current foreground and background colors 
    /// are specified by color_code
    color_code: ColorCode,
    /// A reference to the VGA buffer is stored in buffer
    buffer: &'static mut Buffer,
}


impl Writer {
    /// Writes each byte, with specified color on the last possible
    /// position. If not, adds a new line. If '\n', adds a new line.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
    
    /// To write a whole string, break it into bytes, and write
    /// each byte using the `write-byte()` function.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }

        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// To provide a global writer that can be used as an interface 
/// from other modules without carrying a Writer instance around, 
/// we try to create a static WRITER, using `lazi_static!` macro,
/// to ensure the static type won't initialize itself until the 
/// first call, since it can lead to panic.
/// 
/// To avoid rewriting and other race condition problems, we need
/// to use mutex and locking systems, that are not in the standard
/// library (we don't use std). Hence, we use `spinlock`
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// Creating `print!` and `println!` macros for our own VGA buffer
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

