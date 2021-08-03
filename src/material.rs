use crate::texture::SurfaceTexture;
use crate::texture::Texture;
use crate::utility::*;
use crate::vec3::Point3;
use crate::vec3::{reflect, refract, unit_vector, Vec3};
use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

// Required behaviour:
// 1. Produce a scattered ray (or say it absorbed the incident ray)
// 2. If scattered, determine how much the ray should be attenuated
pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
    fn emit(&self, u: f32, v: f32, p: &Point3) -> Color;
}

#[derive(Copy, Clone)]
pub enum Surface<'a> {
    Lambertian(SurfaceTexture<'a>),
    Metal(Color, f32),
    Dielectric(f32),
    DiffuseLight(SurfaceTexture<'a>),
    Isotropic(SurfaceTexture<'a>),
}

impl<'a> Material for Surface<'a> {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Self::Lambertian(albedo) => {
                let mut scatter_direction =
                    rec.normal + random_unit_vector(&mut rand::thread_rng());

                // Catch degenerate scatter direction (almost exactly opposite to normal)
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                let scattered = Ray::new(rec.p, scatter_direction, ray.time());
                let attenuation = albedo.value(rec.u, rec.v, &rec.p);

                Some((scattered, attenuation))
            }
            Self::Metal(albedo, fuzz) => {
                let reflected = reflect(&unit_vector(ray.direction()), &rec.normal);
                let scattered = Ray::new(
                    rec.p,
                    reflected + random_unit_sphere(&mut rand::thread_rng()) * *fuzz,
                    ray.time(),
                );
                let attenuation = *albedo;

                Some((scattered, attenuation))
            }
            Self::Dielectric(refraction_index) => {
                // Dielectric surfaces do not absorb light
                let attenuation = Vec3::new(1.0, 1.0, 1.0);
                let refraction_ratio =
                    if rec.front_face { 1.0 / *refraction_index } else { *refraction_index };

                let unit_direction = unit_vector(ray.direction()); // Make incident ray unit vector to simplify formula
                let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let schlick = reflectance(cos_theta, refraction_ratio);
                let direction = if refraction_ratio * sin_theta > 1.0
                    || schlick > random_double(&mut rand::thread_rng())
                {
                    reflect(&unit_direction, &rec.normal)
                } else {
                    refract(&unit_direction, &rec.normal, refraction_ratio)
                };

                let scattered = Ray::new(rec.p, direction, ray.time());

                Some((scattered, attenuation))
            }
            Self::Isotropic(albedo) => {
                // Isotropic volumes scatter light in random directions with certain probability
                let scattered =
                    Ray::new(rec.p, random_unit_sphere(&mut rand::thread_rng()), ray.time());
                let attenuation = albedo.value(rec.u, rec.v, &rec.p);

                Some((scattered, attenuation))
            }
            _ => None,
        }
    }
    fn emit(&self, u: f32, v: f32, p: &Point3) -> Color {
        match self {
            Self::DiffuseLight(texture) => texture.value(u, v, p),
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
