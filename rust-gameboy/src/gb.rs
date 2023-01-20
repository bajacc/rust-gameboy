use crate::cpu::Cpu;
use crate::desassembler::disassemble;
use crate::dma::Dma;
use crate::mbc::Mbc;
use crate::mmu::Mmu;

pub struct GameBoy {
    pub mmu: Mmu,
    pub cpu: Cpu,
    pub dma: Dma,
}

impl GameBoy {
    pub fn new(mbc: Mbc) -> Self {
        GameBoy {
            mmu: Mmu::new(mbc),
            cpu: Cpu::new(),
            dma: Dma::new(),
        }
    }

    pub fn cycle(&mut self) {
        self.mmu.cycle();
        self.cpu.cycle(&mut self.mmu);
        self.dma.cycle(&mut self.mmu);
    }

    pub fn disassemble(&self, num: usize) {
        disassemble(&self.cpu, &self.mmu, num);
    }
}
