use crate::cpu::{alu::add, Cpu};

pub struct Mmu {
    cpu: Cpu,
}

impl Mmu {
    pub fn new() -> Self {
        Mmu { cpu: Cpu::new() }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xffff => self.cpu.interrupt_enable,
            0xff0f => self.cpu.interrupt_flag,
            _ => {
                println!("{:?}", addr);
                0xff
            }
        }
    }

    pub fn write(&self, addr: u16, value: u8) {
        match addr {
            _ => println!("{:?}, {:?}", addr, value),
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        return (self.read(addr) as u16) | ((self.read(addr + 1) as u16) << 8);
    }

    pub fn write16(&self, addr: u16, value: u16) {
        self.write(addr, value as u8);
        self.write(addr + 1, (value >> 8) as u8);
    }
}
