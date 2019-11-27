use std::sync::Arc;

use crate::{
    camera::Camera,
    material::{Dielectric, Lambertian, Metal},
    shapes::{Hittable, Sphere},
    vector::Vec3,
};

pub fn load_scene(scene_name: String, x: u64, y: u64) -> (Camera, Vec<Box<dyn Hittable>>) {
    match &scene_name[..] {
        "default" => default_scene(x, y),
        _ => panic!("Could not load scene."),
    }
}

fn default_scene(x: u64, y: u64) -> (Camera, Vec<Box<dyn Hittable>>) {
    // Materials
    let mat_one = Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.2)));
    let mat_two = Arc::new(Lambertian::new(Vec3::new(0.5, 0.4, 0.1)));
    let mat_three = Arc::new(Metal::new(Vec3::new(0.3, 0.2, 0.8), 0.1));
    let mat_four = Arc::new(Dielectric::new(1.5));

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat_one)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_two)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_three)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_four)),
    ];

    // Camera setup
    let aspect_ratio = x as f32 / y as f32;
    let from = Vec3::new(-2.0, 2.0, 1.0);
    let to = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.5;
    let focus_dist = (from - to).get_mag();
    let cam = Camera::new(from, to, up, 50.0, aspect_ratio, aperture, focus_dist);

    (cam, world)
}
