use crate::pdf::random_cosine_direction;
use crate::texture::SurfaceTexture;
use crate::texture::Texture;
use crate::utility::*;
use crate::vec3;
use crate::vec3::Point3;
use crate::vec3::{reflect, refract, unit_vector, Vec3};
use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

// Required behaviour:
// 1. Produce a scattered ray (or say it absorbed the incident ray)
// 2. If scattered, determine how much the ray should be attenuated
pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color, f32)>;
    fn scattering_pdf(&self, ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f32;
    fn emit(&self, ray: &Ray, rec: &HitRecord, u: f32, v: f32, p: &Point3) -> Color;
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
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color, f32)> {
        match self {
            Self::Lambertian(albedo) => {
                // Construct BRDF space.
                let uvw = vec3::coordinate_system(&rec.normal);
                let random_cos = random_cosine_direction(&mut rand::thread_rng());
                let mut scatter_direction = uvw.local_vec(&random_cos);

                // Catch degenerate scatter direction (almost exactly opposite to normal)
                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                let scattered = Ray::new(rec.p, vec3::unit_vector(&scatter_direction), ray.time());
                let alb = albedo.value(rec.u, rec.v, &rec.p);
                // Importance sampling by allowing PDF to be equal to scattering_pdf. (cos_theta / PI)
                let pdf = uvw.w().dot(scattered.direction()) / PI;

                Some((scattered, alb, pdf))
            }
            Self::Metal(albedo, fuzz) => {
                let reflected = reflect(&unit_vector(ray.direction()), &rec.normal);
                let scattered = Ray::new(
                    rec.p,
                    reflected + random_unit_sphere(&mut rand::thread_rng()) * *fuzz,
                    ray.time(),
                );

                // Assume that metal absorbs all grazing rays.
                if scattered.direction().dot(&rec.normal) <= 0.0 {
                    return None;
                }

                let attenuation = *albedo;

                Some((scattered, attenuation, 1.0))
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

                Some((scattered, attenuation, 1.0))
            }
            Self::Isotropic(albedo) => {
                // Isotropic volumes scatter light in random directions with certain probability
                let scattered =
                    Ray::new(rec.p, random_unit_sphere(&mut rand::thread_rng()), ray.time());
                let attenuation = albedo.value(rec.u, rec.v, &rec.p);

                Some((scattered, attenuation, 1.0))
            }
            _ => None,
        }
    }
    fn scattering_pdf(&self, ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f32 {
        match self {
            Self::Lambertian(_) => {
                let cosine = vec3::unit_vector(scattered.direction()).dot(&rec.normal);
                if cosine < 0.0 {
                    0.0
                } else {
                    cosine / PI
                }
            }
            _ => panic!(),
        }
    }
    fn emit(&self, ray: &Ray, rec: &HitRecord, u: f32, v: f32, p: &Point3) -> Color {
        match self {
            Self::DiffuseLight(texture) => {
                // Only allow lights to emit light from their front surfaces.
                if rec.front_face {
                    texture.value(u, v, p)
                } else {
                    Color::new(0.0, 0.0, 0.0)
                }
            }
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
