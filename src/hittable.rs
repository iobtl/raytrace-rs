use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f32, front_face: bool) -> Self {
        HitRecord { p, normal, t, front_face }
    }

    // Returns true if ray is incident from outside surface, false if from inside surface
    pub fn face_normal(r: &Ray, outward_normal: &Vec3) -> bool {
        let ray_norm = r.direction().dot(outward_normal);
        ray_norm < 0.0
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord>;
}

// Using generics implementation since only dealing with spheres for now
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
}