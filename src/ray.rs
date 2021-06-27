use crate::vec3::{Point3, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Ray(Point3, Vec3, f32);

impl Ray {
    pub fn new(p: Point3, v: Vec3, time: f32) -> Self {
        Ray(p, v, time)
    }

    pub fn origin(&self) -> &Point3 {
        &self.0
    }

    pub fn direction(&self) -> &Vec3 {
        &self.1
    }

    pub fn time(&self) -> f32 {
        self.2
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.0 + self.1 * t
    }
}
