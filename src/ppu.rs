pub const VRAM_BEGIN: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;
pub const VRAM_SIZE: usize = VRAM_END - VRAM_BEGIN + 1;

const LCD_WIDTH: usize = 160;
const LCD_HEIGHT: usize = 144;
const BACKGROUND_SIZE: usize = 255;

#[derive(Copy, Clone, Debug, PartialEq)]
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

fn tile_pixel_value_to_color(value: TilePixelValue, palette: u8) -> [u8; 3] {
    let value = match value {
        TilePixelValue::Zero => (palette & 0x03) >> 0,
        TilePixelValue::One => (palette & 0x0C) >> 2,
        TilePixelValue::Two => (palette & 0x30) >> 4,
        TilePixelValue::Three => (palette & 0xC0) >> 6,
    };
    get_color(value)
}

fn get_color(value: u8) -> [u8; 3] {
    match value {
        0b00 => [255, 255, 255],
        0b01 => [170, 170, 170],
        0b10 => [85, 85, 85],
        0b11 => [0, 0, 0],
        _ => panic!("should not reach"),
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
        // 0x91
        LcdControlRegisters {
            enabled: true,
            window_tile_map: false,
            window_enabled: false,
            tiles: true,
            bg_tile_map: false,
            obj_size: false,
            obj_enabled: false,
            bg_window_enabled: true,
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Sprite {
    index: u8,
    y: u8,
    x: u8,
    tile_index: u8,
    attributes: u8,
}

impl Sprite {
    fn new(index: u8, y: u8, x: u8, tile_index: u8, attributes: u8) -> Self {
        Sprite {
            index,
            y,
            x,
            tile_index,
            attributes,
        }
    }
}

pub enum PPUInterrupt {
    NONE,
    VBALNK,
    LCD,
}

pub struct PPU {
    vram: [u8; VRAM_SIZE],
    pub ly: u8,  // 0xFF44
    pub lyc: u8, // 0xFF45 (LY compare)
    pub control: LcdControlRegisters,
    pub status: LcdStatusRegisters,
    pub scy: u8,  // $FF42
    pub scx: u8,  // $FF43
    pub dma: u8,  // $FF46
    pub bgp: u8,  // $FF47
    pub obp0: u8, // $FF48
    pub obp1: u8, // $FF49
    pub wy: u8,   // $FF4A
    pub wx: u8,   // $FF4B
    oam: [u8; 0xA0],
    sprites: [Sprite; 40],
    // TODO 0xFF47が必要。palette
    tile_set: [Tile; 384],
    scanline_counter: u16,
    pub frame: [u8; LCD_WIDTH * 3 * LCD_HEIGHT],
    pub frame_updated: bool,
    pub bg1: [u8; 256 * 3 * 256],
    pub bg2: [u8; 256 * 3 * 256],
    line_index: [TilePixelValue; LCD_WIDTH],
    last_ly: u8,
    window_line: u8,
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [0; VRAM_SIZE],
            oam: [0; 0xA0],
            sprites: [Sprite::new(0, 0, 0, 0, 0); 40],
            ly: 0,
            lyc: 0,
            control: LcdControlRegisters::new(),
            status: LcdStatusRegisters::new(),
            scy: 0,
            scx: 0,
            dma: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            tile_set: [empty_tile(); 384],
            scanline_counter: 0,
            frame: [0 as u8; 160 * 3 * 144],
            frame_updated: false,
            bg1: [0 as u8; 256 * 3 * 256],
            bg2: [0 as u8; 256 * 3 * 256],
            line_index: [TilePixelValue::Zero; LCD_WIDTH],
            last_ly: 0,
            window_line: 0,
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

    pub fn write_oam(&mut self, address: usize, value: u8) {
        self.oam[address - 0xFE00] = value;

        for n in 0..40 {
            let i = n * 4;
            let y = self.oam[i];
            let x = self.oam[i + 1];
            let tile_index = self.oam[i + 2];
            let attributes = self.oam[i + 3];
            self.sprites[n].index = n as u8;
            self.sprites[n].y = y;
            self.sprites[n].x = x;
            self.sprites[n].tile_index = tile_index;
            self.sprites[n].attributes = attributes;
        }
    }

    pub fn read_oam(&mut self, address: usize) -> u8 {
        self.oam[address - 0xFE00]
    }

    pub fn update(&mut self, cycles: u16) -> PPUInterrupt {
        self.scanline_counter += cycles;
        // FIXME VBLANKと同時発生時におかしくなるかも。
        let interrupt = self.set_lcd_status();

        if !self.control.enabled {
            return PPUInterrupt::NONE;
        }

        if self.scanline_counter >= 456 {
            // println!(
            //     "LY: {}, SCX: {}, SCY: {}, CONTROL: {:?}",
            //     self.ly + 1,
            //     self.scx,
            //     self.scy,
            //     self.control
            // );
            // 1ライン描画した
            self.scanline_counter -= 456;
            self.ly += 1;
            let currentline = self.ly;
            if currentline == 144 {
                // VBLANKに突入。
                //   VBRANK割り込み発生させる
                self.draw_all(); // for test
                self.frame_updated = true;
                return PPUInterrupt::VBALNK;
            } else if currentline > 153 {
                // 1フレーム描画完了
                self.ly = 0;
                self.window_line = 0;
            }
        }
        return interrupt;
    }

    fn set_lcd_status(&mut self) -> PPUInterrupt {
        if !self.control.enabled {
            self.scanline_counter = 0;
            self.ly = 0;
            self.window_line = 0;
            self.status.ppu_mode = 0;
            return PPUInterrupt::NONE;
        }

        let currentline = self.ly;
        let currentmode = self.status.ppu_mode;

        let mut mode = 0;
        let mut reqInt = false;

        if currentline >= 144 {
            mode = 1;
            self.status.ppu_mode = 1;
            reqInt = self.status.mode1_int_select;
        } else {
            let mode2bounds = 80;
            let mode3bounds = 80 + 172;

            if self.scanline_counter < mode2bounds {
                mode = 2;
                self.status.ppu_mode = 2;
                reqInt = self.status.mode2_int_select;
            } else if self.scanline_counter < mode3bounds {
                mode = 3;
                self.status.ppu_mode = 3;
            } else {
                mode = 0;
                if self.status.ppu_mode != 0 {
                    self.draw_scan_line(currentline);
                }
                self.status.ppu_mode = 0;
                reqInt = self.status.mode0_int_select;
            }
        }

        let mut interrupt = PPUInterrupt::NONE;

        if reqInt && (mode != currentmode) {
            interrupt = PPUInterrupt::LCD;
        }

        if self.ly == self.lyc && self.ly != self.last_ly {
            self.status.lyc_eq_ly = true;
            if self.status.lyc_int_select {
                interrupt = PPUInterrupt::LCD;
            }
        } else {
            self.status.lyc_eq_ly = false;
        }
        self.last_ly = self.ly;
        return interrupt;
    }

    fn draw_scan_line(&mut self, line: u8) {
        // 1ラインを描画する。
        // self.frame
        // println!(
        //     "line: {} SCREEN: ({}, {}) WINDOW: ({}, {})",
        //     line, self.scx, self.scy, self.wx, self.wy
        // );

        // TODO ここの実装
        // 8x16モードの対応（スプライト）=> 夢を見る島
        // ウインドウとスプライトの前後関係

        // self.frame を全書き換えする
        // self.frame = [255 as u8; 160 * 3 * 144];
        self.draw_bg_line(line);
        self.draw_window_line(line);
        self.draw_sprites_line(line);
    }

    fn draw_bg_line(&mut self, line: u8) {
        if !self.control.bg_window_enabled {
            return;
        }

        let vram_base_index = if self.control.bg_tile_map {
            0x9C00 - VRAM_BEGIN
        } else {
            0x9800 - VRAM_BEGIN
        };

        // どこを描くのかを割り出す
        let ox = self.scx;
        let oy = self.scy.wrapping_add(line);

        // 何列目の描画かを割り出す
        let row = oy / 8;

        let index_offset = row as u16 * 32;

        // そのタイルの何行目を描くのか
        let src_y = oy % 8;

        // そのピクセルデータをとってきて、描く
        // xを起点にLCD_WIDTH分繰り返す
        for x in 0..LCD_WIDTH {
            let src_x = ox.wrapping_add(x as u8);
            let dest_y = line;
            let dest_x = x;

            let vram_index = index_offset + (ox.wrapping_add(x as u8) / 8) as u16;

            // tilemapのインデックスを取得する
            let index = self.vram[vram_base_index + vram_index as usize] as usize;
            let index = if self.control.tiles {
                index
            } else {
                if index < 128 {
                    index + 256
                } else {
                    index
                }
            };

            // タイルを取ってくる
            let tile = self.tile_set[index];

            // タイルの描画ピクセルを取得する
            let value = tile[(src_y % 8) as usize][(src_x % 8) as usize];
            let color = tile_pixel_value_to_color(value, self.bgp);
            self.line_index[dest_x as usize] = value;

            let o = (dest_y as usize * LCD_WIDTH + dest_x as usize) * 3;
            self.frame[o] = color[0];
            self.frame[o + 1] = color[1];
            self.frame[o + 2] = color[2];
        }
    }

    fn draw_window_line(&mut self, line: u8) {
        if !self.control.window_enabled || self.wx > 166 || self.wy > 143 {
            return;
        }

        // 描く場所（Y座標）が現在描画位置より小さい場合は描かなくて良い。
        if self.wy > line {
            return;
        }

        let vram_base_index = if self.control.window_tile_map {
            0x1C00
        } else {
            0x1800
        };

        // どこを描くのかを割り出す
        // let ox = 0;
        let oy = self.window_line;
        self.window_line += 1;

        // どこに描くのか
        // dest_x = wx + x - 7
        // dest_y = line;

        // 何列目の描画かを割り出す
        let row = oy / 8;
        let index_offset = row as u16 * 32;

        // そのピクセルデータをとってきて、描く
        // xを起点にLCD_WIDTH分繰り返す
        for x in 0..LCD_WIDTH {
            let src_x = x;
            let src_y = oy;

            if self.wx.wrapping_add(x as u8) < self.wx {
                continue;
            }

            if self.wx + (x as u8) < 7 {
                continue;
            }

            let dest_x = self.wx.wrapping_add(x as u8).wrapping_sub(7);
            let dest_y = line;

            if dest_x >= LCD_WIDTH as u8 || dest_y >= LCD_HEIGHT as u8 {
                continue;
            }

            let vram_index = index_offset + (x as u8 / 8) as u16;

            // tilemapのインデックスを取得する
            let index = self.vram[vram_base_index + vram_index as usize] as usize;
            let index = if self.control.tiles {
                index
            } else {
                if index < 128 {
                    index + 256
                } else {
                    index
                }
            };

            // タイルを取ってくる
            let tile = self.tile_set[index];

            // タイルの描画ピクセルを取得する
            let value = tile[(src_y % 8) as usize][(src_x % 8) as usize];
            let color = tile_pixel_value_to_color(value, self.bgp);
            self.line_index[dest_x as usize] = value;

            let o = (dest_y as usize * LCD_WIDTH + dest_x as usize) * 3;
            self.frame[o] = color[0];
            self.frame[o + 1] = color[1];
            self.frame[o + 2] = color[2];
        }
    }

    fn draw_sprites_line(&mut self, line: u8) {
        if !self.control.obj_enabled {
            return;
        }

        let y_size = if self.control.obj_size { 16 } else { 8 };

        let mut sprites: Vec<Sprite> = Vec::new();
        for sprite in self.sprites {
            let sy = sprite.y as i32 - 16;
            // lineにspriteが掛かっているかをチェック
            if (line as i32) < sy || (line as i32) >= sy + y_size {
                continue;
            }
            sprites.push(sprite);
        }

        sprites.sort_by(|a, b| {
            let y_diff = a.y.abs_diff(b.y);
            let x_diff = a.x.abs_diff(b.x);
            if y_diff < y_size as u8 && x_diff < 8 {
                a.x.cmp(&b.x)
            } else {
                a.index.cmp(&b.index)
            }
        });

        let sprites = if sprites.len() > 10 {
            &sprites[0..10]
        } else {
            &sprites
        };

        for sprite in sprites.into_iter().rev() {
            let sx = sprite.x as i32;
            let sy = sprite.y as i32;
            let attribute = sprite.attributes;
            let priority = (attribute & 0x80) != 0;
            let y_flip = (attribute & 0x40) != 0;
            let x_flip = (attribute & 0x20) != 0;
            let palette = (attribute & 0x10) >> 4;

            let sx = sx - 8;
            let sy = sy - 16;

            let y_size = if self.control.obj_size { 16 } else { 8 };

            let ty = line as i32 - sy;
            let ty = if y_flip { (y_size - 1) - ty } else { ty };

            for tx in 0..8 {
                let value = if self.control.obj_size {
                    if ty >= 8 {
                        let tile = self.tile_set[sprite.tile_index as usize | 0x01];
                        tile[ty as usize - 8][tx]
                    } else {
                        let tile = self.tile_set[sprite.tile_index as usize & 0xFE];
                        tile[ty as usize][tx]
                    }
                } else {
                    let tile = self.tile_set[sprite.tile_index as usize];
                    tile[ty as usize][tx]
                };

                if value == TilePixelValue::Zero {
                    continue;
                }

                let color = tile_pixel_value_to_color(
                    value,
                    if palette == 0 { self.obp0 } else { self.obp1 },
                );
                let tx = if x_flip { 7 - tx } else { tx };
                let x = sx + (tx as i32);
                let y = line as i32;

                if x < 0 || x >= LCD_WIDTH as i32 || y < 0 || y >= LCD_HEIGHT as i32 {
                    continue;
                }

                if priority && self.line_index[x as usize] != TilePixelValue::Zero {
                    continue;
                }

                let o = ((y * LCD_WIDTH as i32 + x) * 3) as usize;
                self.frame[o] = color[0];
                self.frame[o + 1] = color[1];
                self.frame[o + 2] = color[2];
            }
        }
    }

    fn draw_all(&mut self) {
        self.draw_bg(true);
        self.draw_bg(false);
    }

    fn draw_bg(&mut self, bg1: bool) {
        let frame = if bg1 { &mut self.bg1 } else { &mut self.bg2 };
        let range = if bg1 {
            0x9800..=0x9BFF
        } else {
            0x9C00..=0x9FFF
        };
        for addr in range {
            let addr = addr as usize - VRAM_BEGIN;
            let index = self.vram[addr] as usize;
            let index = if self.control.tiles {
                index
            } else {
                if index < 128 {
                    index + 256
                } else {
                    index
                }
            };
            let tile = self.tile_set[index];
            let i = addr - if bg1 { 0x1800 } else { 0x1C00 };
            let sx = (i % 32) * 8;
            let sy = (i / 32) * 8;
            for tx in 0..8 {
                for ty in 0..8 {
                    let value = tile[ty][tx];
                    let color = tile_pixel_value_to_color(value, self.bgp);
                    let x = sx + tx;
                    let y = sy + ty;
                    let o = ((y * 256 + x) * 3) as usize;
                    frame[o] = color[0];
                    frame[o + 1] = color[1];
                    frame[o + 2] = color[2];
                }
            }
        }
    }
}
