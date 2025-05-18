use x86_64::instructions::port::Port;
use crate::vga_buffer::WRITER;

pub struct Keyboard {
    port: Port<u8>,
    last_scancode: Option<u8>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            port: Port::new(0x60),
            last_scancode: None,
        }
    }

    pub fn read_scancode(&mut self) -> u8 {
        unsafe { self.port.read() }
    }

    pub fn handle_scancode(&mut self, scancode: u8) {
        if Some(scancode) == self.last_scancode {
            return;
        }
        
        if scancode < 0x80 {
            match scancode {
                0x1C => { // Enter key
                    let mut writer = WRITER.lock();
                    writer.write_string("\n");
                    writer.write_string("> ");
                }
                0x0E => { // Backspace
                    let mut writer = WRITER.lock();
                    if writer.column_position > 2 { // Allow backspace until after the prompt
                        writer.column_position -= 1;
                        writer.write_byte(b' ');
                        writer.column_position -= 1;
                    }
                }
                scancode => {
                    if let Some(c) = scancode_to_char(scancode) {
                        WRITER.lock().write_byte(c);
                    }
                }
            }
        }
        
        self.last_scancode = Some(scancode);
    }
}

fn scancode_to_char(scancode: u8) -> Option<u8> {
    match scancode {
        0x02 => Some(b'1'),
        0x03 => Some(b'2'),
        0x04 => Some(b'3'),
        0x05 => Some(b'4'),
        0x06 => Some(b'5'),
        0x07 => Some(b'6'),
        0x08 => Some(b'7'),
        0x09 => Some(b'8'),
        0x0A => Some(b'9'),
        0x0B => Some(b'0'),
        0x10 => Some(b'q'),
        0x11 => Some(b'w'),
        0x12 => Some(b'e'),
        0x13 => Some(b'r'),
        0x14 => Some(b't'),
        0x15 => Some(b'y'),
        0x16 => Some(b'u'),
        0x17 => Some(b'i'),
        0x18 => Some(b'o'),
        0x19 => Some(b'p'),
        0x1E => Some(b'a'),
        0x1F => Some(b's'),
        0x20 => Some(b'd'),
        0x21 => Some(b'f'),
        0x22 => Some(b'g'),
        0x23 => Some(b'h'),
        0x24 => Some(b'j'),
        0x25 => Some(b'k'),
        0x26 => Some(b'l'),
        0x2C => Some(b'z'),
        0x2D => Some(b'x'),
        0x2E => Some(b'c'),
        0x2F => Some(b'v'),
        0x30 => Some(b'b'),
        0x31 => Some(b'n'),
        0x32 => Some(b'm'),
        0x39 => Some(b' '),
        _ => None,
    }
} 