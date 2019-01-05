use ray::Ray;
use vec3::Vec3;
use hitable::HitRecord;
use random::random_in_unit_sphere;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
    fn box_clone(&self) -> Box<Material>;
}

pub struct Scatter<'a> {
    pub attenuation: &'a Vec3,
    pub scattered: Ray,
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3
}
impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let target = &hit_record.p + &hit_record.normal + random_in_unit_sphere();

        Some(
            Scatter {
                attenuation: &self.albedo,
                scattered: Ray {
                    origin: Vec3::clone(&hit_record.p),
                    direction: &target - &hit_record.p
                }
            }
        )
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new(Lambertian {
            albedo: Vec3::clone(&self.albedo)
        })
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Vec3
}
impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&r_in.direction.unit_vector(), &hit_record.normal);

        let scattered = Ray {
            origin: Vec3::clone(&hit_record.p),
            direction: reflected
        };

        if Vec3::dot(&scattered.direction, &hit_record.normal) > 0.0 {
            Some(
                Scatter {
                    attenuation: &self.albedo,
                    scattered: scattered
                }
            )
        } else {
            None
        }
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new(Metal {
            albedo: Vec3::clone(&self.albedo)
        })
    }
}
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * Vec3::dot(v, n) * n)
}
