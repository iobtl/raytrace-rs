use rand::{prelude::ThreadRng, Rng};

use crate::vec3::Vec3;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_vec(rng: &mut ThreadRng) -> Vec3 {
    let i1 = random_double(rng);
    let i2 = random_double(rng);
    let i3 = random_double(rng);
    Vec3::new(i1, i2, i3)
}

pub fn random_vec_range(rng: &mut ThreadRng, min: f32, max: f32) -> Vec3 {
    let i1 = random_double_range(rng, min, max);
    let i2 = random_double_range(rng, min, max);
    let i3 = random_double_range(rng, min, max);
    Vec3::new(i1, i2, i3)
}

pub fn random_double(rng: &mut ThreadRng) -> f32 {
    (*rng).gen::<f32>()
}

pub fn random_double_range(rng: &mut ThreadRng, min: f32, max: f32) -> f32 {
    min + (max - min) * random_double(rng)
}

pub fn random_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    // Reject points picked from unit cube until falls inside a unit sphere
    let p = random_vec_range(rng, -1.0, 1.0);

    if p.length_squared() >= 1.0 {
        random_unit_sphere(rng)
    } else {
        p
    }
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
