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

    pub interrupt_master_enable: bool,
    pub interrupt_enable: u8,
    pub interrupt_flag: u8,
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
            interrupt_master_enable: false,
            interrupt_enable: 0,
            interrupt_flag: 0,
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

pub mod alu {
    pub fn add(a: u8, b: u8, c: bool) -> (u8, bool, bool) {
        let r = a + b + c as u8;
        let h = (a & 0xf) + (b & 0xf) + c as u8 > 0xf;
        let newc = (a as u16 & 0xff) + (b as u16 & 0xff) + c as u16 > 0xff;
        return (r, h, newc);
    }

    pub fn sub(a: u8, b: u8, c: bool) -> (u8, bool, bool) {
        let r = a - b - c as u8;
        let h = (a as i8 & 0xf) - (b as i8 & 0xf) - (c as i8) < 0;
        let newc = (a as i16) - (b as i16) - (c as i16) < 0xff;
        return (r, h, newc);
    }

    pub fn add16h(a: u16, b: u16) -> (u16, bool, bool) {
        let r = a + b;
        let h = (a & 0xfff) + (b & 0xfff) > 0xfff;
        let newc = (a as u32 & 0xffff) + (b as u32 & 0xffff) > 0xffff;
        return (r, h, newc);
    }

    pub fn add16l(a: u16, b: u16) -> (u16, bool, bool) {
        let r = a + b;
        let h = (a & 0xf) + (b & 0xf) > 0xf;
        let newc = (a & 0xff) + (b & 0xff) > 0xff;
        return (r, h, newc);
    }

    pub fn bcd_adjust(v: u8, n: bool, h: bool, c: bool) -> (u8, bool, bool) {
        let fix_l = h | (!n & ((v & 0xf) > 0x9));
        let fix_h = c | (!n & (v > 0x99));
        let fix: u8 = if fix_h { 0x60 } else { 0 } + if fix_l { 0x6 } else { 0 };
        let va: u8 = if n { v - fix } else { v + fix };
        return (va, n, h);
    }
}
