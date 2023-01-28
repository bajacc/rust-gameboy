use crate::mmu;

#[derive(Default)]
pub struct Sound {
    wave: Wave,
}

#[derive(Default)]
struct Wave {
    on: bool,
    length: u8,
    volume: u8,
    nr33: u8,
    nr34: u8,
    ram: [u8; 0x10],

    counter: usize,
    position: usize,
    sample: u8
}

impl Wave {
    pub fn cycle(&mut self) {
        if self.counter == 0 {
            self.reset_counter();
            self.position = (self.position + 1) % (self.ram.len() * 2);
            
            self.sample = self.ram[self.position / 2];
            self.sample >>= (self.position & 1) * 4;
            self.sample &= 0x0f;
        }

        self.counter -= 1;
    }

    fn reset_counter(&mut self) {
        let x = self.nr33 as usize | (self.nr34 as usize & 7) << 8;
        // freq = 65536/(2048 - x) in Hz
        // period = (4194304 / freq) in s

        // period in cycle
        self.counter = 2 * (1 << 20) * (2048 - x); // todo: mutiply by 2 or 64?
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff1a => (self.on as u8) << 7,
            0xff1b => self.length,
            0xff1c => self.volume << 5,
            0xff1d => self.nr33,
            0xff1e => self.nr34,
            0xff30..=0xff3f => self.ram[addr as usize - 0xff30],
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xff1a => self.on = value & 0x80 != 0,
            0xff1b => self.length = value,
            0xff1c => self.volume = (value >> 5) & 3,
            0xff1d => self.nr33 = value,
            0xff1e => self.nr34 = value,
            0xff30..=0xff3f => self.ram[addr as usize - 0xff30] = value,
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }
}


impl Sound {
    pub fn new() -> Self {
        Sound::default()
    }

    pub fn cycle(&mut self) {
        self.wave.cycle();
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff1a..=0xff1e|0xff30..=0xff3f => self.wave.read(addr),
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xff1a..=0xff1e|0xff30..=0xff3f => self.wave.write(addr, value),
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }
}