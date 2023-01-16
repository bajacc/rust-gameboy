{%- macro opcodeString(i) -%}
{{i.mnemonic}}
{%- for o in i.operands -%}
    {%- if not loop.first -%}
        ,
    {%- endif -%}
    {% raw %} {%endraw%}
    {%- if not o.immediate -%}
    (
    {%- endif -%}

    {%- if o.name == "d8" -%}
        {:#02x}
    {%- elif o.name == "d16" -%}
        {:#04x}
    {%- elif o.name == "a8" -%}
        {:#02x}
    {%- elif o.name == "a16" -%}
        {:#04x}
    {%- elif o.name == "r8" -%}
        {}
    {%- else -%}
        {{o.name}}
    {%- endif -%}
    {%- if o.sign -%}
    {{o.sign}}
    {%- endif -%}
    {%- if not o.immediate -%}
    )
    {%- endif -%}
{%- endfor -%}
{%- endmacro opcodeString -%}

{%- macro opcodeParameter(i, vec, pos) -%}
{%- for o in i.operands -%}
    {%- if o.name in ["d8", "d16", "a8", "a16", "r8"] -%}
    ,
    {%- raw %} {%endraw-%}
    {%- endif -%}
    
    {%- if o.name in ["d8", "a8"] -%}
        mmu.read({{pos}} + 1)
    {%- elif o.name in ["d16", "a16"] -%}
        mmu.read16({{pos}} + 1)
    {%- elif o.name == "r8" -%}
        mmu.read({{pos}} + 1) as i8
    {%- endif -%}
{%- endfor -%}
{%- endmacro opcodeParameter -%}

{%- macro load(operand) -%}
    {%- set r = operand.name -%}
    {%- if r in ["HL", "BC", "DE"] -%}
        {%- if operand.immediate -%}
            read_{{r | lower}}(cpu)
        {%- else -%}
            mmu.read(read_{{r | lower}}(cpu))
        {%- endif -%}
    {%- elif r == "a8" -%}
    mmu.read(arg + 0xff00);
    {%- elif r == "a16" -%}
        {%- if operand.immediate -%}
            arg
        {%- else -%}
            mmu.read(arg)
        {%- endif -%}
    {%- elif r == "d8" -%}
    arg as u8
    {%- elif r == "d16" -%}
    arg
    {%- elif r == "r8" -%}
    arg as u8 as i8 as i16 as u16
    {%- elif r in ["A","B","C","D","E","H","L","SP","PC"] -%}
    cpu.{{r | lower}}
    {%- else -%}
    ------- {# make the compilation fail #}
    {%- endif -%}
{%- endmacro load -%}

{%- macro write(operand, value) -%}
    {%- set r = operand.name -%}
    {%- if r in ["HL", "BC", "DE"] -%}
        {%- if operand.immediate -%}
        write_{{r | lower}}(cpu, {{value}});
        {%- else -%}
        mmu.write(read_{{r | lower}}(cpu), {{value}});
        {%- endif -%}
    
    {%- elif r == "a8" -%}
    mmu.write(arg + 0xff00, {{value}});
    {%- elif r == "a16" -%}
        {%- if operand.doublewrite -%}
            mmu.write16(arg, {{value}});
        {%- else -%}
            mmu.write(arg, {{value}});
        {%- endif -%}
    {%- elif r in ["A","B","C","D","E","H","L","SP","PC"] -%}
    cpu.{{r | lower}} = {{value}};
    {%- else -%}
    ------- {# make the compilation fail #}
    {%- endif -%}
{%- endmacro write -%}

{%- macro updateFlag(flags) -%}
{% if flags.Z != "-" or flags.N != "-" or flags.H != "-" or flags.C != "-" %}
    cpu.f = 
    {%- for flag, value in flags -%}
        {%- set shift = 4 + loop.index0 -%}
        (
        {%- if value == "C" -%}
        (c as u8) << {{shift}}
        {%- elif value == "Z" -%}
        (z as u8) << {{shift}}
        {%- elif value == "H" -%}
        (h as u8) << {{shift}}
        {%- elif value == "N" -%}
        (n as u8) << {{shift}}
        {%- elif value == "-" -%}
        cpu.f & (1 << {{shift}})
        {%- elif value == "1" -%}
        1 << {{shift}}
        {%- elif value == "0" -%}
        0 << {{shift}}
        {%- endif -%}
        )
        {%- if not loop.last -%}
        |
        {%- endif %}
    {%- endfor -%};

{% endif %}
{%- endmacro updateFlag -%}

{%- macro jmpcond(operand) -%}
{% if operand.name == "NZ" %}
!flag_z(cpu)
{% elif operand.name == "Z" %}
flag_z(cpu)
{% elif operand.name == "NC" %}
!flag_c(cpu)
{% elif operand.name == "C" %}
flag_c(cpu)
{% endif %}
{%- endmacro jmpcond -%}

