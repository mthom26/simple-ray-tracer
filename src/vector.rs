use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
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

    pub fn get_mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
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

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
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

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

// Return dot product of two Vec3 vectors
pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
    (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
}

// Return cross product of two Vec3 vectors
pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    let x = a.y * b.z - a.z * b.y;
    let y = -(a.x * b.z - a.z * b.x);
    let z = a.x * b.y - a.y * b.x;
    Vec3::new(x, y, z)
}

#[cfg(test)]
mod tests {
    use crate::vector::{cross, Vec3};

    #[test]
    fn addition() {
        let mut a = Vec3::new(6.0, 4.0, 2.0);
        let b = Vec3::new(1.0, 1.0, 1.0);
        a += b;
        let expected = Vec3::new(7.0, 5.0, 3.0);
        assert_eq!(a, expected);
    }

    #[test]
    fn division() {
        let mut a = Vec3::new(6.0, 4.0, 2.0);
        a /= 2.0;
        let expected = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(a, expected);
    }

    #[test]
    fn cross_product() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = cross(&a, &b);
        assert_eq!(0.0, c.x);
        assert_eq!(0.0, c.y);
        assert_eq!(1.0, c.z);

        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let c = cross(&a, &b);
        assert_eq!(-3.0, c.x);
        assert_eq!(6.0, c.y);
        assert_eq!(-3.0, c.z);
    }
}
