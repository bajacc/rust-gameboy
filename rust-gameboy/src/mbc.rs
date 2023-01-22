pub enum Mbc {
    Mbc0(Mbc0),
    Mbc1(Mbc1),
    Mbc2(Mbc2),
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
            _ => panic!("mbc {} is not implemented", code),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match self {
            Mbc::Mbc0(mbc0) => mbc0.read(addr),
            Mbc::Mbc1(mbc1) => mbc1.read(addr),
            Mbc::Mbc2(mbc2) => mbc2.read(addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match self {
            Mbc::Mbc0(mbc0) => mbc0.write(addr, value),
            Mbc::Mbc1(mbc1) => mbc1.write(addr, value),
            Mbc::Mbc2(mbc2) => mbc2.write(addr, value),
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
}
