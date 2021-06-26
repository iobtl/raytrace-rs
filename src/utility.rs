use rand::Rng;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double(rng: &mut rand::rngs::ThreadRng) -> f32 {
    (*rng).gen::<f32>()
}

pub fn random_double_range(rng: &mut rand::rngs::ThreadRng, min: f32, max: f32) -> f32 {
    min + (max - min) * random_double(rng)
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
