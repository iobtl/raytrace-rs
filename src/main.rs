use std::io::{self, Write};

pub mod camera;
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

use crate::{camera::Camera, color::write_color};
use utility::*;

// Image dimensions
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: usize = 256;
const IMG_HEIGHT: usize = (IMG_WIDTH as f32 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color(r: Ray, world: &HittableList<Sphere>, depth: i32) -> Color {
    // Limit number of ray bounces
    if depth <= 0 {
        Vec3::new(0.0, 0.0, 0.0)
    } else {
        match world.hit(&r, 0.001, INFINITY) {
            Some(hit_rec) => {
                let target =
                    hit_rec.p + random_in_hemipshere(&mut rand::thread_rng(), &hit_rec.normal);
                let diffuse_ray = Ray::new(hit_rec.p, target - hit_rec.p);
                ray_color(diffuse_ray, world, depth - 1) * 0.5
            }
            None => {
                // Using `y` height _after_ normalizing gives a horizontal gradient
                let unit_direction = vec3::unit_vector(&r.direction());
                let t = (unit_direction.y() + 1.0) * 0.5;
                Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.write_all(format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT).as_bytes())?;

    // Camera
    let camera = Camera::new();

    // World initialization
    let mut world: HittableList<Sphere> = HittableList::new();
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            let mut rng = rand::thread_rng();

            // Anti-aliasing -- generating multiple rays per pixel
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f32) + random_double(&mut rng)) / ((IMG_WIDTH - 1) as f32);
                let v = ((j as f32) + random_double(&mut rng)) / ((IMG_HEIGHT - 1) as f32);
                let r = camera.ray_at(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            write_color(&mut stdout, pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }
    eprintln!("\nDone!\n");
    Ok(())
}
