use std::thread;
use std::time::{Duration, Instant};

use crate::gb::GameBoy;
use crate::joypad;

use crate::lcd::Lcd;
use crate::renderer::Renderer;
use minifb::Key;

const KEY_MAP: [(Key, joypad::Key); 8] = [
    (Key::Z, joypad::Key::A),
    (Key::X, joypad::Key::B),
    (Key::Enter, joypad::Key::Start),
    (Key::Backspace, joypad::Key::Select),
    (Key::Up, joypad::Key::Up),
    (Key::Down, joypad::Key::Down),
    (Key::Right, joypad::Key::Right),
    (Key::Left, joypad::Key::Left),
];
// 2^20 cycle per second
const CYCLE_DURATION: Duration = Duration::from_nanos(10u64.pow(9) / 2u64.pow(20));
// 60 fps
const RENDER_DURATION: Duration = Duration::from_nanos(10u64.pow(9) / 60);

const CYCLE_BETWEEN_RENDER: u128 = RENDER_DURATION.as_nanos() / CYCLE_DURATION.as_nanos();

pub fn run(gb: &mut GameBoy, speed: f64, background: bool) {
    let mut last_render = Instant::now();

    let mut renderer = Renderer::new(Lcd::HEIGHT as usize, Lcd::WIDTH as usize, 4);
    
    let mut bg_renderer = Renderer::new(256, 256, 1);
    let mut bg_renderer2 = Renderer::new(256, 256, 1);
    let mut tile_renderer = Renderer::new(192, 128, 1);
    let mut bg_buffer: [u32; 256 * 256] = [0; 256 * 256];
    let mut tile_buffer: [u32; 192 * 128] = [0; 192 * 128];

    let cycle_between_render = (CYCLE_BETWEEN_RENDER as f64 * speed) as u128;

    while renderer.window.is_open() && !renderer.window.is_key_down(Key::Escape) {
        for (minifb_key, gb_key) in KEY_MAP {
            if renderer.window.is_key_down(minifb_key) {
                gb.mmu.joypad.press_key(gb_key);
            } else {
                gb.mmu.joypad.release_key(gb_key);
            }
        }

        for _ in 0..cycle_between_render {
            gb.cycle();
        }

        // wait for the display to be drawn to avoid half drawn window
        // todo make the number of cycle per loop constant
        while gb.mmu.lcd.get_mode() != 1 {
            gb.cycle();
        }
        if background {
            gb.mmu.lcd.get_background(&mut bg_buffer, false);
            bg_renderer.render(&bg_buffer);
            gb.mmu.lcd.get_background(&mut bg_buffer, true);
            bg_renderer2.render(&bg_buffer);
            gb.mmu.lcd.get_tiles(&mut tile_buffer);
            tile_renderer.render(&tile_buffer);
        }
        renderer.render_u8(&gb.mmu.lcd.display);
        

        let elapsed = last_render.elapsed();
        if elapsed < RENDER_DURATION {
            thread::sleep(RENDER_DURATION - elapsed);
        }
        last_render = Instant::now();
    }
}
