use std::fs::{self, File};
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

use crate::mapper::mbc1::MBC1;
use crate::mapper::mbc3::MBC3;
use crate::mapper::mbc5::MBC5;
use crate::mapper::nombc::NoMBC;
use crate::mapper::Mapper;

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mapper: Mapper,
    pub palette: [[[u8; 3]; 4]; 3], // [BGP, OBJ0, OBJ1]

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
            0x02 => Mapper::MBC1(MBC1::new()), // + RAM
            0x03 => Mapper::MBC1(MBC1::new()), // + RAM + BATTERY
            0x19 => Mapper::MBC5(MBC5::new()),
            0x1B => Mapper::MBC5(MBC5::new()), // + RAM + BATTERY
            0x10 => Mapper::MBC3(MBC3::new()), // TIMER + RAM + BATTERY
            _ => panic!("unsupported cartridge type."),
        };
        // 19 => bm MBC5 0K
        // 10 => gold MBC3+TIMER+RAM+BATTERY
        // 1B => yugi3 MBC5+RAM+BATTERY OK
        // 1B => yugi4 MBC5+RAM+BATTERY OK

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

        // 互換パレット
        let mut made_nintendo = false;
        let licensee_code = rom[0x014B];
        if licensee_code == 0x33 {
            let new_licensee_code1 = rom[0x0144];
            let new_licensee_code2 = rom[0x0145];
            /* equal '01' */
            if new_licensee_code1 == 0x30 && new_licensee_code2 == 0x31 {
                made_nintendo = true;
            }
        } else {
            if licensee_code == 0x01 {
                made_nintendo = true;
            }
        }

        if !made_nintendo {
            // TODO パレットID01を使用する
        }
        // TODO ゲームタイトルの16 バイトすべての合計を計算
        let mut sum: u8 = 0;
        for i in 0..16 {
            sum = sum.wrapping_add(rom[0x0134 + i as usize])
        }

        let checksum_table: [u8; 65] = [
            0x00, 0x88, 0x16, 0x36, 0xD1, 0xDB, 0xF2, 0x3C, 0x8C, 0x92, 0x3D, 0x5C, 0x58, 0xC9,
            0x3E, 0x70, 0x1D, 0x59, 0x69, 0x19, 0x35, 0xA8, 0x14, 0xAA, 0x75, 0x95, 0x99, 0x34,
            0x6F, 0x15, 0xFF, 0x97, 0x4B, 0x90, 0x17, 0x10, 0x39, 0xF7, 0xF6, 0xA2, 0x49, 0x4E,
            0x43, 0x68, 0xE0, 0x8B, 0xF0, 0xCE, 0x0C, 0x29, 0xE8, 0xB7, 0x86, 0x9A, 0x52, 0x01,
            0x9D, 0x71, 0x9C, 0xBD, 0x5D, 0x6D, 0x67, 0x3F, 0x6B,
        ];
        let mut match_index = checksum_table.len();
        for (i, v) in checksum_table.iter().enumerate() {
            if sum == *v {
                match_index = i;
                break;
            }
        }

        if match_index != checksum_table.len() {
            if match_index <= 64 {
                // pallette_id = match_index
            } else {
                // FIXME
                // タイトルの 4 番目の文字に基づいてさらに修正
                let v = rom[0x0134 + 3];
                // "BEFAARBEKEK R-"[v - 0x41];
                //
                // db "BEFAARBEKEK R-"
                // .row
                //   db "URAR INAILICE "
                //   db "R"
            }
        }

        // 0x1C 0x01 0xAA => pokemon midori

        let palette = [
            [
                [0xFF, 0xFF, 0xFF],
                [0x7B, 0xFF, 0x31],
                [0x00, 0x63, 0xC5],
                [0x00, 0x00, 0x00],
            ], // BGP
            [
                [0xFF, 0xFF, 0xFF],
                [0xFF, 0x84, 0x84],
                [0x94, 0x3A, 0x3A],
                [0x00, 0x00, 0x00],
            ], // OBJ0
            [
                [0xFF, 0xFF, 0xFF],
                [0x7B, 0xFF, 0x31],
                [0x00, 0x63, 0xC5],
                [0x00, 0x00, 0x00],
            ], // OBJ1
        ];

        Cartridge {
            rom,
            ram,
            mapper,
            palette,
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
            palette: [
                [
                    [0xFF, 0xFF, 0xFF],
                    [0x7B, 0xFF, 0x31],
                    [0x00, 0x63, 0xC5],
                    [0x00, 0x00, 0x00],
                ], // BGP
                [
                    [0xFF, 0xFF, 0xFF],
                    [0xFF, 0x84, 0x84],
                    [0x94, 0x3A, 0x3A],
                    [0x00, 0x00, 0x00],
                ], // OBJ0
                [
                    [0xFF, 0xFF, 0xFF],
                    [0x7B, 0xFF, 0x31],
                    [0x00, 0x63, 0xC5],
                    [0x00, 0x00, 0x00],
                ], // OBJ1
            ],
            ram_file_path: "_.save".to_string(),
        }
    }
}
