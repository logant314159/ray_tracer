mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use std::fs::*;
use std::io::*;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

fn color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();

    if world.hit(&r, 0.0, std::f32::MAX, &mut rec) {
        0.5 * Vec3::new(
                rec.normal().x() + 1.0,
                rec.normal().y() + 1.0,
                rec.normal().z() + 1.0,
            )
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn render(world: HittableList) {
    let w = 200;
    let h = 100;
    let max_value = 255;

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut image_file = File::create("image.ppm").unwrap();
    image_file.write_all(format!("P3\n{} {}\n{}\n", w, h, max_value).as_bytes()).unwrap();

    for j in (0..h).rev() {
        for i in 0..w {
            let u = i as f32 / w as f32;
            let v = j as f32 / h as f32;

            let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let p = r.point_at_parameter(2.0);
            let col = color(&r, &world);

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            image_file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).unwrap();
        }
    }
}


fn main() {
    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let world = HittableList::new(list);

    render(world);
}
