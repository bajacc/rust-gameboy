use crate::cpu::Interupt;
use crate::joypad::Joypad;
use crate::lcd::Lcd;
use crate::mbc::Mbc;
use crate::sound::Sound;
use crate::timer::Timer;

const BOOT_ROM: [u8; 256] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x00, 0x00, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x00, 0x00, 0x3E, 0x01, 0xE0, 0x50,
];

pub const NO_DATA: u8 = 0xff;

pub struct Mmu {
    pub interupt_enable: u8,
    pub interupt_flag: u8,

    pub disable_boot_rom: bool,
    pub mbc: Mbc,

    pub work_ram: [u8; 0x2000],
    pub graphical_ram: [u8; 0xa0],
    pub high_ram: [u8; 0x7f],

    pub timer: Timer,
    pub lcd: Lcd,
    pub joypad: Joypad,
    pub sound: Sound,

    pub serial_data: Option<u8>,
}

impl Mmu {
    pub fn new(mbc: Mbc) -> Self {
        Mmu {
            interupt_enable: 0,
            interupt_flag: 0,
            disable_boot_rom: false,
            mbc: mbc,
            work_ram: [0; 0x2000],
            graphical_ram: [0; 0xa0],
            high_ram: [0; 0x7f],
            timer: Timer::new(),
            lcd: Lcd::new(),
            joypad: Joypad::new(),
            sound: Sound::new(),
            serial_data: None,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00ff => {
                if self.disable_boot_rom {
                    self.mbc.read(addr)
                } else {
                    BOOT_ROM[addr as usize]
                }
            }
            0x0100..=0x7fff => self.mbc.read(addr),
            0x8000..=0x9fff => self.lcd.read(addr),
            0xa000..=0xbfff => self.mbc.read(addr),
            0xc000..=0xdfff => self.work_ram[addr as usize - 0xc000],
            0xe000..=0xfdff => self.work_ram[addr as usize - 0xe000],
            0xfe00..=0xfe9f => self.lcd.read(addr),
            0xff10..=0xff3f => self.sound.read(addr),
            0xff80..=0xfffe => self.high_ram[addr as usize - 0xff80],

            0xff00 => self.joypad.read(addr),
            0xff01 => NO_DATA, // serial transfert data
            0xff04..=0xff07 => self.timer.read(addr),
            0xff40..=0xff4b => self.lcd.read(addr),

            0xff0f => self.interupt_flag,
            0xffff => self.interupt_enable,
            _ => NO_DATA,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x7fff => self.mbc.write(addr, value),
            0x8000..=0x9fff => self.lcd.write(addr, value),
            0xa000..=0xbfff => self.mbc.write(addr, value),
            0xc000..=0xdfff => self.work_ram[addr as usize - 0xc000] = value,
            0xe000..=0xfdff => self.work_ram[addr as usize - 0xe000] = value,
            0xfe00..=0xfe9f => self.lcd.write(addr, value),
            0xff10..=0xff3f => self.sound.write(addr, value),
            0xff80..=0xfffe => self.high_ram[addr as usize - 0xff80] = value,

            0xff00 => self.joypad.write(addr, value),
            0xff01 => self.serial_data = Some(value),
            0xff04..=0xff07 => self.timer.write(addr, value),
            0xff40..=0xff4b => self.lcd.write(addr, value),

            0xff0f => self.interupt_flag = value & (Interupt::Mask as u8),
            0xff50 => self.disable_boot_rom = true,
            0xffff => self.interupt_enable = value & (Interupt::Mask as u8),
            _ => (),
        }
    }

    pub fn extract_serial_data(&mut self) -> Option<u8> {
        let r = self.serial_data;
        self.serial_data = None;
        return r;
    }

    pub fn read16(&self, addr: u16) -> u16 {
        return (self.read(addr) as u16) | ((self.read(addr + 1) as u16) << 8);
    }

    pub fn write16(&mut self, addr: u16, value: u16) {
        self.write(addr, value as u8);
        self.write(addr + 1, (value >> 8) as u8);
    }

    pub fn get_interupts(&mut self) -> u8 {
        self.interupt_flag |= self.timer.extract_interupt();
        self.interupt_flag |= self.lcd.extract_interupt();
        self.interupt_flag |= self.joypad.extract_interupt();
        return self.interupt_flag & self.interupt_enable;
    }

    pub fn cycle(&mut self) {
        self.timer.cycle();
        self.lcd.cycle();
        self.joypad.cycle();
        self.mbc.cycle();
        self.sound.cycle();
    }
}
