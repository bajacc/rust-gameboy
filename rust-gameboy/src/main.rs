mod alu;
mod cpu;
mod debugger;
mod desassembler;
mod dma;
mod emulator;
mod gb;
mod joypad;
mod lcd;
mod mbc;
mod mmu;
mod opcodes;
mod opcodes_const;
mod renderer;
mod timer;

use debugger::Debugger;
use std::path::{Path, PathBuf};
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

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // path
    path: PathBuf,

    // use debuging mode
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long, default_value_t = 1f64)]
    speed: f64,
}

// todo: use clap to give more option at launch
fn main() {
    let cli = Cli::parse();

    let f = File::open(cli.path).expect("couldn't read file");
    let mut reader = BufReader::new(f);

    let mut arr: [u8; 0x8000] = [0; 0x8000];
    reader.read(&mut arr).expect("couldn't read file");

    let mbc = Mbc::new(arr);
    let mut gb = GameBoy::new(mbc);

    if cli.debug {
        Debugger::new(gb).run();
    } else {
        emulator::run(&mut gb, cli.speed);
    }
}
