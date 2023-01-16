pub struct Mbc {
    mem: [u8; 0x8000],
}

impl Mbc {
    pub fn new(mem: [u8; 0x8000]) -> Self {
        Mbc { mem: mem }
    }

    pub fn read(&self, addr: u16) -> u8 {
        return self.mem[addr as usize];
    }

    #[allow(unused_variables)]
    pub fn write(&mut self, addr: u16, value: u8) {}
}
