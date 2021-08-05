use crate::{
    aabb::AABB,
    hittable::{HitModel, HitRecord, Hittable},
    ray::Ray,
    utility::{degrees_to_radians, INFINITY},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Translate<'a> {
    hit_model: Box<HitModel<'a>>,
    offset: Vec3,
}

impl<'a> Translate<'a> {
    pub fn new(hit_model: HitModel<'a>, offset: Vec3) -> Self {
        Translate { hit_model: Box::new(hit_model), offset }
    }
}

impl Hittable for Translate<'_> {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let origin = *r.origin();
        let direction = *r.direction();
        let time = r.time();

        // Note: we subtract from the ray origin in this case, instead of modifying
        // the object coordinates
        let moved_r = Ray::new(origin - self.offset, direction, time);
        if let Some(mut hit_rec) = self.hit_model.hit(&moved_r, tmin, tmax) {
            hit_rec.p += self.offset; // add offset again?
            let front_face = HitRecord::face_normal(&moved_r, &hit_rec.normal);

            if front_face {
                Some(hit_rec)
            } else {
                hit_rec.normal *= -1.0;
                Some(hit_rec)
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(bbox) = self.hit_model.bounding_box(t0, t1) {
            Some(AABB::new(*bbox.min() + self.offset, *bbox.max() + self.offset))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct RotateY<'a> {
    hit_model: Box<HitModel<'a>>,
    sin_theta: f32,
    cos_theta: f32,
    has_box: bool,
    bbox: Option<AABB>,
}

impl<'a> RotateY<'a> {
    pub fn new(hit_model: HitModel<'a>, angle: f32) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = hit_model.bounding_box(0.0, 1.0);
        let has_box = bbox.is_some();

        let mut min: [f32; 3] = [INFINITY; 3];
        let mut max: [f32; 3] = [INFINITY, -INFINITY, -INFINITY];

        if has_box {
            let bbox = bbox.unwrap();
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let i = i as f32;
                        let j = j as f32;
                        let k = k as f32;

                        let x = i * bbox.max().x() + (1.0 - i) * bbox.min().x();
                        let y = j * bbox.max().y() + (1.0 - i) * bbox.min().y();
                        let z = k * bbox.max().z() + (1.0 - i) * bbox.min().z();

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester: [f32; 3] = [new_x, y, new_z];

                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }

            let min = Vec3::new(min[0], min[1], min[2]);
            let max = Vec3::new(max[0], max[1], max[2]);
            let bbox = Some(AABB::new(min, max));

            RotateY { hit_model: Box::new(hit_model), sin_theta, cos_theta, has_box, bbox }
        } else {
            RotateY { hit_model: Box::new(hit_model), sin_theta, cos_theta, has_box, bbox }
        }
    }
}

impl<'a> Hittable for RotateY<'a> {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        let origin = *r.origin();
        let direction = *r.direction();
        let cos_theta = self.cos_theta;
        let sin_theta = self.sin_theta;

        // Changing ray origin
        let new_origin_x = cos_theta * origin.x() - sin_theta * origin.z();
        let new_origin_z = sin_theta * origin.x() + cos_theta * origin.z();

        let new_dir_x = cos_theta * direction.x() - sin_theta * direction.z();
        let new_dir_z = sin_theta * direction.x() + cos_theta * direction.z();

        let rotated_r = Ray::new(
            Vec3::new(new_origin_x, origin.y(), new_origin_z),
            Vec3::new(new_dir_x, direction.y(), new_dir_z),
            r.time(),
        );

        if let Some(mut hit_rec) = self.hit_model.hit(&rotated_r, tmin, tmax) {
            let new_p = Vec3::new(
                cos_theta * hit_rec.p.x() + sin_theta * hit_rec.p.z(),
                hit_rec.p.y(),
                -sin_theta * hit_rec.p.x() + cos_theta * hit_rec.p.z(),
            );

            let new_normal = Vec3::new(
                cos_theta * hit_rec.normal.x() + sin_theta * hit_rec.normal.z(),
                hit_rec.normal.y(),
                -sin_theta * hit_rec.normal.x() + cos_theta * hit_rec.normal.z(),
            );

            hit_rec.p = new_p;
            let front_face = HitRecord::face_normal(&rotated_r, &new_normal);
            if front_face {
                Some(hit_rec)
            } else {
                hit_rec.normal *= -1.0;
                Some(hit_rec)
            }
        } else {
            None
        }
    }
    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        self.bbox
    }
}

#[derive(Clone)]
pub struct FlipFace<'a> {
    hit_model: Box<HitModel<'a>>,
}

impl<'a> FlipFace<'a> {
    pub fn new(hit_model: HitModel<'a>) -> Self {
        FlipFace { hit_model: Box::new(hit_model) }
    }
}

impl Hittable for FlipFace<'_> {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        // Flip light faces so normals point in -y direction.
        self.hit_model.hit(r, tmin, tmax).and_then(|mut rec| {
            rec.front_face = !rec.front_face;
            Some(rec)
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.hit_model.bounding_box(t0, t1)
    }
}
