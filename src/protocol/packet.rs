use ws::Message;
use std::ops::Shr;

#[allow(dead_code)]
pub struct Packet {
    pub position: i32,
    pub bytes: Vec<u8>
}

impl Default for Packet {
    fn default() -> Packet {
        Packet {
            position: 0,
            bytes: Vec::new()
        }
    }
}

impl Packet {

    const EIGHT_BITS: u32 = 8;

    pub fn new() -> Packet {
        Packet::default()
    }

    pub fn write_char(mut self, value: char) -> Packet {
        self.bytes.push(value as u8);
        self.position += 1;
        return self;
    }

    pub fn write_u8(mut self, value: u8) -> Packet {
        self.bytes.push(value);
        self.position += 1;
        return self;
    }

    pub fn write_u16(mut self, value: u16) -> Packet {
        self.bytes.push(value.shr(1 as u16 * Packet::EIGHT_BITS as u16) as u8);
        self.bytes.push(value as u8);
        self.position += 2;
        return self;
    }

    pub fn write_u24(mut self, value: u32) -> Packet {
        self.bytes.push(value.shr(2 * Packet::EIGHT_BITS) as u8);
        self.bytes.push(value.shr(1 * Packet::EIGHT_BITS) as u8);
        self.bytes.push(value as u8);
        self.position += 3;
        return self;
    }

    pub fn write_u32(mut self, value: u32) -> Packet {
        self.bytes.push(value.shr(3 * Packet::EIGHT_BITS) as u8);
        self.bytes.push(value.shr(2 * Packet::EIGHT_BITS) as u8);
        self.bytes.push(value.shr(1 * Packet::EIGHT_BITS) as u8);
        self.bytes.push(value as u8);
        self.position += 4;
        return self;
    }

    pub fn write_f8(self, value: f32) -> Packet {
        return self.write_u8(value as u8);
    }

    pub fn write_f16(self, value: f32) -> Packet {
        return self.write_u16(value as u16);
    }

    pub fn write_f32(self, value: f32) -> Packet {
        return self.write_u32(value as u32);
    }

    pub fn to_binary_msg(&self) -> Message {
        let bytes: Vec<u8> = self.bytes.clone();
        return Message::Binary(bytes);
    }

}