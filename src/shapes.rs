use crate::{
    ray::{Ray, RayHit},
    vector::{dot, Vec3},
};

#[derive(Clone, Copy)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let oc = ray.origin - self.center;
        let a = dot(&ray.dir, &ray.dir);
        let b = dot(&oc, &ray.dir);
        let c = dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = (b * b) - (a * c);

        if discriminant > 0.0 {
            let mut ray_hit = RayHit::default();

            let temp = (-b - f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                ray_hit.t = temp;
                ray_hit.point = ray.point_at_parameter(ray_hit.t);
                ray_hit.normal = (ray_hit.point - self.center) / self.radius;
                return Some(ray_hit);
            }
            let temp = (-b + f32::sqrt(discriminant)) / a;
            if temp < t_max && temp > t_min {
                ray_hit.t = temp;
                ray_hit.point = ray.point_at_parameter(ray_hit.t);
                ray_hit.normal = (ray_hit.point - self.center) / self.radius;
                return Some(ray_hit);
            }
        }
        None
    }
}
