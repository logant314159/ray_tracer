use crate::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(v1: Vec3, v2: Vec3) -> Self { Ray { origin: v1, direction: v2 } }

    pub fn at(&self, t: f32) -> Vec3 { self.origin + self.direction * t }
}