use crate::cpu::Interupt;
use crate::mmu;

pub struct Lcd {
    pub video_ram: [u8; 0x2000],
    pub oam_ram: [u8; 0xa0],
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

    pub display: [u8; Lcd::NUM_PIXELS],
}

pub enum LcdcBit {
    Bg = 0,
    Obj = 1,
    ObjSize = 2,
    BgArea = 3,
    TileSource = 4,
    Win = 5,
    WinArea = 6,
    LcdStatus = 7,
}

pub enum StatBit {
    Mode0 = 0,
    Mode1 = 1,
    LycEqLy = 2,
    IntMode0 = 3,
    IntMode1 = 4,
    IntMode2 = 5,
    IntLyc = 6,
}

pub enum SpriteBit {
    PALETTE = 4,
    FLIP_H = 5,
    FLIP_V = 6,
    BEHIND_BG = 7,
}

macro_rules! bit {
    ($x:expr, $pos:expr) => {
        ($x & (1 << ($pos as u8))) != 0
    };
}

impl Lcd {
    pub const HEIGHT: u8 = 144;
    pub const WIDTH: u8 = 160;
    pub const NUM_PIXELS: usize = (Lcd::HEIGHT as usize) * (Lcd::WIDTH as usize);

    pub fn new() -> Self {
        Lcd {
            video_ram: [0; 0x2000],
            oam_ram: [0; 0xa0],
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
            display: [0; Lcd::NUM_PIXELS],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9fff => self.video_ram[addr as usize - 0x8000],
            0xfe00..=0xfe9f => self.oam_ram[addr as usize - 0xfe00],
            0xff40 => self.lcdc,
            0xff41 => self.stat,
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,
            0xff46 => mmu::NO_DATA, // dma
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
            0xfe00..=0xfe9f => self.oam_ram[addr as usize - 0xfe00] = value,
            0xff40 => {
                self.lcdc = value;
                if !bit!(self.lcdc, LcdcBit::LcdStatus) {
                    // switch off lcd
                    self.ly = 0; // todo interupts?
                    for pixel in self.display.iter_mut() {
                        *pixel = 0;
                    }
                }
            }
            0xff41 => self.stat = (self.stat & 0x07) | (value & !0x07),
            0xff42 => self.scy = value,
            0xff43 => self.scx = value,
            0xff44 => (), // ly
            0xff45 => self.lyc = value,
            0xff46 => {
                // DMA
                // todo: make the dma transfer take 1 cycle per copy
                let source = (value as u16) << 8;
                for i in 0..self.oam_ram.len() {
                    self.oam_ram[i] = self.read(source + (i as u16));
                }
            }
            0xff47 => self.bgp = value,
            0xff48 => self.obp0 = value,
            0xff49 => self.obp1 = value,
            0xff4a => self.wx = value,
            0xff4b => self.wy = value,
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }

    fn get_pixel(&self, y: usize, x: usize, tile_index_addr: usize) -> u8 {
        let tile_num = (y / 8) * 32 + (x / 8);
        let mut tile_addr = self.video_ram[tile_index_addr + tile_num] as usize;
        if !bit!(self.lcdc, LcdcBit::TileSource) {
            tile_addr += 256;
        }
        let lsb_byte = self.video_ram[Lcd::TILE_ADDR + tile_addr * 16 + (y % 8) * 2];
        let msb_byte = self.video_ram[Lcd::TILE_ADDR + tile_addr * 16 + (y % 8) * 2 + 1];
        let bit_pos = 7 - (x % 8);
        let lsb = (lsb_byte >> bit_pos) & 1;
        let msb = (msb_byte >> bit_pos) & 1;
        return msb * 2 + lsb;
    }

    const OBJ_SIZE_Y: [i16; 2] = [8, 16];

    fn get_sprite_pixel(&self, bg: &mut [u8], fg: &mut [u8], y: i16) {
        let mut counter = 0;
        for id_sprite in 0..40 {
            let sprite_y = self.oam_ram[id_sprite * 4] as i16 - 16;
            let sprite_x = self.oam_ram[id_sprite * 4 + 1] as i16 - 8;
            let tile_addr = self.oam_ram[id_sprite * 4 + 2] as usize;
            let sprite_bits = self.oam_ram[id_sprite * 4 + 3];
            if !(y >= sprite_y
                && y < sprite_y + Lcd::OBJ_SIZE_Y[bit!(self.lcdc, LcdcBit::ObjSize) as usize])
            {
                continue;
            }
            counter += 1;
            if counter == 10 {
                return;
            }

            let lsb_byte = self.video_ram[Lcd::TILE_ADDR + tile_addr * 16];
            let msb_byte = self.video_ram[Lcd::TILE_ADDR + tile_addr * 16];

            for x in 0..8 {
                // todo
            }
        }
    }

    fn draw_line(&mut self) {
        for x in 0..Lcd::WIDTH {
            if bit!(self.lcdc, LcdcBit::Bg) {
                let tile_index_addr =
                    Lcd::TILE_INDEX_ADDR[bit!(self.lcdc, LcdcBit::BgArea) as usize];
                let pixel = self.get_pixel(
                    (self.ly + self.scy) as usize,
                    (x + self.scx) as usize,
                    tile_index_addr,
                );
            }
            let wx = self.wx as isize - 7;
            let wy = self.wy as isize;
            if bit!(self.lcdc, LcdcBit::Win) && x as isize >= wx && self.ly as isize >= wy {
                // todo count line for window
                let tile_index_addr =
                    Lcd::TILE_INDEX_ADDR[bit!(self.lcdc, LcdcBit::WinArea) as usize];
                let pixel = self.get_pixel(
                    (self.ly as isize - wy) as usize,
                    (x as isize - wx) as usize,
                    tile_index_addr,
                );
            }

            if bit!(self.lcdc, LcdcBit::Obj) {}
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

    fn inc_ly(&mut self) {
        self.ly += 1;
        if self.ly == Lcd::NB_LY {
            self.ly = 0;
        }
        if self.ly == self.lyc {
            self.stat |= 1 << (StatBit::LycEqLy as u8);
        } else {
            self.stat &= !(1 << (StatBit::LycEqLy as u8));
        }
        if bit!(self.stat, StatBit::IntLyc) {
            self.interupt |= Interupt::LcdStats as u8;
        }
    }

    fn change_mode(&mut self) {
        match self.get_mode() {
            0 => {
                if !bit!(self.lcdc, LcdcBit::LcdStatus) {
                } else if self.ly + 1 == Lcd::HEIGHT {
                    self.inc_ly();
                    self.interupt |= Interupt::Vblank as u8;
                    if bit!(self.stat, StatBit::IntMode1) {
                        self.interupt |= Interupt::LcdStats as u8;
                    }
                    self.set_mode(1);
                } else {
                    self.inc_ly();
                    if bit!(self.stat, StatBit::IntMode2) {
                        self.interupt |= Interupt::LcdStats as u8;
                    }
                    self.set_mode(2);
                }
            }
            1 => {
                if self.ly + 1 == Lcd::NB_LY {
                    self.inc_ly();
                    if bit!(self.stat, StatBit::IntMode2) {
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
                // todo draw in multiple cycles
                self.draw_line();
                if bit!(self.stat, StatBit::IntMode0) {
                    self.interupt |= Interupt::LcdStats as u8;
                }
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

    const TILE_INDEX_ADDR: [usize; 2] = [0x1800, 0x1C00];
    const TILE_ADDR: usize = 0x0000;
    const COLOR: [u32; 4] = [0, 90, 180, 255];

    fn apply_palette(&self, color: u8) -> u8 {
        return (self.bgp >> (color * 2)) & 0x3;
    }

    pub fn get_background(&self, background: &mut [u32], bg_area: bool) {
        let tile_index_addr = Lcd::TILE_INDEX_ADDR[bg_area as usize];

        for y in 0..256 {
            for x in 0..256 {
                let pixel = self.get_pixel(y, x, tile_index_addr);
                background[y * 256 + x] = Lcd::COLOR[self.apply_palette(pixel) as usize];
            }
        }
    }

    pub fn get_tiles(&self, buffer: &mut [u32]) {
        // buffer size 384 = 3 * 128 = 24 * 16 tiles. 192 * 128
        for y in 0..24 * 8 {
            for x in 0..16 * 8 {
                let tile_addr = (y / 8) * 16 + (x / 8);
                let lsb_byte = self.video_ram[Lcd::TILE_ADDR + tile_addr * 16 + (y % 8) * 2];
                let msb_byte = self.video_ram[Lcd::TILE_ADDR + tile_addr * 16 + (y % 8) * 2 + 1];
                let bit_pos = 7 - (x % 8);
                let lsb = (lsb_byte >> bit_pos) & 1;
                let msb = (msb_byte >> bit_pos) & 1;
                buffer[y * 128 + x] = Lcd::COLOR[(msb * 2 + lsb) as usize];
            }
        }
    }
}
