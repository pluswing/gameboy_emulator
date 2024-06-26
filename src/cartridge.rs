use mapper::MBC1;
use std::fs::{self, File};
use std::io::Read;

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
    mapper: MBC1,
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

        Cartridge {
            raw,
            mapper: match raw[0x0147] {
                // 0x00 => CartridgeType::RomOnly,
                0x01 => MBC1::new(),
                _ => panic!("unsupported cartridge type."),
            },
            rom_size: match raw[0x0148] {
                0x00 => RomSize::Bank2,
                0x01 => RomSize::Bank4,
                _ => panic!("unsupported rom size."),
            },
            ram_size: match raw[0x0149] {
                0x00 => RamSize::No,
                0x01 => RamSize::Unused,
                0x02 => RamSize::Bank1,
                _ => panic!("unsupported ram size."),
            },
        }
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        return self.mapper.read_byte(&self.raw, addr);
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        return self.mapper.write_byte(&self.raw, addr, value);
    }
}
