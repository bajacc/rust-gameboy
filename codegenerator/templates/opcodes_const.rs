
pub const OPCODE_BYTES_LEN: [usize; 256] = [
{% for hex, i in unprefixed -%}
    {%- if hex != "0xCB" -%} {{i.bytes}},
    {%- else -%} 2,
    {%- endif -%}
{% endfor %}
];

pub const DURATION_UNPREFIXED: [usize; 256] = [
{% for hex, i in unprefixed -%}
    {{i.cycles | last / 4}},
{%- endfor %}
];

pub const DURATION_PREFIXED: [usize; 256] = [
{% for hex, i in cbprefixed -%}
    {{i.cycles | last / 4}},
{%- endfor %}
];