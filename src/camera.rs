use std::f32::consts::PI;

use crate::{ray::Ray, vector::Vec3};

pub struct Camera {
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect: f32) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let lower_left = Vec3::new(-half_width, -half_height, -1.0);
        let horizontal = Vec3::new(2.0 * half_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0 * half_height, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);

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
