use indicatif::{ProgressBar, ProgressStyle};

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
use shapes::Hittable;
mod camera;
mod material;
mod utils;
use utils::gen_random;
mod scene;
use scene::load_scene;
mod config;
use config::get_config;

fn color(ray: Ray, world: &dyn Hittable, depth: usize) -> Vec3 {
    match world.hit(&ray, 0.001, MAX) {
        Some(hit) => {
            if depth < 50 {
                if let Some((att, scattered)) = hit.mat.scatter(ray, hit.clone()) {
                    att * color(scattered, world, depth + 1)
                } else {
                    Vec3::new(0.0, 0.0, 0.0)
                }
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
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
    let (x, y, scene) = get_config();
    let s: u64 = 100;

    if !Path::new("output").is_dir() {
        create_dir("output").unwrap();
    }

    let mut output = File::create("output/output.ppm").unwrap();
    let header = format!("P3\n{} {}\n255\n", x, y);
    output.write_all(header.as_bytes()).unwrap();

    // Setup progress indicator
    let progress = initialise_progress_indicator(y);

    // Get Scene
    let (cam, world) = load_scene(scene, x, y);

    for i in 0..y {
        for j in 0..x {
            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..s {
                let u = (j as f32 + gen_random()) / x as f32;
                let v = 1.0 - ((i as f32 + gen_random()) / y as f32);

                let ray = cam.get_ray(u, v);

                col += color(ray, &world, 0);
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

fn initialise_progress_indicator(steps: u64) -> ProgressBar {
    let progress_style = ProgressStyle::default_bar()
        .template("{msg} {bar:80.green/white} {pos:>4}/{len} [{elapsed}]")
        .progress_chars("=>-");
    let progress = ProgressBar::new(steps);
    progress.set_style(progress_style);
    progress.set_message("Generating image...");
    progress
}
