pub struct MBC1 {
    bank: u8,
    ram_enabled: u8,
    extra: u8, // u2
    mode: u8,  // u1
}

impl MBC1 {
    pub fn new() -> Self {
        MBC1 {
            bank: 1,
            ram_enabled: 0,
            extra: 0,
            mode: 1,
        }
    }

    pub fn read_byte(&mut self, raw: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            // bank0
            0x0000..=0x3FFF => raw[addr as usize],
            // bank1
            0x4000..=0x7FFF => {
                let mut bank = ((self.extra & 0x02) << 5) | (self.bank & 0x1F);
                if bank == 0 {
                    bank = 1
                }
                let addr = addr as usize + ((bank as usize - 1) * 0x4000);
                raw[addr as usize]
            }
            0xA000..=0xBFFF => {
                // TODO RAMアクセス
                panic!("should not reach!")
            }
            _ => panic!("should not reach!"),
        }
    }

    pub fn write_byte(&mut self, raw: &mut Vec<u8>, addr: u16, value: u8) {
        // FIXME for test
        // raw[addr as usize] = value;

        match addr {
            0x0000..=0x1FFF => {
                // TODO - RAM有効フラグ (W)
                self.ram_enabled = value;
            }
            0x2000..=0x3FFF => {
                self.bank = value;
            }
            0x400..=0x5FFF => {
                // TODO RAMバンク番号 / ROMバンク番号の上位bit (W)
                self.extra = value;
            }
            0x6000..=0x7FFF => {
                // TODO バンクモードセレクト (W)
                self.mode = value;
            }
            _ => panic!("should not reach!"),
        }
    }
}
