use std::io::{self, Write};

use crate::{
    utility::{clamp, random_double, random_double_range},
    vec3::{Color, Vec3},
};

pub fn write_color(
    out: &mut dyn Write,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide color by number of samples and gamma-correct for gamma=2.0
    let scale = 1.0 / (samples_per_pixel as f32);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // Write translated [0..255] value of each color component
    r = 256.0 * clamp(r, 0.0, 0.999);
    g = 256.0 * clamp(g, 0.0, 0.999);
    b = 256.0 * clamp(b, 0.0, 0.999);

    out.write_all(format!("{} ", r as i32).as_bytes())?;
    out.write_all(format!("{} ", g as i32).as_bytes())?;
    out.write_all(format!("{} ", b as i32).as_bytes())?;
    out.write_all(b"\n")?;

    Ok(())
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
