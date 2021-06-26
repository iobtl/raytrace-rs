use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
    ASPECT_RATIO,
};

// Camera
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

pub struct Camera {
    origin: Point3,
    lower_left: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
        let lower_left =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera { origin, lower_left, horizontal, vertical }
    }

    pub fn ray_at(&self, u: f32, v: f32) -> Ray {
        let Camera { origin, lower_left, horizontal, vertical } = *self;
        Ray::new(origin, lower_left + horizontal * u + vertical * v - origin)
    }
}
