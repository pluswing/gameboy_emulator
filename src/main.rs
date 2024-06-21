mod cartridge;
mod cpu;
mod instruction;

use cartridge::Cartridge;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::EventPump;

fn main() {
    Cartridge::test("rom/cpu_instruction_test.gb");
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

    let mut screen_state = [0 as u8; 160 * 3 * 144];
    loop {
        handle_user_input(&mut event_pump);
        read_screen_state(&mut screen_state);
        texture.update(None, &screen_state, 160 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
        ::std::thread::sleep(std::time::Duration::new(0, 70_000));
    }
}

fn handle_user_input(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            _ => { /* do nothing */ }
        }
    }
}

fn read_screen_state(frame: &mut [u8; 160 * 3 * 144]) {
    let (b1, b2, b3) = sdl2::pixels::Color::WHITE.rgb();
    frame[0] = b1;
    frame[1] = b2;
    frame[2] = b3;

    let (b1, b2, b3) = sdl2::pixels::Color::GREEN.rgb();
    frame[3] = b1;
    frame[4] = b2;
    frame[5] = b3;
}
