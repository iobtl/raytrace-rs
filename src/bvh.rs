use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::{surrounding_box, AABB},
    hittable::{HitModel, HitRecord, Hittable},
    ray::Ray,
    utility::random_int_range,
    vec3,
};

// Abstract tree structure to represent bounding volumes hierarchy
#[derive(Clone)]
pub struct BVHNode<'a> {
    left: Arc<HitModel<'a>>,
    right: Arc<HitModel<'a>>,
    bbox: AABB,
}

impl<'a> BVHNode<'a> {
    pub fn new(
        objects: &mut Vec<Arc<HitModel<'a>>>,
        start: usize,
        end: usize,
        t0: i32,
        t1: i32,
    ) -> Self {
        let axis = random_int_range(&mut rand::thread_rng(), 0, 2) as usize;
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!("Invalid axis found"),
        };

        let object_span = end - start;
        let left: Arc<HitModel<'a>>;
        let right: Arc<HitModel<'a>>;
        if object_span == 1 {
            left = Arc::clone(&objects[start]);
            right = Arc::clone(&objects[start]);
        } else if object_span == 2 {
            let first = Arc::clone(&objects[start]);
            let second = Arc::clone(&objects[start + 1]);

            if let Ordering::Less = comparator(&first, &second) {
                left = first;
                right = second;
            } else {
                left = second;
                right = first;
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;

            // Post-order construction of BVH tree
            left = Arc::new(HitModel::BVH(BVHNode::new(objects, start, mid, t0, t1)));
            right = Arc::new(HitModel::BVH(BVHNode::new(objects, mid, end, t0, t1)));
        }

        let box_left = left.bounding_box(t0 as f32, t1 as f32);
        let box_right = right.bounding_box(t0 as f32, t1 as f32);

        // Current primitives should all have bounding boxes
        if box_left.is_none() || box_right.is_none() {
            eprintln!("No bounding box in BVHNode constructor.\n");
        }

        let bbox = surrounding_box(box_left.unwrap(), box_right.unwrap());

        BVHNode { left, right, bbox }
    }
}

impl Hittable for BVHNode<'_> {
    // Recursively performs sub-dividing of hit models until hit found or not hits found
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        if !self.bbox.hit(r, tmin, tmax) {
            return None;
        }
        let hit_left = self.left.hit(r, tmin, tmax);

        let max_right = if let Some(left_rec) = hit_left { left_rec.t } else { tmax };
        let hit_right = self.right.hit(r, tmin, max_right);
        hit_right.or(hit_left)
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}

// For comparison functions, choose to intentionally panic if unable to properly order objects
fn box_compare<'a>(a: &Arc<HitModel<'a>>, b: &Arc<HitModel<'a>>, axis: usize) -> Option<Ordering> {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        eprintln!("No bounding box in BVHNode constructor.\n");
    }

    let box_a = vec3::unpack(box_a.unwrap().min());
    let box_b = vec3::unpack(box_b.unwrap().min());

    box_a[axis].partial_cmp(&box_b[axis])
}

fn box_x_compare<'a>(a: &Arc<HitModel<'a>>, b: &Arc<HitModel<'a>>) -> Ordering {
    box_compare(a, b, 0).unwrap()
}

fn box_y_compare<'a>(a: &Arc<HitModel<'a>>, b: &Arc<HitModel<'a>>) -> Ordering {
    box_compare(a, b, 1).unwrap()
}

fn box_z_compare<'a>(a: &Arc<HitModel<'a>>, b: &Arc<HitModel<'a>>) -> Ordering {
    box_compare(a, b, 2).unwrap()
}
