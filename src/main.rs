use std::fs::*;
use std::io::*;

mod vec3; // Magically link vec3.rs to this module by name at compile time.
use vec3::Vec3;

type Point3 = Vec3;
type Color = Vec3;

const IMAGE_HEIGHT:i32  = 256;
const IMAGE_WIDTH: i32 = 256;

/// Converts the 0 -> 1 f32 values of a Vec3 into a usable PPM pixel string.
fn make_pixel(pixel_color: Color) -> String {
    format!("{} {} {}\n", 255.99 * pixel_color.x(), 255.99 * pixel_color.y(), 255.99 * pixel_color.z())
}

fn generate_image() -> String {
    let mut image = String::new();
    image.push_str(&format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("{}[2J", 27 as char); // Clear screen. (ANSI escape code.)
        print!("\rScanlines remaining: {} ", j);

        for i in 0..IMAGE_WIDTH {
            let r = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.25;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            image.push_str(&format!("{} {} {}\n", ir, ig, ib))
        }
    }

    image
}

fn main() {
    let image = generate_image();
    let mut file = File::create("image2.ppm").unwrap();
    file.write_all(image.as_bytes()).unwrap();
}
