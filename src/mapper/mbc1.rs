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

    pub fn read_byte(&mut self, rom: &Vec<u8>, ram: &Vec<u8>, addr: u16) -> u8 {
        if self.mode == 0x00 {
            self.read_byte_normal(rom, ram, addr)
        } else {
            self.read_byte_advanced(rom, ram, addr)
        }
    }

    fn is_big_rom(&mut self, raw: &Vec<u8>) -> bool {
        raw.len() >= 1024 * 1024
    }

    fn is_big_ram(&mut self, raw: &Vec<u8>) -> bool {
        raw.len() >= 8 * 1024
    }

    fn bank1_addr(&mut self, addr: u16) -> usize {
        let mut bank = self.bank & 0x1F;
        if bank == 0 {
            bank = 1
        }
        let addr = addr as usize + (bank as usize * 0x4000);
        addr - 0x4000
    }

    fn bank1_addr_with_secondary_bank(&mut self, addr: u16) -> usize {
        let mut bank = ((self.secondary_bank & 0x03) << 5) | (self.bank & 0x1F);
        if bank == 0 {
            bank = 1
        }
        let addr = addr as usize + (bank as usize * 0x4000);
        addr as usize - 0x4000
    }

    pub fn read_byte_normal(&mut self, rom: &Vec<u8>, ram: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            // bank0
            0x0000..=0x3FFF => rom[addr as usize],
            // bank1
            0x4000..=0x7FFF => {
                if !self.is_big_rom(rom) && !self.is_big_ram(ram) {
                    // カートリッジのROMが1MiB未満で、RAMが8KiBより大きい場合、
                    // secondary_bankの影響を受けない
                    rom[self.bank1_addr(addr)]
                } else {
                    rom[self.bank1_addr_with_secondary_bank(addr)]
                }
            }
            0xA000..=0xBFFF => ram[addr as usize - 0xA000],
            _ => panic!("should not reach!"),
        }
    }

    pub fn read_byte_advanced(&mut self, rom: &Vec<u8>, ram: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            // bank0
            0x0000..=0x3FFF => {
                if self.is_big_rom(rom) {
                    let bank = self.secondary_bank & 0x03 << 5;
                    let addr = addr as usize + (bank as usize * 0x4000);
                    rom[addr as usize]
                } else {
                    rom[addr as usize]
                }
            }
            // bank1
            0x4000..=0x7FFF => {
                if self.is_big_rom(rom) {
                    rom[self.bank1_addr_with_secondary_bank(addr)]
                } else {
                    rom[self.bank1_addr(addr)]
                }
            }
            0xA000..=0xBFFF => {
                if self.is_big_ram(ram) {
                    // 大容量RAMカートリッジ
                    let bank = self.secondary_bank & 0x03;
                    let addr = addr as usize + (bank as usize * 0x2000);
                    ram[addr as usize - 0xA000]
                } else {
                    ram[addr as usize - 0xA000]
                }
            }
            _ => panic!("should not reach!"),
        }
    }

    pub fn write_byte(&mut self, rom: &mut Vec<u8>, ram: &mut Vec<u8>, addr: u16, value: u8) {
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
                if !self.ram_enabled {
                    return;
                }
                if self.mode == 0x00 {
                    // normal
                    ram[addr as usize - 0xA000] = value;
                } else {
                    // advanced
                    if self.is_big_ram(ram) {
                        // 大容量RAMカートリッジ
                        let bank = self.secondary_bank & 0x03;
                        let addr = addr as usize + (bank as usize * 0x2000);
                        ram[addr as usize - 0xA000] = value;
                    } else {
                        ram[addr as usize - 0xA000] = value;
                    }
                }
            }
            _ => panic!("should not reach!"),
        }
    }
}
