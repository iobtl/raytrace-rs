use image::{ImageBuffer, Pixel, Rgb};

use crate::{
    perlin::Perlin,
    utility::clamp,
    vec3::{Color, Point3, Vec3},
};

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

#[derive(Copy, Clone)]
pub enum SurfaceTexture<'a> {
    Solid(Color),
    Checkered(Color, Color), // only raw colors to make implementation simpler
    Noise(Perlin, f32),
    Image { buffer: &'a ImageBuffer<Rgb<u8>, Vec<u8>>, width: u32, height: u32 },
}

impl<'a> Texture for SurfaceTexture<'a> {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        match self {
            Self::Solid(color) => *color,
            Self::Checkered(odd, even) => {
                let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();

                if sines < 0.0 {
                    *odd
                } else {
                    *even
                }
            }
            Self::Noise(noise, scale) => {
                Vec3::new(1.0, 1.0, 1.0)
                    * 0.5
                    * (1.0 + (scale * p.z() + 10.0 * noise.turb(p)).sin())
            }
            Self::Image { buffer, width, height } => {
                // Clamp input texture coordinates to [0, 1] x [1, 0]
                let u = clamp(u, 0.0, 1.0);
                let v = 1.0 - clamp(v, 0.0, 1.0);

                let mut i = (u * *width as f32) as i32;
                let mut j = (v * *height as f32) as i32;

                // Clamp integer mapping, since actual integer coordinates should be <= 1.0
                if i >= (*width as i32) {
                    i = (width - 1) as i32;
                }

                if j >= (*height as i32) {
                    j = (height - 1) as i32;
                }

                let color_scale = 1.0 / 255.0;
                let pixel = buffer.get_pixel(i as u32, j as u32);
                let rgb = pixel.channels();

                Vec3::new(
                    color_scale * rgb[0] as f32,
                    color_scale * rgb[1] as f32,
                    color_scale * rgb[1] as f32,
                )
            }
        }
    }
}
