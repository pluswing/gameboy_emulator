use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use crate::mapper::mbc1::MBC1;
use crate::mapper::nombc::NoMBC;
use crate::mapper::Mapper;

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mapper: Mapper,

    ram_file_path: String,
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

        // let mut ram =

        let mut path = PathBuf::from(filename);
        path.set_extension("save");
        let ram_file_path = path.to_str().unwrap();

        let ram = if path.is_file() {
            let mut f = File::open(&ram_file_path).expect("no save file found");
            let metadata = fs::metadata(&ram_file_path).expect("unable to read metadata");
            if metadata.len() != ram_size {
                panic!("save file size is not match.");
            }

            let mut ram = vec![0; metadata.len() as usize];
            f.read(&mut ram).expect("buffer overflow");
            ram
        } else {
            vec![0; ram_size as usize]
        };

        Cartridge {
            rom,
            ram,
            mapper,
            ram_file_path: ram_file_path.to_string(),
        }
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        return self.mapper.read_byte(&self.rom, &self.ram, addr);
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        return self
            .mapper
            .write_byte(&mut self.rom, &mut self.ram, addr, value);
    }

    pub fn save_ram(&mut self) {
        let mut file = File::create(self.ram_file_path.as_str()).unwrap();
        file.write_all(&self.ram).unwrap();
        file.flush().unwrap();
    }

    pub fn for_test() -> Self {
        Cartridge {
            rom: vec![0; 0x8000 as usize],
            ram: vec![0; 0x0000 as usize],
            mapper: Mapper::NoMBC(NoMBC::new()),
            ram_file_path: "_.save".to_string(),
        }
    }
}
