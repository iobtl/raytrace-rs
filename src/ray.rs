use crate::vec3::{Point3, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray(Point3, Vec3);

impl Ray {
    pub fn new(p: Point3, v: Vec3) -> Self {
        Ray(p, v)
    }

    pub fn origin(&self) -> Point3 {
        self.0
    }

    pub fn direction(&self) -> Vec3 {
        self.1
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.0 + self.1 * t
    }
}
