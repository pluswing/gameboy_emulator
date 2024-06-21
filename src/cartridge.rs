use std::fs::{self, File};
use std::io::Read;

pub struct Cartridge {}

impl Cartridge {
    pub fn test(filename: &str) {
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
    }
}

/*
pub fn new(raw: &Vec<u8>) -> Result<Rom, String> {
       if &raw[0..4] != NES_TAG {
           return Err("File is not in iNES file format".to_string());
       }

*/
