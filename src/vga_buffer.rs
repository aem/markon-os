#![allow(dead_code)]

use core::fmt;
use volatile::Volatile;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | foreground as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaChar {
    character: u8,
    color_code: ColorCode,
}

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<VgaChar>; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= SCREEN_WIDTH {
                    self.new_line();
                }
                let row = SCREEN_HEIGHT - 1;
                let col = self.column_position;
                self.buffer.chars[row][col].write(VgaChar {
                    character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn clear_line(&mut self, row: usize) {
        self.column_position = 0;
        for i in 0..SCREEN_WIDTH {
            self.buffer.chars[row][i].write(VgaChar {
                character: 0x20,
                color_code: ColorCode::new(Color::Black, Color::Black),
            });
        }
    }

    fn new_line(&mut self) {
        for row in 0..(SCREEN_HEIGHT - 1) {
            for col in 0..SCREEN_WIDTH {
                let chatacter = self.buffer.chars[row + 1][col].read();
                self.buffer.chars[row][col].write(chatacter);
            }
        }
        self.clear_line(SCREEN_HEIGHT - 1);
        self.column_position = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
