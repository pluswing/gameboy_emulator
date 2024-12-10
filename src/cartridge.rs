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
    pub palette: [[u16; 4]; 3], // [BGP, OBJ0, OBJ1]

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

        let checksum_table: [u8; 65 + 14] = [
            0x00, 0x88, 0x16, 0x36, 0xD1, 0xDB, 0xF2, 0x3C, 0x8C, 0x92, 0x3D, 0x5C, 0x58, 0xC9,
            0x3E, 0x70, 0x1D, 0x59, 0x69, 0x19, 0x35, 0xA8, 0x14, 0xAA, 0x75, 0x95, 0x99, 0x34,
            0x6F, 0x15, 0xFF, 0x97, 0x4B, 0x90, 0x17, 0x10, 0x39, 0xF7, 0xF6, 0xA2, 0x49, 0x4E,
            0x43, 0x68, 0xE0, 0x8B, 0xF0, 0xCE, 0x0C, 0x29, 0xE8, 0xB7, 0x86, 0x9A, 0x52, 0x01,
            0x9D, 0x71, 0x9C, 0xBD, 0x5D, 0x6D, 0x67, 0x3F, 0x6B, 0xB3, 0x46, 0x28, 0xA5, 0xC6,
            0xD3, 0x27, 0x61, 0x18, 0x66, 0x6A, 0xBF, 0x0D, 0xF4,
        ];
        let palette_index_table: [u8; 94] = [
            0, 4, 5, 35, 34, 3, 31, 15, 10, 5, 19, 36, 7, 37, 30, 44, 21, 32, 31, 20, 5, 33, 13,
            14, 5, 29, 5, 18, 9, 3, 2, 26, 25, 25, 41, 42, 26, 45, 42, 45, 36, 38, 26, 42, 30, 41,
            34, 34, 5, 42, 6, 5, 33, 25, 42, 42, 40, 2, 16, 25, 42, 42, 5, 0, 39, 36, 22, 25, 6,
            32, 12, 36, 11, 39, 18, 39, 24, 31, 50, 17, 46, 6, 27, 0, 47, 41, 41, 0, 0, 19, 34, 23,
            18, 29,
        ];
        let palette_comb_table: [[u8; 3]; 51] = [
            [4, 4, 29],
            [18, 18, 18],
            [20, 20, 20],
            [24, 24, 24],
            [9, 9, 9],
            [0, 0, 0],
            [27, 27, 27],
            [5, 5, 5],
            [12, 12, 12],
            [26, 26, 26],
            [16, 8, 8],
            [4, 28, 28],
            [4, 2, 2],
            [3, 4, 4],
            [4, 29, 29],
            [28, 4, 28],
            [2, 17, 2],
            [16, 16, 8],
            [4, 4, 7],
            [4, 4, 18],
            [4, 4, 20],
            [19, 19, 9],
            [4 * 4 - 1, 4 * 4 - 1, 11 * 4],
            [17, 17, 2],
            [4, 4, 2],
            [4, 4, 3],
            [28, 28, 0],
            [3, 3, 0],
            [0, 0, 1],
            [18, 22, 18],
            [20, 22, 20],
            [24, 22, 24],
            [16, 22, 8],
            [17, 4, 13],
            [28 * 4 - 1, 0 * 4, 14 * 4],
            [28 * 4 - 1, 4 * 4, 15 * 4],
            [19, 22, 9],
            [16, 28, 10],
            [4, 23, 28],
            [17, 22, 2],
            [4, 0, 2],
            [4, 28, 3],
            [28, 3, 0],
            [3, 28, 4],
            [21, 28, 4],
            [3, 28, 0],
            [25, 3, 28],
            [0, 28, 8],
            [4, 3, 28],
            [28, 3, 6],
            [4, 28, 29],
        ];
        let palette_table: [[u16; 4]; 30] = [
            [0x7FFF, 0x32BF, 0x00D0, 0x0000],
            [0x639F, 0x4279, 0x15B0, 0x04CB],
            [0x7FFF, 0x6E31, 0x454A, 0x0000],
            [0x7FFF, 0x1BEF, 0x0200, 0x0000],
            [0x7FFF, 0x421F, 0x1CF2, 0x0000],
            [0x7FFF, 0x5294, 0x294A, 0x0000],
            [0x7FFF, 0x03FF, 0x012F, 0x0000],
            [0x7FFF, 0x03EF, 0x01D6, 0x0000],
            [0x7FFF, 0x42B5, 0x3DC8, 0x0000],
            [0x7E74, 0x03FF, 0x0180, 0x0000],
            [0x67FF, 0x77AC, 0x1A13, 0x2D6B],
            [0x7ED6, 0x4BFF, 0x2175, 0x0000],
            [0x53FF, 0x4A5F, 0x7E52, 0x0000],
            [0x4FFF, 0x7ED2, 0x3A4C, 0x1CE0],
            [0x03ED, 0x7FFF, 0x255F, 0x0000],
            [0x036A, 0x021F, 0x03FF, 0x7FFF],
            [0x7FFF, 0x01DF, 0x0112, 0x0000],
            [0x231F, 0x035F, 0x00F2, 0x0009],
            [0x7FFF, 0x03EA, 0x011F, 0x0000],
            [0x299F, 0x001A, 0x000C, 0x0000],
            [0x7FFF, 0x027F, 0x001F, 0x0000],
            [0x7FFF, 0x03E0, 0x0206, 0x0120],
            [0x7FFF, 0x7EEB, 0x001F, 0x7C00],
            [0x7FFF, 0x3FFF, 0x7E00, 0x001F],
            [0x7FFF, 0x03FF, 0x001F, 0x0000],
            [0x03FF, 0x001F, 0x000C, 0x0000],
            [0x7FFF, 0x033F, 0x0193, 0x0000],
            [0x0000, 0x4200, 0x037F, 0x7FFF],
            [0x7FFF, 0x7E8C, 0x7C00, 0x0000],
            [0x7FFF, 0x1BEF, 0x6180, 0x0000],
        ];

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

        let cgb_game = (rom[0x0143] & 0x80) != 0;

        let match_index = if !made_nintendo || cgb_game {
            1
        } else {
            // ゲームタイトルの16 バイトすべての合計を計算
            let mut sum: u8 = 0;
            for i in 0..16 {
                sum = sum.wrapping_add(rom[0x0134 + i as usize])
            }

            let mut match_index = 0xFF;
            for (i, v) in checksum_table.iter().enumerate() {
                if sum == *v {
                    match_index = i;
                    break;
                }
            }
            if match_index == 0xFF {
                1
            } else {
                match_index
            }
        };

        let match_index = if match_index <= 64 {
            match_index
        } else {
            println!("TITLE 4: {:02X}", match_index);
            // FIXME
            // タイトルの 4 番目の文字に基づいてさらに修正
            // let v = rom[0x0134 + 3];
            // "BEFAARBEKEK R-"[v - 0x41];
            //
            // db "BEFAARBEKEK R-"
            // .row
            //   db "URAR INAILICE "
            //   db "R"
            match_index
        };
        let palette_index = palette_index_table[match_index];
        let palette_comb = palette_comb_table[palette_index as usize];

        let obj0 = palette_table[palette_comb[0] as usize];
        let obj1 = palette_table[palette_comb[1] as usize];
        let bgp = palette_table[palette_comb[2] as usize];

        Cartridge {
            rom,
            ram,
            mapper,
            palette: [bgp, obj0, obj1],
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
            palette: [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
            ram_file_path: "_.save".to_string(),
        }
    }
}
