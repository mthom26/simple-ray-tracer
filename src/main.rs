use std::{fs::File, io::Write};

mod vector;
use vector::Vec3;
mod ray;

fn main() {
    let x = 200;
    let y = 100;

    let mut output = File::create("output/output.ppm").unwrap();
    let header = format!("P3\n{} {}\n255\n", x, y);
    output.write_all(header.as_bytes()).unwrap();

    for i in 0..y {
        for j in 0..x {
            let vec = Vec3::new(j as f32 / x as f32, i as f32 / y as f32, 0.2);

            let r = (vec.x * 255.0) as usize;
            let g = (vec.y * 255.0) as usize;
            let b = (vec.z * 255.0) as usize;

            let pixel = format!("{} {} {}\n", r, 255 - g, b);
            output.write_all(pixel.as_bytes()).unwrap();
        }
    }
}
