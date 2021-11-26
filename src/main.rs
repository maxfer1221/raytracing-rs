extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
mod scene;
mod rays;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let size = (800, 800);
    let samples = (200, 200); 

    let true_width = size.0 / samples.0;
    let true_height = size.1 / samples.1;
    
    // println!("true_width: {}, th: {}", true_width, true_height);
    
    let window = video_subsystem.window("rust-sdl2 demo", size.0, size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut scene = scene::Scene::new(); 
    scene::setup(&mut scene);

    let mut cl;
    let mut fov;
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i: f64 = 0.0;
    let mut j: f64 = 1.0;
    let mut k: f64 = -10.0;

    'running: loop {
        let frame_dur = std::time::Instant::now();
        i += 0.2;
        j += 0.2;
        k += 0.5;
        canvas.set_draw_color(scene.sky());
        canvas.clear();
        let lights = scene.lights_mut();
        match &mut lights[0] {
            scene::Obj::Light(l) => {
                let lc = l.coords();
                l.set(k, lc.1, lc.2);
            },
            _ => {}
        };
        match &mut scene.objs_mut()[0] {
            scene::Obj::Sphere(s) => {
                s.y += 0.06 * i.cos();
                // s.z += k;
            }
            _ => {}
        }
        match &mut scene.objs_mut()[1] {
            scene::Obj::Sphere(s) => s.y += 0.06 * j.cos(),
            _ => {}
        }
        cl = scene.camera_loc();
        fov = scene.camera_fov();
        for x in 0..samples.0 {
            for y in 0..samples.1 {
                let ray = rays::Ray::new(
                    cl,
                    (
                        cl.0 - fov / 2.0 + (x + 1) as f64 * fov / (samples.0 as f64 + 1.0), 
                        cl.1 - fov / 2.0 + (y + 1) as f64 * fov / (samples.1 as f64 + 1.0), 
                    )
                );

                // println!("{:?}", ray);
                if let Some(color) = rays::final_color(&ray, &mut scene) {
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
        println!("{:?}", frame_dur.elapsed());
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

