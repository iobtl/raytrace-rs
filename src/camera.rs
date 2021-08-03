use crate::{
    ray::Ray,
    utility::{degrees_to_radians, random_double_range, random_unit_disk},
    vec3::{unit_vector, Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
    t0: f32,
    t1: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        t0: f32,
        t1: f32,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&vup.cross(&w));
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let lens_radius = aperture / 2.0;

        Camera { origin, lower_left, horizontal, vertical, u, v, w, lens_radius, t0, t1 }
    }

    pub fn ray_at(&self, s: f32, t: f32) -> Ray {
        let mut rng = rand::thread_rng();
        let rd = random_unit_disk(&mut rng) * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left + self.horizontal * s + self.vertical * t - self.origin - offset,
            random_double_range(&mut rng, 0.0, 1.0),
        )
    }
}
