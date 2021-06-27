use crate::utility::*;
use crate::vec3::{reflect, refract, unit_vector, Vec3};
use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

// Required behaviour:
// 1. Produce a scattered ray (or say it absorbed the incident ray)
// 2. If scattered, determine how much the ray should be attenuated
pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Copy, Clone)]
pub enum Surface {
    Lambertian(Color),
    Metal(Color, f32),
    Dielectric(f32),
}

impl Material for Surface {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Self::Lambertian(albedo) => {
                let scatter_direction = rec.normal + random_unit_vector(&mut rand::thread_rng());

                // Catch degenerate scatter direction (almost exactly opposite to normal)
                if scatter_direction.near_zero() {
                    let scatter_direction = rec.normal;
                }

                let scattered = Ray::new(rec.p, scatter_direction);
                let attenuation = *albedo;

                Some((scattered, attenuation))
            }
            Self::Metal(albedo, fuzz) => {
                let reflected = reflect(&unit_vector(ray.direction()), &rec.normal);
                let scattered = Ray::new(
                    rec.p,
                    reflected + random_unit_sphere(&mut rand::thread_rng()) * *fuzz,
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

                let scattered = Ray::new(rec.p, direction);

                Some((scattered, attenuation))
            }
        }
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
