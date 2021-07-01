use std::mem::swap;

use crate::{
    ray::Ray,
    vec3::{unpack, Point3, Vec3},
};

#[derive(Copy, Clone)]
pub struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        AABB { minimum, maximum }
    }

    pub fn min(&self) -> &Point3 {
        &self.minimum
    }

    pub fn max(&self) -> &Point3 {
        &self.maximum
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        let ray_origins = unpack(r.origin());
        let ray_directions = unpack(r.direction());

        let AABB { minimum, maximum } = *self;
        let min_dims = unpack(&minimum);
        let max_dims = unpack(&maximum);

        for a in 0..3 {
            let inv_d = 1.0 / ray_directions[a];
            let mut t0 = (min_dims[a] - ray_origins[a]) * inv_d;
            let mut t1 = (max_dims[a] - ray_origins[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }

            let tmin = t0.max(tmin);
            let tmax = t1.min(tmax);

            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

// Try to get bounding box that encapsulates both boxes
pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Vec3::new(
        box0.min().x().min(box1.min().x()),
        box0.min().y().min(box1.min().y()),
        box0.min().z().min(box1.min().z()),
    );

    let big = Vec3::new(
        box0.max().x().max(box1.max().x()),
        box0.max().y().max(box1.max().y()),
        box0.max().z().max(box1.max().z()),
    );

    AABB::new(small, big)
}
