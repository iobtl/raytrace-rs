use image::{open, Rgb};
use indicatif::{ParallelProgressIterator, ProgressBar};
use rand::Rng;
use rayon::prelude::*;
use std::io::{self, BufWriter, Write};

pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod instances;
pub mod material;
pub mod perlin;
pub mod ray;
pub mod rect;
pub mod scenes;
pub mod sphere;
pub mod texture;
pub mod utility;
pub mod vec3;
pub mod volumes;

use hittable::{Hittable, HittableList};
use material::Material;
use ray::Ray;
use vec3::{Color, Vec3};

use crate::color::process_color;
use utility::*;

// Image dimensions
const ASPECT_RATIO: f32 = 1.0;
const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: i32 = 10000;
const MAX_DEPTH: i32 = 50;

fn ray_color<T: Hittable>(
    r: Ray,
    background: &Color,
    world: &HittableList<T>,
    depth: i32,
) -> Color {
    // Limit number of ray bounces
    if depth <= 0 {
        Vec3::new(0.0, 0.0, 0.0)
    } else {
        if let Some(hit_rec) = world.hit(&r, 0.001, INFINITY) {
            let emitted = hit_rec.material.emit(hit_rec.u, hit_rec.v, &hit_rec.p);
            if let Some((scattered, attenuation)) = hit_rec.material.scatter(&r, &hit_rec) {
                emitted + attenuation * ray_color(scattered, background, world, depth - 1)
            } else {
                emitted
            }
        } else {
            *background
        }
    }
}

fn main() -> io::Result<()> {
    let mut stream = BufWriter::new(io::stdout());

    stream.write_all(format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT).as_bytes())?;

    // World initialization
    //let earth_image = open("earth.jpg").unwrap().into_rgb8();
    let earth_image = image::io::Reader::open("earth.jpg")?.decode().unwrap();
    let earth_image = earth_image.as_rgb8().unwrap();
    let (world, camera, background) = scenes::random_bvh();

    let height_range = (0..IMG_HEIGHT).rev().collect::<Vec<u32>>();
    let t0 = std::time::Instant::now();
    let pb = ProgressBar::new(IMG_HEIGHT.into());
    eprintln!("Tracing rays\n");
    let scene: Vec<Vec<Vec3>> = height_range
        .into_par_iter()
        .progress_with(pb)
        .map(|j| {
            (0..IMG_WIDTH)
                .into_par_iter()
                .map(|i| {
                    let mut rng = rand::thread_rng();
                    let color = (0..SAMPLES_PER_PIXEL).fold(Vec3::new(0.0, 0.0, 0.0), |acc, _| {
                        let u = ((i as f32) + rng.gen::<f32>()) / ((IMG_WIDTH - 1) as f32);
                        let v = ((j as f32) + rng.gen::<f32>()) / ((IMG_HEIGHT - 1) as f32);
                        let r = camera.ray_at(u, v);
                        acc + ray_color(r, &background, &world, MAX_DEPTH)
                    });
                    process_color(color, SAMPLES_PER_PIXEL)
                })
                .collect::<Vec<Vec3>>()
        })
        .collect();

    let scene: Vec<Vec3> = scene.into_par_iter().flatten().collect();

    eprintln!("\rWriting to file");
    let buffer: &[u8] = bytemuck::cast_slice(&scene);
    image::save_buffer("image.png", buffer, IMG_WIDTH, IMG_HEIGHT, image::ColorType::Rgb8).unwrap();

    eprintln!("\nDone!\n");
    eprintln!("Time elapsed: {}s\n", t0.elapsed().as_secs_f64());
    Ok(())
}
