use sdl2::pixels::Color;
use rand;

pub struct Ray {
    x: f64,
    y: f64,
    a: f64,
    b: f64,
}

impl Ray {
    pub fn new(l: (f64, f64), a: (f64, f64)) -> Self {
        Ray {
            x: l.0,
            y: l.1,
            a: a.0,
            b: a.1,
        }
    }
}

pub fn final_color() -> Option<Color> {
    Some(Color::RGBA(255, 255, 0, 255))
}
