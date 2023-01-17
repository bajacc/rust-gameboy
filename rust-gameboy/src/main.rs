mod alu;
mod cpu;
mod desassembler;
mod gb;
mod lcd;
mod mbc;
mod mmu;
mod opcodes;
mod opcodes_const;
mod timer;

use gb::GameBoy;
use mbc::Mbc;

use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

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

    gb.mmu.disable_boot_rom = true;
    gb.cpu.pc = 0x100;

    gb.disassemble(10);

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read input");

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
                for _ in 0..100000 {
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
                while gb.mmu.read(gb.cpu.pc) != 0xfb {
                    gb.cycle();
                }
                gb.disassemble(10);
                for i in 0..16 {
                    print!("{:02x} ", gb.mmu.read(0xffff - i));
                }
                gb.cpu.print();
            }
            "r" => loop {
                gb.cycle();
            },
            "c" => gb.cpu.print(),
            "p" => gb.disassemble(30),
            _ => println!("command {} not found", s.trim()),
        }
    }
}
