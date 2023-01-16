use crate::cpu::Cpu;
use crate::desassembler::disassemble;
use crate::mbc::Mbc;
use crate::mmu::Mmu;

pub struct GameBoy {
    pub mmu: Mmu,
    pub cpu: Cpu,
}

impl GameBoy {
    pub fn new(mbc: Mbc) -> Self {
        GameBoy {
            mmu: Mmu::new(mbc),
            cpu: Cpu::new(),
        }
    }

    pub fn cycle(&mut self) {
        self.mmu.cycle();
        self.cpu.cycle(&mut self.mmu);
    }

    pub fn step(&mut self) {
        let pc = self.cpu.pc;
        while pc == self.cpu.pc {
            self.cycle();
        }
    }

    pub fn disassemble(&self, num: usize) {
        disassemble(&self.cpu, &self.mmu, num);
    }
}
