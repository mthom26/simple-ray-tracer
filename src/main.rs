use indicatif::{ProgressBar, ProgressStyle};
use rand::{thread_rng, Rng};
use std::{
    f32::MAX,
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

mod vector;
use vector::Vec3;
mod ray;
use ray::Ray;
mod shapes;
use shapes::{Hittable, Sphere};
mod camera;
use camera::Camera;

fn color(ray: Ray, world: &dyn Hittable) -> Vec3 {
    match world.hit(&ray, 0.001, MAX) {
        Some(hit) => {
            let target = hit.point + hit.normal + random_in_unit_sphere();
            let new_ray = Ray::new(hit.point, target - hit.point);
            color(new_ray, world) * 0.5
        }
        None => color_background(ray),
    }
}

fn color_background(ray: Ray) -> Vec3 {
    let unit_dir = ray.dir.get_unit();
    // Interpolate along y axis
    let t = (unit_dir.y + 1.0) * 0.5;
    ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
    let x = 200;
    let y = 100;
    let s = 100;

    if !Path::new("output").is_dir() {
        create_dir("output").unwrap();
    }

    let mut output = File::create("output/output.ppm").unwrap();
    let header = format!("P3\n{} {}\n255\n", x, y);
    output.write_all(header.as_bytes()).unwrap();

    // Setup progress indicator
    let progress = initialise_progress_indicator(y);

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let cam = Camera::new(lower_left, horizontal, vertical, origin);

    for i in 0..y {
        for j in 0..x {
            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..s {
                let u = (j as f32 + gen_random()) / x as f32;
                let v = 1.0 - ((i as f32 + gen_random()) / y as f32);

                let ray = cam.get_ray(u, v);

                col += color(ray, &world);
            }

            col /= s as f32;

            // Gamma correction
            col.x = col.x.sqrt();
            col.y = col.y.sqrt();
            col.z = col.z.sqrt();

            let r = (col.x * 255.0) as usize;
            let g = (col.y * 255.0) as usize;
            let b = (col.z * 255.0) as usize;

            let pixel = format!("{} {} {}\n", r, g, b);
            output.write_all(pixel.as_bytes()).unwrap();
        }
        progress.inc(1);
    }
    progress.finish_with_message("Finished!");
}

fn gen_random() -> f32 {
    // Return random number between 0.0 and 1.0
    thread_rng().gen()
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p =
        (2.0 * Vec3::new(gen_random(), gen_random(), gen_random())) - Vec3::new(1.0, 1.0, 1.0);
    while (p.x.powi(2) + p.y.powi(2) + p.z.powi(2)) >= 1.0 {
        p = (2.0 * Vec3::new(gen_random(), gen_random(), gen_random())) - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn initialise_progress_indicator(steps: u64) -> ProgressBar {
    let progress_style = ProgressStyle::default_bar()
        .template("{msg} {bar:80.green/white} {pos:>4}/{len} [{elapsed}]")
        .progress_chars("=>-");
    let progress = ProgressBar::new(steps);
    progress.set_style(progress_style);
    progress.set_message("Generating image...");
    progress
}
