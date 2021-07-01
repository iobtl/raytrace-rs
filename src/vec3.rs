use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec3(f32, f32, f32);

impl Vec3 {
    pub fn new(i1: f32, i2: f32, i3: f32) -> Self {
        Vec3(i1, i2, i3)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }

    pub fn z(&self) -> f32 {
        self.2
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

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, other: f32) -> Self::Output {
        Self(self.0 + other, self.1 + other, self.2 + other)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Self(self.0 * other, self.1 * other, self.2 * other);
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self::Output {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
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

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, other: Vec3) -> Self::Output {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    let b = v.dot(n);
    *v - (*n * b * 2.0)
}

pub fn refract(uv: &Vec3, n: &Vec3, eta_etaprime: f32) -> Vec3 {
    let cos_theta = (-*uv).dot(n).min(1.0);

    let r_out_h = (*uv + *n * cos_theta) * eta_etaprime;
    let r_out_v = *n * -((1.0 - r_out_h.length_squared()).abs().sqrt());
    r_out_h + r_out_v
}

pub fn unpack(v: &Vec3) -> [f32; 3] {
    [v.x(), v.y(), v.z()]
}
