use crate::{ray::Ray, vector::Vec3};

pub struct Camera {
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(lower_left: Vec3, horizontal: Vec3, vertical: Vec3, origin: Vec3) -> Self {
        Camera {
            lower_left,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let dir = self.lower_left + (u * self.horizontal) + (v * self.vertical);
        Ray::new(self.origin, dir)
    }
}
