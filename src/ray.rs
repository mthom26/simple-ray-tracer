use std::default::Default;

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

pub struct RayHit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

impl RayHit {
    pub fn new(t: f32, point: Vec3, normal: Vec3) -> Self {
        RayHit { t, point, normal }
    }
}

impl Default for RayHit {
    fn default() -> Self {
        RayHit {
            t: 0.0,
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
