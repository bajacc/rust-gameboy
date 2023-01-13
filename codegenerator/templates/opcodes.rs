{% import "macros.rs" as macros %}

use crate::cpu::Cpu;
use crate::mmu::Mmu;

{% for r16 in ["bc", "de", "hl"] %}
{%- set arr = r16 | split(pat="") -%}
pub fn read_{{r16}}(cpu: &mut Cpu) -> u16 {
    return (cpu.{{arr[2]}} as u16) | ((cpu.{{arr[1]}} as u16) << 8);
}

pub fn write_{{r16}}(cpu: &mut Cpu, v: u16) {
    cpu.{{arr[1]}} = (v >> 8) as u8;
    cpu.{{arr[2]}} = (v & 0xff) as u8;
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
        {%- if opright.addnext -%}
            {%- set value = value ~ "+ (arg as u8 as i8 as i16 as u16)" -%}
        {%- endif -%}
        {{macros::write(operand=opleft, value=value)}}
    {% else %}
    panic!("not implemented");
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

