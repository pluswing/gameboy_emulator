mod cartridge;
mod cpu;
mod instruction;
mod joypad;
mod mapper;
mod memory_bus;
mod ppu;

use cartridge::Cartridge;
use cpu::CPU;
use joypad::Joypad;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let scale = 3;
    let window = video_subsystem
        .window("GameBoy Emulator", 160 * scale as u32, 144 * scale as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(scale as f32, scale as f32).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 160, 144)
        .unwrap();

    let dq = "rom/GB/ROM/DQ_MONSTERS/31/Dragon Quest Monsters - Terry no Wonderland (Japan) (SGB Enhanced) (GB Compatible).gbc";
    let kaeru = "rom/GB/ROM/KAERUNOTAMENI/35/Kaeru no Tame ni Kane wa Naru (Japan).gb";
    let kinka = "rom/GB/ROM/MARIOLAND2/34/Super Mario Land 2 - 6-tsu no Kinka (Japan) (Rev 2).gb";
    let pokemon =
        "rom/GB/ROM/POKEMON GREEN/-1/Pocket Monsters - Midori (Japan) (Rev 1) (SGB Enhanced).gb";
    let mario = "rom/GB/ROM/SUPER MARIOLAND/32/Super Mario Land (World).gb";
    let yugioh = "rom/GB/ROM/YUGIOU/30/Yu-Gi-Oh! Duel Monsters (Japan) (SGB Enhanced).gb";
    let zelda = "rom/GB/ROM/ZELDA/33/Zelda no Densetsu - Yume o Miru Shima (Japan).gb";

    let cartridge = Cartridge::new(pokemon);
    let mut cpu = CPU::new(cartridge);

    loop {
        cpu.step();
        if cpu.bus.ppu.frame_updated {
            handle_user_input(&mut event_pump, &mut cpu.bus.joypad);
            cpu.bus.ppu.frame_updated = false;
            let screen_state = cpu.bus.ppu.frame;
            texture.update(None, &screen_state, 160 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
            // ::std::thread::sleep(std::time::Duration::new(0, 70_000));
        }
    }
}

fn handle_user_input(event_pump: &mut EventPump, joypad: &mut Joypad) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),

            // joypad
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => joypad.a = true,
            Event::KeyUp {
                keycode: Some(Keycode::A),
                ..
            } => joypad.a = false,

            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => joypad.b = true,
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => joypad.b = false,

            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => joypad.start = true,
            Event::KeyUp {
                keycode: Some(Keycode::Return),
                ..
            } => joypad.start = false,

            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => joypad.select = true,
            Event::KeyUp {
                keycode: Some(Keycode::Space),
                ..
            } => joypad.select = false,

            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => joypad.up = true,
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            } => joypad.up = false,

            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => joypad.down = true,
            Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            } => joypad.down = false,

            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => joypad.left = true,
            Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            } => joypad.left = false,

            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => joypad.right = true,
            Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => joypad.right = false,

            _ => { /* do nothing */ }
        }
    }
}
