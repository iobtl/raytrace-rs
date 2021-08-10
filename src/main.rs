use indicatif::{ParallelProgressIterator, ProgressBar};
use pdf::{HittablePDF, MixturePDF, UniformPDF, PDF};
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
pub mod onb;
pub mod pdf;
pub mod perlin;
pub mod ray;
pub mod rect;
pub mod scenes;
pub mod sphere;
pub mod texture;
pub mod utility;
pub mod vec3;
pub mod volumes;

use hittable::{HitModel, Hittable, HittableList};
use material::Material;
use ray::Ray;
use vec3::{Color, Vec3};

use crate::{
    color::process_color, material::Surface, rect::XZRect, sphere::Sphere, texture::SurfaceTexture,
};
use utility::*;

// Image dimensions
const ASPECT_RATIO: f32 = 1.0;
const IMG_WIDTH: u32 = 600;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn ray_color<'a>(
    r: Ray,
    background: Color,
    world: &HittableList<HitModel<'a>>,
    lights: &HittableList<HitModel<'a>>,
    depth: i32,
) -> Color {
    // Limit number of ray bounces
    if depth <= 0 {
        Color::new(0.0, 0.0, 0.0)
    } else {
        if let Some(hit_rec) = world.hit(&r, 0.001, INFINITY) {
            let emitted = hit_rec.material.emit(&r, &hit_rec, hit_rec.u, hit_rec.v, &hit_rec.p);
            if let Some(srec) = hit_rec.material.scatter(&r, &hit_rec) {
                if srec.is_specular {
                    return srec.attenuation
                        * ray_color(
                            srec.specular_ray.unwrap(),
                            background,
                            world,
                            lights,
                            depth - 1,
                        );
                }

                let p0 = PDF::Hittable(HittablePDF::new(&hit_rec.p, lights));
                let p1 = srec.pdf.unwrap_or(PDF::Uniform(UniformPDF::new(&hit_rec.normal)));
                let mixed_pdf = MixturePDF::new(p0, p1);

                let scattered = Ray::new(hit_rec.p, mixed_pdf.generate(), r.time());
                let pdf = mixed_pdf.value(scattered.direction());

                emitted
                    + srec.attenuation // 'reflectance'
                        * hit_rec.material.scattering_pdf(&r, &hit_rec, &scattered)
                        * ray_color(scattered, background, world, lights, depth - 1)
                        / pdf
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
    let (world, camera, background) = scenes::cornell_box();
    let light = Surface::DiffuseLight(SurfaceTexture::Solid(Vec3::new(15.0, 15.0, 15.0)));
    let mut lights = Box::new(HittableList::new());
    lights.add(HitModel::XZRect(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    // lights.add(HitModel::Sphere(Sphere::new(Point3::new(190.0, 90.0, 190.0), 90.0, light)));

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
                            acc + ray_color(r, background, &world, &lights, MAX_DEPTH)
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
    eprintln!("Time elapsed: {:.2}s\n", t0.elapsed().as_secs_f64());
    Ok(())
}
