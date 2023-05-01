use crate::Vec3;
use crate::Ray;

pub struct HitRecord {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

trait Hittable {
    fn hit(r: &Ray, t_min: f32, t_max: f32, rec: &HitRecord) -> bool {
        false
    }
}