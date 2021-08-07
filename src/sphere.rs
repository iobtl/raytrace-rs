use rand::thread_rng;

use crate::{
    aabb::{self, AABB},
    hittable::{HitRecord, Hittable},
    material::Surface,
    ray::Ray,
    utility::{random_to_sphere, PI},
    vec3::{self, Point3, Vec3},
};

#[derive(Copy, Clone)]
pub struct Sphere<'a> {
    center: Point3,
    radius: f32,
    material: Surface<'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f32, material: Surface<'a>) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere<'_> {
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
                let (u, v) = sphere_uv(&p);
                let normal = (p - self.center) / self.radius;
                let front_face = HitRecord::face_normal(r, &normal);

                // Surface normal is always against the incident ray
                if front_face {
                    Some(HitRecord::new(p, normal, t, u, v, front_face, &self.material))
                } else {
                    Some(HitRecord::new(p, -normal, t, u, v, front_face, &self.material))
                }
            }
        }
    }
    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        let radius = self.radius;
        let rad_vec = Vec3::new(radius, radius, radius);
        let aabb = AABB::new(self.center - rad_vec, self.center + rad_vec);

        Some(aabb)
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f32 {
        // TODO: inefficient? checking for another hit
        if self.hit(&Ray::new(*origin, *v, 0.0), 0.001, f32::INFINITY).is_none() {
            return 0.0;
        }

        let cos_theta_max =
            1.0 - self.radius * self.radius / (self.center - *origin).length_squared();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let direction = self.center - *origin;
        let dist_squared = direction.length_squared();
        let uvw = vec3::coordinate_system(&direction);

        uvw.local_vec(&random_to_sphere(&mut thread_rng(), self.radius, dist_squared))
    }
}

#[derive(Copy, Clone)]
pub struct MovingSphere<'a> {
    center0: Point3,
    center1: Point3,
    t0: f32,
    t1: f32,
    radius: f32,
    material: Surface<'a>,
}

impl<'a> MovingSphere<'a> {
    pub fn new(
        center0: Point3,
        center1: Point3,
        t0: f32,
        t1: f32,
        radius: f32,
        material: Surface<'a>,
    ) -> Self {
        MovingSphere { center0, center1, t0, t1, radius, material }
    }

    pub fn center(&self, time: f32) -> Point3 {
        self.center0 + (self.center1 - self.center0) * ((time - self.t0) / (self.t1 - self.t0))
    }
}

impl Hittable for MovingSphere<'_> {
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
                let (u, v) = sphere_uv(&p);
                let normal = (p - self.center(r.time())) / self.radius;
                let front_face = HitRecord::face_normal(r, &normal);

                // Surface normal is always against the incident ray
                if front_face {
                    Some(HitRecord::new(p, normal, t, u, v, front_face, &self.material))
                } else {
                    Some(HitRecord::new(p, -normal, t, u, v, front_face, &self.material))
                }
            }
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let radius = self.radius;
        let box0 = AABB::new(
            self.center(t0) - Vec3::new(radius, radius, radius),
            self.center(t0) + Vec3::new(radius, radius, radius),
        );

        let box1 = AABB::new(
            self.center(t1) - Vec3::new(radius, radius, radius),
            self.center(t1) + Vec3::new(radius, radius, radius),
        );

        Some(aabb::surrounding_box(box0, box1))
    }
}

// Returns spherical coordinates mapped to (u, v) in interval [0, 1]
fn sphere_uv(p: &Point3) -> (f32, f32) {
    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;

    (phi / (2.0 * PI), theta / PI)
}
