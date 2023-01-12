pub struct Mmu {}

impl Mmu {
    pub fn read(&self, addr: u16) -> u8 {
        println!("{:?}", addr);
        return 0;
    }

    pub fn write(&self, addr: u16, value: u8) {
        println!("{:?}, {:?}", addr, value);
    }
}
