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
use shapes::Sphere;

fn color_sphere(ray: Ray, sphere: Sphere) -> Vec3 {
    match sphere.hit(&ray, 0.0, MAX) {
        Some(hit) => Vec3::new(hit.normal.x + 1.0, hit.normal.y + 1.0, hit.normal.z + 1.0) * 0.5,
        None => panic!("This function shouldn't be called if the ray doesn't hit."),
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

    if !Path::new("output").is_dir() {
        create_dir("output").unwrap();
    }

    let mut output = File::create("output/output.ppm").unwrap();
    let header = format!("P3\n{} {}\n255\n", x, y);
    output.write_all(header.as_bytes()).unwrap();

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let t_min = 0.0;
    let t_max = MAX;

    let spheres = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    for i in 0..y {
        for j in 0..x {
            let u = j as f32 / x as f32;
            let v = 1.0 - (i as f32 / y as f32);

            let ray = Ray::new(
                origin.clone(),
                lower_left.clone() + (u * horizontal.clone()) + (v * vertical.clone()),
            );

            let mut did_hit = false;
            let mut closest = t_max;
            let mut target_index = 0;

            // Find closest hit point in 'spheres'
            for (index, sphere) in spheres.iter().enumerate() {
                match sphere.hit(&ray, t_min, closest) {
                    Some(hit) => {
                        did_hit = true;
                        closest = hit.t;
                        target_index = index;
                    }
                    None => {}
                }
            }

            let col = match did_hit {
                true => color_sphere(ray, spheres[target_index]),
                false => color_background(ray),
            };

            let r = (col.x * 255.0) as usize;
            let g = (col.y * 255.0) as usize;
            let b = (col.z * 255.0) as usize;

            let pixel = format!("{} {} {}\n", r, g, b);
            output.write_all(pixel.as_bytes()).unwrap();
        }
    }
}
