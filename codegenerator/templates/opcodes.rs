{% import "macros.rs" as macros %}

use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::cpu::alu;

fn push(cpu: &mut Cpu, mmu: &mut Mmu, v: u16) {
    cpu.sp -= 2;
    mmu.write16(cpu.sp, v);
}

fn pop(cpu: &mut Cpu, mmu: &mut Mmu) -> u16{
    let r = mmu.read16(cpu.sp);
    cpu.sp += 2;
    return r;
}

{% for r16 in ["bc", "de", "hl", "af"] %}
{%- set arr = r16 | split(pat="") -%}
pub fn read_{{r16}}(cpu: &mut Cpu) -> u16 {
    return (cpu.{{arr[2]}} as u16) | ((cpu.{{arr[1]}} as u16) << 8);
}

pub fn write_{{r16}}(cpu: &mut Cpu, v: u16) {
    cpu.{{arr[1]}} = (v >> 8) as u8;
    cpu.{{arr[2]}} = (v & 0xff) as u8;
}
{% endfor %}

{% for f in ["z", "n", "h", "c"] %}
pub fn flag_{{f}}(cpu: &mut Cpu) -> bool {
    return (cpu.f & (1 << {{7 - loop.index0}})) != 0;
}
{% endfor %}


{% for hex, i in cbprefixed %}
#[allow(unused_variables)]
pub fn execute_prefixed{{hex | lower}}(cpu: &mut Cpu, mmu: &mut Mmu) {

    {% set operand = i.operands | last %}

    let mut v = {{macros::load(operand=operand)}};

    {%- if i.mnemonic == "RLC" -%}
    let c = v >> 7;
    v = (v << 1) | c;

    {%- elif i.mnemonic == "RL" -%}
    let c = v >> 7;
    v = (v << 1) | ((cpu.f >> 4) & 1);

    {%- elif i.mnemonic == "RRC" -%}
    let c = v & 1;
    v = (v >> 1) | (c << 7);

    {%- elif i.mnemonic == "RR" -%}
    let c = v & 1;
    v = (v >> 1) | ((cpu.f << 3) & 0x80);

    {%- elif i.mnemonic == "SLA" -%}
    let c = v >> 7;
    v <<= 1;

    {%- elif i.mnemonic == "SRA" -%}
    let c = v & 1;
    v = (v >> 1) | (v & 0x80);

    {%- elif i.mnemonic == "SRL" -%}
    let c = v & 1;
    v >>= 1;

    {%- elif i.mnemonic == "SET" -%}
    v |= 1 << {{i.operands[0].name}};

    {%- elif i.mnemonic == "RES" -%}
    v &= !(1 << {{i.operands[0].name}});

    {%- elif i.mnemonic == "SWAP" -%}
    v = (v >> 4) | (v << 4);

    {%- endif -%}

    {{macros::write(operand=operand, value="v")}}

    {%- if i.mnemonic == "BIT" -%}
    let z = (v & (1 << {{i.operands[0].name}})) == 0;
    {%- elif i.flags["Z"] == "Z" -%}
    let z = v == 0;
    {%- endif -%}

    {# update flag register #}
    {{macros::updateFlag(flags=i.flags)}}
}
{%- endfor -%}

{% for hex, i in unprefixed %}
#[allow(unused_variables)]
pub fn execute_unprefixed{{hex | lower}}(cpu: &mut Cpu, mmu: &mut Mmu, arg: u16) {
    {%- if i.mnemonic == "LD" -%}
        {%- set opleft = i.operands[0] -%}
        {%- set opright = i.operands[1] -%}
        {%- set value = macros::load(operand=opright) -%}
        {%- if hex == "0xF8" -%} {# LD HL, SP + r8 #}
            let (r, h, c) = alu::add16l({{value}}, {{macros::load(operand=i.operands[2])}});
            {%- set value = "r" -%}
        {%- endif -%}
        {{macros::write(operand=opleft, value=value)}}
        {%- if opleft.add -%}
            let hl = read_hl(cpu) {{opleft.add}};
            write_hl(cpu, hl);
        {%- elif opright.add -%}
            let hl = read_hl(cpu) {{opright.add}};
            write_hl(cpu, hl);
        {%- endif -%}
    {%- elif i.mnemonic in ["ADD", "ADC"] -%}
        {%- set opleft = i.operands[0] -%}
        {%- set opright = i.operands[1] -%}
        let a = {{macros::load(operand=opleft)}};
        let b = {{macros::load(operand=opright)}};
        {%- if opleft.name == "HL" -%}
            let (r, h, c) = alu::add16h(a, b);
        {%- elif opleft.name == "SP" -%}
            let (r, h, c) = alu::add16l(a, b);
        {%- elif i.mnemonic == "ADC" -%}
            let (r, h, c) = alu::add(a, b, flag_c(cpu));
        {%- elif i.mnemonic == "ADD" -%}
            let (r, h, c) = alu::add(a, b, false);
        {%- endif -%}
        {{macros::write(operand=opleft, value="r")}}
    {%- elif i.mnemonic in ["SUB", "SBC"] -%}
        {%- set opright = i.operands | last -%}
        {% set b = macros::load(operand=opright) %};
        {%- if i.mnemonic == "SBC" -%}
            let (r, h, c) = alu::sub(cpu.a, {{b}}, flag_c(cpu));
        {%- elif i.mnemonic == "SUB" -%}
            let (r, h, c) = alu::sub(cpu.a, {{b}}, false);
        {%- endif -%}
        cpu.a = r;
    {%- elif i.mnemonic in ["INC", "DEC"] -%}
        {%- set operand = i.operands[0] -%}
        let mut r = {{macros::load(operand=i.operands[0])}};
        {%- if i.mnemonic == "SUB" -%}
            let h = 0;
            r -= 1;
        {%- else -%}
            let h = r & 0xf == 0xf;
            r += 1;
        {%- endif -%}
        {{macros::write(operand=i.operands[0], value="r")}}
    {%- elif i.mnemonic == "AND" -%}
        cpu.a &= {{macros::load(operand=i.operands[0])}};
    {%- elif i.mnemonic == "OR" -%}
        cpu.a |= {{macros::load(operand=i.operands[0])}};
    {%- elif i.mnemonic == "XOR" -%}
        cpu.a ^= {{macros::load(operand=i.operands[0])}};
    {%- elif i.mnemonic == "CPL" -%}
        cpu.a = !cpu.a;
    {%- elif i.mnemonic == "CCF" -%}
        let c = !flag_c(cpu);
    {%- elif i.mnemonic == "NOP" -%}
    {%- elif i.mnemonic == "POP" -%}
        {%- set name = i.operands[0].name | lower -%}
        cpu.sp += 2;
        write_{{name}}(cpu, mmu.read16(cpu.sp));
    {%- elif i.mnemonic == "PUSH" -%}
        {%- set name = i.operands[0].name | lower -%}
        mmu.write16(cpu.sp, read_{{name}}(cpu));
        cpu.sp -= 2;

    {%- elif i.mnemonic == "JP" -%}
        {%- set operand = i.operands[0] -%}
        {% if operand.name in ["NZ", "Z", "NC", "C"] %}
            if {{macros::jmpcond(operand=operand)}} {
                cpu.pc = {{macros::load(operand=i.operands[1])}};
            }
        {% else %}
            cpu.pc = {{macros::load(operand=operand)}};
        {% endif %}

    {%- elif i.mnemonic == "JR" -%}
        {%- set operand = i.operands[0] -%}
        {% if operand.name in ["NZ", "Z", "NC", "C"] %}
            if {{macros::jmpcond(operand=operand)}} {
                cpu.pc += {{macros::load(operand=i.operands[1])}};
            }
        {% else %}
            cpu.pc += {{macros::load(operand=operand)}};
        {% endif %}
    
    {%- elif i.mnemonic == "CALL" -%}
        {%- set operand = i.operands[0] -%}
        {% if operand.name in ["NZ", "Z", "NC", "C"] %}
            if {{macros::jmpcond(operand=operand)}} {
                push(cpu, mmu, cpu.pc);
                cpu.pc = {{macros::load(operand=i.operands[1])}};
            }
        {% else %}
            push(cpu, mmu, cpu.pc);
            cpu.pc = {{macros::load(operand=operand)}};
        {% endif %}
    
    {%- elif i.mnemonic == "RET" -%}
        {% if i.operands %}
            if {{macros::jmpcond(operand=i.operands[0])}} {
                cpu.pc = pop(cpu, mmu);
            }
        {% else %}
            cpu.pc = pop(cpu, mmu);
        {% endif %}

    {%- elif i.mnemonic == "RST" -%}
        {%- set operand = i.operands[0] -%}
        push(cpu, mmu, cpu.pc);
        cpu.pc = 0x{{operand.name}};
        
    {% else %}
        panic!("invalid opcode {{hex}}");
    {% endif %}

    {% if i.flags["Z"] == "Z" %}
    let z = 0; // todo
    {% endif %}

    {# update flag register #}
    {#{{macros::updateFlag(flags=i.flags)}}#}
}
{% endfor %}

pub fn execute_prefixed(cpu: &mut Cpu, mmu: &mut Mmu, opcode: u8) {
    match opcode {
        {% for hex, i in cbprefixed -%}
            {{hex}} => execute_prefixed{{hex | lower}}(cpu, mmu),
        {% endfor -%}
    }
}

pub fn execute_unprefixed(cpu: &mut Cpu, mmu: &mut Mmu, opcode: u8, arg: u16) {
    match opcode {
        {% for hex, i in unprefixed -%}
            {{hex}} => execute_unprefixed{{hex | lower}}(cpu, mmu, arg),
        {% endfor -%}
    }
}

