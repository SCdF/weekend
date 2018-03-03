extern crate rand;
extern crate crossbeam_utils;

mod vec3;
mod ray;
mod hitable;
mod camera;

use rand::Rng;

use vec3::Vec3;
use ray::Ray;
use hitable::Hitable;
use hitable::HitableList;
use hitable::Sphere;
use camera::Camera;

fn random() -> f32 {
    rand::thread_rng().gen_range(0.0, 1.0)
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    loop {
        p = 2.0 * Vec3::new(random(), random(), random()) -
                  Vec3::new(1.0, 1.0, 1.0);

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

            0.5 * color(&Ray::new(rec.p, target - rec.p), world)
        }
        None => {
            // Sky
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);

            let white = Vec3::new(1.0, 1.0, 1.0);
            let blue = Vec3::new(0.5, 0.7, 1.0);

            (1.0 - t) * white + t * blue
        }
    }
}

fn xy_from_i(max_x: usize, max_y: usize, i: usize) -> (usize, usize) {
    let y = max_y - (i / max_x) as usize;
    let x = i % max_x;
    (x, y)
}

fn render(t1a: &mut [(i32, i32, i32)], offset: usize, nx: usize, ny: usize, aa_samples: usize, cam: &Camera, world: &HitableList) {
    for idx in 0..t1a.len() {
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        let (x, y) = xy_from_i(nx, ny, offset + idx);

        for _ in 0..aa_samples {
            let u = (x as f32 + random()) / nx as f32;
            let v = (y as f32 + random()) / ny as f32;

            let r = cam.get_ray(u, v);

            col = col + color(&r, world);
        }

        col = col / aa_samples as f32;

        // Gamma 2
        col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

        let ir = (255.99 * col.x) as i32;
        let ig = (255.99 * col.y) as i32;
        let ib = (255.99 * col.z) as i32;
        t1a[idx] = (ir, ig, ib);
    }
}

fn main() {
    let nx = 800;
    let ny = 400;
    let aa_samples = 100;

    println!("P3\n{} {} \n255\n", nx, ny);

    let cam = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0)
    };

    let ball = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5
    };
    let globe = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0
    };

    let mut results = vec!((255, 0, 255); ny * nx);
    {
        let length = ny * nx;
        let (t1a, t3a) = results.split_at_mut(length / 2);
        let (t1a, t2a) = t1a.split_at_mut(length / 4);
        let (t3a, t4a) = t3a.split_at_mut(length / 4);
        crossbeam_utils::scoped::scope(|scope| {
            let t1 = scope.spawn(|| {
                let world = HitableList {
                    list: vec![&ball, &globe]
                };

                render(t1a, (length / 4) * 0, nx, ny, aa_samples, &cam, &world);
            });
            let t2 = scope.spawn(|| {
                let world = HitableList {
                    list: vec![&ball, &globe]
                };

                render(t2a, (length / 4) * 1, nx, ny, aa_samples, &cam, &world);
            });
            let t3 = scope.spawn(|| {
                let world = HitableList {
                    list: vec![&ball, &globe]
                };

                render(t3a, (length / 4) * 2, nx, ny, aa_samples, &cam, &world);
            });
            let t4 = scope.spawn(|| {
                let world = HitableList {
                    list: vec![&ball, &globe]
                };

                render(t4a, (length / 4) * 3, nx, ny, aa_samples, &cam, &world);
            });
            t1.join();
            t2.join();
            t3.join();
            t4.join();
        });
    }

    for (_, data) in results.iter().enumerate() {
           println!("{} {} {}\n", data.0, data.1, data.2);
    }
}
