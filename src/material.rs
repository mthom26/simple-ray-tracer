use std::default::Default;

use crate::{
    ray::{Ray, RayHit},
    utils::random_in_unit_sphere,
    vector::{dot, Vec3},
};

pub trait Material {
    fn scatter(&self, ray: Ray, hit: RayHit) -> Option<(Vec3, Ray)>;
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
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

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: RayHit) -> Option<(Vec3, Ray)> {
        let reflected = reflected(ray.dir.get_unit(), hit.normal);
        let new_ray = Ray::new(hit.point, reflected);
        if dot(&new_ray.dir, &hit.normal) > 0.0 {
            return Some((self.albedo, new_ray));
        }
        None
    }
}

fn reflected(input: Vec3, normal: Vec3) -> Vec3 {
    input - (2.0 * dot(&input, &normal) * normal)
}
