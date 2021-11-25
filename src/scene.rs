use sdl2::pixels::Color;

pub struct Scene {
    camera: Camera,
    objects: Vec<Obj>,
    sky: Color,
}

impl Scene {
    pub fn camera_loc(&self) -> (f64, f64) {
        (self.camera.x, self.camera.y)
    }

    pub fn sky(&self) -> Color {
        self.sky
    }
}

struct Camera {
    x: f64,
    y: f64,
    z: f64,
    a: f64,
    b: f64,
    fov: f64,
}

enum Obj {
    Sphere(Sphere),
    Prism(Prism),
}

struct Sphere {
    x: f64,
    y: f64,
    z: f64,
    r: f64,
}

struct Prism {
    x: f64,
    y: f64,
    z: f64,
    l: f64,
    w: f64,
    h: f64,
}


pub fn setup() -> Scene {
    let mut scene = Scene {
        camera: Camera { x: 0.0, y: 0.0, z: 0.0, a: 0.0, b: 0.0, fov: 90.0 },
        objects: Vec::new(),
        sky: Color::RGB(30, 30, 200),
    };

    scene.objects.append(&mut vec![
        Obj::Sphere(Sphere { x: 100.0, y: 0.0, z: 100.0, r: 10.0 }),
        Obj::Prism(Prism {
            x: 0.0, y: -10.0, z: 50.0, l: 20.0, w: 20.0, h: 20.0
        })
    ]);
    scene
}



