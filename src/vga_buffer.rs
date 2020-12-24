use volatile::Volatile;
use core::fmt;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_char: ColorCode,
}
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize, //default will be at 0
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // b is a byte literal, transforms char to u8 value
            byte => {
                if (self.column_position >= BUFFER_WIDTH) {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_char: color_code,
                });
                self.column_position += 1;
            }
        }
    }
    fn new_line(&mut self) {
        //EMPTY
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let cur = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(cur);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
    }
    fn clear_row(&mut self, row: usize) {
        let cur = ScreenChar {
            ascii_character: b' ',
            color_char: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(cur); // will b' ' work
        }
    }
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7E | b'\n' => self.write_byte(byte), // range of ascii chars, as well as new line
                _ => self.write_byte(0xFE) // black square unprintable character
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
pub fn print() {
    use core::fmt::Write;
    let mut buffered_writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color:: Yellow, Color:: Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writeln!(buffered_writer, "Hello Vishal").unwrap();
}
pub static WRITER: Writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
};
