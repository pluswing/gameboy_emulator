use std::fs::{self, File};
use std::io::Read;

use crate::mapper::mbc1::MBC1;
use crate::mapper::nombc::NoMBC;
use crate::mapper::Mapper;

enum RomSize {
    Bank2, // $00	32 KiB	2 (no banking)
    Bank4, // $01	64 KiB	4
           // ...
}

enum RamSize {
    No,     // $00	0	No RAM
    Unused, // $01	–	Unused 12
    Bank1,  // $02	8 KiB	1 bank
            // ...
}

pub struct Cartridge {
    raw: Vec<u8>,
    mapper: Mapper,
    rom_size: RomSize,
    ram_size: RamSize,
}

impl Cartridge {
    pub fn new(filename: &str) -> Self {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut raw = vec![0; metadata.len() as usize];
        f.read(&mut raw).expect("buffer overflow");

        // raw[0x0143] // CGB flag
        // 0x0146 — SGB flag

        // 0x0147 — Cartridge type ==> mapper
        // 0148 — ROM size
        // 0149 — RAM size

        // MBC1 の対応が必要。

        println!("{:02X?}", &raw[0x0147..=0x0149]);

        let mapper = match raw[0x0147] {
            0x00 => Mapper::NoMBC(NoMBC::new()),
            0x01 => Mapper::MBC1(MBC1::new()),
            _ => panic!("unsupported cartridge type."),
        };

        let rom_size = match raw[0x0148] {
            0x00 => RomSize::Bank2,
            0x01 => RomSize::Bank4,
            _ => panic!("unsupported rom size."),
        };

        let ram_size = match raw[0x0149] {
            0x00 => RamSize::No,
            0x01 => RamSize::Unused,
            0x02 => RamSize::Bank1,
            _ => panic!("unsupported ram size."),
        };

        Cartridge {
            raw,
            mapper,
            rom_size,
            ram_size,
        }
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        return self.mapper.read_byte(&self.raw, addr);
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        return self.mapper.write_byte(&mut self.raw, addr, value);
    }

    pub fn for_test() -> Self {
        Cartridge {
            raw: vec![0; 0x8000 as usize],
            mapper: Mapper::NoMBC(NoMBC::new()),
            rom_size: RomSize::Bank2,
            ram_size: RamSize::No,
        }
    }
}
