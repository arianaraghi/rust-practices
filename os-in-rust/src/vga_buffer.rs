/// Creating a module to use VGA text mode to print characters
/// on the screen.
 
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
#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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


///
