use std::sync::Arc;

use crate::{
    material::Material,
    ray::{Ray, RayHit},
    vector::{dot, Vec3},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit>;
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let mut hit: Option<RayHit> = None;

        for object in self.iter() {
            if let Some(object_hit) = object.hit(ray, t_min, t_max) {
                match hit.clone() {
                    // Check if the new hit is closer than the previous hit
                    Some(prev) => {
                        if object_hit.t < prev.t {
                            hit = Some(object_hit);
                        }
                    }
                    None => {
                        hit = Some(object_hit);
                    }
                }
            }
        }

        hit
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, mat: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let ray = ray.clone();
        let oc = ray.origin - self.center;
        let a = dot(&ray.dir, &ray.dir);
        let b = dot(&oc, &ray.dir);
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                return create_ray_hit(temp, &ray, self.center, self.radius, &self.mat);
            }
            let temp = (-b + f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                return create_ray_hit(temp, &ray, self.center, self.radius, &self.mat);
            }
        }
        None
    }
}

fn create_ray_hit(
    temp: f32,
    ray: &Ray,
    center: Vec3,
    radius: f32,
    mat: &Arc<dyn Material>,
) -> Option<RayHit> {
    let ray_hit = RayHit::new(
        temp,
        ray.point_at_parameter(temp),
        (ray.point_at_parameter(temp) - center) / radius,
        Arc::clone(mat),
    );
    Some(ray_hit)
}
