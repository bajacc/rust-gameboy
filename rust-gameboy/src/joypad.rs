use crate::cpu::Interupt;

pub struct Joypad {
    pub p1: u8,
    interupt: u8,

    key_pressed: [u8; 2],
}

pub enum Key {
    Right = 0,
    Left = 1,
    Up = 2,
    Down = 3,
    A = 4,
    B = 5,
    Select = 6,
    Start = 7,
}

macro_rules! bit {
    ($x:expr, $pos:expr) => {
        ($x & (1 << ($pos as u8))) != 0
    };
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            p1: 0xc0,
            interupt: 0,
            key_pressed: [0xff; 2],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff00 => self.p1,
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xff00 => self.p1 = (self.p1 & 0xcf) | (value & !0xcf),
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }

    pub fn extract_interupt(&mut self) -> u8 {
        let r = self.interupt;
        self.interupt = 0;
        return r;
    }

    pub fn cycle(&mut self) {
        let old_p1 = self.p1;
        self.p1 |= 0x0f;
        if !bit!(self.p1, 4) {
            self.p1 &= self.key_pressed[0];
        }
        if !bit!(self.p1, 5) {
            self.p1 &= self.key_pressed[1];
        }

        if (self.p1 & !old_p1) != 0 {
            self.interupt |= Interupt::Joypad as u8;
        }
    }

    pub fn press_key(&mut self, key: Key) {
        let key_num = key as usize;
        self.key_pressed[key_num / 4] &= !(1 << (key_num % 4));
    }

    pub fn release_key(&mut self, key: Key) {
        let key_num = key as usize;
        self.key_pressed[key_num / 4] |= 1 << (key_num % 4);
    }
}
