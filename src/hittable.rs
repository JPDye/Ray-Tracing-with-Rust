use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

/// All shapes have to implement the Hittable trait in order to calculate ray intersections.
pub trait Hittable: Sync {
    /// Calculate if an object was intersected.
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord>;

    /// Calculate the bounding box for an object.
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

/// A HitRecord records a collision between an object and a ray.
#[derive(Copy, Clone)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f64,
        p: Vec3,
        normal: Vec3,
        front_face: bool,
        material: &'a dyn Material,
    ) -> Self {
        Self {
            t,
            p,
            normal,
            front_face,
            material,
        }
    }
}

/// A HittableList stores a collection of HitRecords and has functionality for finding the closes hit to the camera.
pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { list: Vec::new() }
    }

    pub fn push(&mut self, item: Box<dyn Hittable>) {
        self.list.push(item);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_obj: Option<HitRecord> = None;
        let mut closest = t_max;

        for hittable in self.list.iter() {
            if let Some(hit) = hittable.hit(r, t_min, closest) {
                closest = hit.t;
                hit_obj = Some(hit);
            }
        }
        hit_obj
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        match self.list.first() {
            Some(first) => {
                match first.bounding_box(t0, t1) {
                    Some(bbox) => self.list.iter().skip(1).try_fold(bbox, |acc, hittable| {
                        match hittable.bounding_box(t0, t1) {
                            Some(bbox) => Some(acc.merge(bbox)),
                            _ => None,
                        }
                    }),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
