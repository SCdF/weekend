extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod camera;

use rand::distributions::{IndependentSample, Range};

use vec3::Vec3;
use ray::Ray;
use hitable::Hitable;
use hitable::HitableList;
use hitable::Sphere;
use camera::Camera;

fn random() -> f32 {
    // TODO: work out how make these global / static
    // we shouldn't have to re-let them every time
    let step = Range::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

    step.ind_sample(&mut rng)
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3 {
            x: random(),
            y: random(),
            z: random()
        } - Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0
        };

        if p.squared_length() >= 1.0 {
            break;
        }
    }
    p
}

fn color(ray: &Ray, world: &Hitable) -> Vec3 {
    match world.hit(*ray, 0.001, std::f32::MAX) {
        Some(rec) => {
            let target = rec.p + rec.normal + random_in_unit_sphere();

            0.5 * color(&Ray {
                origin: rec.p,
                direction: target - rec.p
            }, world)
        }
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
    let nx = 400;
    let ny = 200;
    let aa_samples = 100;

    println!("P3\n{} {} \n255\n", nx, ny);

    let cam = Camera {
        lower_left_corner: Vec3 {
            x: -2.0,
            y: -1.0,
            z: -1.0,
        },
        horizontal: Vec3 {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        },
        vertical: Vec3 {
            x: 0.0,
            y: 2.0,
            z: 0.0,
        },
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },

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
            // &Sphere {
            //     center: Vec3 {
            //         x: -1.0,
            //         y: -0.0,
            //         z: -2.0
            //     },
            //     radius: 0.5
            // },
            // &Sphere {
            //     center: Vec3 {
            //         x:  1.0,
            //         y: -0.0,
            //         z: -2.0
            //     },
            //     radius: 0.5
            // },
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
            let mut col = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };

            for _ in 0..aa_samples {
                let u = (i as f32 + random()) / nx as f32;
                let v = (j as f32 + random()) / ny as f32;

                let r = cam.get_ray(u, v);

                col = col + color(&r, world);
            }

            col = col / aa_samples as f32;

            // Gamma 2
            col = Vec3 {
                x: col.x.sqrt(),
                y: col.y.sqrt(),
                z: col.z.sqrt()
            };

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
