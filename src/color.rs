use std::io::{self, Write};

use crate::{utility::clamp, vec3::Color};

pub fn write_color(
    out: &mut dyn Write,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide color by number of samples
    let scale = 1.0 / (samples_per_pixel as f32);
    r *= scale;
    g *= scale;
    b *= scale;

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
