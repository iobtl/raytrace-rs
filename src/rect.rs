use crate::{
    aabb::AABB,
    hittable::{HitModel, HitRecord, Hittable, HittableList},
    material::Surface,
    ray::Ray,
    utility::random_double_range,
    vec3::{Point3, Vec3},
};

#[derive(Copy, Clone)]
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

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

#[derive(Copy, Clone)]
pub struct XZRect<'a> {
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Surface<'a>,
}

impl<'a> XZRect<'a> {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Surface<'a>) -> Self {
        XZRect { x0, x1, z0, z1, k, material }
    }
}

impl Hittable for XZRect<'_> {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        // First, solve for t given z = k.
        // Then, check whether x and y values of the ray fall within rectangle boundaries.
        let XZRect { x0, x1, z0, z1, k, .. } = *self;
        let t = (k - r.origin().y()) / r.direction().y();

        if t < tmin || t > tmax {
            None
        } else {
            let x = r.origin().x() + t * r.direction().x();
            let z = r.origin().z() + t * r.direction().z();

            if x < x0 || x > x1 || z < z0 || z > z1 {
                None
            } else {
                let u = (x - x0) / (x1 - x0);
                let v = (z - z0) / (z1 - z0);
                let p = r.at(t);
                let normal = Vec3::new(0.0, 1.0, 0.0);
                let front_face = HitRecord::face_normal(r, &normal);

                if front_face {
                    Some(HitRecord::new(p, normal, t, u, v, front_face, &self.material))
                } else {
                    Some(HitRecord::new(p, -normal, t, u, v, front_face, &self.material))
                }
            }
        }
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.z0, self.k - 0.0001),
            Vec3::new(self.x1, self.z1, self.k + 0.0001),
        ))
    }

    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f32 {
        if let Some(hit_rec) = self.hit(&Ray::new(*origin, *v, 0.0), 0.001, f32::INFINITY) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let dist_squared = hit_rec.t * hit_rec.t * v.length_squared();
            let cosine = (v.dot(&hit_rec.normal) / v.length()).abs();

            dist_squared / (cosine * area)
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let random_point = Point3::new(
            random_double_range(&mut rng, self.x0, self.x1),
            self.k,
            random_double_range(&mut rng, self.z0, self.z1),
        );

        random_point - *origin
    }
}

#[derive(Copy, Clone)]
pub struct YZRect<'a> {
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
    material: Surface<'a>,
}

impl<'a> YZRect<'a> {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Surface<'a>) -> Self {
        YZRect { y0, y1, z0, z1, k, material }
    }
}

impl Hittable for YZRect<'_> {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        // First, solve for t given z = k.
        // Then, check whether x and y values of the ray fall within rectangle boundaries.
        let YZRect { y0, y1, z0, z1, k, .. } = *self;
        let t = (k - r.origin().x()) / r.direction().x();

        if t < tmin || t > tmax {
            None
        } else {
            let y = r.origin().y() + t * r.direction().y();
            let z = r.origin().z() + t * r.direction().z();

            if y < y0 || y > y1 || z < z0 || z > z1 {
                None
            } else {
                let u = (y - y0) / (y1 - y0);
                let v = (z - z0) / (z1 - z0);
                let p = r.at(t);
                let normal = Vec3::new(1.0, 0.0, 0.0);
                let front_face = HitRecord::face_normal(r, &normal);

                if front_face {
                    Some(HitRecord::new(p, normal, t, u, v, front_face, &self.material))
                } else {
                    Some(HitRecord::new(p, -normal, t, u, v, front_face, &self.material))
                }
            }
        }
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.y0, self.z0, self.k - 0.0001),
            Vec3::new(self.y1, self.z1, self.k + 0.0001),
        ))
    }
}

#[derive(Clone)]
pub struct Box<'a> {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList<HitModel<'a>>, // more strictyl, only rectangles
}

impl<'a> Box<'a> {
    pub fn new(p0: Point3, p1: Point3, material: Surface<'a>) -> Self {
        let box_min = p0;
        let box_max = p1;

        let mut sides = HittableList::new();
        sides.add(HitModel::XYRect(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), material)));
        sides.add(HitModel::XYRect(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), material)));

        sides.add(HitModel::XZRect(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), material)));
        sides.add(HitModel::XZRect(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), material)));

        sides.add(HitModel::YZRect(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), material)));
        sides.add(HitModel::YZRect(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), material)));

        Box { box_min, box_max, sides }
    }
}

impl Hittable for Box<'_> {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        self.sides.hit(r, tmin, tmax)
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}
