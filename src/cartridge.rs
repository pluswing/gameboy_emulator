use std::fs::{self, File};

pub struct Cartridge {}

impl Cartridge {
    pub fn test(filename: &str) {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut raw = vec![0; metadata.len() as usize];
        f.read(&mut raw).expect("buffer overflow");

        println!(raw[0x0104..=0x0133]) // 任天堂ロゴ
    }
}

/*
pub fn new(raw: &Vec<u8>) -> Result<Rom, String> {
       if &raw[0..4] != NES_TAG {
           return Err("File is not in iNES file format".to_string());
       }

*/
