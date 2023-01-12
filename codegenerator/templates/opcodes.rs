{% import "macros.rs" as macros %}

use crate::cpu::Cpu;
use crate::mmu::Mmu;

{% for hex, i in cbprefixed %}
#[allow(unused_variables)]
pub fn execute_prefixed{{hex | lower}}(cpu: &mut Cpu, mmu: &mut Mmu) {

    {% set operator = i.operands | last %}

    {% if operator.name == "HL" %}
    let hl: u16 = (cpu.l as u16) | ((cpu.h as u16) << 8);
    let mut v: u8 = mmu.read(hl);
    {% endif %}

    {% set r = macros::register(r = operator.name) %}

    {% if i.mnemonic == "RLC" %}
    let c = {{r}} >> 7;
    {{r}} = ({{r}} << 1) | c;
    let z = {{r}} == 0;

    {% elif i.mnemonic == "RL" %}
    let c = {{r}} >> 7;
    {{r}} = ({{r}} << 1) | ((cpu.f >> 4) & 1);
    let z = {{r}} == 0;

    {% elif i.mnemonic == "RRC" %}
    let c = {{r}} & 1;
    {{r}} = ({{r}} >> 1) | (c << 7);
    let z = {{r}} == 0;

    {% elif i.mnemonic == "RR" %}
    let c = {{r}} & 1;
    {{r}} = ({{r}} >> 1) | ((cpu.f << 3) & 0x80);
    let z = {{r}} == 0;

    {% elif i.mnemonic == "SLA" %}
    let c = {{r}} >> 7;
    {{r}} <<= 1;
    let z = {{r}} == 0;

    {% elif i.mnemonic == "SRA" %}
    let c = {{r}} & 1;
    {{r}} = ({{r}} >> 1) | ({{r}} & 0x80);
    let z = {{r}} == 0;

    {% elif i.mnemonic == "SRL" %}
    let c = {{r}} & 1;
    {{r}} >>= 1;
    let z = {{r}} == 0;

    {% elif i.mnemonic == "BIT" %}
    let z = ({{r}} & (1 << {{i.operands[0].name}})) == 0;

    {% elif i.mnemonic == "SET" %}
    {{r}} |= 1 << {{i.operands[0].name}};

    {% elif i.mnemonic == "RES" %}
    {{r}} &= !(1 << {{i.operands[0].name}});

    {% elif i.mnemonic == "SWAP" %}
    {{r}} &= ({{r}} >> 4) | ({{r}} << 4);
    let z = {{r}} == 0;

    {% endif %}

    {% if operator.name == "HL" %}
    mmu.write(hl, v);
    {% endif %}

    {# create new flag register #}
    {% if i.flags["Z"] != "-" or i.flags["N"] != "-" or i.flags["H"] != "-" or i.flags["C"] != "-" %}
    let mut f = 0;

    {%- for flag, value in i.flags %}
        f <<= 1;
        {%- if value == "C" %}
        f |= c;
        {%- elif value == "Z" %}
        f |= z as u8;
        {%- elif value == "-" %}
        f |= (cpu.f >> {{7 - loop.index0}}) & 1;
        {%- elif value == "1" %}
        f |= 1;
        {%- endif %}
    {%- endfor %}
    cpu.f = f << 4;

    {% endif %}
}
{%- endfor -%}
