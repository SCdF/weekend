mod vec3;
mod ray;
mod hitable;

use vec3::Vec3;
use ray::Ray;
use hitable::Hitable;
use hitable::HitableList;
use hitable::Sphere;

fn color(ray: &Ray, world: &Hitable) -> Vec3 {
    match world.hit(*ray, 0.0, std::f32::MAX) {
        Some(rec) =>
            0.5 * Vec3 {
                x: rec.normal.x + 1.0,
                y: rec.normal.y + 1.0,
                z: rec.normal.z + 1.0,
            },
        None => {
            // Sky
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            let white = Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            };
            let blue = Vec3 {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            };

            (1.0 - t) * white + t * blue
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {} \n255\n", nx, ny);
    let lower_left_corner = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };
    let horizontal = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };
    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let world = &HitableList {
        list: vec![
            &Sphere {
                center: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0
                },
                radius: 0.5
            },
            &Sphere {
                center: Vec3 {
                    x: 0.0,
                    y: -100.5,
                    z: -1.0
                },
                radius: 100.0
            }
        ]
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };

            // eprintln!("{}\t{}\t{:?}", u, v, r);
            let col = color(&r, world);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
