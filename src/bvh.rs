use std::cmp::Ordering;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};

use crate::ray::Ray;
use crate::vec::Axis::{self, *};

pub enum BVHNode {
    Branch { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>)
}

pub struct BVH {
    tree: BVHNode,
    bbox: AABB,
}

impl BVH {
    pub fn new(mut hittables: Vec<Box<dyn Hittable>>, t0: f64, t1: f64) -> Self {
        fn box_compare(
            axis: Axis,
            t0: f64,
            t1: f64,
        ) -> impl FnMut(&Box<dyn Hittable>, &Box<dyn Hittable>) ->  Ordering {
            move |a, b| {
                let a_bbox = a.bounding_box(t0, t1);
                let b_bbox = b.bounding_box(t0, t1);

                if let (Some(a), Some(b)) = (a_bbox, b_bbox) {
                    let ac = a.min[axis] + a.max[axis];
                    let bc = b.min[axis] + b.max[axis];
                    ac.partial_cmp(&bc).unwrap()
                } else {
                    panic!("no bounding box in bvh node");
                }
            }
        }

        fn axis_range(hittables: &Vec<Box<dyn Hittable>>, t0: f64, t1: f64, axis: Axis) -> f64 {
            let (min, max) = hittables.iter().fold((std::f64::MAX, std::f64::MIN), |(bmin, bmax), hit| {
                if let Some(aabb) = hit.bounding_box(t0, t1) {
                    (bmin.min(aabb.min[axis]), bmax.max(aabb.max[axis]))
                } else {
                    (bmin, bmax)
                }
            });
            max - min
        }

        let mut axis_ranges: Vec<(Axis, f64)> = Vec::new();
        axis_ranges.push((X, axis_range(&hittables, t0, t1, X)));
        axis_ranges.push((Y, axis_range(&hittables, t0, t1, Y)));
        axis_ranges.push((Z, axis_range(&hittables, t0, t1, Z)));
        axis_ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let axis = axis_ranges[0].0;
        hittables.sort_unstable_by(box_compare(axis, t0, t1));

        let len = hittables.len();
        match len {
            0 => panic!("no elements in scene"),
            1 => {
                let leaf = hittables.pop().unwrap();
                if let Some(bbox) = leaf.bounding_box(t0, t1) {
                    BVH { tree: BVHNode::Leaf(leaf), bbox }
                } else {
                    panic!("no bounding box in bvh node");
                }
            },

            _ => {
                let right = BVH::new(hittables.drain(len / 2..).collect(), t0, t1);
                let left = BVH::new(hittables, t0, t1);
                let bbox = right.bbox.merge(left.bbox);
                BVH { tree: BVHNode::Branch { left: Box::new(left), right: Box::new(right) }, bbox }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t0: f64, mut t1: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t0, t1) {
            return None;
        }

        match &self.tree {
            BVHNode::Leaf(leaf) => leaf.hit(r, t0, t1),
            BVHNode::Branch { left, right } => {
                let left = left.hit(r, t0, t1);
                if let Some(l) = left {
                    t1 = l.t
                }

                let right = right.hit(r, t0, t1);
                if right.is_some() { right } else { left }
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}
