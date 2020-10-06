use std::cmp::Ordering;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};

use crate::ray::Ray;
use crate::vec::Axis;

pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(hittables: &mut Vec<Box<dyn Hittable>>, t0: f64, t1: f64) -> Self {
        fn box_compare(
            axis: Axis,
            t0: f64,
            t1: f64,
        ) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) -> Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box(t0, t1);
                let b_bbox = b.bounding_box(t0, t1);

                if a_bbox.is_none() || b_bbox.is_none() {
                    panic!("no bounding box in bvh node");
                }

                if a_bbox.unwrap().min[axis] - b_bbox.unwrap().min[axis] < 0.0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
        }

        let axis = rand::random::<Axis>();
        hittables.sort_unstable_by(box_compare(axis, t0, t1));

        let len = hittables.len();
        match len {
            0 => panic!("no elements in scene"),
            1 => {
                let hittable = hittables.pop().unwrap();
                if let Some(bbox) = hittable.bounding_box(t0, t1) {
                    BVHNode {
                        left: hittable,
                        right: hittable,
                        bbox
                    }
                } else {
                    panic!("no bounding box in bvh node");
                }
            },

            _ => {
                let right = Box::new(BVHNode::new(&mut hittables.drain(len / 2..).collect(), t0, t1));
                let left = Box::new(BVHNode::new(hittables, t0, t1));
                let bbox = left.bbox.merge(right.bbox);
                BVHNode { left: left as Box<dyn Hittable>, right: right as Box<dyn Hittable>, bbox }
            }
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t0..t1) {
            return None;
        }

        let left = self.left.hit(r, t0, t1);
        let right = self.right.hit(r, t0, t1);

        match (left, right) {
            (Some(l), Some(r)) => {
                if l.t < r.t {
                    Some(l)
                } else {
                    Some(r)
                }
            }
            (Some(h), None) => Some(h),
            (None, Some(h)) => Some(h),
            _ => None,
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}

//pub struct BVH {
//bounding_box: AABB,
//size: usize,
//contents: BVHContents,
//}

//pub enum BVHContents {
//Node { left: Box<BVH>, right: Box<BVH> },
//Leaf(Box<dyn Hittable>),
//}

//impl BVH {
//pub fn new(mut objs: Vec<Box<dyn Hittable>>, exposure: Range<f64>) -> Self {
//fn axis_range(objs: &[Box<dyn Hittable>], exposure: Range<f64>, axis: Axis) -> f64 {
//let range = objs.iter().fold(std::f64::MAX..std::f64::MIN, |range, o| {
//let bb = o.bounding_box(&exposure).unwrap();
//let min = bb.min[axis].min(bb.max[axis]);
//let max = bb.min[axis].max(bb.max[axis]);
//range.start.min(min)..range.end.max(max)
//});
//range.end - range.start
//}

//let axis = {
//let mut ranges = [
//(X, axis_range(&objs, exposure.clone(), X)),
//(Y, axis_range(&objs, exposure.clone(), Y)),
//(Z, axis_range(&objs, exposure.clone(), Z)),
//];
//ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
//ranges[0].0
//};

//objs.sort_unstable_by(|a, b| {
//let abb = a.bounding_box(&exposure).unwrap();
//let bbb = b.bounding_box(&exposure).unwrap();
//let av = abb.min[axis] + abb.max[axis];
//let bv = bbb.min[axis] + bbb.max[axis];
//av.partial_cmp(&bv).unwrap()
//});

//match objs.len() {
//0 => panic!("Can't create a BVH from zero objects."),
//1 => BVH {
//bounding_box: objs[0].bounding_box(&exposure).unwrap(),
//size: 1,
//contents: BVHContents::Leaf(objs.pop().unwrap()),
//},
//_ => {
//let right = Box::new(BVH::new(
//objs.drain(objs.len() / 2..).collect(),
//exposure.clone(),
//));

//let left = Box::new(BVH::new(objs, exposure.clone()));

//BVH {
//bounding_box: left.bounding_box.merge(right.bounding_box),
//size: left.size + right.size,
//contents: BVHContents::Node { left, right },
//}
//}
//}
//}
//}

//impl Hittable for BVH {
//fn hit(&self, ray: &Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
//// Return 'None' if there is no collision
//if !self.bounding_box.hit(ray, t_min..t_max) {
//return None
//}

//// If there is a collision, return result of collision with child bounding boxes.
//match &self.contents {
//BVHContents::Node { left, right } => {
//let hit_left = left.hit(ray, t_min, t_max);
//if let Some(h) = &hit_left {
//t_max = h.t;
//}

//let hit_right = right.hit(ray, t_min, t_max);

//match (hit_left, hit_right) {
//(h, None) | (None, h) => h,
//(Some(h1), Some(h2)) => {
//if h1.t < h2.t {
//Some(h1)
//} else {
//Some(h2)
//}
//}
//}
//}
//// Base case for ending recursion. Return 'HitRecord' for collision with object.
//BVHContents::Leaf(obj) => obj.hit(ray, t_min, t_max),
//}
//}

//fn bounding_box(&self, exposure: &Range<f64>) -> Option<AABB> {
//Some(self.bounding_box)
//}
//}
