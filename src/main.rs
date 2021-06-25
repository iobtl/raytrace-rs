use std::io::{self, Write};

pub mod color;
pub mod ray;
pub mod vec3;

use ray::Ray;
use vec3::{Color, Point3, Vec3};

use crate::color::write_color;

// Image dimensions
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: usize = 256;
const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;

// Camera
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> f32 {
    // Solving for t in P(t) = A + tb => the equation for the ray `r`
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(&r.direction());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: Ray) -> Color {
    let mut t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = vec3::unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        // Return normalized vector normal
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    // Using `y` height _after_ normalizing gives a horizontal gradient
    let unit_direction = vec3::unit_vector(&r.direction());
    t = (unit_direction.y() + 1.0) * 0.5;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.write_all(format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT).as_bytes())?;

    let origin: Point3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left: Vec3 =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let u = (i as f32) / ((IMG_WIDTH - 1) as f32);
            let v = (j as f32) / ((IMG_HEIGHT - 1) as f32);
            let r = Ray::new(origin, lower_left + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(r);

            write_color(&mut stdout, pixel_color)?;
        }
    }
    eprintln!("\nDone!\n");
    Ok(())
}
