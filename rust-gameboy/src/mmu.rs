pub struct Mmu {}

impl Mmu {
    pub fn read(&self, addr: u16) -> u8 {
        println!("{:?}", addr);
        return 0;
    }

    pub fn write(&self, addr: u16, value: u8) {
        println!("{:?}, {:?}", addr, value);
    }

    pub fn read16(&self, addr: u16) -> u16 {
        return (self.read(addr) as u16) | ((self.read(addr + 1) as u16) << 8);
    }

    pub fn write16(&self, addr: u16, value: u16) {
        self.write(addr, value as u8);
        self.write(addr + 1, (value >> 8) as u8);
    }
}
