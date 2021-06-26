use crate::utility::*;
use crate::vec3::{reflect, unit_vector};
use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

// Required behaviour:
// 1. Produce a scattered ray (or say it absorbed the incident ray)
// 2. If scattered, determine how much the ray should be attenuated
pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub enum Surface {
    Lambertian(Color),
    Metal(Color, f32),
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
        }
    }
}
