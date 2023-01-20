use crate::mmu::Mmu;

pub struct Dma;

impl Dma {
    pub fn new() -> Self {
        Dma {}
    }

    pub fn cycle(&self, mmu: &mut Mmu) {
        if let Some(addr) = mmu.lcd.dma_addr {
            // todo: make the dma transfer take 1 cycle per copy
            let source = (addr as u16) << 8;
            for i in 0..mmu.lcd.oam_ram.len() {
                mmu.lcd.oam_ram[i] = mmu.read(source + (i as u16));
            }
            mmu.lcd.dma_addr = None;
        }
    }
}
