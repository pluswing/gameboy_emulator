use nombc::NoMBC;

pub mod mbc1;
pub mod nombc;

pub enum Mapper {
    NoMBC(nombc::NoMBC),
    MBC1(mbc1::MBC1),
}

impl Mapper {
    pub fn read_byte(&mut self, raw: &Vec<u8>, addr: u16) -> u8 {
        match self {
            Mapper::NoMBC(mapper) => mapper.read_byte(raw, addr),
            Mapper::MBC1(mapper) => mapper.read_byte(raw, addr),
        }
    }

    pub fn write_byte(&mut self, raw: &mut Vec<u8>, addr: u16, value: u8) {
        match self {
            Mapper::NoMBC(mapper) => mapper.write_byte(raw, addr, value),
            Mapper::MBC1(mapper) => mapper.write_byte(raw, addr, value),
        }
    }
}
