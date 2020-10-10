use crate::ray::Ray;
use crate::vec::Vec3;

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

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let inv_d = ray.direction.map(|x| 1.0 / x);
        let t0 = (self.min - ray.origin) * inv_d;
        let t1 = (self.max - ray.origin) * inv_d;

        let (t0, t1) = (
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0.0 { b } else { a }),
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0.0 { a } else { b }),
        );

        let start = t_min.max(t0.reduce(f64::max));
        let end = t_max.min(t1.reduce(f64::min));
        end > start
    }
}
