use rand::{thread_rng, Rng};

use crate::vector::Vec3;

pub fn gen_random() -> f32 {
    // Return random number between 0.0 and 1.0
    thread_rng().gen()
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p =
        (2.0 * Vec3::new(gen_random(), gen_random(), gen_random())) - Vec3::new(1.0, 1.0, 1.0);
    while (p.x.powi(2) + p.y.powi(2) + p.z.powi(2)) >= 1.0 {
        p = (2.0 * Vec3::new(gen_random(), gen_random(), gen_random())) - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}
