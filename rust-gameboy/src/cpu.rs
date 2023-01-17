use crate::mmu::Mmu;
use crate::opcodes;
use crate::opcodes_const;

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
    pub halt: bool,
    num_idle_cycle: usize,
}

#[derive(Copy, Clone)]
pub enum Interupt {
    None = 0,
    Mask = (1 << 5) - 1,

    Vblank = 1 << 0,
    LcdStats = 1 << 1,
    Timer = 1 << 2,
    Serial = 1 << 3,
    Joypad = 1 << 4,
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
            pc: 0,
            interrupt_master_enable: false,
            halt: false,
            num_idle_cycle: 0,
        }
    }

    pub fn print(&self) {
        println!(
            "a: {:#02x}, f: {:#02x}\n\
        b: {:#02x}, c: {:#02x}\n\
        d: {:#02x}, e: {:#02x}\n\
        h: {:#02x}, l: {:#02x}\n\
        sp: {:#04x}\n\
        pc: {:#04x}\n\
        flags: {:08b} znhc0000\n",
            self.a,
            self.f,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc,
            self.f
        );
    }

    fn handle_interupt(&mut self, mmu: &mut Mmu, mut interupts: u8) {
        self.interrupt_master_enable = false;
        let mut id_interupt = 0;
        while interupts & 1 == 0 {
            id_interupt += 1;
            interupts >>= 1;
        }
        mmu.interupt_flag &= !(1 << id_interupt);
        opcodes::push(self, mmu, self.pc);
        self.pc = 0x40 + id_interupt * 8;
    }

    pub fn cycle(&mut self, mmu: &mut Mmu) {
        if self.num_idle_cycle == 0 {
            self.num_idle_cycle = self.step(mmu);
        }
        self.num_idle_cycle -= 1;
    }

    pub fn step(&mut self, mmu: &mut Mmu) -> usize {
        let interupts = mmu.get_interupts();
        if self.halt {
            if interupts == 0 {
                return 1;
            }
            self.halt = false;
        }

        if self.interrupt_master_enable && interupts != 0 {
            self.handle_interupt(mmu, interupts);
            return 5;
        }

        let opcode = mmu.read(self.pc);
        let len = opcodes_const::OPCODE_BYTES_LEN[opcode as usize] as u16;
        let arg = match len {
            2 => mmu.read(self.pc + 1) as u16,
            3 => mmu.read16(self.pc + 1),
            _ => 0,
        };
        let duration = opcodes::duration_opcode(opcode, arg, self.f);

        self.pc += len;
        match opcode {
            0xcb => opcodes::execute_prefixed(self, mmu, arg as u8),
            _ => opcodes::execute_unprefixed(self, mmu, opcode, arg),
        };
        return duration;
    }
}
