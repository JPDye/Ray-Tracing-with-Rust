use std::ops::Range;

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};

use crate::ray::Ray;
use crate::vec::Axis::{self, *};


pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(hittables: Vec<Box<dyn Hittable>>, t_min: f64, t_max: f64) -> Self {
        unimplemented!();
    }
}


impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        unimplemented!();
    }

    fn bounding_box(&self, t_min: f64, t_max: f64) -> Option<AABB> {
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
