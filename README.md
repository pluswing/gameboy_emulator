# gameboy_emulator

## 全般

ブートする方法（までの手順）
=> https://github.com/akatsuki105/gb-docs-ja/blob/main/powerup.md
バンク切り替え
I/O レジスタの中身 (0xFF00 ~ 0xFF7F, 0xFFFF)

- https://gbdev.io/pandocs/Hardware_Reg_List.html
  割り込み
  種類多い

## PPU(GPU,画面描画)

描画タイミング
=> https://gbdev.io/pandocs/Rendering.html
https://gbdev.io/pandocs/LCDC.html
https://gbdev.io/pandocs/STAT.html
Viewport X, Y position
=> https://gbdev.io/pandocs/Scrolling.html
スプライトのデータ構造
描画に関する内容

## APU(Sound)

サウンド生成タイミング

## References

https://rylev.github.io/DMG-01/public/book/introduction.html
https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
https://gbdev.io/pandocs
https://rgbds.gbdev.io/docs/v0.7.0/gbz80.7
http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
https://gekkio.fi/files/gb-docs/gbctr.pdf
https://github.com/akatsuki105/gb-docs-ja/tree/main
http://www.codeslinger.co.uk/pages/projects/gameboy.html


## test results
01-special.gb               passed
02-interrupts.gb            EI #2
03-op sp,hl.gb              passed
04-op r,imm.gb              passed
05-op rp.gb                 passed
06-ld r,r.gb                passed
07-jr,jp,call,ret,rst.gb    passed
08-misc instrs.gb           passed
09-op r,r.gb                passed
10-bit ops.gb               passed
11-op a,(hl).gb             passed
