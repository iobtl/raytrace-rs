use indicatif::ProgressBar;
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

use hittable::{Hittable, HittableList};
use material::Material;
use ray::Ray;
use vec3::{Color, Vec3};

use crate::color::process_color;
use utility::*;

// Image dimensions
const ASPECT_RATIO: f32 = 1.0;
const IMG_WIDTH: u32 = 600;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: i32 = 500;
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
    // let earth_image = open("earth.jpg").unwrap().into_rgb8();
    let (world, camera, background) = scenes::cornell_box();

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
                        pixel_color += ray_color(r, &background, &world, MAX_DEPTH);
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
