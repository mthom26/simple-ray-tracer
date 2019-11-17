use crate::vector::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin.clone() + (t * self.dir.clone())
    }
}
