use std::sync::Arc;

use crate::{
    camera::Camera,
    material::{Checkered, Dielectric, Lambertian, Metal, Noise, SolidColor},
    shapes::{Hittable, MSphere, Sphere},
    utils::gen_random,
    vector::Vec3,
};

pub fn load_scene(scene_name: String, x: u64, y: u64) -> (Camera, Vec<Box<dyn Hittable>>) {
    match &scene_name[..] {
        "default" => default_scene(x, y),
        "spheres" => spheres_scene(x, y),
        "motion" => motion_blur(x, y),
        "textures" => textures_scene(x, y),
        "perlin" => perlin_scene(x, y),
        _ => panic!("Could not load scene."),
    }
}

fn default_scene(x: u64, y: u64) -> (Camera, Vec<Box<dyn Hittable>>) {
    // Materials
    let mat_one = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.8, 0.3, 0.2))));
    let mat_two = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.5, 0.4, 0.1))));
    let mat_three = Arc::new(Metal::new(Arc::new(SolidColor::new(0.3, 0.2, 0.8)), 0.1));
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
    let cam = Camera::new(
        from,
        to,
        up,
        50.0,
        aspect_ratio,
        aperture,
        focus_dist,
        0.0,
        0.0,
    );

    (cam, world)
}

fn spheres_scene(x: u64, y: u64) -> (Camera, Vec<Box<dyn Hittable>>) {
    let ground_mat = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.8, 0.3, 0.2))));
    let mut world: Vec<Box<dyn Hittable>> = vec![Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_mat,
    ))];

    let collisions = vec![
        (Vec3::new(-3.0, 0.6, 1.5), 0.6),
        (Vec3::new(-4.0, 1.4, -2.0), 1.4),
        (Vec3::new(2.0, 0.5, -2.0), 0.5),
    ];

    let mat_one = Arc::new(Metal::new(Arc::new(SolidColor::new(0.3, 0.2, 0.8)), 0.05));
    let mat_two = Arc::new(Metal::new(Arc::new(SolidColor::new(0.6, 0.9, 0.6)), 0.2));
    let mat_three = Arc::new(Dielectric::new(1.5));
    let sphere_one = Box::new(Sphere::new(collisions[0].0, collisions[0].1, mat_one));
    let big_boi = Box::new(Sphere::new(collisions[1].0, collisions[1].1, mat_two));
    let sphere_three = Box::new(Sphere::new(collisions[2].0, collisions[2].1, mat_three));
    world.push(sphere_one);
    world.push(big_boi);
    world.push(sphere_three);

    // Convenience function to make sure a new postion and radius won't collide
    // with the given Vec of spheres
    fn check_for_collision(center: Vec3, radius: f32, collisions: &Vec<(Vec3, f32)>) -> bool {
        for sphere in collisions.into_iter() {
            let a = (sphere.0 - center).get_mag();
            let b = sphere.1 + radius;
            if a < b {
                return true;
            }
        }
        false
    }

    for x in -11..11 {
        for z in -11..11 {
            let radius = gen_random() / 5.0;
            let mat_chance = gen_random();

            let center = Vec3::new(
                x as f32 + 0.25 + gen_random() / 2.0,
                radius,
                z as f32 + 0.25 + gen_random() / 2.0,
            );

            if check_for_collision(center, radius, &collisions) {
                continue;
            }
            if gen_random() < 0.8 {
                match mat_chance {
                    val if val < 0.6 => {
                        // Diffuse
                        let mat = Arc::new(Lambertian::new(Arc::new(SolidColor::new(
                            gen_random(),
                            gen_random(),
                            gen_random(),
                        ))));
                        let sphere = Box::new(Sphere::new(center, radius, mat));
                        world.push(sphere);
                    }
                    val if val < 0.85 => {
                        // Metallic
                        let mat = Arc::new(Metal::new(
                            Arc::new(SolidColor::new(gen_random(), gen_random(), gen_random())),
                            gen_random(),
                        ));
                        let sphere = Box::new(Sphere::new(center, radius, mat));
                        world.push(sphere);
                    }
                    _ => {
                        // Glass
                        let mat = Arc::new(Dielectric::new(gen_random()));
                        let sphere = Box::new(Sphere::new(center, radius, mat));
                        world.push(sphere);
                    }
                }
            }
        }
    }

    let aspect_ratio = x as f32 / y as f32;
    let from = Vec3::new(3.0, 1.5, 2.0);
    let to = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.05;
    let focus_dist = (from - to).get_mag();
    let cam = Camera::new(
        from,
        to,
        up,
        50.0,
        aspect_ratio,
        aperture,
        focus_dist,
        0.0,
        0.0,
    );

    (cam, world)
}

