use chrono::{DateTime, Timelike, Utc};

pub struct MBC3 {
    bank: u8,
    ram_enabled: bool,
    ram_bank: u8,
    latch_clock_data: u8,
    rtc: [u8; 5],
}

impl MBC3 {
    pub fn new() -> Self {
        MBC3 {
            bank: 1,
            ram_enabled: false,
            ram_bank: 0,
            latch_clock_data: 0,
            rtc: [0; 5],
        }
    }

    pub fn read_byte(&mut self, rom: &Vec<u8>, ram: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            // bank0
            0x0000..=0x3FFF => rom[addr as usize],
            // bank1
            0x4000..=0x7FFF => {
                // MEMO: 1MBのソフトのみ対応。2MBの場合は、0x7Fでマスクする必要あり。
                let bank = if (self.bank & 0x3F) == 0 {
                    1
                } else {
                    self.bank & 0x3F
                };
                let addr = addr as usize - 0x4000 + (bank as usize * 0x4000);
                rom[addr]
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    0
                } else if self.ram_bank <= 0x03 {
                    ram[addr as usize - 0xA000 + self.ram_bank as usize * 0x2000]
                } else if self.ram_bank >= 0x08 {
                    self.rtc[self.ram_bank as usize - 0x08]
                } else {
                    0
                }
            }
            _ => panic!("should not reach!"),
        }
    }

    pub fn write_byte(&mut self, rom: &mut Vec<u8>, ram: &mut Vec<u8>, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                // RAM/タイマー有効化フラグ
                self.ram_enabled = value == 0x0A;
            }
            0x2000..=0x3FFF => {
                // ROMバンク番号
                self.bank = value & 0x3F;
                // println!("BANK: {:02X}, {}", value, value);
            }
            0x4000..=0x5FFF => {
                // RAMバンク/RTCレジスタの選択レジスタ
                self.ram_bank = value & 0x0F;
            }
            0x6000..=0x7FFF => {
                // ラッチクロックデータ
                if self.latch_clock_data == 0x00 && value == 0x01 {
                    let now: DateTime<Utc> = Utc::now();
                    self.rtc[0] = now.second() as u8;
                    self.rtc[1] = now.minute() as u8;
                    self.rtc[2] = now.hour() as u8;
                    self.rtc[3] = 0; // FIXME 日付が入るが、省略！
                    self.rtc[4] = 0;
                }
                self.latch_clock_data = value;
            }
            0xA000..=0xBFFF => {
                // RAM書き込み / RTCレジスタ書き込み
                if !self.ram_enabled {
                } else if self.ram_bank <= 0x03 {
                    self.write_ram(ram, addr, value)
                } else if self.ram_bank >= 0x08 {
                    self.write_rtc(value)
                }
            }
            _ => panic!("should not reach!"),
        }
    }

    fn write_ram(&mut self, ram: &mut Vec<u8>, addr: u16, value: u8) {
        ram[addr as usize - 0xA000 + self.ram_bank as usize * 0x2000] = value;
    }

    fn write_rtc(&mut self, value: u8) {
        self.rtc[self.ram_bank as usize - 0x08] = value;
    }
}
