mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod camera;
mod material;

use std::fs::*;
use std::io::*;
use rand::prelude::*;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;
use camera::Camera;
use material::{scatter, Material};

fn color(r: &Ray, world: &HittableList, depth: i32) -> Vec3 {
    if let Some(rec) = world.hit(&r, 0.001, std::f32::MAX) {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attenuation = Vec3::default();

        if depth < 50 && scatter(&rec.material, r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, world, depth + 1);
        } else {
            return Vec3::default();
        }
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn render(world: HittableList) {
    let w = 2000;
    let h = 1000;
    let samples = 100;
    let max_value = 255;

    let mut image_file = File::create("image.ppm").unwrap();
    image_file.write_all(format!("P3\n{} {}\n{}\n", w, h, max_value).as_bytes()).unwrap();

    let cam = Camera::camera();
    let mut rng = rand::thread_rng();

    let mut stdout = stdout();
    for j in (0..h).rev() {
        write!(stdout, "\rScanlines remaining: {}", j).unwrap();
        stdout.flush().unwrap();
        for i in 0..w {
            let mut col = Vec3::default();

            for _ in 0..samples {
                let u = (i as f32 + rng.gen::<f32>()) / w as f32;
                let v = (j as f32 + rng.gen::<f32>()) / h as f32;
    
                let r = &cam.get_ray(u, v); //Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
                let p = r.point_at_parameter(2.0);
                col = col + color(&r, &world, 0);
            }

            col = col / samples as f32;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            image_file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).unwrap();
        }
    }
}


fn main() {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, Material::Lambertian{albedo: Vec3::new(0.8, 0.3, 0.3)})));
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -100.5, -1.0), 100.0, Material::Lambertian{albedo: Vec3::new(0.8, 0.8, 0.0)})));
    list.push(Box::new(Sphere::sphere(Vec3::new(1.0, 0.0, -1.0), 0.5, Material::Metal{albedo: Vec3::new(0.8, 0.6, 0.2), fuzz: 1.0})));
    list.push(Box::new(Sphere::sphere(Vec3::new(-1.0, 0.0, -1.0), 0.5, Material::Metal{albedo: Vec3::new(0.8, 0.8, 0.8), fuzz: 0.1})));
    let world = HittableList::new(list);

    render(world);
}
