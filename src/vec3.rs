use std::ops::{Add, Sub, Index, IndexMut, Mul, Div, Neg};

#[derive(Debug)]
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

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            e: [self.e[0] * other,
                self.e[1] * other,
                self.e[2] * other]
        }
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mil(self, other: Self) -> Self {
        Self {
            e:  [self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2]]
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        Self {
            e: [self.e[0] / other,
                self.e[1] / other,
                self.e[2] / other]
        }
    }
}

impl Div<Self> for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            e:  [self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2]]
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            e: [-self.e[0],
                -self.e[1],
                -self.e[2]]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.e[0], 1.0);
        assert_eq!(v.e[1], 2.0);
        assert_eq!(v.e[2], 3.0);
    }

    #[test]
    fn test_x_y_z() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_add() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let vpb = v + b;
        assert_eq!(vb.e[0], 5.0);
        assert_eq!(vb.e[1], 7.0);
        assert_eq!(vb.e[2], 9.0);
    }

    #[test]
    fn test_sub() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 6.0, 8.0);
        let bmv = b - v;
        assert_eq!(bmv.e[2], 4.0);
        assert_eq!(bmv.e[0], 5.0);
        assert_eq!(bmv.e[1], 3.0);
    }

    #[test]
    fn test_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn test_mul_f32() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let vm = v * 2;
        assert_eq!(v.e[0], 2.0);
        assert_eq!(v.e[1], 4.0);
        assert_eq!(v.e[2], 6.0);
    }

    #[test]
    fn test_mul_self() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let vb = v * b;
        assert_eq!(vb.e[0], 4.0);
        assert_eq!(vb.e[1], 10.0);
        assert_eq!(vb.e[2], 18.0);
    }

    #[test]
    fn test_div_f32() {
        let v = Vec3::new(9.0, 3.0, 12.0);
        let vd = v / 3;
        assert_eq!(vd.e[0], 3.0);
        assert_eq!(vd.e[1], 1.0);
        assert_eq!(vd.e[2], 4.0);
    }

    #[test]
    fn test_div_self() {
        let v = Vec3::new(10.0, 21.0, 30.0);
        let b = Vec3::new(5.0, 7.0, 3.0);
        let vdb = v / b;
        assert_eq!(vdb.e[0], 2.0);
        assert_eq!(vdb.e[1], 3.0);
        assert_eq!(vdb.e[2], 10.0);
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let nv = -v;
        assert_eq!(nv.e[0], -1.0);
        assert_eq!(nv.e[1], -2.0);
        assert_eq!(nv.e[2], -3.0);
    }
}