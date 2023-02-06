mod alu;
mod blargg_test;
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
mod sound;
mod speaker;
mod timer;

use debugger::Debugger;
use std::path::PathBuf;

use gb::GameBoy;

use crate::lcd::Lcd;

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

    // display background and window
    #[arg(short, long)]
    background: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut gb = GameBoy::from_path(cli.path);

    if cli.debug {
        Debugger::new(gb).run();
    } else {
        emulator::run(&mut gb, cli.speed, cli.background);
    }
}
