/// Creating a module to use VGA text mode to print characters
/// on the screen.
/// 
use volatile::Volatile;
 
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

    fn new_line(&mut self) {/* TODO */}
    
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




