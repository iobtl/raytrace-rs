use std::io::{self, Write};

pub mod color;
pub mod vec3;

use crate::color::write_color;
use crate::vec3::Color;
use crate::vec3::Vec3;

const IMG_WIDTH: usize = 256;
const IMG_HEIGHT: usize = 256;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.write_all(format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT).as_bytes())?;

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let pixel_color: Color = Vec3(
                (i as f32) / ((IMG_WIDTH - 1) as f32),
                (j as f32) / ((IMG_HEIGHT - 1) as f32),
                0.25,
            );

            write_color(&mut stdout, pixel_color)?;
        }
    }
    eprintln!("\nDone!\n");
    Ok(())
}
