use ray::Ray;
use vec3::Vec3;

pub struct HitRecord {
    // TODO: this is 't' in the book. Is this actually time?
    time: f32,
    p: Vec3,
    pub normal: Vec3
}

pub trait Hitable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitableList<'a> {
    pub list: Vec<&'a Hitable>
}

impl<'a> Hitable for HitableList<'a> {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut best: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hitable in self.list.iter() {
            match hitable.hit(r, t_min, closest_so_far) {
                Some(hit) => {
                    closest_so_far = hit.time;
                    // TODO: can we get a reference to the wrapping object so
                    // we don't have to create another one?
                    best = Some(hit);
                }
                _ => ()
            }
        }

        best
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
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
                    time: temp,
                    p: p,
                    normal: (p - self.center) / self.radius
                })
            }

            let temp2 = (-b - (b * b + a * c).sqrt()) / a;
            if temp2 < t_max && temp2 > t_min {
                let p = r.point_at_parameter(temp2);

                return Some(HitRecord {
                    time: temp2,
                    p: p,
                    normal: (p - self.center) / self.radius
                })
            }

            None
        } else {
            None
        }
    }
}
