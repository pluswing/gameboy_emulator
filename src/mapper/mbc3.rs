pub struct MBC3 {
    bank: u8,
    ram_enabled: bool,
    ram_bank: u8,
    latch_clock_data: u8,
    rtc_value: u8,
}

impl MBC3 {
    pub fn new() -> Self {
        MBC3 {
            bank: 1,
            ram_enabled: false,
            ram_bank: 0,
            latch_clock_data: 0,
            rtc_value: 0,
        }
    }

    pub fn read_byte(&mut self, rom: &Vec<u8>, ram: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            // bank0
            0x0000..=0x3FFF => rom[addr as usize],
            // bank1
            0x4000..=0x7FFF => {
                let bank = if self.bank == 0 { 1 } else { self.bank };
                let addr = addr as usize - 0x4000 + (bank as usize * 0x4000);
                rom[addr]
            }
            0xA000..=0xBFFF => {
                if !self.ram_enabled {
                    0
                } else if self.ram_bank <= 0x03 {
                    ram[addr as usize - 0xA000 + self.ram_bank as usize * 0x2000]
                } else {
                    self.rtc_value
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
                self.bank = value;
            }
            0x4000..=0x5FFF => {
                // RAMバンク/RTCレジスタの選択レジスタ
                self.ram_bank = value;
            }
            0x6000..=0x7FFF => {
                // ラッチクロックデータ
                if self.latch_clock_data == 0x00 && value == 0x01 {
                    // 現在の時刻がRTCレジスタにラッチする。
                    self.rtc_value = value;
                }
                self.latch_clock_data = value;
            }
            0xA000..=0xBFFF => {
                // RAM書き込み / RTCレジスタ書き込み
                if !self.ram_enabled {
                } else if self.ram_bank <= 0x03 {
                    self.write_ram(ram, addr, value)
                } else {
                    self.write_rtc(addr, value)
                }
            }
            _ => panic!("should not reach!"),
        }
    }

    fn write_ram(&mut self, ram: &mut Vec<u8>, addr: u16, value: u8) {
        ram[addr as usize - 0xA000 + self.ram_bank as usize * 0x2000] = value;
    }

    fn write_rtc(&mut self, addr: u16, value: u8) {
        // TODO
    }
}
