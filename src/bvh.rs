use std::{cmp::Ordering, rc::Rc};

use crate::{
    aabb::{surrounding_box, AABB},
    hittable::{HitRecord, Hittable},
    ray::Ray,
    utility::random_int_range,
    vec3,
};

// Abstract tree structure to represent bounding volumes hierarchy
pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(
        objects: &mut Vec<Rc<dyn Hittable>>,
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
        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;
        if object_span == 1 {
            left = Rc::clone(&objects[start]);
            right = Rc::clone(&objects[start]);
        } else if object_span == 2 {
            let first = Rc::clone(&objects[start]);
            let second = Rc::clone(&objects[start + 1]);

            if let Ordering::Less = comparator(&first, &second) {
                left = first;
                right = second;
            } else {
                left = second;
                right = first;
            }
        } else {
            objects.sort_by(comparator);

            let mid = start + object_span / 2;
            left = Rc::new(BVHNode::new(objects, start, mid, t0, t1));
            right = Rc::new(BVHNode::new(objects, mid, end, t0, t1));
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

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<HitRecord> {
        if self.bbox.hit(r, tmin, tmax) {
            let hit_left = self.left.hit(r, tmin, tmax);
            let hit_right = self.right.hit(r, tmin, tmax);

            if hit_left.is_some() {
                hit_left
            } else {
                hit_right
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}

// For comparison functions, choose to intentionally panic if unable to properly order objects
fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> Option<Ordering> {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        eprintln!("No bounding box in BVHNode constructor.\n");
    }

    let box_a = vec3::unpack(box_a.unwrap().min());
    let box_b = vec3::unpack(box_b.unwrap().min());

    box_a[axis].partial_cmp(&box_b[axis])
}

fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0).unwrap()
}

fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1).unwrap()
}

fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2).unwrap()
}
