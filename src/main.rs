use std::io::{self, Write};

pub mod color;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod utility;
pub mod vec3;

use hittable::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

use crate::color::write_color;
use utility::*;

// Image dimensions
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: usize = 256;
const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;

// Camera
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

fn ray_color(r: Ray, world: &HittableList<Sphere>) -> Color {
    match world.hit(&r, 0.0, INFINITY) {
        Some(hit_rec) => (hit_rec.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5,
        None => {
            // Using `y` height _after_ normalizing gives a horizontal gradient
            let unit_direction = vec3::unit_vector(&r.direction());
            let t = (unit_direction.y() + 1.0) * 0.5;
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.write_all(format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT).as_bytes())?;

    // Pre-defined vectors
    let origin: Point3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left: Vec3 =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // World initialization
    let mut world: HittableList<Sphere> = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let u = (i as f32) / ((IMG_WIDTH - 1) as f32);
            let v = (j as f32) / ((IMG_HEIGHT - 1) as f32);
            let r = Ray::new(origin, lower_left + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(r, &world);

            write_color(&mut stdout, pixel_color)?;
        }
    }
    eprintln!("\nDone!\n");
    Ok(())
}
