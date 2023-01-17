use crate::cpu::Interupt;

#[macro_export]
macro_rules! bit {
    ($x:expr, $pos:expr) => {
        ($x & (1 << $pos)) != 0
    };
}

const BIT_IN_DIV: [u8; 4] = [9, 3, 5, 7];

pub struct Timer {
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    state: bool,
    interupt: Interupt,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            state: false,
            interupt: Interupt::None,
        }
    }

    pub fn extract_interupt(&mut self) -> u8 {
        let r = self.interupt as u8;
        self.interupt = Interupt::None;
        return r;
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff04 => (self.div >> 8) as u8,
            0xff05 => self.tima,
            0xff06 => self.tma,
            0xff07 => self.tac,
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xff04 => {
                self.div = 0;
                self.inc_if_change()
            }
            0xff05 => self.tima = value,
            0xff06 => self.tma = value,
            0xff07 => {
                self.tac = value & 0x7;
                self.inc_if_change()
            }
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }

    fn inc_if_change(&mut self) {
        let curr_state = bit!(self.tac, 2) & bit!(self.div, BIT_IN_DIV[(self.tac & 0x3) as usize]);

        if self.state && !curr_state {
            if self.tima == 0xff {
                self.tima = self.tma;
                self.interupt = Interupt::Timer;
            } else {
                self.tima += 1;
            }
        }
        self.state = curr_state;
    }

    pub fn cycle(&mut self) {
        self.div = self.div.wrapping_add(4);
        self.inc_if_change();
    }
}
