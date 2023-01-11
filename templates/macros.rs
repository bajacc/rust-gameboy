{%- macro opcodeString(i) -%}
{{i.mnemonic}}
{%- for o in i.operands -%}
    {%- if not loop.first -%}
        ,
    {%- endif -%}
    {% raw %} {%endraw%}
    {%- if o.name == "d8" -%}
        {:#02x}
    {%- elif o.name == "d16" -%}
        {:#04x}
    {%- elif o.name == "a8" -%}
        *{:#02x}
    {%- elif o.name == "a16" -%}
        *{:#04x}
    {%- elif o.name == "r8" -%}
        {}
    {%- else -%}
        {{o.name}}
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