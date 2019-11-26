use std::f32::consts::PI;

use crate::{
    ray::Ray,
    vector::{cross, Vec3},
};

pub struct Camera {
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, up_dir: Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).get_unit();
        let u = cross(&up_dir, &w).get_unit();
        let v = cross(&w, &u);

        let lower_left = look_from - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;
        let origin = look_from;

        Camera {
            lower_left,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let dir = self.lower_left + (u * self.horizontal) + (v * self.vertical) - self.origin;
        Ray::new(self.origin, dir)
    }
}
