{% import "macros.rs" as macros %}

mod opcodes_const;

use opcodes_const::OPCODE_BYTES_LEN;

pub fn disassemble(mem: &[u8], number: usize) {
    let pos = 0;
    for _ in 0..number {
        let len = OPCODE_BYTES_LEN[mem[pos] as usize];
        if pos + len > mem.len() {
            return;
        }
        for i in pos..pos+len {
            print!("{:02x} ", mem[i]);
        }
        match mem[pos] {
            0xCB => disassemble_prefixed(mem, pos + 1),
            _ => disassemble_unprefixed(mem, pos)
        };
        pos += len;
    }
}

fn disassemble_unprefixed(mem: &[u8], pos: usize){
    match mem[pos] {
    {% for hex, i in unprefixed -%}
        {{hex}} => println!("{{macros::opcodeString(i=i)}}"{{macros::opcodeParameter(i=i, vec="mem", pos="pos")}}),
    {% endfor -%}
    }
}

fn disassemble_prefixed(mem: &[u8], pos: usize) {
    match mem[pos] {
    {% for hex, i in cbprefixed -%}
        {{hex}} => println!("{{macros::opcodeString(i=i)}}"{{macros::opcodeParameter(i=i, vec="mem", pos="pos")}}),
    {% endfor -%}
    }
}

