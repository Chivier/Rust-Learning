extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::thread::sleep;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL Video subsystem");

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to Create Window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to convert anvas into window");

    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
