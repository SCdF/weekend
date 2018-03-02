extern crate rand;

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

fn main() {
    let nx = 400;
    let ny = 200;
    let aa_samples = 100;

    println!("P3\n{} {} \n255\n", nx, ny);

    let cam = Camera {
        lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0)
    };

    let ball = &Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5
    };
    let globe = &Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0
    };

    let world = &HitableList {
        list: vec![ball, globe]
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..aa_samples {
                let u = (i as f32 + random()) / nx as f32;
                let v = (j as f32 + random()) / ny as f32;

                let r = cam.get_ray(u, v);

                col = col + color(&r, world);
            }

            col = col / aa_samples as f32;

            // Gamma 2
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
