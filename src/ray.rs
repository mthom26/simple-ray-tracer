use std::sync::Arc;

use crate::material::Material;
use crate::vector::Vec3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3, time: f32) -> Self {
        Ray { origin, dir, time }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin.clone() + (t * self.dir.clone())
    }
}

#[derive(Clone)]
pub struct RayHit {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
}

impl RayHit {
    pub fn new(t: f32, u: f32, v: f32, point: Vec3, normal: Vec3, mat: Arc<dyn Material>) -> Self {
        RayHit {
            t,
            u,
            v,
            point,
            normal,
            mat,
        }
    }
}
