extern crate minifb;
use crate::{gb::GameBoy, Lcd};
use minifb::{Scale, Window, WindowOptions};

pub struct Renderer {
    pub lcd_window: Window,
    pub tiles_window: Option<Window>,
    pub bg_window: Vec<Window>,
}

const LCD_WINDOW_WIDTH: usize = Lcd::WIDTH as usize;
const LCD_WINDOW_HEIGH: usize = Lcd::HEIGHT as usize;

const TILE_WINDOW_WIDTH: usize = 128;
const TILE_WINDOW_HEIGHT: usize = 192;

const BG_WINDOW_WIDTH: usize = 256;
const BG_WINDOW_HEIGHT: usize = 256;

impl Renderer {
    pub fn new(has_tile: bool, has_bg: bool) -> Self {
        let lcd_window = Window::new(
            "GameBoy",
            LCD_WINDOW_WIDTH,
            LCD_WINDOW_HEIGH,
            WindowOptions {
                resize: false,
                scale: Scale::X4,
                ..WindowOptions::default()
            },
        )
        .unwrap();
        // Limit to max ~60 fps update rate
        //lcd_window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let mut tiles_window = None;
        if has_tile {
            let window = Window::new(
                "GameBoy",
                TILE_WINDOW_WIDTH,
                TILE_WINDOW_HEIGHT,
                WindowOptions::default(),
            )
            .unwrap();
            tiles_window = Some(window);
        }

        let mut bg_window = Vec::new();
        if has_bg {
            let window1 = Window::new(
                "GameBoy",
                BG_WINDOW_WIDTH,
                BG_WINDOW_HEIGHT,
                WindowOptions::default(),
            )
            .unwrap();
            bg_window.push(window1);
            let window2 = Window::new(
                "GameBoy",
                BG_WINDOW_WIDTH,
                BG_WINDOW_HEIGHT,
                WindowOptions::default(),
            )
            .unwrap();
            bg_window.push(window2);
        }

        Renderer {
            lcd_window: lcd_window,
            tiles_window: tiles_window,
            bg_window: bg_window,
        }
    }

    pub fn render(&mut self, gb: &GameBoy) {
        if let Some(tiles_window) = self.tiles_window.as_mut() {
            let mut tile_buffer = [0; TILE_WINDOW_WIDTH * TILE_WINDOW_HEIGHT];
            gb.mmu.lcd.get_tiles(&mut tile_buffer);
            tiles_window
                .update_with_buffer(&tile_buffer, TILE_WINDOW_WIDTH, TILE_WINDOW_HEIGHT)
                .unwrap();
        }
        for i in 0..self.bg_window.len() {
            let window = &mut self.bg_window[i];
            let mut bg_buffer = [0; BG_WINDOW_WIDTH * BG_WINDOW_HEIGHT];
            gb.mmu.lcd.get_background(&mut bg_buffer, i == 1);
            window
                .update_with_buffer(&bg_buffer, BG_WINDOW_WIDTH, BG_WINDOW_HEIGHT)
                .unwrap();
        }
        self.render_u8(&gb.mmu.lcd.display);
    }

    const COLOR: [u32; 4] = [0xffffff, 0xc0c0c0, 0x606060, 0x000000];

    pub fn render_u8(&mut self, buffer: &[u8]) {
        let mut b = [0; LCD_WINDOW_HEIGH * LCD_WINDOW_WIDTH];
        for i in 0..buffer.len() {
            b[i] = Renderer::COLOR[buffer[i] as usize];
        }
        self.lcd_window
            .update_with_buffer(&b, LCD_WINDOW_WIDTH, LCD_WINDOW_HEIGH)
            .unwrap();
    }
}
