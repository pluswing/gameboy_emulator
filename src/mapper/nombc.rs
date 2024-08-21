pub struct NoMBC {}

impl NoMBC {
    pub fn new() -> Self {
        NoMBC {}
    }

    pub fn read_byte(&mut self, rom: &Vec<u8>, ram: &Vec<u8>, addr: u16) -> u8 {
        rom[addr as usize]
    }

    pub fn write_byte(&mut self, rom: &mut Vec<u8>, ram: &mut Vec<u8>, addr: u16, value: u8) {
        panic!("should not reach!")
    }
}
