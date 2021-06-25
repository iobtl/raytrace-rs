use std::ops::{AddAssign, Div, DivAssign, MulAssign};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn new(i1: f32, i2: f32, i3: f32) -> Self {
        Vec3(i1, i2, i3)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2) as f32
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self(self.0 * other, self.1 * other, self.2 * other);
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Self(self.0 / other, self.1 / other, self.2 / other);
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        Self(self.0 / other, self.1 / other, self.2 / other)
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}