use std::{
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

mod vector;
use vector::{dot, Vec3};
mod ray;
use ray::Ray;

fn color(ray: Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray);

    if t > 0.0 {
        let vector = ray.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0);
        let unit = vector.get_unit();
        return Vec3::new(unit.x + 1.0, unit.y + 1.0, unit.z + 1.0) * 0.5;
    }

    let unit_dir = ray.dir.get_unit();
    // Interpolate along y axis
    let t = (unit_dir.y + 1.0) * 0.5;
    ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin.clone() - center;
    let a = dot(&ray.dir, &ray.dir);
    let b = 2.0 * dot(&oc, &ray.dir);
    let c = dot(&oc, &oc) - (radius * radius);
    let discriminant = (b * b) - (4.0 * a * c);

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - f32::sqrt(discriminant)) / (2.0 * a);
    }
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

    for i in 0..y {
        for j in 0..x {
            let u = j as f32 / x as f32;
            let v = 1.0 - (i as f32 / y as f32);

            let ray = Ray::new(
                origin.clone(),
                lower_left.clone() + (u * horizontal.clone()) + (v * vertical.clone()),
            );
            let col = color(ray);

            let r = (col.x * 255.0) as usize;
            let g = (col.y * 255.0) as usize;
            let b = (col.z * 255.0) as usize;

            let pixel = format!("{} {} {}\n", r, g, b);
            output.write_all(pixel.as_bytes()).unwrap();
        }
    }
}
