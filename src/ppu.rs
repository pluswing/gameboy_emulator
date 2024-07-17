pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

#[derive(Copy, Clone, Debug)]
enum TilePixelValue {
    Zero,
    One,
    Two,
    Three,
}

type Tile = [[TilePixelValue; 8]; 8];
fn empty_tile() -> Tile {
    [[TilePixelValue::Zero; 8]; 8]
}

fn tile_pixel_value_to_color(value: TilePixelValue) -> [u8; 3] {
    // TODO 0xFF47を見る必要あり。
    match value {
        TilePixelValue::Zero => [255, 255, 255],
        TilePixelValue::One => [170, 170, 170],
        TilePixelValue::Two => [85, 85, 85],
        TilePixelValue::Three => [0, 0, 0],
    }
}

// 0xFF40
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LcdControlRegisters {
    enabled: bool,           // LCD & PPU enable
    window_tile_map: bool,   // Window tile map
    window_enabled: bool,    // Window enable
    tiles: bool,             // BG & Window tiles
    bg_tile_map: bool,       // BG tile map
    obj_size: bool,          // OBJ size
    obj_enabled: bool,       // OBJ enable
    bg_window_enabled: bool, // BG & Window enable / priority
}

impl LcdControlRegisters {
    pub fn new() -> Self {
        LcdControlRegisters {
            enabled: false,
            window_tile_map: false,
            window_enabled: false,
            tiles: false,
            bg_tile_map: false,
            obj_size: false,
            obj_enabled: false,
            bg_window_enabled: false,
        }
    }
}

impl std::convert::From<LcdControlRegisters> for u8 {
    fn from(r: LcdControlRegisters) -> u8 {
        (if r.enabled { 1 } else { 0 }) << 7
            | (if r.window_tile_map { 1 } else { 0 }) << 6
            | (if r.window_enabled { 1 } else { 0 }) << 5
            | (if r.tiles { 1 } else { 0 }) << 4
            | (if r.bg_tile_map { 1 } else { 0 }) << 3
            | (if r.obj_size { 1 } else { 0 }) << 2
            | (if r.obj_enabled { 1 } else { 0 }) << 1
            | (if r.bg_window_enabled { 1 } else { 0 }) << 0
    }
}

impl std::convert::From<u8> for LcdControlRegisters {
    fn from(byte: u8) -> Self {
        let enabled = ((byte >> 7) & 0x01) != 0;
        let window_tile_map = ((byte >> 6) & 0x01) != 0;
        let window_enabled = ((byte >> 5) & 0x01) != 0;
        let tiles = ((byte >> 4) & 0x01) != 0;
        let bg_tile_map = ((byte >> 3) & 0x01) != 0;
        let obj_size = ((byte >> 2) & 0x01) != 0;
        let obj_enabled = ((byte >> 1) & 0x01) != 0;
        let bg_window_enabled = ((byte >> 0) & 0x01) != 0;
        LcdControlRegisters {
            enabled,
            window_tile_map,
            window_enabled,
            tiles,
            bg_tile_map,
            obj_size,
            obj_enabled,
            bg_window_enabled,
        }
    }
}

// 0xFF41
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LcdStatusRegisters {
    lyc_int_select: bool,   // LYC int select
    mode2_int_select: bool, // Mode 2 int select
    mode1_int_select: bool, // Mode 1 int select
    mode0_int_select: bool, // Mode 0 int select
    lyc_eq_ly: bool,        // LYC == LY
    ppu_mode: u8,           // (2bit) PPU mode
}

impl LcdStatusRegisters {
    pub fn new() -> Self {
        LcdStatusRegisters {
            lyc_int_select: false,
            mode2_int_select: false,
            mode1_int_select: false,
            mode0_int_select: false,
            lyc_eq_ly: false,
            ppu_mode: 0,
        }
    }
}

impl std::convert::From<LcdStatusRegisters> for u8 {
    fn from(r: LcdStatusRegisters) -> u8 {
        (if r.lyc_int_select { 1 } else { 0 }) << 6
            | (if r.mode2_int_select { 1 } else { 0 }) << 5
            | (if r.mode1_int_select { 1 } else { 0 }) << 4
            | (if r.mode0_int_select { 1 } else { 0 }) << 3
            | (if r.lyc_eq_ly { 1 } else { 0 }) << 2
            | (r.ppu_mode & 0x03)
    }
}

