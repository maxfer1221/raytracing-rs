extern crate sdl2;

use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
mod scene;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let size = (800, 600);
    let step = 2;

    let window = video_subsystem.window("rust-sdl2 demo", size.0, size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();


    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        i = (i + 1) % 255;
        for x in (1..size.0).step_by(step) {
            for y in (1..size.1).step_by(step) {
                canvas.set_draw_color(Color::RGB((255 % x) as u8, (255 % y) as u8, 0));
                canvas.draw_point(Point::new(((x+i)%800) as i32, ((y+i)%800) as i32)); 
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

