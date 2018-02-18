use vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        let nv = self.origin + t * self.direction;
        Vec3 {
            x: nv.x,
            y: nv.y,
            z: nv.z,
        }
    }
}
