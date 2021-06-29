use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Surface,
    ray::Ray,
    vec3::Vec3,
};

pub struct XYRect<'a> {
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
    material: Surface<'a>,
}

impl<'a> XYRect<'a> {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Surface<'a>) -> Self {
        XYRect { x0, x1, y0, y1, k, material }
    }
}

impl Hittable for XYRect<'_> {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        // First, solve for t given z = k.
        // Then, check whether x and y values of the ray fall within rectangle boundaries.
        let XYRect { x0, x1, y0, y1, k, .. } = *self;
        let t = (k - r.origin().z()) / r.direction().z();

        if t < tmin || t > tmax {
            None
        } else {
            let x = r.origin().x() + t * r.direction().x();
            let y = r.origin().y() + t * r.direction().y();

            if x < x0 || x > x1 || y < y0 || y > y1 {
                None
            } else {
                let u = (x - x0) / (x1 - x0);
                let v = (y - y0) / (y1 - y0);
                let p = r.at(t);
                let normal = Vec3::new(0.0, 0.0, 1.0);
                let front_face = HitRecord::face_normal(r, &normal);

                if front_face {
                    Some(HitRecord::new(p, normal, t, u, v, front_face, &self.material))
                } else {
                    Some(HitRecord::new(p, -normal, t, u, v, front_face, &self.material))
                }
            }
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
