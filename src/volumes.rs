use crate::{
    aabb::AABB,
    hittable::{HitModel, HitRecord, Hittable},
    material::Surface,
    ray::Ray,
    texture::SurfaceTexture,
    utility::{random_double, INFINITY},
    vec3::{Color, Vec3},
};

#[derive(Clone)]
pub struct Constant<'a> {
    neg_inv_density: f32,
    boundary: Box<HitModel<'a>>,
    phase_function: Surface<'a>,
}

impl<'a> Constant<'a> {
    pub fn new(density: f32, boundary: HitModel<'a>, phase_function: Color) -> Self {
        let neg_inv_density = -1.0 / density;
        let boundary = Box::new(boundary);
        let phase_function = Surface::Isotropic(SurfaceTexture::Solid(phase_function));
        Constant { neg_inv_density, boundary, phase_function }
    }
}

impl<'a> Hittable for Constant<'a> {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, -INFINITY, INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, INFINITY) {
                if rec1.t < tmin {
                    rec1.t = tmin;
                }

                if rec2.t > tmax {
                    rec2.t = tmax;
                }

                if rec1.t >= rec2.t {
                    return None;
                }

                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = r.direction().length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance =
                    self.neg_inv_density * random_double(&mut rand::thread_rng()).log10();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let p = r.at(t);
                let normal = Vec3::new(1.0, 0.0, 0.0);
                let front_face = true;
                let material = &self.phase_function;

                Some(HitRecord::new(p, normal, t, 0.0, 0.0, front_face, material))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
