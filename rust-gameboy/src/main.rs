mod alu;
mod cpu;
mod desassembler;
mod dma;
mod gb;
mod joypad;
mod lcd;
mod mbc;
mod mmu;
mod opcodes;
mod opcodes_const;
mod renderer;
mod timer;

use std::thread;
use std::time::{Duration, Instant};

use gb::GameBoy;
use mbc::Mbc;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

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

const SPEED_UP: u128 = 1;

const NUM_CYCLE_BETWEEN_RENDER: u128 =
    RENDER_DURATION.as_nanos() * SPEED_UP / CYCLE_DURATION.as_nanos();

// todo: use clap to give more option at launch
fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let mut last_render = Instant::now();

    let f = File::open(&args[1]).expect("couldn't read file");
    let mut reader = BufReader::new(f);

    let mut arr: [u8; 0x8000] = [0; 0x8000];
    // Read file into vector.
    reader.read(&mut arr).expect("couldn't read file");

    let mbc = Mbc::new(arr);
    let mut gb = GameBoy::new(mbc);
    let mut renderer = Renderer::new(Lcd::HEIGHT as usize, Lcd::WIDTH as usize, 4);

    while renderer.window.is_open() && !renderer.window.is_key_down(Key::Escape) {
        for (minifb_key, gb_key) in KEY_MAP {
            if renderer.window.is_key_down(minifb_key) {
                gb.mmu.joypad.press_key(gb_key);
            } else {
                gb.mmu.joypad.release_key(gb_key);
            }
        }

        for _ in 0..NUM_CYCLE_BETWEEN_RENDER {
            gb.cycle();
        }

        // wait for the display to be drawn to avoid half drawn window
        while gb.mmu.lcd.get_mode() != 1 {
            gb.cycle();
        }

        renderer.render_u8(&gb.mmu.lcd.display);

        let elapsed = last_render.elapsed();
        if elapsed < RENDER_DURATION {
            thread::sleep(RENDER_DURATION - elapsed);
        }
        last_render = Instant::now();
    }
}
