use ray::Ray;
use vec3::Vec3;
use material::*;

pub struct HitRecord {
    t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<Material>
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitableList {
    pub list: Vec<Box<Hitable>>
}

impl Hitable for HitableList {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut best: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        // TODO: look into converting this into a fold
        for hitable in self.list.iter() {
            if let Some(hit) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                // TODO: can we get a reference to the wrapping object so
                // we don't have to create another one?
                best = Some(hit);
            }
        }

        best
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<Material>
}

impl Hitable for Sphere {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = Vec3::dot(&r.direction, &r.direction);
        let b = Vec3::dot(&oc, &r.direction);
        let c = Vec3::dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            // TODO: temp? What is going on here…
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);

                return Some(HitRecord {
                    t: temp,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: self.material.box_clone()
                })
            }

            let temp2 = (-b - (b * b + a * c).sqrt()) / a;
            if temp2 < t_max && temp2 > t_min {
                let p = r.point_at_parameter(temp2);

                return Some(HitRecord {
                    t: temp2,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: self.material.box_clone()
                })
            }

            None
        } else {
            None
        }
    }
}
