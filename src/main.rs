use vec3::Vec3;
use ray::Ray;
mod vec3;
mod ray;

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = Vec3::dot(&r.direction, &r.direction);
    let b = 2.0 * Vec3::dot(&oc, &r.direction);
    let c = Vec3::dot(&oc, &oc) - (radius * radius);
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn color(ray: &Ray) -> Vec3 {
    let sphere_loc = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    let sphere_radius = 0.5;

    if hit_sphere(sphere_loc, sphere_radius, ray) {
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    } else {
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
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };
            // eprintln!("{}\t{}\t{:?}", u, v, r);
            let col = color(&r);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
