struct Cpu {
    a: u8,
    f: u8, // 0xznhc0000
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u8,
    pc: u8,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0, f: 0, 
            b: 0, c: 0, 
            d: 0, e: 0, 
            h: 0, l: 0,
            sp: 0, pc: 0x100,
        }
    } 
}