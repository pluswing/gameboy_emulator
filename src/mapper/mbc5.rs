pub struct MBC5 {
    bank: u16,
    ram_enabled: bool,
    ram_bank: u8,
}

impl MBC5 {
    pub fn new() -> Self {
        MBC5 {
            bank: 1,
            ram_enabled: false,
            ram_bank: 0,
        }
    }

    pub fn read_byte(&mut self, rom: &Vec<u8>, ram: &Vec<u8>, addr: u16) -> u8 {
        match addr {
            // bank0
            0x0000..=0x3FFF => rom[addr as usize],
            // bank1
            0x4000..=0x7FFF => rom[self.bank1_addr(addr)],
            0xA000..=0xBFFF => {
                let bank = self.ram_bank & 0x0F;
                let addr = addr as usize + (bank as usize * 0x2000);
                ram[addr as usize - 0xA000]
            }
            _ => panic!("should not reach!"),
        }
    }

    fn bank1_addr(&mut self, addr: u16) -> usize {
        let mut bank = self.bank & 0x1FF;
        let addr = addr as usize + (bank as usize * 0x4000);
        addr - 0x4000
    }

    pub fn write_byte(&mut self, rom: &mut Vec<u8>, ram: &mut Vec<u8>, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => {
                // RAM有効フラグ (W)
                self.ram_enabled = value == 0x0A;
            }
            0x2000..=0x2FFF => {
                // ROMバンク番号の下位8bit
                self.bank = (self.bank & 0xFF00) | value as u16;
            }
            0x3000..=0x3FFF => {
                // ROMバンク番号のbit9
                self.bank = (self.bank & 0x00FF) | ((value as u16) << 8);
            }
            0x4000..=0x5FFF => {
                // RAMバンク番号
                self.ram_bank = value;
            }
            _ => panic!("should not reach!"),
        }
    }
}
