use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
// TODO refactor Vec3 into just a tuple.
// xyz doesn't always make sense, let's not name them, it makes the code more
// confusing
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }
    // Manual clone function so we have to be explicit about where we want to
    // use it. For now. At some point when we know what we're doing remove this
    // and just derive Clone.
    pub fn clone(v: &Vec3) -> Vec3 {
        Vec3 { x: v.x, y: v.y, z: v.z }
    }
    // TODO: read about dot product
    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)
    }

    pub fn squared_length(&self) -> f32 {
        Vec3::dot(&self, &self)
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    // TODO: understand the difference between make_unit_vector and unit_vector
    pub fn make_unit_vector(&self) -> Vec3 {
        let k = 1.0 / self.length();
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    // TODO: read about cross product
    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
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
impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
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
impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
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
impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
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
impl Mul<f32> for &Vec3 {
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

impl Mul<&Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: &Vec3) -> Vec3 {
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
impl Div for &Vec3 {
    type Output = Vec3;

    fn div(self, other: &Vec3) -> Vec3 {
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
impl Div<f32> for &Vec3 {
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
