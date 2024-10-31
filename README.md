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
https://hackmd.io/@anqou/HJcvRrwy9
https://dic.nicovideo.jp/a/gb%E9%9F%B3%E6%BA%90
https://gbdev.gg8.se/wiki/articles/Gameboy_sound_hardware#Register_Reading

## test results
01-special.gb               passed
02-interrupts.gb            Timer doesn't work
03-op sp,hl.gb              passed
04-op r,imm.gb              passed
05-op rp.gb                 passed
06-ld r,r.gb                passed
07-jr,jp,call,ret,rst.gb    passed
08-misc instrs.gb           passed
09-op r,r.gb                passed
10-bit ops.gb               passed
11-op a,(hl).gb             passed

## cartridge types
Dragon Quest Monsters - Terry no Wonderland (Japan) (SGB Enhanced) (GB Compatible).gbc: [03, 06, 02]
Kaeru no Tame ni Kane wa Naru (Japan).gb: [03, 04, 02]
Super Mario Land 2 - 6-tsu no Kinka (Japan) (Rev 2).gb: [03, 04, 02]
Pocket Monsters - Midori (Japan) (Rev 1) (SGB Enhanced).gb: [03, 04, 03]
Super Mario Land (World).gb: [01, 01, 00]
Yu-Gi-Oh! Duel Monsters (Japan) (SGB Enhanced).gb: [03, 05, 02]
Zelda no Densetsu - Yume o Miru Shima (Japan): [03, 04, 02]

## カラー対応

### できた
- ゲームボーイカラーの機能のアンロック
  - CGB対応ソフトかどうかのチェック
- ゲームボーイカラー (ゲームボーイアドバンス) の見分け方
  - ハード(emulator)がCGB対応してると言う
- 速度切り替え準備 (STOP命令)

### まだ
- 速度調整
  - LCD コントローラ (PPU) => DONE
  - VRAM bank / WRAM bank
  - HDMA ※
  - サウンドのタイミングと周波数 (APU) => DONE

- PPU実装
  - ...BGPIとか

