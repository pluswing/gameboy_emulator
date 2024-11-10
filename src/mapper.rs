use nombc::NoMBC;

pub mod mbc1;
pub mod mbc5;
pub mod nombc;

pub enum Mapper {
    NoMBC(nombc::NoMBC),
    MBC1(mbc1::MBC1),
    MBC5(mbc5::MBC5),
}

impl Mapper {
    pub fn read_byte(&mut self, rom: &Vec<u8>, ram: &Vec<u8>, addr: u16) -> u8 {
        match self {
            Mapper::NoMBC(mapper) => mapper.read_byte(rom, ram, addr),
            Mapper::MBC1(mapper) => mapper.read_byte(rom, ram, addr),
            Mapper::MBC5(mapper) => mapper.read_byte(rom, ram, addr),
        }
    }

    pub fn write_byte(&mut self, rom: &mut Vec<u8>, ram: &mut Vec<u8>, addr: u16, value: u8) {
        match self {
            Mapper::NoMBC(mapper) => mapper.write_byte(rom, ram, addr, value),
            Mapper::MBC1(mapper) => mapper.write_byte(rom, ram, addr, value),
            Mapper::MBC5(mapper) => mapper.write_byte(rom, ram, addr, value),
        }
    }
}
