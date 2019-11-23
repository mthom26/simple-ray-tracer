use std::default::Default;

use crate::{
    ray::{Ray, RayHit},
    utils::random_in_unit_sphere,
    vector::Vec3,
};

pub trait Material {
    fn scatter(&self, ray: Ray, hit: RayHit) -> Option<(Vec3, Ray)>;
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hit: RayHit) -> Option<(Vec3, Ray)> {
        let target = hit.point + hit.normal + random_in_unit_sphere();
        let new_ray = Ray::new(hit.point, target - hit.point);
        Some((self.albedo, new_ray))
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Lambertian {
            albedo: Vec3::new(0.5, 0.5, 0.5),
        }
    }
}
