
pub const OPCODE_BYTES_LEN: [usize; 256] = [
{% for hex, i in unprefixed -%}
    {%- if hex != "0xCB" -%} {{i.bytes}},
    {%- else -%} 2,
    {%- endif -%}
{% endfor %}
];

pub const OPCODE_DURATION: [usize; 256] = [
{% for hex, i in unprefixed -%}
    {{i.cycles | last}},
{%- endfor %}
];