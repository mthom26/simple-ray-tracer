use std::sync::Arc;

use crate::vector::Vec3;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

#[derive(Clone, Copy)]
pub struct SolidColor {
    color: Vec3,
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        SolidColor {
            color: Vec3::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.color
    }
}

#[derive(Clone)]
pub struct Checkered {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl Checkered {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Checkered { odd, even }
    }
}

impl Texture for Checkered {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        match sines {
            val if val < 0.0 => self.odd.value(u, v, p),
            _ => self.even.value(u, v, p),
        }
    }
}
