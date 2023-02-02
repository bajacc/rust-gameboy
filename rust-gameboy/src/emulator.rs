use std::thread;
use std::time::{Duration, Instant};

use crate::gb::GameBoy;
use crate::speaker::Speaker;
use crate::{joypad, speaker};

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
const RENDER_DURATION: Duration = Duration::from_nanos(10u64.pow(9) / 20);

const CYCLE_BETWEEN_RENDER: u128 = RENDER_DURATION.as_nanos() / CYCLE_DURATION.as_nanos();

pub fn run(gb: &mut GameBoy, speed: f64, background: bool) {

    let mut renderer = Renderer::new(background, background);
    let mut speaker = Speaker::new(2);

    let cycle_between_render = (CYCLE_BETWEEN_RENDER as f64 * speed) as u128;

    while renderer.lcd_window.is_open() && !renderer.lcd_window.is_key_down(Key::Escape) {
        let last_render = Instant::now();
        for (minifb_key, gb_key) in KEY_MAP {
            if renderer.lcd_window.is_key_down(minifb_key) {
                gb.mmu.joypad.press_key(gb_key);
            } else {
                gb.mmu.joypad.release_key(gb_key);
            }
        }

        let mut rendered = false;
        for _ in 0..cycle_between_render {
            gb.cycle();
            speaker.cycle(gb);
            // only render full window
            if !rendered && gb.mmu.lcd.get_mode() == 1 {
                renderer.render(&gb);
                rendered = true;
            }
        }

        let elapsed = last_render.elapsed();
        if elapsed < RENDER_DURATION {
            thread::sleep(RENDER_DURATION - elapsed);
        } else {
            println!("no sleep loop {} {}", RENDER_DURATION.as_secs_f64(), elapsed.as_secs_f64());
        }
    }
}