impl std::convert::From<u8> for LcdStatusRegisters {
    fn from(byte: u8) -> Self {
        let lyc_int_select = ((byte >> 6) & 0x01) != 0;
        let mode2_int_select = ((byte >> 5) & 0x01) != 0;
        let mode1_int_select = ((byte >> 4) & 0x01) != 0;
        let mode0_int_select = ((byte >> 3) & 0x01) != 0;
        let lyc_eq_ly = ((byte >> 2) & 0x01) != 0;
        let ppu_mode = byte & 0x03;
        LcdStatusRegisters {
            lyc_int_select,
            mode2_int_select,
            mode1_int_select,
            mode0_int_select,
            lyc_eq_ly,
            ppu_mode,
        }
    }
}

pub struct PPU {
    vram: [u8; VRAM_SIZE],
    pub ly: u8,  // 0xFF44
    pub lyc: u8, // 0xFF45 (LY compare)
    pub control: LcdControlRegisters,
    pub status: LcdStatusRegisters,
    // TODO 0xFF47が必要。palette
    tile_set: [Tile; 384],
    scanline_counter: u16,
    pub frame: [u8; 160 * 3 * 144],
    pub frame_updated: bool,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [0; VRAM_SIZE],
            ly: 0,
            lyc: 0,
            control: LcdControlRegisters::new(),
            status: LcdStatusRegisters::new(),
            tile_set: [empty_tile(); 384],
            scanline_counter: 0,
            frame: [0 as u8; 160 * 3 * 144],
            frame_updated: false,
        }
    }
    pub fn read_vram(&self, address: usize) -> u8 {
        self.vram[address]
    }
    pub fn write_vram(&mut self, index: usize, value: u8) {
        self.vram[index] = value;

        if index >= 0x1800 {
            return;
        }

        let normalized_index = index & 0xFFFE;
        let byte1 = self.vram[normalized_index];
        let byte2 = self.vram[normalized_index + 1];
        let tile_index = index / 16;
        let row_index = (index % 16) / 2;

        for pixel_index in 0..8 {
            let mask = 1 << (7 - pixel_index);
            let lsb = byte1 & mask;
            let msb = byte2 & mask;
            let value = match (lsb != 0, msb != 0) {
                (true, true) => TilePixelValue::Three,
                (false, true) => TilePixelValue::Two,
                (true, false) => TilePixelValue::One,
                (false, false) => TilePixelValue::Zero,
            };
            self.tile_set[tile_index][row_index][pixel_index] = value;
        }
    }

    pub fn update(&mut self, cycles: u16) -> bool {
        // SetLCDStatus( ) ;

        // if (!IsLCDEnabled()) {
        //   return
        //  }
        self.scanline_counter += cycles;

        if self.scanline_counter >= 456 {
            // 1ライン描画した
            self.scanline_counter -= 456;
            self.ly += 1;
            let currentline = self.ly;
            if currentline == 144 {
                // VBLANKに突入。
                //   VBRANK割り込み発生させる
                self.draw_all(); // for test
                self.frame_updated = true;
                return true;
            } else if currentline > 153 {
                // 1フレーム描画完了
                self.ly = 0;
            } else if currentline <= 144 {
                self.draw_scan_line(currentline);
            }
        }
        return false;
    }

    fn draw_scan_line(&mut self, line: u8) {
        // 1ラインを描画する。
        // self.frame
    }

    fn draw_all(&mut self) {
        // self.frame を全書き換えする
        // $9800-$9BFF のデータを見て、どのタイルがどこに配置されるかを計算する
        for addr in 0x9800..=0x9BFF {
            let addr = addr as usize - VRAM_BEGIN;
            let index = self.vram[addr] as usize;
            let tile = self.tile_set[index];
            let i = addr - 0x1800;
            let sx = (i % 32) * 8;
            let sy = (i / 32) * 8;
            for tx in 0..8 {
                for ty in 0..8 {
                    let value = tile[ty][tx];
                    let color = tile_pixel_value_to_color(value);
                    let x = sx + tx;
                    let y = sy + ty;
                    if x >= 160 || y >= 144 {
                        continue;
                    }
                    let o = ((y * 160 + x) * 3) as usize;
                    self.frame[o] = color[0];
                    self.frame[o + 1] = color[1];
                    self.frame[o + 2] = color[2];
                }
            }
        }
    }
}
