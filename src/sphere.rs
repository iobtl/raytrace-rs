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
                root = (-half_b + sqrtd) / a;
            }

            if root < tmin || tmax < root {
                None
            } else {
                let p = r.at(root);
                let t = root;
                let normal = (p - self.center) / self.radius;
                let front_face = HitRecord::face_normal(r, &normal);

                // Surface normal is always against the incident ray
                if front_face {
                    Some(HitRecord::new(p, normal, t, front_face, &self.material))
                } else {
                    Some(HitRecord::new(p, -normal, t, front_face, &self.material))
                }
            }
        }
    }
}

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    t0: f32,
    t1: f32,
    radius: f32,
    material: Surface,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        t0: f32,
        t1: f32,
        radius: f32,
        material: Surface,
    ) -> Self {
        MovingSphere { center0, center1, t0, t1, radius, material }
    }

    pub fn center(&self, time: f32) -> Point3 {
        self.center0 + (self.center1 - self.center0) * ((time - self.t0) / (self.t1 - self.t0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let oc = *r.origin() - self.center(r.time());
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
                root = (-half_b + sqrtd) / a;
            }

            if root < tmin || tmax < root {
                None
            } else {
                let p = r.at(root);
                let t = root;
                let normal = (p - self.center(r.time())) / self.radius;
                let front_face = HitRecord::face_normal(r, &normal);

                // Surface normal is always against the incident ray
                if front_face {
                    Some(HitRecord::new(p, normal, t, front_face, &self.material))
                } else {
                    Some(HitRecord::new(p, -normal, t, front_face, &self.material))
                }
            }
        }
    }
}
