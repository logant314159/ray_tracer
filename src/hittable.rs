use crate::Vec3;

pub struct Hit_Record {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32
}

pub struct Hittable;

impl Hittable {
    pub fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut Hit_Record) -> bool {
        false
    }
}