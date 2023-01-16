use crate::cpu::Interupt;

pub struct Timer {
    pub div: u16,
    pub tima: u8,
    pub tma: u8,
    pub tac: u8,
    pub state: bool,
}

const BIT_IN_DIV: [u8; 4] = [9, 3, 5, 7];

impl Timer {
    pub fn new() -> Self {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            state: false,
        }
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
            0xff04 => self.div = 0,
            0xff05 => self.tima = value,
            0xff06 => self.tma = value,
            0xff07 => self.tac = value,
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }

    pub fn cycle(&mut self) -> Interupt {
        self.div = self.div.wrapping_add(4);
        let mut curr_state = (self.tac >> 3) & 1 == 1;
        curr_state &= (self.div >> BIT_IN_DIV[(self.tac & 0x3) as usize]) & 1 == 1;

        if self.state && !curr_state {
            if self.tima == 0xff {
                self.tima = self.tma;
                return Interupt::Timer;
            } else {
                self.tima += 1;
            }
        }
        self.state = curr_state;
        return Interupt::None;
    }
}
