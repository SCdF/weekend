use vec3::Vec3;

use rand::{thread_rng, Rng};

pub fn random() -> f32 {
    thread_rng().gen_range(0.0, 1.0)
}

pub fn random_in_unit_sphere() -> Vec3 {
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
