pub struct MBC1 {
    bank: u8,
    ram_enabled: bool,
    secondary_bank: u8, // u2
    mode: u8,           // u1
}

impl MBC1 {
    pub fn new() -> Self {
        MBC1 {
            bank: 1,
            ram_enabled: false,
            secondary_bank: 0,
            mode: 1,
        }
    }

    pub fn read_byte(&mut self, raw: &Vec<u8>, addr: u16) -> u8 {
        if self.mode == 0 {
            self.read_byte_normal(raw, addr)
        } else {
            self.read_byte_advanced(raw, addr)
        }
    }

    pub fn read_byte_normal(&mut self, raw: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            // bank0
            0x0000..=0x3FFF => raw[addr as usize],
            // bank1
            0x4000..=0x7FFF => {
                let mut bank = self.bank & 0x1F;
                if bank == 0 {
                    bank = 1
                }
                let addr = addr as usize + ((bank as usize - 1) * 0x4000);
                raw[addr as usize]
            }
            0xA000..=0xBFFF => {
                // TODO RAMアクセス
                // ram[addr as usize - 0xA000];
                panic!("should not reach!")
            }
            _ => panic!("should not reach!"),
        }
    }

    fn is_big_rom(&mut self, raw: &Vec<u8>) -> bool {
        raw.len() >= 1024 * 1024
    }

    pub fn read_byte_advanced(&mut self, raw: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            // bank0
            0x0000..=0x3FFF => {
                if self.is_big_rom(raw) {
                    let bank = self.secondary_bank & 0x03 << 5;
                    let addr = addr as usize + ((bank as usize - 1) * 0x4000);
                    raw[addr as usize]
                } else {
                    raw[addr as usize]
                }
            }
            // bank1
            0x4000..=0x7FFF => {
                if self.is_big_rom(raw) {
                    let mut bank = ((self.secondary_bank & 0x03) << 5) | (self.bank & 0x1F);
                    if bank == 0 {
                        bank = 1
                    }
                    let addr = addr as usize + ((bank as usize - 1) * 0x4000);
                    raw[addr as usize]
                } else {
                    let mut bank = self.bank & 0x1F;
                    if bank == 0 {
                        bank = 1
                    }
                    let addr = addr as usize + ((bank as usize - 1) * 0x4000);
                    raw[addr as usize]
                }
            }
            0xA000..=0xBFFF => {
                // TODO RAMアクセス
                // if self.is_big_rom(raw) {
                //     ram[addr as usize - 0xA000];
                // } else {
                //     // 大容量RAMカートリッジ
                //     let bank = self.secondary_bank & 0x03;
                //     let addr = addr as usize + ((bank as usize - 1) * 0x2000);
                //     ram[addr as usize - 0xA000]
                // }
                panic!("should not reach!")
            }
            _ => panic!("should not reach!"),
        }
    }

    pub fn write_byte(&mut self, raw: &mut Vec<u8>, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                // RAM有効フラグ (W)
                self.ram_enabled = value == 0x0A;
            }
            0x2000..=0x3FFF => {
                self.bank = value;
            }
            0x4000..=0x5FFF => {
                // RAMバンク番号 / ROMバンク番号の上位bit (W)
                self.secondary_bank = value;
            }
            0x6000..=0x7FFF => {
                // バンクモードセレクト (W)
                self.mode = value;
            }
            0xA000..=0xBFFF => {
                // TODO RAM書き込み
                // ram[addr as usize - 0xA000] = value;
            }
            _ => panic!("should not reach!"),
        }
    }
}
