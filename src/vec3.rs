use std::ops::{Add, Sub, Index, IndexMut, Mul, Div, Neg};

pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 { Vec3 { e: [e0, e1, e2] } }

    pub fn x(&self) -> f32 { self.e[0] }
    pub fn y(&self) -> f32 { self.e[1] }
    pub fn z(&self) -> f32 { self.e[2] }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            e: [self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]]
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            e: [self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]]
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 { &self.e[index] }
}

impl IndexMut<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> f32 { &self.e[index] }
}