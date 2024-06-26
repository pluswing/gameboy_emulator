pub struct MBC1 {
    bank: u8,
}

impl MBC1 {
    pub fn new() -> Self {
        MBC1 { bank: 1 }
    }

    pub fn read_byte(raw: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            // bank0
            0x0000..=0x3FFF => raw[addr],
            // bank1
            0x4000..=0x7FFF => {
                let mut bank = self.bank & 0x1F;
                if bank == 0 {
                    bank = 1
                }
                let addr = addr + ((bank - 1) * 0x4000);
                raw[addr]
            }
        }
    }

    pub fn write_byte(raw: &mut Vec<u8>, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                // TODO - RAM有効フラグ (W)
            }
            0x2000..=0x3FFF => {
                self.bank = value;
            }
            0x400..=0x5FFF => {
                // TODO RAMバンク番号 / ROMバンク番号の上位bit (W)
            }
            0x6000..=0x7FFF => {
                // TODO バンクモードセレクト (W)
            }
        }
    }
}
