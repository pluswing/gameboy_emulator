use crate::{
    cartridge::Cartridge,
    ppu::{LcdControlRegisters, LcdStatusRegisters, GPU, VRAM_BEGIN, VRAM_END},
};

pub struct MemoryBus {
    memory: [u8; 0x10000],
    cartridge: Cartridge,
    pub gpu: GPU,
}

impl MemoryBus {
    pub fn new(cartridge: Cartridge) -> Self {
        MemoryBus {
            memory: [0; 0x10000],
            cartridge,
            gpu: GPU::new(),
        }
    }
    pub fn read_byte(&mut self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..=0x7FFF => self.cartridge.read_byte(address as u16),
            VRAM_BEGIN..=VRAM_END => self.gpu.read_vram(address - VRAM_BEGIN),
            0xFF44 => self.gpu.ly,
            0xFF45 => self.gpu.lyc,
            0xFF40 => u8::from(self.gpu.control),
            0xFF41 => u8::from(self.gpu.status),
            0xFF50 => self.memory[address],
            _ => self.memory[address],
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            0x0000..=0x7FFF => self.cartridge.write_byte(address as u16, value),
            VRAM_BEGIN..=VRAM_END => self.gpu.write_vram(address - VRAM_BEGIN, value),
            0xFF44 => {
                panic!("LY is read only!")
            }
            0xFF45 => self.gpu.lyc = value,
            0xFF40 => self.gpu.control = LcdControlRegisters::from(value),
            0xFF41 => self.gpu.status = LcdStatusRegisters::from(value),
            0xFF50 => self.memory[address] = value, // FIXME boot rom bank switch
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
}
