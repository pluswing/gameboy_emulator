use std::fs::{self, File};
use std::io::Read;

use crate::mapper::mbc1::MBC1;
use crate::mapper::nombc::NoMBC;
use crate::mapper::Mapper;

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mapper: Mapper,
}

impl Cartridge {
    pub fn new(filename: &str) -> Self {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut rom = vec![0; metadata.len() as usize];
        f.read(&mut rom).expect("buffer overflow");

        // raw[0x0143] // CGB flag
        // 0x0146 — SGB flag

        // 0x0147 — Cartridge type ==> mapper
        // 0148 — ROM size
        // 0149 — RAM size

        // MBC1 の対応が必要。

        println!("{:02X?}", &rom[0x0147..=0x0149]);

        let mapper = match rom[0x0147] {
            0x00 => Mapper::NoMBC(NoMBC::new()),
            0x01 => Mapper::MBC1(MBC1::new()),
            0x02 => Mapper::MBC1(MBC1::new()), // FIXME: + RAM
            0x03 => Mapper::MBC1(MBC1::new()), // FIXME: + RAM + BATTERY
            _ => panic!("unsupported cartridge type."),
        };

        let ram_size = match rom[0x0149] {
            0x00 => 0,
            0x01 => 0,
            0x02 => 8 * 1024,
            0x03 => 32 * 1024,
            0x04 => 128 * 1024,
            0x05 => 64 * 1024,
            _ => panic!("unsupported ram size."),
        };

        let mut ram = vec![0; ram_size as usize];

        Cartridge { rom, ram, mapper }
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        return self.mapper.read_byte(&self.rom, &self.ram, addr);
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        return self
            .mapper
            .write_byte(&mut self.rom, &mut self.ram, addr, value);
    }

    pub fn for_test() -> Self {
        Cartridge {
            rom: vec![0; 0x8000 as usize],
            ram: vec![0; 0x0000 as usize],
            mapper: Mapper::NoMBC(NoMBC::new()),
        }
    }
}
