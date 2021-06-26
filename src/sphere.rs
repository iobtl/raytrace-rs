use crate::{
    hittable::{HitRecord, Hittable},
    material::Surface,
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Surface,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Surface) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();

            // Finding the nearest root that lies in the acceptable range
            let mut root = (-half_b - sqrtd) / a;
            if root < tmin || tmax < root {
                root = (-half_b - sqrtd) / a;
            }

            if root < tmin || tmax < root {
                None
            } else {
                let p = r.at(root);
                let t = root;
                let normal = (p - self.center) / self.radius;
                let front_face = HitRecord::face_normal(r, &normal);
                Some(HitRecord::new(p, normal, t, front_face, &self.material))
            }
        }
    }
}
