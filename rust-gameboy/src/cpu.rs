use crate::mmu::Mmu;
use crate::opcodes;
use crate::opcodes_const::OPCODE_BYTES_LEN;

pub struct Cpu {
    pub a: u8,
    pub f: u8, // 0xznhc0000
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100,
        }
    }

    pub fn step(&mut self, mmu: &mut Mmu) {
        let opcode = mmu.read(self.pc);
        let len = OPCODE_BYTES_LEN[opcode as usize] as u16;
        let arg = match len {
            2 => mmu.read(self.pc + 1) as u16,
            3 => mmu.read16(self.pc + 1),
            _ => 0,
        };
        match opcode {
            0xCB => opcodes::execute_prefixed(self, mmu, arg as u8),
            _ => opcodes::execute_unprefixed(self, mmu, opcode, arg),
        };
        self.sp += len;
    }
}
