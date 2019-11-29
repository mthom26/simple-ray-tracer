use std::{default::Default, sync::Arc};

use crate::{
    material::{SolidColor, Texture},
    ray::{Ray, RayHit},
    utils::{gen_random, random_in_unit_sphere},
    vector::{dot, Vec3},
};

pub trait Material {
    fn scatter(&self, ray: Ray, hit: RayHit) -> Option<(Vec3, Ray)>;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hit: RayHit) -> Option<(Vec3, Ray)> {
        let target = hit.point + hit.normal + random_in_unit_sphere();
        let new_ray = Ray::new(hit.point, target - hit.point, ray.time);
        Some((self.albedo.value(0.0, 0.0, hit.point), new_ray))
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        let tex = SolidColor::new(0.5, 0.5, 0.5);
        Lambertian {
            albedo: Arc::new(tex),
        }
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Arc<dyn Texture>,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Arc<dyn Texture>, fuzz: f32) -> Self {
        let fuzz = if fuzz > 1.0 { 1.0 } else { fuzz };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: RayHit) -> Option<(Vec3, Ray)> {
        let reflected = reflected(ray.dir.get_unit(), hit.normal);
        let fuzz = self.fuzz * random_in_unit_sphere();
        let new_ray = Ray::new(hit.point, reflected + fuzz, ray.time);
        if dot(&new_ray.dir, &hit.normal) > 0.0 {
            return Some((self.albedo.value(0.0, 0.0, hit.point), new_ray));
        }
        None
    }
}

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit: RayHit) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let (outward_normal, rfx, cosine) = if dot(&ray.dir, &hit.normal) > 0.0 {
            let cosine = self.refractive_index * dot(&ray.dir, &hit.normal) / ray.dir.get_mag();
            (-hit.normal, self.refractive_index, cosine)
        } else {
            let cosine = -dot(&ray.dir, &hit.normal) / ray.dir.get_mag();
            (hit.normal, 1.0 / self.refractive_index, cosine)
        };

        if let Some(refracted) = refracted(ray.dir, outward_normal, rfx) {
            let reflect_prob = schlick(cosine, self.refractive_index);
            if gen_random() >= reflect_prob {
                let refracted_ray = Ray::new(hit.point, refracted, ray.time);
                return Some((attenuation, refracted_ray));
            }
        }

        let reflected_ray = Ray::new(hit.point, reflected(ray.dir, hit.normal), ray.time);
        Some((attenuation, reflected_ray))
    }
}

fn reflected(input: Vec3, normal: Vec3) -> Vec3 {
    input - (2.0 * dot(&input, &normal) * normal)
}

fn refracted(input: Vec3, normal: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let unit = input.get_unit();
    let dt = dot(&unit, &normal);
    let discriminant = 1.0 - (ni_over_nt * ni_over_nt * (1.0 - dt * dt));
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (unit - normal * dt) - normal * (discriminant.sqrt());
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r + (1.0 - r) * (1.0 - cosine).powi(5)
}
