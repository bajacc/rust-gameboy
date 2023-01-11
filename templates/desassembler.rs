{% import "macros.rs" as macros %}

pub fn disassemble(mem: &[u8]) {
    let mut pos = 0;
    while pos < mem.len() {
        let len = OPCODE_BYTES_LEN[mem[pos] as usize];
        for i in pos..pos+len {
            print!("{:02x} ", mem[i]);
            if i >= mem.len() {
                println!("truncated instruction");
                return;
            }
        }
        match mem[pos] {
            0xCB => disassemble_prefixed(mem, pos + 1),
            _ => disassemble_unprefixed(mem, pos)
        };
        pos += len;
    }
}

const OPCODE_BYTES_LEN: [usize; 256] = [
{% for hex, i in unprefixed -%}
    {%- if hex != "0xCB" -%} {{i.bytes}},
    {%- else -%} 2,
    {%- endif -%}
{% endfor %}
];

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

