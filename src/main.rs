mod apu;
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
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::video::Window;
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

    let scale = 1;
    let bg1_window = video_subsystem
        .window("BG1", 256 * scale as u32, 256 * scale as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut bg1_canvas = bg1_window.into_canvas().present_vsync().build().unwrap();
    bg1_canvas.set_scale(scale as f32, scale as f32).unwrap();
    let bg1_creator = bg1_canvas.texture_creator();
    let mut bg1_texture = bg1_creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 256)
        .unwrap();

    let bg2_window = video_subsystem
        .window("BG2", 256 * scale as u32, 256 * scale as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut bg2_canvas = bg2_window.into_canvas().present_vsync().build().unwrap();
    bg2_canvas.set_scale(scale as f32, scale as f32).unwrap();
    let bg2_creator = bg2_canvas.texture_creator();
    let mut bg2_texture = bg2_creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 256)
        .unwrap();

    bg1_canvas.window_mut().hide();
    bg2_canvas.window_mut().hide();

    // init audio
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(2),
        samples: None, // default sample size
    };

    let device: AudioQueue<f32> = audio_subsystem
        .open_queue::<f32, _>(None, &desired_spec)
        .unwrap();
    //

    // let target_bytes = device.spec().freq * device.spec().channels as i32 * 2; // 秒
    // let period_left = device.spec().freq / 256;
    // let period_right = device.spec().freq / 512;
    // let mut wave = Vec::new();
    // for x in 0..target_bytes / 2 {
    //     wave.push(if (x / period_left) % 2 == 0 {
    //         0.02
    //     } else {
    //         -0.02
    //     });
    //     wave.push(if (x / period_right) % 2 == 0 {
    //         0.02
    //     } else {
    //         -0.02
    //     });
    // }
    // device.queue_audio(&wave).unwrap();
    // println!("TB: {}, SIZE: {}", target_bytes, device.size() / 4);

    device.resume();

    let dq = "rom/GB/ROM/DQ_MONSTERS/31/Dragon Quest Monsters - Terry no Wonderland (Japan) (SGB Enhanced) (GB Compatible).gbc";
    let kaeru = "rom/GB/ROM/KAERUNOTAMENI/35/Kaeru no Tame ni Kane wa Naru (Japan).gb";
    let kinka = "rom/GB/ROM/MARIOLAND2/34/Super Mario Land 2 - 6-tsu no Kinka (Japan) (Rev 2).gb";
    let pokemon =
        "rom/GB/ROM/POKEMON GREEN/-1/Pocket Monsters - Midori (Japan) (Rev 1) (SGB Enhanced).gb";
    let mario = "rom/GB/ROM/SUPER MARIOLAND/32/Super Mario Land (World).gb";
    let zelda = "rom/GB/ROM/ZELDA/33/Zelda no Densetsu - Yume o Miru Shima (Japan).gb";
    let yugioh = "rom/GB/ROM/YUGIOU/30/Yu-Gi-Oh! Duel Monsters (Japan) (SGB Enhanced).gb";

    let cartridge = Cartridge::new(zelda);
    let mut cpu = CPU::new(cartridge, device);

    loop {
        cpu.step();
        if cpu.bus.ppu.frame_updated {
            handle_user_input(&mut event_pump, &mut cpu.bus.joypad);
            cpu.bus.ppu.frame_updated = false;
            let screen_state = cpu.bus.ppu.frame;
            texture.update(None, &screen_state, 160 * 3).unwrap();
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();

            bg1_texture.update(None, &cpu.bus.ppu.bg1, 256 * 3).unwrap();
            bg1_canvas.copy(&bg1_texture, None, None).unwrap();
            bg1_canvas.present();

            bg2_texture.update(None, &cpu.bus.ppu.bg2, 256 * 3).unwrap();
            bg2_canvas.copy(&bg2_texture, None, None).unwrap();
            bg2_canvas.present();
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
