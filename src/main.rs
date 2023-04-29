use std::fs::*;
use std::io::*;

mod vec3; // Magically link vec3.rs to this module by name at compile time.
use vec3::Vec3;

const IMAGE_HEIGHT:i32  = 256;
const IMAGE_WIDTH: i32 = 256;

/// Converts the 0 -> 1 f32 values of a Vec3 into a usable PPM pixel string.
fn make_pixel(pixel_color: Vec3) -> String {
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

            image.push_str(&make_pixel(Vec3::new(r, g, b)))
        }
    }

    image
}

fn main() {
    let image = generate_image();
    let mut file = File::create("image3.ppm").unwrap();
    file.write_all(image.as_bytes()).unwrap();
}
