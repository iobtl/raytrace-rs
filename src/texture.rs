use crate::{
    perlin::Perlin,
    vec3::{Color, Point3, Vec3},
};

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}

#[derive(Copy, Clone)]
pub enum SurfaceTexture {
    Solid(Color),
    Checkered(Color, Color), // only raw colors to make implementation simpler
    Noise(Perlin, f32),
}

impl Texture for SurfaceTexture {
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
                let gen_noise = *p * *scale + 1.0;
                Vec3::new(1.0, 1.0, 1.0) * 0.5 * noise.noise(&gen_noise)
            }
        }
    }
}
