use crate::{
    utils::{gen_random, gen_range},
    vector::{dot, Vec3},
};

#[derive(Clone)]
pub struct Perlin {
    ran_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            ran_vec: generate(),
            perm_x: generate_perm(),
            perm_y: generate_perm(),
            perm_z: generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        // let u = u * u * (3.0 - 2.0 * u);
        // let v = v * v * (3.0 - 2.0 * v);
        // let w = w * w * (3.0 - 2.0 * w);

        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;

        let mut arr = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for ii in 0..2 {
            for jj in 0..2 {
                for kk in 0..2 {
                    arr[ii][jj][kk] = self.ran_vec[self.perm_x[(i + ii) & 255]
                        ^ self.perm_y[(j + jj) & 255]
                        ^ self.perm_z[(k + kk) & 255]];
                }
            }
        }

        interpolate(&arr, u, v, w)
        // let i = ((p.x * 4.0) as i32 & 255) as usize;
        // let j = ((p.y * 4.0) as i32 & 255) as usize;
        // let k = ((p.z * 4.0) as i32 & 255) as usize;

        // self.ran_float[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k])]
    }

    pub fn turb(&self, p: Vec3, depth: usize) -> f32 {
        let mut tmp = p;
        let mut accum = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            tmp *= 2.0;
        }

        accum.abs()
    }
}

fn generate() -> Vec<Vec3> {
    let mut v = Vec::with_capacity(256);
    for _ in 0..256 {
        let x_rand = gen_random() * 2.0 - 1.0;
        let y_rand = gen_random() * 2.0 - 1.0;
        let z_rand = gen_random() * 2.0 - 1.0;
        v.push(Vec3::new(x_rand, y_rand, z_rand).get_unit());
    }
    v
}

fn permute(p: &mut Vec<usize>, n: usize) {
    for i in (0..n).rev() {
        let target = gen_range(0.0, (i + 1) as f32);
        p.swap(i, target as usize);
    }
}

fn generate_perm() -> Vec<usize> {
    let mut v = Vec::with_capacity(256);
    for i in 0..256 {
        v.push(i);
    }
    permute(&mut v, 256);
    v
}

fn interpolate(arr: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);

    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let (ii, jj, kk) = (i as f32, j as f32, k as f32);

                let weight = Vec3::new(u - ii, v - jj, w - kk);
                let a = (ii * uu) + (1.0 - ii) * (1.0 - uu);
                let b = (jj * vv) + (1.0 - jj) * (1.0 - vv);
                let c = (kk * ww) + (1.0 - kk) * (1.0 - ww);
                let d = dot(&arr[i][j][k], &weight);

                accum += a * b * c * d;
            }
        }
    }

    accum
}
