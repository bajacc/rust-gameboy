pub enum Mbc {
    Mbc0(Mbc0),
    Mbc1(Mbc1),
    Mbc2(Mbc2),
    Mbc3(Mbc3),
}

impl Mbc {
    pub fn cartridge_type(mem: &Vec<u8>) -> u8 {
        return mem[0x147];
    }
    pub fn new(mem: Vec<u8>) -> Self {
        let code = Self::cartridge_type(&mem);
        match code {
            0x00 => Mbc::Mbc0(Mbc0::new(mem)),
            0x01..=0x03 => Mbc::Mbc1(Mbc1::new(mem)),
            0x05..=0x06 => Mbc::Mbc2(Mbc2::new(mem)),
            0x0f..=0x13 => Mbc::Mbc3(Mbc3::new(mem)),
            _ => panic!("mbc {:02b} is not implemented", code),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match self {
            Mbc::Mbc0(mbc) => mbc.read(addr),
            Mbc::Mbc1(mbc) => mbc.read(addr),
            Mbc::Mbc2(mbc) => mbc.read(addr),
            Mbc::Mbc3(mbc) => mbc.read(addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match self {
            Mbc::Mbc0(mbc) => mbc.write(addr, value),
            Mbc::Mbc1(mbc) => mbc.write(addr, value),
            Mbc::Mbc2(mbc) => mbc.write(addr, value),
            Mbc::Mbc3(mbc) => mbc.write(addr, value),
        }
    }

    pub fn cycle(&mut self) {
        match self {
            Mbc::Mbc0(mbc) => mbc.cycle(),
            Mbc::Mbc1(mbc) => mbc.cycle(),
            Mbc::Mbc2(mbc) => mbc.cycle(),
            Mbc::Mbc3(mbc) => mbc.cycle(),
        }
    }
}

pub struct Mbc0 {
    mem: Vec<u8>,
}

impl Mbc0 {
    pub fn new(mem: Vec<u8>) -> Self {
        Mbc0 { mem: mem }
    }

    pub fn read(&self, addr: u16) -> u8 {
        return self.mem[addr as usize];
    }

    #[allow(unused_variables)]
    pub fn write(&mut self, addr: u16, value: u8) {}

    pub fn cycle(&mut self) {}
}

pub struct Mbc1 {
    rom: Vec<u8>,
    ram: [[u8; 0x2000]; 4],
    rom_bank_number: u8,
    ram_bank_number: u8,
    ram_select_mode: bool,
    ram_enable: bool,
}

impl Mbc1 {
    pub fn new(mem: Vec<u8>) -> Self {
        Mbc1 {
            rom: mem,
            ram: [[0; 0x2000]; 4],
            rom_bank_number: 0,
            ram_bank_number: 0,
            ram_select_mode: false,
            ram_enable: false,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3fff => self.rom[addr as usize],
            0x4000..=0x7fff => {
                self.rom[(addr as usize - 0x4000) + self.rom_bank_number as usize * 0x4000]
            }
            0xa000..=0xbfff => {
                if self.ram_enable {
                    self.ram[self.ram_bank_number as usize][(addr - 0xa000) as usize]
                } else {
                    0xff
                }
            }
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1fff => self.ram_enable = value & 0x0f == 0xa,
            0x2000..=0x3fff => {
                self.rom_bank_number = value & 0x1f;
                if self.rom_bank_number % 0x20 == 0 {
                    self.rom_bank_number += 1;
                }
            }
            0x4000..=0x5fff => {
                if self.ram_select_mode {
                    self.ram_bank_number = value & 3;
                } else {
                    self.rom_bank_number = ((value & 3) << 5) | (self.rom_bank_number & 0x1f);
                }
            }
            0x6000..=0x7FFF => self.ram_select_mode = value & 1 == 1,
            0xa000..=0xbfff => {
                if self.ram_enable {
                    self.ram[self.ram_bank_number as usize][(addr - 0xa000) as usize] = value
                }
            }
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }

    pub fn cycle(&mut self) {}
}

pub struct Mbc2 {
    rom: Vec<u8>,
    ram: [u8; 0x200],
    rom_bank_number: u8,
    ram_enable: bool,
}

impl Mbc2 {
    pub fn new(mem: Vec<u8>) -> Self {
        Mbc2 {
            rom: mem,
            ram: [0; 0x200],
            rom_bank_number: 0,
            ram_enable: false,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3fff => self.rom[addr as usize],
            0x4000..=0x7fff => {
                self.rom[(addr as usize - 0x4000) + self.rom_bank_number as usize * 0x4000]
            }
            0xa000..=0xa1ff => {
                if self.ram_enable {
                    self.ram[(addr - 0xa000) as usize]
                } else {
                    0xff
                }
            }
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1fff => {
                if addr & 0x10 != 0 {
                    self.ram_enable = !self.ram_enable
                }
            }
            0x2000..=0x3fff => {
                if addr & 0x10 != 0 {
                    self.rom_bank_number = value & 0x0f;
                }
            }
            0xa000..=0xa1ff => {
                if self.ram_enable {
                    self.ram[(addr - 0xa000) as usize] = value & 0x0f
                }
            }
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }
    pub fn cycle(&mut self) {}
}

pub struct Mbc3 {
    rom: Vec<u8>,
    ram: [[u8; 0x2000]; 4],
    rtc: [u8; 5],
    rom_bank_number: u8,
    ram_bank_number: u8,
    ram_enable: bool,

    prelatched: bool,
    num_cycle: u128,
}

impl Mbc3 {
    pub fn new(mem: Vec<u8>) -> Self {
        Mbc3 {
            rom: mem,
            ram: [[0; 0x2000]; 4],
            rtc: [0; 5],
            rom_bank_number: 0,
            ram_bank_number: 0,
            ram_enable: false,

            prelatched: false,
            num_cycle: 0,
        }
    }

    pub fn cycle(&mut self) {
        self.num_cycle += 1;
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3fff => self.rom[addr as usize],
            0x4000..=0x7fff => {
                self.rom[(addr as usize - 0x4000) + self.rom_bank_number as usize * 0x4000]
            }
            0xa000..=0xbfff => {
                if self.ram_enable {
                    match self.ram_bank_number {
                        0x00..=0x07 => {
                            self.ram[self.ram_bank_number as usize][(addr - 0xa000) as usize]
                        }
                        0x08..=0x0c => self.rtc[self.ram_bank_number as usize - 0x08],
                        _ => panic!("unknow ram bank number {}", self.ram_bank_number),
                    }
                } else {
                    0xff
                }
            }
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1fff => self.ram_enable = value & 0x0f == 0xa,
            0x2000..=0x3fff => {
                self.rom_bank_number = value & 0x7f;
                if self.rom_bank_number == 0 {
                    self.rom_bank_number += 1;
                }
            }
            0x4000..=0x5fff => match value {
                0x00..=0x0c => self.ram_bank_number = value,
                _ => unimplemented!("write unknown ram bank number {}", value),
            },
            0x6000..=0x7FFF => match value {
                0x00 => self.prelatched = true,
                0x01 if self.prelatched => {
                    self.counter_to_rtc();
                    self.prelatched = false;
                }
                _ => self.prelatched = false,
            },
            0xa000..=0xbfff => {
                if self.ram_enable {
                    match self.ram_bank_number {
                        0x00..=0x07 => {
                            self.ram[self.ram_bank_number as usize][(addr - 0xa000) as usize] =
                                value
                        }
                        0x08..=0x0c => {
                            self.counter_to_rtc();
                            self.rtc[self.ram_bank_number as usize - 0x08] = value;
                            self.rtc_to_counter();
                        }
                        _ => panic!("unknow ram bank number {}", self.ram_bank_number),
                    }
                }
            }
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }

    const NUM_CYCLE_NEEDED: [u128; 4] = [2u128.pow(20), 60, 60, 24];

    fn rtc_to_counter(&mut self) {
        self.num_cycle = self.rtc[5] as u128 & 1;
        for i in (0..4).rev() {
            self.num_cycle += self.rtc[i] as u128;
            self.num_cycle *= Mbc3::NUM_CYCLE_NEEDED[i];
        }
    }

    fn counter_to_rtc(&mut self) {
        let mut time = self.num_cycle;
        // 1 second every 2^20 cycle
        time /= Mbc3::NUM_CYCLE_NEEDED[0];
        for i in 1..4 {
            self.rtc[i - 1] = (time % Mbc3::NUM_CYCLE_NEEDED[i]) as u8;
            time /= Mbc3::NUM_CYCLE_NEEDED[i];
        }
        let num_day = time % 512;
        self.rtc[0x03] = (num_day & 0xff) as u8;
        self.rtc[0x04] = (self.rtc[0x04] & !1) | (num_day >> 8) as u8;
        time /= 512;
        if time == 0 {
            self.rtc[0x04] &= !(1 << 2);
        } else {
            self.rtc[0x04] |= 1 << 2;
        }
    }
}
