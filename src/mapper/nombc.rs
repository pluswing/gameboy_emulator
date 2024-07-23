pub struct NoMBC {}

impl NoMBC {
    pub fn new() -> Self {
        NoMBC {}
    }

    pub fn read_byte(&mut self, raw: &Vec<u8>, addr: u16) -> u8 {
        raw[addr as usize]
    }

    pub fn write_byte(&mut self, raw: &mut Vec<u8>, addr: u16, value: u8) {
        panic!("should not reach!")
    }
}
