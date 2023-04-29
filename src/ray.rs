mod vec3;
use vec3::Vec3;

pub struct ray {
    origin: Vec3,
    direction: Vec3,
}

impl ray {
    pub fn new(v1: Vec3, v2: Vec3) -> Self { ray { origin: v1, direction: v2 } }

    pub fn at(&self, t: f32) -> Vec3 { self.origin + self.direction * t }
}