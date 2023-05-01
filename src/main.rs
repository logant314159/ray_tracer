use std::fs::*;
use std::io::*;

mod vec3; // Magically link vec3.rs to this module by name at compile time.
mod ray;
use vec3::Vec3;
use ray::Ray;

// Image dimensions.
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

// Camera details.
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;
const CAMERA_ORIGIN: Vec3 = Vec3 { e: [0.0, 0.0, 0.0] };

/// Converts the 0 -> 1 f32 values of a Vec3 into a usable PPM pixel string.
fn make_pixel(pixel_color: Vec3) -> String {
    format!("{} {} {}\n", 255.99 * pixel_color[0], 255.99 * pixel_color[1], 255.99 * pixel_color[2])
}

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - center;
    let a = r.direction.length_squared();
    let half_b = oc.dot(r.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 { -1.0 }
    else { (-half_b - discriminant.sqrt()) / a }
}

/// For a given ray 
fn ray_color(r: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Vec3::new(n.e[0] + 1.0, n.e[1] + 1.0, n.e[2] + 1.0) * 0.5;
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction[1] + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

/// Writes a PPM file to current directory.
fn render(vert: Vec3, horz: Vec3, llc: Vec3) {
    let mut file = File::create("image.ppm").unwrap();
    file.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes()).unwrap();
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f32) / ((IMAGE_WIDTH - 1) as f32);
            let v = (j as f32) / ((IMAGE_HEIGHT - 1) as f32);
            let r = Ray::new(CAMERA_ORIGIN, llc + horz * u + vert * v - CAMERA_ORIGIN);
            let pixel_color = ray_color(&r);
            file.write_all(make_pixel(pixel_color).as_bytes()).unwrap();
        }
    }
}

fn main() {
    // I would love to make these constants but rust doesn't allow for floating-point operations at compile time. Except for ASPECT_RATIO which we won't talk about.
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let lower_left_corner = CAMERA_ORIGIN - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    render(vertical, horizontal, lower_left_corner);
}
