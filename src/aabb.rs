use crate::vec::Vec3;

use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn merge(self, other: AABB) -> Self {
        AABB {
            min: self.min.zip_with(other.min, f64::min),
            max: self.max.zip_with(other.max, f64::max),
        }
    }

    pub fn hit(&self, ray: &Ray, t_range: std::ops::Range<f64>) -> bool {
        let inv_d = ray.direction.map(|x| 1.0 / x);
        let t0 = (self.min - ray.origin) * inv_d;
        let t1 = (self.max - ray.origin) * inv_d;

        let (t0, t1) = (
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0.0 { b } else { a }),
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0.0 { a } else { b }),
        );

        let start = t_range.start.max(t0.reduce(f64::max));
        let end = t_range.end.min(t1.reduce(f64::max));
        end > start
    }
}

///// Compute the bounding box of two bounding boxes.
//pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
//let min = Vec3(
//f64::min(box0.min[X], box1.min[X]),
//f64::min(box0.min[Y], box1.min[Y]),
//f64::min(box0.min[Z], box1.min[Z]),
//);

//let max = Vec3(
//f64::max(box0.max[X], box1.max[X]),
//f64::max(box0.max[Y], box1.max[X]),
//f64::max(box0.max[Z], box1.max[Z]),
//);
//AABB { min, max }
//}

//pub struct AABB {
//pub min: Vec3,
//pub max: Vec3,
//}

//impl AABB {
//pub fn new(min: Vec3, max: Vec3) -> Self {
//Self { min, max }
//}

//}
//}
