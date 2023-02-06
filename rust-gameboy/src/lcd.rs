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
    window_line: u8,

    pub display: [u8; Lcd::NUM_PIXELS],
    pub dma_addr: Option<u8>,
}

struct Sprite {
    pub y: u8,
    pub x: u8,
    pub idx_tile: u8,
    pub palette: bool,
    pub flip_h: bool,
    pub flip_v: bool,
    pub behind_bg: bool,

    pub idx_sprite: u8,
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
    Palette = 4,
    FlipH = 5,
    FlipV = 6,
    BehindBg = 7,
}

macro_rules! bit {
    ($x:expr, $pos:expr) => {
        ($x & (1 << ($pos as u8))) != 0
    };
}

const TRANSPARENT: u8 = 0xff;

fn apply_palette(palette: u8, color: u8) -> u8 {
    return (palette >> (color * 2)) & 0x3;
}

impl Lcd {
    pub const HEIGHT: u8 = 144;
    pub const WIDTH: u8 = 160;
    pub const NUM_PIXELS: usize = (Lcd::HEIGHT as usize) * (Lcd::WIDTH as usize);

    const MODE_DURATION: [usize; 4] = [51, 114, 20, 43];
    const NB_LY: u8 = 154;
    const TILE_INDEX_ADDR: [usize; 2] = [0x1800, 0x1C00];
    const TILE_ADDR: usize = 0x0000;

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
            window_line: 0,
            display: [0; Lcd::NUM_PIXELS],
            dma_addr: None,
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
            0xff4a => self.wy,
            0xff4b => self.wx,
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
                    self.ly = 0;
                    self.set_mode(0);
                    self.check_ly_eq_lyc();
                    self.window_line = 0;
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
            0xff46 => self.dma_addr = Some(value),
            0xff47 => self.bgp = value,
            0xff48 => self.obp0 = value,
            0xff49 => self.obp1 = value,
            0xff4a => self.wy = value,
            0xff4b => self.wx = value,
            _ => panic!("0x{:04x}, 0x{:02x}", addr, value),
        }
    }

    fn get_pixel_bg(&self, y: usize, x: usize, tile_index_addr: usize) -> u8 {
        let tile_num = (y / 8) * 32 + (x / 8);
        let mut tile_addr = self.video_ram[tile_index_addr + tile_num] as usize;
        if !bit!(self.lcdc, LcdcBit::TileSource) && tile_addr < 0x80 {
            tile_addr += 256;
        }
        let lsb_byte = self.video_ram[Lcd::TILE_ADDR + tile_addr * 16 + (y % 8) * 2];
        let msb_byte = self.video_ram[Lcd::TILE_ADDR + tile_addr * 16 + (y % 8) * 2 + 1];
        let bit_pos = 7 - (x % 8);
        let lsb = (lsb_byte >> bit_pos) & 1;
        let msb = (msb_byte >> bit_pos) & 1;
        return msb * 2 + lsb;
    }

    const OBJ_SIZE_Y: [isize; 2] = [8, 16];

    fn get_sprite_on_line(&self, y: isize) -> Vec<Sprite> {
        let mut sprites = Vec::new();
        let mut counter = 0;
        let obj_size = Lcd::OBJ_SIZE_Y[bit!(self.lcdc, LcdcBit::ObjSize) as usize];
        for idx_sprite in 0..40 {
            let sprite_y = self.oam_ram[idx_sprite * 4] as isize - 16;
            if !(sprite_y <= y && y < sprite_y + obj_size) {
                continue;
            }

            let sprite_bits = self.oam_ram[idx_sprite * 4 + 3];
            sprites.push(Sprite {
                y: self.oam_ram[idx_sprite * 4],
                x: self.oam_ram[idx_sprite * 4 + 1],
                idx_tile: self.oam_ram[idx_sprite * 4 + 2],
                idx_sprite: idx_sprite as u8,
                palette: bit!(sprite_bits, SpriteBit::Palette),
                flip_h: bit!(sprite_bits, SpriteBit::FlipH),
                flip_v: bit!(sprite_bits, SpriteBit::FlipV),
                behind_bg: bit!(sprite_bits, SpriteBit::BehindBg),
            });

            counter += 1;
            if counter == 10 {
                break;
            }
        }

        sprites.sort_unstable_by_key(|s| -((s.x as isize) << 8 | s.idx_sprite as isize));
        return sprites;
    }

    fn get_sprite_lines(&self, bg: &mut [u8], fg: &mut [u8], y: isize) {
        let sprites = self.get_sprite_on_line(y);
        for sprite in sprites {
            let mut idx_tile = sprite.idx_tile as usize;
            let sprite_y = sprite.y as isize - 16;
            let mut y_on_tile = (y - sprite_y) as usize;

            if y_on_tile >= 8 {
                y_on_tile -= 8;
                idx_tile ^= 1;
            }

            if sprite.flip_v {
                idx_tile ^= 1;
                y_on_tile = 7 - y_on_tile;
            }

            let lsb_byte = self.video_ram[Lcd::TILE_ADDR + idx_tile * 16 + y_on_tile * 2];
            let msb_byte = self.video_ram[Lcd::TILE_ADDR + idx_tile * 16 + y_on_tile * 2 + 1];

            let arr = if sprite.behind_bg { &mut *bg } else { &mut *fg };
            let palette = if sprite.palette { self.obp1 } else { self.obp0 };

            for i in 0..8 {
                let x = if sprite.flip_h { i } else { 7 - i };
                let lsb = (lsb_byte >> x) & 1;
                let msb = (msb_byte >> x) & 1;
                let mut pixel = msb * 2 + lsb;
                pixel = if pixel == 0 {
                    TRANSPARENT
                } else {
                    apply_palette(palette, pixel)
                };
                let x_on_screen = sprite.x as isize + i - 8;

                if 0 <= x_on_screen && x_on_screen < Lcd::WIDTH as isize && pixel != TRANSPARENT {
                    arr[x_on_screen as usize] = pixel;
                }
            }
        }
    }

    fn draw_line(&mut self) {
        let mut sprite_bg = [TRANSPARENT; Lcd::WIDTH as usize];
        let mut sprite_fg = [TRANSPARENT; Lcd::WIDTH as usize];

        if bit!(self.lcdc, LcdcBit::Obj) {
            self.get_sprite_lines(&mut sprite_bg, &mut sprite_fg, self.ly as isize);
        }

        let tile_index_addr_bg = Lcd::TILE_INDEX_ADDR[bit!(self.lcdc, LcdcBit::BgArea) as usize];
        let tile_index_addr_win = Lcd::TILE_INDEX_ADDR[bit!(self.lcdc, LcdcBit::WinArea) as usize];

        let bg_y = (self.ly as usize + self.scy as usize) & 0xff;
        let wx = self.wx as isize - 7;
        for x in 0..(Lcd::WIDTH as usize) {
            let bg_x = (x + self.scx as usize) & 0xff;
            let mut pixel = TRANSPARENT;
            if bit!(self.lcdc, LcdcBit::Bg) {
                pixel = self.get_pixel_bg(bg_y, bg_x, tile_index_addr_bg);
                pixel = apply_palette(self.bgp, pixel);
                if pixel == 0 {
                    pixel = TRANSPARENT;
                }
            }
            if bit!(self.lcdc, LcdcBit::Win) && self.ly >= self.wy && x as isize >= wx {
                let x_on_window = (x as isize - wx) as usize & 255;
                pixel =
                    self.get_pixel_bg(self.window_line as usize, x_on_window, tile_index_addr_win);
                pixel = apply_palette(self.bgp, pixel);
                if pixel == 0 {
                    pixel = TRANSPARENT;
                }
            }
            let pixel_addr = self.ly as usize * Lcd::WIDTH as usize + x;
            if pixel != TRANSPARENT || sprite_bg[x] == TRANSPARENT {
                self.display[pixel_addr] = pixel;
            } else {
                self.display[pixel_addr] = sprite_bg[x]
            }
            if sprite_fg[x] != TRANSPARENT {
                self.display[pixel_addr] = sprite_fg[x];
            }
            if self.display[pixel_addr] == TRANSPARENT {
                self.display[pixel_addr] = 0;
            }
        }

        if bit!(self.lcdc, LcdcBit::Win) && self.ly >= self.wy {
            self.window_line += 1
        }
    }

    pub fn extract_interupt(&mut self) -> u8 {
        let r = self.interupt;
        self.interupt = 0;
        return r;
    }

    pub fn get_mode(&self) -> u8 {
        return self.stat & 3;
    }

    fn set_mode(&mut self, value: u8) {
        self.stat &= !3;
        self.stat |= value & 3;
    }

    fn check_ly_eq_lyc(&mut self) {
        if self.ly == self.lyc {
            self.stat |= 1 << (StatBit::LycEqLy as u8);
            if bit!(self.stat, StatBit::IntLyc) {
                self.interupt |= Interupt::LcdStats as u8;
            }
        } else {
            self.stat &= !(1 << (StatBit::LycEqLy as u8));
        }
    }

    fn inc_ly(&mut self) {
        self.ly = (1 + self.ly) % Lcd::NB_LY;
        if self.ly == 0 {
            self.window_line = 0;
        }
        self.check_ly_eq_lyc();
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

    const COLOR: [u32; 4] = [0xffffff, 0xc0c0c0, 0x606060, 0x000000];

    pub fn get_background(&self, background: &mut [u32], bg_area: bool) {
        let tile_index_addr = Lcd::TILE_INDEX_ADDR[bg_area as usize];

        for y in 0..256 {
            for x in 0..256 {
                let pixel = self.get_pixel_bg(y, x, tile_index_addr);
                background[y * 256 + x] = Lcd::COLOR[pixel as usize];
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
