use std::io::{self, Write};

pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utility;
pub mod vec3;

use hittable::HittableList;
use material::Material;
use ray::Ray;
use sphere::Sphere;
use vec3::{Color, Point3, Vec3};

use crate::{camera::Camera, color::write_color, material::Surface};
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
        if let Some(hit_rec) = world.hit(&r, 0.001, INFINITY) {
            if let Some((scattered, attenuation)) = hit_rec.material.scatter(&r, &hit_rec) {
                attenuation * ray_color(scattered, world, depth - 1)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        } else {
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

    // Camera
    let camera = Camera::new();

    // World initialization
    let mut world: HittableList<Sphere> = HittableList::new();
    let material_ground = Surface::Lambertian(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Surface::Lambertian(Vec3::new(0.7, 0.3, 0.3));
    let material_left = Surface::Metal(Vec3::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Surface::Metal(Vec3::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center));
    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right));

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
