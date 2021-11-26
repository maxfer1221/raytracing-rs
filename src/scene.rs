use sdl2::pixels::Color;
use std::f64::consts::PI;

pub const RENDER_DISTANCE: f64 = 10_000.0;

pub struct Scene<'a> {
    pub camera: Camera,
    pub all_objects: Vec<&'a Obj>,
    pub objects: Vec<Obj>,
    pub lights: Vec<Obj>,
    pub sky: Color,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Scene {
            camera: Camera::new(),
            all_objects: Vec::new(),
            objects: Vec::new(),
            lights: Vec::new(),
            sky: Color::RGBA(0, 0, 0, 0),
        }
    }
    pub fn camera_loc(&self) -> (f64, f64, f64) {
        (self.camera.x, self.camera.y, self.camera.z)
    }

    pub fn camera_fov(&self) -> f64 { 
        self.camera.fov
    }

    pub fn set_camera(&mut self, x: f64, y: f64, z: f64) {
        self.camera.x = x;
        self.camera.y = y;
        self.camera.z = z;
    }

    pub fn sky(&self) -> Color {
        self.sky
    }

    pub fn objs(&self) -> &Vec<Obj> {
        &self.objects
    }
    
    pub fn objs_mut(&mut self) -> &mut Vec<Obj> {
        &mut self.objects
    }
    
    pub fn lights(&self) -> &Vec<Obj> {
        &self.lights
    }

    pub fn lights_mut(&mut self) -> &mut Vec<Obj> {
        &mut self.lights
    }
}

struct Camera {
    x: f64, y: f64, z: f64,
    a: f64, b: f64,
    fov: f64,
}

impl Camera {
    fn new() -> Self {
        Camera {
            x: 0.0, y: 0.0, z: 0.0,
            a: 0.0, b: 0.0,
            fov: PI / 2.0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Obj {
    Sphere(Sphere),
    Light(Light),
    // Prism(Prism),
}

#[derive(Clone, Debug)]
pub struct Sphere {
    pub x: f64, pub y: f64, pub z: f64,
    pub r: f64,
    pub c: Color,
    pub transparent: bool,
}

#[derive(Clone, Debug)]
pub struct Light {
    pub x: f64, pub y: f64, pub z: f64,
    pub r: f64,
    pub c: Color,
}

impl Light {
    pub fn coords(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn set(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}
// struct Prism {
//     x: f64, y: f64, z: f64,
//     l: f64, w: f64, h: f64,
// }


pub fn setup<'a>(scene: &'a mut Scene<'a>) {
    scene.lights.push(Obj::Light(Light {
        x: 0.0, y: -8.0, z: 20.0,
        r: 1.0,
        c: Color::RGBA(255, 255, 255, 255),
    }));
    scene.objects.push(Obj::Sphere(Sphere { 
        x: 0.0, y: 0.0, z: 20.0, 
        r: 5.0,
        c: Color::RGBA(100, 200, 30, 255),
        transparent: true,
    }));
    scene.objects.push(Obj::Sphere(Sphere { 
        x: 6.0, y: 0.0, z: 24.5, 
        r: 3.0,
        c: Color::RGBA(150, 50, 20, 255),
        transparent: false,
    }));
        // Obj::Prism(Prism {
        //     x: 0.0, y: -10.0, z: 50.0, 
        //     l: 20.0, w: 20.0, h: 20.0
        // })
    for obj in &scene.objects {
        scene.all_objects.push(&obj);
    } 
    for light in &scene.lights {
        scene.all_objects.push(&light);
    }
    scene.sky = Color::RGB(30, 30, 200);
}



