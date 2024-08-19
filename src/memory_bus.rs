use crate::{
    cartridge::Cartridge,
    ppu::{LcdControlRegisters, LcdStatusRegisters, PPU, VRAM_BEGIN, VRAM_END},
};

pub struct MemoryBus {
    pub memory: [u8; 0x10000],
    cartridge: Cartridge,
    pub ppu: PPU,
}

impl MemoryBus {
    pub fn new(cartridge: Cartridge) -> Self {
        MemoryBus {
            memory: [0; 0x10000],
            cartridge,
            ppu: PPU::new(),
        }
    }
    pub fn read_byte(&mut self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..=0x7FFF => self.cartridge.read_byte(address as u16),
            VRAM_BEGIN..=VRAM_END => self.ppu.read_vram(address - VRAM_BEGIN),
            0xFF40 => u8::from(self.ppu.control),
            0xFF41 => u8::from(self.ppu.status),
            0xFF42 => self.ppu.scy,
            0xFF43 => self.ppu.scx,
            0xFF44 => self.ppu.ly,
            0xFF45 => self.ppu.lyc,
            0xFF46 => self.ppu.dma,
            0xFF47 => self.ppu.bgp,
            0xFF48 => self.ppu.obp0,
            0xFF49 => self.ppu.obp1,
            0xFF4A => self.ppu.wy,
            0xFF4B => self.ppu.wx,
            0xFE00..=0xFE9F => self.ppu.read_oam(address),
            0xFF50 => self.memory[address],
            0xFF00 => 0x0F, // FIXME joypad
            _ => self.memory[address],
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            0x0000..=0x7FFF => self.cartridge.write_byte(address as u16, value),
            VRAM_BEGIN..=VRAM_END => self.ppu.write_vram(address - VRAM_BEGIN, value),
            0xFF40 => self.ppu.control = LcdControlRegisters::from(value),
            0xFF41 => self.ppu.status = LcdStatusRegisters::from(value),
            0xFF42 => self.ppu.scy = value,
            0xFF43 => self.ppu.scx = value,
            0xFF44 => {} // FIXME ドラクエモンスターズで書いてる // panic!("LY is read only!"),
            0xFF45 => self.ppu.lyc = value,
            0xFF46 => self.do_dma_transfer(value),
            0xFF47 => self.ppu.bgp = value,
            0xFF48 => self.ppu.obp0 = value,
            0xFF49 => self.ppu.obp1 = value,
            0xFF4A => self.ppu.wy = value,
            0xFF4B => self.ppu.wx = value,
            0xFE00..=0xFE9F => self.ppu.write_oam(address, value),

            0xFF50 => self.memory[address] = value, // FIXME boot rom bank switch
            0xFF01 => {
                // 本当はシリアル通信.
                // テストROMがここに出力をするので、hook
                let res = [value, 0x00].iter().map(|&s| s as char).collect::<String>();
                print!("{}", res);
            }
            0xFF04 => {
                // DIV
                self.memory[address] = 0;
            }
            _ => self.memory[address] = value,
        }
    }
    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }

    pub fn read_word(&mut self, address: u16) -> u16 {
        return self.read_byte(address) as u16 | (self.read_byte(address + 1) as u16) << 8;
    }

    pub fn do_dma_transfer(&mut self, value: u8) {
        self.ppu.dma = value;
        let address = (value as u16) << 8;
        for i in 0x00..=0x9F {
            let value = self.read_byte(address + i);
            self.write_byte(0xFE00 + i, value);
        }
        // TODO 160サイクルかかる
    }
}
