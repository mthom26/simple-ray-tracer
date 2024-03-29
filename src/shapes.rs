use std::{
    f32::consts::{FRAC_PI_2, PI},
    sync::Arc,
};

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

// A moving sphere
#[derive(Clone)]
pub struct MSphere {
    center_0: Vec3,
    center_1: Vec3,
    time_0: f32,
    time_1: f32,
    radius: f32,
    mat: Arc<dyn Material>,
}

impl MSphere {
    pub fn new(
        center_0: Vec3,
        center_1: Vec3,
        radius: f32,
        time_0: f32,
        time_1: f32,
        mat: Arc<dyn Material>,
    ) -> Self {
        MSphere {
            center_0,
            center_1,
            time_0,
            time_1,
            radius,
            mat,
        }
    }

    fn center(&self, time: f32) -> Vec3 {
        let current_time = time - self.time_0;
        let total_time = self.time_1 - self.time_0;
        let total_move = self.center_1 - self.center_0;
        self.center_0 + (current_time / total_time) * total_move
    }
}

impl Hittable for MSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let ray = ray.clone();
        let oc = ray.origin - self.center(ray.time);
        let a = dot(&ray.dir, &ray.dir);
        let b = dot(&oc, &ray.dir);
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        if discriminant > 0.0 {
            let temp = (-b - f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                return create_ray_hit(temp, &ray, self.center(ray.time), self.radius, &self.mat);
            }
            let temp = (-b + f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                return create_ray_hit(temp, &ray, self.center(ray.time), self.radius, &self.mat);
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
    let normal = (ray.point_at_parameter(temp) - center) / radius;
    let (u, v) = get_sphere_uv(normal);
    let ray_hit = RayHit::new(
        temp,
        u,
        v,
        ray.point_at_parameter(temp),
        normal,
        Arc::clone(mat),
    );
    Some(ray_hit)
}

fn get_sphere_uv(p: Vec3) -> (f32, f32) {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();

    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + FRAC_PI_2) / PI;
    (u, v)
}
