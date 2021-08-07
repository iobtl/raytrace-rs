use crate::{
    utility::{clamp, random_double, random_double_range},
    vec3::{Color, Vec3},
};

pub fn process_color(pixel_color: Color, samples_per_pixel: i32) -> Color {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Replace NaN components with zero to handle surface acne.
    if r.is_nan() || r != r {
        r = 0.0;
    }

    if g.is_nan() || g != g {
        g = 0.0;
    }

    if b.is_nan() || b != b {
        b = 0.0;
    }

    // Divide color by number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / (samples_per_pixel as f32);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // Write translated [0..255] value of each color component
    let r = 256.0 * clamp(r, 0.0, 0.999);
    let g = 256.0 * clamp(g, 0.0, 0.999);
    let b = 256.0 * clamp(b, 0.0, 0.999);

    Vec3::new(r, g, b)
}

pub fn random() -> Color {
    let mut rng = rand::thread_rng();
    Vec3::new(random_double(&mut rng), random_double(&mut rng), random_double(&mut rng))
}

pub fn random_range(min: f32, max: f32) -> Color {
    let mut rng = rand::thread_rng();
    Vec3::new(
        random_double_range(&mut rng, min, max),
        random_double_range(&mut rng, min, max),
        random_double_range(&mut rng, min, max),
    )
}
