use sdl2::audio::AudioQueue;

use crate::{
    apu::APU,
    cartridge::Cartridge,
    joypad::Joypad,
    ppu::{LcdControlRegisters, LcdStatusRegisters, PPU, VRAM_BEGIN, VRAM_END},
};

pub struct MemoryBus {
    pub memory: [u8; 0x10000],
    pub cartridge: Cartridge,
    pub ppu: PPU,
    pub joypad: Joypad,
    pub apu: APU,

    // CGB
    svbk: u8,
    wram: [u8; 0x8000], // 32KB
}

impl MemoryBus {
    pub fn new(cartridge: Cartridge, device: AudioQueue<f32>) -> Self {
        MemoryBus {
            memory: [0; 0x10000],
            cartridge,
            ppu: PPU::new(),
            joypad: Joypad::new(),
            apu: APU::new(device),
            svbk: 0,
            wram: [0; 0x8000],
        }
    }
    pub fn read_byte(&mut self, address: u16) -> u8 {
        let address = address as usize;
        match address {
            0x0000..=0x7FFF => self.cartridge.read_byte(address as u16),
            0x8000..=0x9FFF => self.ppu.read_vram(address - VRAM_BEGIN),
            0xA000..=0xBFFF => self.cartridge.read_byte(address as u16),
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
            0xFF00 => self.joypad.read(),

            // HDMA
            0xFF51 => self.ppu.hdma1,
            0xFF52 => self.ppu.hdma2,
            0xFF53 => self.ppu.hdma3,
            0xFF54 => self.ppu.hdma4,
            0xFF55 => self.ppu.hdma5,
            // VBK
            0xFF4F => self.ppu.vbk | 0xFE,
            // SVBK
            0xFF70 => self.svbk,
            // WRAM
            0xC000..=0xDFFF => self.read_wram(address as u16),
            // BCPS
            0xFF68 => self.ppu.bcps,
            0xFF69 => self.ppu.read_bg_palette(),
            // OCPS
            0xFF6A => self.ppu.ocps,
            0xFF6B => self.ppu.read_sprite_palette(),

            // APU
            0xFF26 | 0xFF25 | 0xFF24 => self.apu.global.read(address as u16),
            // CH1
            0xFF10 | 0xFF11 | 0xFF12 | 0xFF13 | 0xFF14 => self.apu.ch1.read(address as u16),
            // CH2
            0xFF16 | 0xFF17 | 0xFF18 | 0xFF19 => self.apu.ch2.read(address as u16),
            // CH3
            0xFF1A | 0xFF1B | 0xFF1C | 0xFF1D | 0xFF1E | 0xFF30..=0xFF3F => {
                self.apu.ch3.read(address as u16)
            }
            // CH4
            0xFF20 | 0xFF21 | 0xFF22 | 0xFF23 => self.apu.ch4.read(address as u16),
            // key0, key1 (0xFF4C, 0xFF4D)
            _ => self.memory[address],
        }
    }
    pub fn write_byte(&mut self, address: u16, value: u8) {
        let address = address as usize;
        match address {
            0x0000..=0x7FFF => self.cartridge.write_byte(address as u16, value),
            0x8000..=0x9FFF => self.ppu.write_vram(address - VRAM_BEGIN, value),
            0xA000..=0xBFFF => self.cartridge.write_byte(address as u16, value),
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

            // HDMA
            0xFF51 => self.ppu.hdma1 = value,
            0xFF52 => self.ppu.hdma2 = value,
            0xFF53 => self.ppu.hdma3 = value,
            0xFF54 => self.ppu.hdma4 = value,
            0xFF55 => self.do_hdma_transfer(value),
            // VBK
            0xFF4F => self.ppu.vbk = value,
            // SVBK
            0xFF70 => self.svbk = value,
            // WRAM
            0xC000..=0xDFFF => self.write_wram(address as u16, value),
            // BCPS
            0xFF68 => self.ppu.bcps = value,
            0xFF69 => self.ppu.write_bg_palette(value),
            // OCPS
            0xFF6A => self.ppu.ocps = value,
            0xFF6B => self.ppu.write_sprite_palette(value),

            0xFF50 => self.memory[address] = value, // FIXME boot rom bank switch
            0xFF01 => {
                // 本当はシリアル通信.
                // テストROMがここに出力をするので、hook
                // let res = [value, 0x00].iter().map(|&s| s as char).collect::<String>();
                // print!("{}", res);
            }
            0xFF02 => {
                // BEAT MANIA2の起動時にTransfer enableを立てていて、
                // 落ちるのを待つので、書き込まれないようにする。
                // println!("SC: {:02X}", value);
            }
            0xFF04 => {
                // DIV
                self.memory[address] = 0;
            }
            0xFF00 => self.joypad.write(value),

            // APU
            0xFF26 | 0xFF25 | 0xFF24 => self.apu.global.write(address as u16, value),
            // CH1
            0xFF10 | 0xFF11 | 0xFF12 | 0xFF13 | 0xFF14 => self.apu.ch1.write(address as u16, value),
            // CH2
            0xFF16 | 0xFF17 | 0xFF18 | 0xFF19 => self.apu.ch2.write(address as u16, value),
            // CH3
            0xFF1A | 0xFF1B | 0xFF1C | 0xFF1D | 0xFF1E | 0xFF30..=0xFF3F => {
                self.apu.ch3.write(address as u16, value)
            }
            // CH4
            0xFF20 | 0xFF21 | 0xFF22 | 0xFF23 => self.apu.ch4.write(address as u16, value),

            // ...
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
        // FIXME 160サイクルかかる
    }

    pub fn do_hdma_transfer(&mut self, value: u8) {
        self.ppu.hdma5 = value;

        // 下位7bitは転送サイズ（10hで割った値から1を引いた値）を指定し、つまり、$00..7Fの値で$10..800バイトの長さを定義することができます。
        let size = self.ppu.hdma5 & 0x7F;
        // 実際の転送サイズを計算する
        let size = (size as u16 + 1) * 16;

        // mode = true ==> HBlank DMA, false => 汎用DMA
        let mode = self.ppu.hdma5 & 0x80 != 0;

        // 汎用DMAで処理を行う。
        // FIXME HBlank DMAは必要に応じて実装する。（必要ないかもしれない。。。）

        let src = ((self.ppu.hdma1 as u16) << 8 | self.ppu.hdma2 as u16) & 0xFFF0;
        let dest = (((self.ppu.hdma3 as u16) << 8 | self.ppu.hdma4 as u16) & 0x1FF0) | 0x8000;

        // println!("HDMA mode: {} s{:04X} d{:04X} {}", mode, src, dest, size);

        for i in 0..size {
            let v = self.read_byte(src + i);
            self.write_byte(dest + i, v);
        }

        self.ppu.hdma5 = 0xFF;
    }

    fn read_wram(&mut self, address: u16) -> u8 {
        match address {
            0xC000..=0xCFFF => self.wram[address as usize - 0xC000],
            0xD000..=0xDFFF => {
                let mut bank = (self.svbk & 0x07) as usize;
                if bank == 0 {
                    bank = 1
                }
                let offset = bank * 0x1000;
                let address = address as usize - 0xD000 + offset;
                self.wram[address]
            }
            _ => panic!("should not reach."),
        }
    }

    fn write_wram(&mut self, address: u16, value: u8) {
        match address {
            0xC000..=0xCFFF => self.wram[address as usize - 0xC000] = value,
            0xD000..=0xDFFF => {
                let mut bank = (self.svbk & 0x07) as usize;
                if bank == 0 {
                    bank = 1
                }
                let offset = bank * 0x1000;
                let address = address as usize - 0xD000 + offset;
                self.wram[address] = value
            }
            _ => panic!("should not reach."),
        }
    }
}
