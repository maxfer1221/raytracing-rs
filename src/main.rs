extern crate sdl2;

use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
mod scene;
mod rays;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let size = (800, 600);
    let samples = (800, 600); 

    let true_width = size.0 / samples.0;
    let true_height = size.1 / samples.1;
    
    println!("true_width: {}, th: {}", true_width, true_height);
    
    let window = video_subsystem.window("rust-sdl2 demo", size.0, size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let scene = scene::setup();
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.set_draw_color(scene.sky());
        canvas.clear();
        for x in 0..samples.0 {
            for y in 0..samples.1 {
                let ray = rays::Ray::new(
                    scene.camera_loc(),
                    (0.0, 0.0)
                );
                if let Some(color) = rays::final_color() {
                    canvas.set_draw_color(color);
                    // canvas.set_draw_color(Color::RGBA((x % 255) as u8, 0, 0, 255));
                    canvas.fill_rect(Rect::new((x * true_width) as i32, (y * true_height) as i32, true_width, true_height));
                }
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

