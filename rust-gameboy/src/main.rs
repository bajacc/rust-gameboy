mod alu;
mod cpu;
mod desassembler;
mod gb;
mod lcd;
mod mbc;
mod mmu;
mod opcodes;
mod opcodes_const;
mod renderer;
mod timer;

use gb::GameBoy;
use mbc::Mbc;

use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

use crate::renderer::Renderer;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let f = File::open(&args[1]).expect("couldn't read file");
    let mut reader = BufReader::new(f);

    let mut arr: [u8; 0x8000] = [0; 0x8000];
    // Read file into vector.
    reader.read(&mut arr).expect("couldn't read file");

    let mbc = Mbc::new(arr);
    let mut gb = GameBoy::new(mbc);
    let mut renderer = Renderer::new(256, 256);

    gb.disassemble(10);

    let mut buffer: [u32; 256 * 256] = [0; 256 * 256];

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read input");

        gb.mmu.lcd.get_background(&mut buffer);
        renderer.render(&buffer);

        match s.trim() {
            "s" => {
                gb.step();
                gb.disassemble(1);
                for i in 0..16 {
                    print!("{:02x} ", gb.mmu.read(0xffff - i));
                }
                println!();
            }
            "ss" => {
                for _ in 0..1000000 {
                    gb.step();
                }
                gb.disassemble(10);
                for i in 0..16 {
                    print!("{:02x} ", gb.mmu.read(0xffff - i));
                }
                println!();
            }
            "l" => {
                let pc = gb.cpu.pc;
                gb.step();
                while pc != gb.cpu.pc {
                    gb.step();
                }
                gb.disassemble(10);
                for i in 0..16 {
                    print!("{:02x} ", gb.mmu.read(0xffff - i));
                }
                gb.cpu.print();
            }
            "f" => {
                let pc = gb.cpu.pc;
                while gb.cpu.pc <= pc {
                    gb.cycle();
                }
                gb.disassemble(10);
                for i in 0..16 {
                    print!("{:02x} ", gb.mmu.read(0xffff - i));
                }
                gb.cpu.print();
            }
            "r" => loop {
                gb.mmu.lcd.get_background(&mut buffer);
                renderer.render(&buffer);
                for i in 0..100000 {
                    gb.cycle();
                }
            },
            "c" => gb.cpu.print(),
            "p" => gb.disassemble(30),
            _ => panic!("command {} not found", s.trim()),
        }
    }
}
