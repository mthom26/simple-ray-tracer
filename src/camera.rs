use std::f32::consts::PI;

use crate::{
    ray::Ray,
    utils::{gen_random, random_in_unit_disk},
    vector::{cross, Vec3},
};

pub struct Camera {
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    time_0: f32,
    time_1: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up_dir: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
        time_0: f32,
        time_1: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = (look_from - look_at).get_unit();
        let u = cross(&up_dir, &w).get_unit();
        let v = cross(&w, &u);

        let lower_left =
            look_from - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;
        let origin = look_from;

        Camera {
            lower_left,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
            lens_radius,
            time_0,
            time_1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let dir =
            self.lower_left + (s * self.horizontal) + (t * self.vertical) - self.origin - offset;

        let time = self.time_0 + gen_random() * (self.time_1 - self.time_0);
        Ray::new(self.origin + offset, dir, time)
    }
}
