use image::open;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::io::{self, BufWriter, Write};

pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod scenes;
pub mod sphere;
pub mod texture;
pub mod utility;
pub mod vec3;

use hittable::{Hittable, HittableList};
use material::Material;
use ray::Ray;
use vec3::{Color, Vec3};

use crate::{camera::Camera, color::process_color, scenes::earth};
use utility::*;

// Image dimensions
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 1200;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: i32 = 50;
const MAX_DEPTH: i32 = 50;

fn ray_color<T: Hittable>(r: Ray, world: &HittableList<T>, depth: i32) -> Color {
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
    let mut stream = BufWriter::new(io::stdout());

    stream.write_all(format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT).as_bytes())?;

    // Camera
    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 12.0;
    let aperture = 0.1;
    let camera =
        Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

    // World initialization
    let earth_image = open("earth.jpg").unwrap().into_rgb8();
    let world = scenes::earth(&earth_image);
    let height_range = (0..IMG_HEIGHT).rev().collect::<Vec<u32>>();

    let t0 = std::time::Instant::now();
    let pb = ProgressBar::new(IMG_HEIGHT.into());
    eprintln!("Tracing rays\n");
    let scene: Vec<Vec<Vec3>> = height_range
        .into_par_iter()
        .map(|j| {
            pb.inc(1);
            (0..IMG_WIDTH)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    let mut rng = rand::thread_rng();

                    // Anti-aliasing -- generating multiple rays per pixel
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let u = ((i as f32) + random_double(&mut rng)) / ((IMG_WIDTH - 1) as f32);
                        let v = ((j as f32) + random_double(&mut rng)) / ((IMG_HEIGHT - 1) as f32);
                        let r = camera.ray_at(u, v);
                        pixel_color += ray_color(r, &world, MAX_DEPTH);
                    }
                    process_color(pixel_color, SAMPLES_PER_PIXEL)
                })
                .collect()
        })
        .collect();

    let scene: Vec<Vec3> = scene.into_par_iter().flatten().collect();

    eprintln!("\rWriting to file");
    scene.iter().for_each(move |p| {
        stream
            .write(format!("{} {} {}\n", p.x() as i32, p.y() as i32, p.z() as i32).as_bytes())
            .expect("Unable to write to file");
    });

    eprintln!("\nDone!\n");
    eprintln!("Time elapsed: {}s\n", t0.elapsed().as_secs_f64());
    Ok(())
}
