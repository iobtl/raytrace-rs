use rand::{prelude::ThreadRng, Rng};

use crate::vec3::{self, Vec3};

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

#[inline]
pub fn random_double(rng: &mut ThreadRng) -> f32 {
    rng.gen::<f32>()
}

#[inline]
pub fn random_double_range(rng: &mut ThreadRng, min: f32, max: f32) -> f32 {
    min + (max - min) * random_double(rng)
}

pub fn random_int(rng: &mut ThreadRng) -> i32 {
    rng.gen::<i32>()
}

pub fn random_int_range(rng: &mut ThreadRng, min: i32, max: i32) -> i32 {
    rng.gen_range(min..=max)
}

#[inline]
// Reject points picked from unit cube until falls inside a unit sphere
pub fn random_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let p = random_vec_range(rng, -1.0, 1.0);

    if p.length_squared() >= 1.0 {
        random_unit_sphere(rng)
    } else {
        p
    }
}

#[inline]
pub fn random_to_sphere(rng: &mut ThreadRng, radius: f32, dist_squared: f32) -> Vec3 {
    let r1 = random_double(rng);
    let r2 = random_double(rng);
    let z = 1.0 + r2 * ((1.0 - radius * radius / dist_squared).sqrt() - 1.0);
    let sqrt_z = (1.0 - z * z).sqrt();

    let phi = 2.0 * PI * r1;

    Vec3::new(phi.cos() * sqrt_z, phi.sin() * sqrt_z, z)
}

#[inline]
pub fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    vec3::unit_vector(&random_unit_sphere(rng))
}

#[inline]
pub fn random_in_hemisphere(rng: &mut ThreadRng, normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_unit_sphere(rng);
    if in_unit_sphere.dot(normal) > 0.0 {
        // In same hemisphere as normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

#[inline]
pub fn random_unit_disk(rng: &mut ThreadRng) -> Vec3 {
    let p =
        Vec3::new(random_double_range(rng, -1.0, 1.0), random_double_range(rng, -1.0, 1.0), 0.0);

    if p.length_squared() < 1.0 {
        p
    } else {
        random_unit_disk(rng)
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