fn motion_blur(x: u64, y: u64) -> (Camera, Vec<Box<dyn Hittable>>) {
    let ground_mat = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.5, 0.5, 0.5))));
    let mat_one = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.8, 0.2, 0.2))));
    let mat_two = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.2, 0.8, 0.2))));
    let mat_three = Arc::new(Lambertian::new(Arc::new(SolidColor::new(0.2, 0.2, 0.8))));

    let sphere_two = Box::new(MSphere::new(
        Vec3::new(0.0, 0.75, -1.0),
        Vec3::new(0.0, 0.5, -1.0),
        0.5,
        0.0,
        1.0,
        mat_two,
    ));
    let sphere_three = Box::new(MSphere::new(
        Vec3::new(1.25, 1.0, -1.0),
        Vec3::new(1.25, 0.5, -1.0),
        0.5,
        0.0,
        1.0,
        mat_three,
    ));

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, -500.0, -1.0), 500.0, ground_mat)),
        Box::new(Sphere::new(Vec3::new(-1.25, 0.5, -1.0), 0.5, mat_one)),
        sphere_two,
        sphere_three,
    ];

    let aspect_ratio = x as f32 / y as f32;
    let from = Vec3::new(0.0, 0.5, 2.0);
    let to = Vec3::new(0.0, 0.3, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.0;
    let focus_dist = (from - to).get_mag();
    let cam = Camera::new(
        from,
        to,
        up,
        70.0,
        aspect_ratio,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );

    (cam, world)
}

fn textures_scene(x: u64, y: u64) -> (Camera, Vec<Box<dyn Hittable>>) {
    let ground_mat = Arc::new(Lambertian::new(Arc::new(Checkered::new(
        Arc::new(SolidColor::new(0.35, 0.35, 0.45)),
        Arc::new(SolidColor::new(0.5, 0.5, 0.6)),
    ))));
    let mat_metal = Arc::new(Metal::new(Arc::new(SolidColor::new(0.8, 0.8, 0.8)), 0.0));
    let check_metal = Arc::new(Metal::new(
        Arc::new(Checkered::new(
            Arc::new(SolidColor::new(0.8, 0.2, 0.2)),
            Arc::new(SolidColor::new(0.2, 0.8, 0.2)),
        )),
        0.0,
    ));
    let check_lam = Arc::new(Lambertian::new(Arc::new(Checkered::new(
        Arc::new(SolidColor::new(0.0, 0.0, 0.0)),
        Arc::new(SolidColor::new(1.0, 1.0, 1.0)),
    ))));

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, -500.0, -1.0), 500.0, ground_mat)),
        Box::new(Sphere::new(Vec3::new(0.0, 0.8, -1.2), 0.8, mat_metal)),
        Box::new(Sphere::new(Vec3::new(1.8, 0.5, -0.8), 0.5, check_metal)),
        Box::new(Sphere::new(Vec3::new(-1.8, 0.5, -0.8), 0.5, check_lam)),
    ];

    // Camera setup
    let aspect_ratio = x as f32 / y as f32;
    let from = Vec3::new(0.0, 1.0, 1.5);
    let to = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.0;
    let focus_dist = (from - to).get_mag();
    let cam = Camera::new(
        from,
        to,
        up,
        70.0,
        aspect_ratio,
        aperture,
        focus_dist,
        0.0,
        0.0,
    );

    (cam, world)
}

// This perlin noise texture doesn't look like the in the book, skipping it for now...
fn perlin_scene(x: u64, y: u64) -> (Camera, Vec<Box<dyn Hittable>>) {
    let mat_one = Arc::new(Lambertian::new(Arc::new(Noise::new(4.0))));
    let mat_two = Arc::new(Lambertian::new(Arc::new(Noise::new(4.0))));

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 2.0, -1.0), 2.0, mat_one)),
        Box::new(Sphere::new(Vec3::new(0.0, -500.0, -1.0), 500.0, mat_two)),
    ];

    // Camera setup
    let aspect_ratio = x as f32 / y as f32;
    let from = Vec3::new(-7.0, 3.2, 1.0);
    let to = Vec3::new(0.0, 0.0, -1.0);
    let up = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.0;
    let focus_dist = (from - to).get_mag();
    let cam = Camera::new(
        from,
        to,
        up,
        60.0,
        aspect_ratio,
        aperture,
        focus_dist,
        0.0,
        0.0,
    );

    (cam, world)
}
