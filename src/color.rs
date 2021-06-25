use std::io::{self, Write};

use crate::vec3::Color;

pub fn write_color(out: &mut dyn Write, pixel_color: Color) -> io::Result<()> {
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;

    out.write_all(format!("{} ", r).as_bytes())?;
    out.write_all(format!("{} ", g).as_bytes())?;
    out.write_all(format!("{} ", b).as_bytes())?;
    out.write_all(b"\n")?;

    Ok(())
}
