use crate::vec;
use crate::ray;

/// All shapes have to implement the Hittable trait in order to calculate ray intersections.
pub trait Hittable {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


/// A HitRecord records a collosion between an object and a ray.
pub struct HitRecord {
    pub t: f64,
    pub p: vec::Vec3,
    pub norm: vec::Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: vec::Vec3, norm: vec::Vec3, front_face: bool) -> Self {
        Self { t, p, norm, front_face }
    }
}

/// A HittableList stores a collection of HitRecords and has functionality for finding the closes hit to the camera.
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(list: Vec<Box<Hittable>>) -> Self {
        HittableList { list }
    }
}


impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
}
