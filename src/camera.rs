use vec3::Vec3;
use ray::Ray;

#[derive(Debug)]
pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: Vec3::clone(&self.origin),
            direction: &self.lower_left_corner + &(&(u * &self.horizontal) + &(v * &self.vertical)),
        }
    }
}
