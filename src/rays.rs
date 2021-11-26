use sdl2::pixels::Color;
use crate::scene::{Sphere, Obj, Scene, RENDER_DISTANCE};

#[derive(Debug)]
pub struct Ray {
    x: f64, y: f64, z: f64,
    a: f64, b: f64,
}

impl Ray {
    pub fn new(l: (f64, f64, f64), t: (f64, f64)) -> Self {
        Ray {
            x: l.0, y: l.1, z: l.2,
            a: t.0, b: t.1,
        }
    }
}

// fn intersection_ray<'a>(r: &'a Ray, obj: &'a Obj) -> Option<Vec<(f64, f64, f64, &'a Obj)>> {
//     match obj {
//         Obj::Sphere(s) => {
//             let mut d = (r.a.cos(), r.a.sin(), r.b.sin());
//             let l = dot(d, d).sqrt();
//             d = (d.1 / l, d.2 / l, d.0 / l);
//             let pc = (r.x - s.x, r.y - s.y, r.z - s.z);
            
//             let pd = dot(pc, d);
//             let S = pd.powf(2.0) + s.r.powf(2.0) - dot(pc, pc);
          
//             // println!("{}", S);

//             if S < 0.0 {
//                 return None;
//             }

//             let mut l = -pd + S.sqrt();
//             let sol1 = (r.x + l * d.0, r.y + l * d.1, r.z + l * d.2, obj);
//             // println!("here");

//             if S == 0.0 {
//                 return Some(vec![sol1]);
//             }

//             l = -pd - S.sqrt();
//             let sol2 = (r.x + l * d.0, r.y + l * d.1, r.z + l * d.2, obj);
//             return Some(vec![sol1, sol2]);
//         },
//         _ => None,
//     }
// }

fn intersection(p1: (f64, f64, f64), p2: (f64, f64, f64), obj: &Obj) 
    -> Option<Vec<(f64, f64, f64, &Obj)>> {
    match obj {
        Obj::Sphere(s) => sphere_intersection(p1, p2, obj, s.clone()),
        Obj::Light(l) => sphere_intersection(p1, p2, obj, Sphere {
            x: l.x, y: l.y, z: l.z,
            r: l.r,
            c: l.c,
            transparent: false,
        }),
        _ => None
    }
}

fn sphere_intersection<'a>(p1: (f64, f64, f64), p2: (f64, f64, f64), obj: &'a Obj, s: Sphere) -> Option<Vec<(f64, f64, f64, &'a Obj)>> {
    let A: f64 = dot(p2, p2);
    let B: f64 = 
        2.0 * (p1.0 * p2.0 + p1.1 * p2.1 + p1.2 * p2.2 
               - p2.0 * s.x - p2.1 * s.y - p2.2 * s.z);
    let C: f64 = 
        dot(p1, p1) - 2.0 * p1.0 * s.x + s.x * s.x
        - 2.0 * p1.1 * s.y + s.y * s.y - 2.0 * p1.2 * s.z
        + s.z * s.z - s.r * s.r;
        

    let D = B * B - 4.0 * A * C;
   
    if D < 0.0 {
        return None;
    }

    let t1 = (( -B - D.sqrt() ) / ( 2.0 * A )) - 0.001;
    let sol1 = (p1.0 * ( 1.0 - t1 ) + t1 * p2.0,
                p1.1 * ( 1.0 - t1 ) + t1 * p2.1,
                p1.2 * ( 1.0 - t1 ) + t1 * p2.2,
                obj,
    );
   
    if D == 0.0 {
        return Some(vec![sol1]);
    }

    let t2 = (( -B + D.sqrt() ) / ( 2.0 * A )) - 0.001;
    let sol2 = (p1.0 * ( 1.0 - t2 ) + t2 * p2.0,
                p1.1 * ( 1.0 - t2 ) + t2 * p2.1,
                p1.2 * ( 1.0 - t2 ) + t2 * p2.2,
                obj,
    );

    Some(vec![sol1, sol2])
}

// fn intersection_ray_all<'a>(ray: &'a Ray, objs: &'a Vec<Obj>, sort: bool) 
//     -> Option<Vec<(f64, f64, f64, &'a Obj)>> {
//     let mut all = Vec::new();
//     for obj in objs {
//         match intersection_ray(ray, obj) {
//             Some(mut p) => {
//                 if sort {
//                     all.append(&mut p);
//                 } else {
//                     all.append(&mut p);
//                     break;
//                 }
//             },   
//             _ => {}
//         }
//     }
//     // println!("{:?}", all); 
//     if all.len() > 0 {
//         return Some(all)
//     }
//     None
// }

fn intersection_all<'a>(p1: (f64, f64, f64), p2: (f64, f64, f64), objs: &'a Vec<Obj>, sort: bool) 
    -> Option<Vec<(f64, f64, f64, &'a Obj)>> {
    let mut all = Vec::new();
    for obj in objs {
        // println!("{:?}", obj);
        match intersection(p1, p2, obj) {
            Some(mut p) => {
                if sort {
                    all.append(&mut p);
                } else {
                    all.push(p[0]);
                    break;
                }
            },   
            _ => {}
        }
    }

    if all.len() == 0 {
        return None;
    }

    if all.len() == 1 {
        return Some(all);
    }

    all.sort_by(|a, b| dist((a.0, a.1, a.2), p1).partial_cmp(&dist((b.0, b.1, b.2), p1)).unwrap());
    Some(all)
}

pub fn final_color(ray: &Ray, scene: &Scene) -> Option<Color> {
    let mut d = (ray.a.cos(), ray.a.sin(), ray.b.sin());
    let l = dot(d, d).sqrt();
    d = (d.0 / l, d.1 / l, d.2 / l);
    let p1 = (
        ray.y,
        ray.z,
        ray.x,
    );
    let p2 = (
        ray.y + d.1 * RENDER_DISTANCE,
        ray.z + d.2 * RENDER_DISTANCE,
        ray.x + d.0 * RENDER_DISTANCE,
    );
    if let Some(i) = intersection_all(p1, p2, scene.objs(), true) {
        if let Obj::Sphere(s) = i[0].3 {
            match &scene.lights()[0] {
                Obj::Light(light) => {
                    let light = light.coords();
                    let intersections = match intersection_all(
                        (i[0].0, i[0].1, i[0].2),
                        light,
                        scene.objs(),
                        false,
                    ) {
                        Some(i) => i,
                        None => Vec::new()
                    };
                    if intersections.len() != 0 {
                        return Some(Color::RGBA(0, 0, 0, 255));
                    };
                    return Some(match i[0].3 {
                        Obj::Sphere(s) => s.c,
                        _ => Color::RGBA(0, 0, 0, 255),
                    });
                },
                Obj::Light(l) => return Some(l.c),
                _ => {}
            };
        } else if let Obj::Light(l) = i[0].3 {
            return Some(l.c);
        }
    }
    Some(Color::RGBA(20, 50, 100, 255))
}

fn dot(t: (f64, f64, f64), r: (f64, f64, f64)) -> f64 {
    t.0 * r.0 + t.1 * r.1 + t.2 * r.2
}

fn dist(t: (f64, f64, f64), r: (f64, f64, f64)) -> f64 {
    let d = ((t.0 - r.0).powf(2.0) + (t.1 - r.1).powf(2.0) + (t.2 - r.2).powf(2.0)).sqrt();
    d.max(0.000001)
}
