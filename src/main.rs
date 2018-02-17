// (((((((((((((((((((((((((((((())))))))))))))))))))))))))))))
// To be a module: Vec3
// (((((((((((((((((((((((((((((())))))))))))))))))))))))))))))
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

// TODO: work out the difference between Vec3 and &Vec3: when would I want to use
// one over the other, how should I re-write this code to use those properly
#[allow(dead_code)]
impl Vec3 {
    fn squared_length(&self) -> f32 {
        // TODO: work out how to just call dot
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    // TODO: understand the difference between make_unit_vector and unit_vector
    fn make_unit_vector(&self) -> Vec3 {
        let k = 1.0 / self.length();
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
    // TODO: get this working
    // fn unit_vector(&self) -> Vec3 {
    //     *self / self.length()
    // }

    // TODO: read about dot product
    fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    // TODO: read about cross product
    fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            x: v1.y * v2.z - v1.z * v1.y,
            y: -(v1.x * v2.z - v1.z + v2.x),
            z: v1.x * v2.y - v1.y * v2.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}
impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: vec.x / self,
            y: vec.y / self,
            z: vec.z / self,
        }
    }
}

// (((((((((((((((((((((((((((((())))))))))))))))))))))))))))))
// Main code
fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {} \n255\n", nx, ny);
    for j in 0..ny - 1 {
        for i in 0..nx {
            let col = Vec3 {
                x: i as f32 / nx as f32,
                y: j as f32 / ny as f32,
                z: 0.2
            };
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
