{% import "macros.rs" as macros %}

use crate::opcodes_const::OPCODE_BYTES_LEN;
use crate::cpu::Cpu;
use crate::mmu::Mmu;

pub fn disassemble(cpu: &Cpu, mmu: &Mmu, num: usize) {
    let mut pos = cpu.pc;
    for _ in 0..num {
        let len = OPCODE_BYTES_LEN[mmu.read(pos) as usize] as u16;
        print!("{:02x}: ", pos);
        for i in pos..pos+len {
            print!("{:02x} ", mmu.read(i));
        }
        print!("\t");
        match mmu.read(pos) {
            0xCB => disassemble_prefixed(mmu, pos + 1),
            _ => disassemble_unprefixed(mmu, pos)
        };
        pos += len;
    }
}

fn disassemble_unprefixed(mmu: &Mmu, pos: u16){
    match mmu.read(pos) {
    {% for hex, i in unprefixed -%}
        {{hex}} => println!("{{macros::opcodeString(i=i)}}"{{macros::opcodeParameter(i=i, vec="mem", pos="pos")}}),
    {% endfor -%}
    }
}

fn disassemble_prefixed(mmu: &Mmu, pos: u16) {
    match mmu.read(pos) {
    {% for hex, i in cbprefixed -%}
        {{hex}} => println!("{{macros::opcodeString(i=i)}}"{{macros::opcodeParameter(i=i, vec="mem", pos="pos")}}),
    {% endfor -%}
    }
}

