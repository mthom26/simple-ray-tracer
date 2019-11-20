use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn get_unit(&self) -> Vec3 {
        let mag = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        Vec3::new(self.x / mag, self.y / mag, self.z / mag)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        Vec3::new(self.x + other, self.y + other, self.z + other)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}
impl Sub<f32> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f32) -> Vec3 {
        Vec3::new(self.x - other, self.y - other, self.z - other)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}

// Return dot product of two Vec3 vectors
pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}
