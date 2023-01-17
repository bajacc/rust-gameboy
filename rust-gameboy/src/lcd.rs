use crate::cpu::Interupt;
use crate::mmu::Mmu;

pub struct Lcd {
    pub video_ram: [u8; 0x2000],
    pub lcdc: u8,
    pub stat: u8,
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyc: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub wx: u8,
    pub wy: u8,

    num_idle_cycle: usize,
    interupt: u8,
}

pub enum LcdcBit {
    BG = 0,
    OBJ = 1,
    OBJ_SIZE = 2,
    BG_AREA = 3,
    TILE_SOURCE = 4,
    WIN = 5,
    WIN_AREA = 6,
    LCD_STATUS = 7,
}

pub enum StatBit {
    MODE0 = 0,
    MODE1 = 1,
    LYC_EQ_LY = 2,
    INT_MODE0 = 3,
    INT_MODE1 = 4,
    INT_MODE2 = 5,
    INT_LYC = 6,
}

macro_rules! bit {
    ($x:expr, $pos:expr) => {
        ($x & (1 << ($pos as u8))) != 0
    };
}

impl Lcd {
    pub fn new() -> Self {
        Lcd {
            video_ram: [0; 0x2000],
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wx: 0,
            wy: 0,
            num_idle_cycle: 0,
            interupt: 0,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.video_ram[addr as usize - 0x8000],
            0xff40 => self.lcdc,
            0xff41 => self.stat,
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff47 => self.bgp,
            0xff48 => self.obp0,
            0xff49 => self.obp1,
            0xff4a => self.wx,
            0xff4b => self.wy,
            _ => panic!("0x{:04x}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9fff => self.video_ram[addr as usize - 0x8000] = value,
            0xff40 => self.lcdc = value,
            0xff41 => (), //stat,
            0xff42 => self.scy = value,
            0xff43 => self.scx = value,
            0xff44 => (), // ly
            0xff45 => self.lyc = value,
            0xff47 => self.bgp = value,
            0xff48 => self.obp0 = value,
            0xff49 => self.obp1 = value,
            0xff4a => self.wx = value,
            0xff4b => self.wy = value,
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }

    pub fn extract_interupt(&mut self) -> u8 {
        let r = self.interupt as u8;
        self.interupt = 0;
        return r;
    }

    fn get_mode(&self) -> u8 {
        return self.stat & 3;
    }

    fn set_mode(&mut self, value: u8) {
        self.stat &= !3;
        self.stat |= value & 3;
    }

    const MODE_DURATION: [usize; 4] = [51, 114, 20, 43];
    const NB_LY: u8 = 154;
    pub const HEIGHT: u8 = 144;

    fn inc_ly(&mut self) {
        self.ly += 1;
        if self.ly == Lcd::NB_LY {
            self.ly = 0;
        }
        if self.ly == self.lyc {
            self.stat |= 1 << (StatBit::LYC_EQ_LY as u8);
        } else {
            self.stat &= !(1 << (StatBit::LYC_EQ_LY as u8));
        }
        if bit!(self.stat, StatBit::INT_LYC) {
            self.interupt |= Interupt::LcdStats as u8;
        }
    }

    fn change_mode(&mut self) {
        match self.get_mode() {
            0 => {
                if self.ly + 1 == Lcd::HEIGHT {
                    self.inc_ly();
                    self.interupt |= Interupt::Vblank as u8;
                    if bit!(self.stat, StatBit::INT_MODE1) {
                        self.interupt |= Interupt::LcdStats as u8;
                    }
                    self.set_mode(1);
                } else {
                    self.inc_ly();
                    if bit!(self.stat, StatBit::INT_MODE1) {
                        self.interupt |= Interupt::LcdStats as u8;
                    }
                    self.set_mode(2);
                }
            }
            1 => {
                if self.ly + 1 == Lcd::NB_LY {
                    self.inc_ly();
                    if bit!(self.stat, StatBit::INT_MODE2) {
                        self.interupt |= Interupt::LcdStats as u8;
                    }
                    self.set_mode(2);
                } else {
                    self.inc_ly();
                }
            }
            2 => {
                self.set_mode(3);
            }
            3 => {
                self.set_mode(0);
            }
            _ => panic!("mode {} does not exist", self.get_mode()),
        };
    }

    pub fn cycle(&mut self) {
        if self.num_idle_cycle == 0 {
            self.change_mode();
            self.num_idle_cycle = Lcd::MODE_DURATION[self.get_mode() as usize];
        }

        self.num_idle_cycle -= 1;
    }

    const BG_ADDR_1: usize = 0x0800;
    const BG_ADDR_2: usize = 0x1000;
    const TILE_ADDR: usize = 0x0000;
    const COLOR: [u32; 4] = [0, 90, 180, 255];

    fn apply_palette(&self, color: u8) -> u8 {
        return (self.bgp >> (color * 2)) & 0x3;
    }

    pub fn get_background(&self, background: &mut [u32]) {
        let bg_addr = if bit!(self.lcdc, LcdcBit::BG_AREA) {
            Lcd::BG_ADDR_1
        } else {
            Lcd::BG_ADDR_2
        };

        for y in 0..256 {
            for x in 0..256 {
                let tile_num = (y / 8) * 32 + (x / 8);
                let mut tile_addr = self.video_ram[bg_addr + tile_num] as usize;
                if !bit!(self.lcdc, LcdcBit::TILE_SOURCE) {
                    tile_addr += 256;
                }
                let lsb_byte = self.video_ram[Lcd::TILE_ADDR + tile_addr * 16 + (y % 8) * 2];
                let msb_byte = self.video_ram[Lcd::TILE_ADDR + tile_addr * 16 + (y % 8) * 2 + 1];
                let bit_pos = 7 - (x % 8);
                let lsb = (lsb_byte >> bit_pos) & 1;
                let msb = (msb_byte >> bit_pos) & 1;
                background[y * 256 + x] = Lcd::COLOR[self.apply_palette(msb * 2 + lsb) as usize];
            }
        }
    }
}
