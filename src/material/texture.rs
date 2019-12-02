use std::sync::Arc;

use crate::{material::Perlin, vector::Vec3};

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

#[derive(Clone)]
pub struct Noise {
    noise: Perlin,
    scale: f32,
}

impl Noise {
    pub fn new(scale: f32) -> Self {
        Noise {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        // Vec3::new(1.0, 1.0, 1.0) * self.noise.noise(p * self.scale)
        // Vec3::new(1.0, 1.0, 1.0) * self.noise.turb(p * self.scale, 7)
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f32::sin(self.scale * p.x + 5.0 * self.noise.turb(p, 7)))
    }
}

#[derive(Clone)]
pub struct Image {
    data: Vec<u8>,
    nx: usize,
    ny: usize,
}

impl Image {
    pub fn new(data: Vec<u8>, nx: usize, ny: usize) -> Self {
        Image { data, nx, ny }
    }
}

impl Texture for Image {
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Vec3 {
        let (nx, ny) = (self.nx, self.ny);
        let mut i = (u * nx as f32) as usize;
        let mut j = ((1.0 - v) * ny as f32) as usize;
        if i > (nx - 1) {
            i = nx - 1;
        }
        if j > (ny - 1) {
            j = ny - 1;
        }

        let ri = 3 * i + 3 * nx * j;
        let gi = 3 * i + 3 * nx * j + 1;
        let bi = 3 * i + 3 * nx * j + 2;
        let r = self.data[ri] as f32 / 255.0;
        let g = self.data[gi] as f32 / 255.0;
        let b = self.data[bi] as f32 / 255.0;

        Vec3::new(r, g, b)
    }
}
