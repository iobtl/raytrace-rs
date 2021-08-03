use crate::{
    aabb::{surrounding_box, AABB},
    bvh::BVHNode,
    instances::{RotateY, Translate},
    material::Surface,
    ray::Ray,
    rect::{Box, XYRect, XZRect, YZRect},
    sphere::{MovingSphere, Sphere},
    vec3::{Point3, Vec3},
    volumes::Constant,
};

#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a Surface<'a>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f32,
        u: f32,
        v: f32,
        front_face: bool,
        material: &'a Surface,
    ) -> Self {
        HitRecord { p, normal, material, t, u, v, front_face }
    }

    // Returns true if ray is incident from outside surface, false if from inside surface
    pub fn face_normal(r: &Ray, outward_normal: &Vec3) -> bool {
        let ray_norm = r.direction().dot(outward_normal);
        ray_norm < 0.0
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}
// Wrapper class to avoid dealing with trait objects
#[derive(Clone)]
pub enum HitModel<'a> {
    Sphere(Sphere<'a>),
    MovingSphere(MovingSphere<'a>),
    BVH(BVHNode<'a>),
    XYRect(XYRect<'a>),
    XZRect(XZRect<'a>),
    YZRect(YZRect<'a>),
    Box(Box<'a>),
    Translate(Translate<'a>),
    RotateY(RotateY<'a>),
    Constant(Constant<'a>),
}

impl Hittable for HitModel<'_> {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        match self {
            Self::Sphere(sphere) => sphere.hit(r, tmin, tmax),
            Self::MovingSphere(sphere) => sphere.hit(r, tmin, tmax),
            Self::BVH(bvh) => bvh.hit(r, tmin, tmax),
            Self::XYRect(rect) => rect.hit(r, tmin, tmax),
            Self::XZRect(rect) => rect.hit(r, tmin, tmax),
            Self::YZRect(rect) => rect.hit(r, tmin, tmax),
            Self::Box(_box) => _box.hit(r, tmin, tmax),
            Self::Translate(translate) => translate.hit(r, tmin, tmax),
            Self::RotateY(rotate) => rotate.hit(r, tmin, tmax),
            Self::Constant(volume) => volume.hit(r, tmin, tmax),
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        match self {
            Self::Sphere(sphere) => sphere.bounding_box(t0, t1),
            Self::MovingSphere(sphere) => sphere.bounding_box(t0, t1),
            Self::BVH(bvh) => bvh.bounding_box(t0, t1),
            Self::XYRect(rect) => rect.bounding_box(t0, t1),
            Self::XZRect(rect) => rect.bounding_box(t0, t1),
            Self::YZRect(rect) => rect.bounding_box(t0, t1),
            Self::Box(_box) => _box.bounding_box(t0, t1),
            Self::Translate(translate) => translate.bounding_box(t0, t1),
            Self::RotateY(rotate) => rotate.bounding_box(t0, t1),
            Self::Constant(volume) => volume.bounding_box(t0, t1),
        }
    }
}

// Using generics implementation since only dealing with spheres for now
#[derive(Clone)]
pub struct HittableList<T> {
    objects: Vec<T>,
}

impl<T> HittableList<T>
where
    T: Hittable,
{
    pub fn new() -> Self {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: T) {
        Vec::push(&mut self.objects, object);
    }

    pub fn objects(&mut self) -> &mut Vec<T> {
        &mut self.objects
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_t = tmax;

        // Returns closest object hit among list of hittable objects
        for object in self.objects.iter() {
            match object.hit(r, tmin, closest_t) {
                Some(hit_rec) => {
                    closest_t = hit_rec.t;
                    temp_rec = Some(hit_rec);
                }
                None => (),
            }
        }

        temp_rec
    }

    pub fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            None
        } else {
            let mut temp_box: Option<AABB> = None;

            for object in self.objects.iter() {
                match object.bounding_box(t0, t1) {
                    Some(bbox) if temp_box.is_none() => {
                        temp_box = Some(bbox);
                    }
                    Some(bbox) => {
                        temp_box = Some(surrounding_box(bbox, temp_box.unwrap()));
                    }
                    // Every object should have a bounding box
                    None => (),
                }
            }

            temp_box
        }
    }
}
