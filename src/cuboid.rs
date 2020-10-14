use crate::material::Material;

use crate::{Hittable, HitRecord, HittableList};
use crate::rect::{XYRect, XZRect, YZRect};

use crate::aabb::AABB;

use crate::ray::Ray;
use crate::vec::Vec3;
use crate::vec::Axis::*;

pub struct Cuboid {
    box_min: Vec3,
    box_max: Vec3,
    sides: HittableList,
}

impl Cuboid {
    #[allow(dead_code)]
    pub fn new<M: Material + Clone + 'static>(p0: Vec3, p1: Vec3, mat: M) -> Self {
        let box_min = p0;
        let box_max = p1;

        let mut sides = HittableList::new();

        sides.push(Box::new(XYRect::new(p0[X], p1[X], p0[Y], p1[Y], p1[Z], mat.clone())));
        sides.push(Box::new(XYRect::new(p0[X], p1[X], p0[Y], p1[Y], p0[Z], mat.clone())));

        sides.push(Box::new(XZRect::new(p0[X], p1[X], p0[Z], p1[Z], p1[Y], mat.clone())));
        sides.push(Box::new(XZRect::new(p0[X], p1[X], p0[Z], p1[Z], p0[Y], mat.clone())));

        sides.push(Box::new(YZRect::new(p0[Y], p1[Y], p0[Z], p1[Z], p1[X], mat.clone())));
        sides.push(Box::new(YZRect::new(p0[Y], p1[Y], p0[Z], p1[Z], p0[X], mat.clone())));

        Self { box_min, box_max, sides }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        self.sides.hit(r, t0, t1)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB { min: self.box_min, max: self.box_max })
    }

}
