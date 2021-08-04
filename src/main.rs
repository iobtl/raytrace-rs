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
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 800;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: i32 = 200;
const MAX_DEPTH: i32 = 50;

fn ray_color<T: Hittable>(r: Ray, background: Color, world: &HittableList<T>, depth: i32) -> Color {
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
            background
        }
    }
}

fn main() -> io::Result<()> {
    let mut stream = BufWriter::new(io::stdout());

    stream.write_all(format!("P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT).as_bytes())?;

    // World initialization
    let (world, camera, background) = scenes::random_bvh();

    let t0 = std::time::Instant::now();
    let pb = ProgressBar::new(IMG_HEIGHT.into());

    eprintln!("Tracing rays\n");
    let scene: Vec<Vec3> = (0..IMG_HEIGHT)
        .into_par_iter()
        .rev()
        .progress_with(pb)
        .flat_map(|j| {
            (0..IMG_WIDTH)
                .into_par_iter()
                .map(|i| {
                    let mut rng = rand::thread_rng();
                    let color = (0..SAMPLES_PER_PIXEL).into_iter().fold(
                        Vec3::new(0.0, 0.0, 0.0),
                        |acc, _| {
                            let u = ((i as f32) + rng.gen::<f32>()) / ((IMG_WIDTH - 1) as f32);
                            let v = ((j as f32) + rng.gen::<f32>()) / ((IMG_HEIGHT - 1) as f32);
                            let r = camera.ray_at(u, v);
                            acc + ray_color(r, background, &world, MAX_DEPTH)
                        },
                    );

                    process_color(color, SAMPLES_PER_PIXEL)
                })
                .collect::<Vec<Vec3>>()
        })
        .collect();

    eprintln!("\rWriting to file");
    scene.into_iter().for_each(|p| {
        stream
            .write(format!("{} {} {}\n", p.x() as i32, p.y() as i32, p.z() as i32).as_bytes())
            .expect("Unable to write to file");
    });

    eprintln!("\nDone!\n");
    eprintln!("Time elapsed: {}s\n", t0.elapsed().as_secs_f64());
    Ok(())
}
