use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};

use crate::ray::Ray;
use crate::vec::Axis::{self, *};

pub struct BVH {
    bounding_box: AABB,
    size: usize,
    contents: BVHContents,
}

pub enum BVHContents {
    Node { left: Box<BVH>, right: Box<BVH> },
    Leaf(Box<dyn Hittable>),
}

impl BVH {
    pub fn new(mut objs: Vec<Box<dyn Hittable>>, t0: f64, t1: f64) -> Self {
        fn axis_range(objs: &[Box<dyn Hittable>], t0: f64, t1: f64, axis: Axis) -> f64 {
            let range = objs.iter().fold(std::f64::MAX..std::f64::MAX, |range, o| {
                let bb = o.bounding_box(t0, t1).unwrap();
                let min = bb.min[axis].min(bb.max[axis]);
                let max = bb.min[axis].max(bb.max[axis]);
                range.start.min(min)..range.end.max(max)
            });
            range.end - range.start
        }

        let axis = {
            let mut ranges = [
                (X, axis_range(&objs, t0, t1, X)),
                (Y, axis_range(&objs, t0, t1, Y)),
                (Z, axis_range(&objs, t0, t1, Z)),
            ];
            ranges.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            ranges[0].0
        };

        objs.sort_unstable_by(|a, b| {
            let abb = a.bounding_box(t0, t1).unwrap();
            let bbb = b.bounding_box(t0, t1).unwrap();
            let av = abb.min[axis] + abb.max[axis];
            let bv = bbb.min[axis] + bbb.max[axis];
            av.partial_cmp(&bv).unwrap()
        });

        match objs.len() {
            0 => panic!("can't create BVH from 0 objects"),
            1 => BVH {
                bounding_box: objs[0].bounding_box(t0, t1).unwrap(),
                size: 1,
                contents: BVHContents::Leaf(objs.pop().unwrap()),
            },

            _ => {
                let right = Box::new(BVH::new(objs.split_off(objs.len() / 2), t0, t1));
                let left = Box::new(BVH::new(objs, t0, t1));

                BVH {
                    bounding_box: right.bounding_box.merge(left.bounding_box),
                    size: left.size + right.size,
                    contents: BVHContents::Node { left, right },
                }
            }
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, r: &Ray, t0: f64, mut t1: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, t0, t1) {
            return None;
        }

        match &self.contents {
            BVHContents::Leaf(obj) => obj.hit(r, t0, t1),
            BVHContents::Node { left, right } => {
                let hit_left = left.hit(r, t0, t1);
                if let Some(h) = &hit_left {
                    t1 = h.t
                }
                let hit_right = right.hit(r, t0, t1);

                match (hit_left, hit_right) {
                    (h, None) | (None, h) => h,
                    (Some(hl), Some(hr)) => {
                        if hl.t < hr.t {
                            Some(hl)
                        } else {
                            Some(hr)
                        }
                    }
                }
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.bounding_box)
    }
}
