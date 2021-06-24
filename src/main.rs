use std::io::{self, Write};

const IMG_WIDTH: usize = 256;
const IMG_HEIGHT: usize = 256;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.write_all(format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT).as_bytes())?;

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let r: f32 = (i as f32) / (IMG_WIDTH - 1) as f32;
            let g: f32 = (j as f32) / (IMG_HEIGHT - 1) as f32;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            stdout.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    eprintln!("\nDone!\n");
    Ok(())
}
