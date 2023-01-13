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
        {{vec}}[{{pos}} + 1]
    {%- elif o.name in ["d16", "a16"] -%}
        ({{vec}}[{{pos}} + 1] as u16) | (({{vec}}[{{pos}} + 2] as u16) << 8)
    {%- elif o.name == "r8" -%}
        {{vec}}[{{pos}} + 1] as i8
    {%- endif -%}
{%- endfor -%}
{%- endmacro opcodeParameter -%}

{%- macro load(operand) -%}
    {%- set r = operand.name -%}
    {%- if r in ["HL", "BC", "DE"] -%}
        {%- if operand.immediate -%}
            read_{{r | lower}}(cpu);
        {%- else -%}
            mmu.read(read_{{r | lower}}(cpu));
            {%- if operand.add -%}
            let v = read_{{r | lower}}(cpu);
            write_{{r | lower}}(cpu, v {{operand.add}});
            {%- endif-%}
        {%- endif -%}
    {%- elif r == "a8" -%}
    mmu.read(arg + 0xff00);
    {%- elif r == "a16" -%}
    mmu.read(arg);
    {%- elif r == "d8" -%}
    arg as u8
    {%- elif r == "d16" -%}
    arg
    {%- elif r == "r8" -%}
    arg as u8 as i8
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
        {%- if operand.add -%}
        let v = read_{{r | lower}}(cpu);
        write_{{r | lower}}(cpu, v {{operand.add}});
        {%- endif-%}
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
        c
        {%- elif value == "Z" -%}
        (z as u8)
        {%- elif value == "-" -%}
        ((cpu.f >> {{7 - loop.index0}}) & 1)
        {%- elif value == "1" -%}
        1
        {%- elif value == "0" -%}
        0
        {%- endif -%}
        
        << {{shift}}
        )
        {%- if not loop.last -%}
        |
        {%- endif %}
    {%- endfor -%};

{% endif %}
{%- endmacro updateFlag -%}
