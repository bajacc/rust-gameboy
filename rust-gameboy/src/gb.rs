use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::cpu::Cpu;
use crate::desassembler::disassemble;
use crate::dma::Dma;
use crate::mbc::Mbc;
use crate::mmu::Mmu;

pub struct GameBoy {
    pub mmu: Mmu,
    pub cpu: Cpu,
    pub dma: Dma,
    pub num_cycle: u128,
}

impl GameBoy {
    pub fn new(mbc: Mbc) -> Self {
        GameBoy {
            mmu: Mmu::new(mbc),
            cpu: Cpu::new(),
            dma: Dma::new(),
            num_cycle: 0,
        }
    }

    pub fn from_path(path: PathBuf) -> Self {
        let mut f = File::open(path).expect("couldn't read file");
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).expect("couldn't read file");

        let mbc = Mbc::new(buffer);
        return GameBoy::new(mbc);
    }

    pub fn cycle(&mut self) {
        self.mmu.cycle();
        self.cpu.cycle(&mut self.mmu);
        self.dma.cycle(&mut self.mmu);
        self.num_cycle += 1;
    }

    pub fn disassemble(&self, num: usize) {
        disassemble(&self.cpu, &self.mmu, num);
    }
}
