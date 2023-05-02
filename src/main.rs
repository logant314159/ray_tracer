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
    let w = 1000;
    let h = 500;
    let samples = 100;
    let max_value = 255;

    let mut image_file = File::create("image.ppm").unwrap();
    image_file.write_all(format!("P3\n{} {}\n{}\n", w, h, max_value).as_bytes()).unwrap();

    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at= Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 0.1;

    let cam = Camera::camera(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (w as f32) / (h as f32),
        aperture,
        dist_to_focus
    );
    
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

    let mut rng = rand::thread_rng();

    // Set up the world
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Material::Lambertian{albedo: Vec3::new(0.5, 0.5, 0.5)})));;

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    list.push(Box::new(Sphere::sphere(center, 0.2, Material::Lambertian{albedo: Vec3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>())})));
                } else if choose_mat < 0.95 {
                    list.push(Box::new(Sphere::sphere(center, 0.2, Material::Metal{albedo: Vec3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0)), fuzz:  0.5 * rng.gen::<f32>()})))
                } else {
                    list.push(Box::new(Sphere::sphere(center, 0.2, Material::Dielectric{ref_index: 1.5})));
                }
            }
        }
    }

    list.push(Box::new(Sphere::sphere(Vec3::new(4.0, 1.0, -1.0), 1.0, Material::Dielectric{ref_index: 1.5})));
    list.push(Box::new(Sphere::sphere(Vec3::new(-4.0, 1.0, -1.0), 1.0, Material::Lambertian{albedo: Vec3::new(0.4, 0.2, 0.1)})));
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 1.0, -1.0), 1.0, Material::Metal{albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.3})));
    let world = HittableList::new(list);

    render(world);
}
