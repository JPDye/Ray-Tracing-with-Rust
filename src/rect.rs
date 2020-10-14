use crate::aabb::AABB;

use crate::ray::Ray;

use crate::vec::Vec3;
use crate::vec::Axis::{self, *};

use crate::material::Material;
use crate::hittable::{Hittable, HitRecord};

/// Rectangle aligned along the Z-axis.
pub struct XYRect<M: Material> {
    albedo: M,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,

}

impl<M: Material> XYRect<M> {
    #[allow(dead_code)]
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, albedo: M) -> Self {
        Self { x0, x1, y0, y1, k, albedo }
    }
}

impl<M: Material> Hittable for XYRect<M> {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin[Z]) / r.direction[Z];
        if t < t0 || t > t1 {
            return None;
        }

        let x = r.origin[X] + t * r.direction[X];
        let y = r.origin[Y] + t * r.direction[Y];
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let p = r.point_at(t);

        let front_face = true;
        let mut outward_norm = Vec3(0.0, 0.0, 1.0);
        if r.direction.dot(outward_norm) > 0.0 {
            outward_norm = -outward_norm;
        }

        Some(HitRecord::new(u, v, t, p, outward_norm, front_face, &self.albedo))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Vec3(self.x0, self.y0, self.k-0.0001),
            max: Vec3(self.x1, self.y1, self.k+0.0001),
        })
    }

}

/// Rectangle aligned along the Y-axis.
pub struct XZRect<M: Material> {
    albedo: M,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,

}

impl<M: Material> XZRect<M> {
    #[allow(dead_code)]
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, albedo: M) -> Self {
        Self { x0, x1, z0, z1, k, albedo }
    }
}

impl<M: Material> Hittable for XZRect<M> {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin[Y]) / r.direction[Y];
        if t < t0 || t > t1 {
            return None;
        }

        let x = r.origin[X] + t * r.direction[X];
        let z = r.origin[Z] + t * r.direction[Z];
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = r.point_at(t);

        let front_face = true;
        let mut outward_norm = Vec3(0.0, 1.0, 0.0);
        if r.direction.dot(outward_norm) > 0.0 {
            outward_norm = -outward_norm;
        }

        Some(HitRecord::new(u, v, t, p, outward_norm, front_face, &self.albedo))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Vec3(self.x0, self.k-0.0001, self.z0),
            max: Vec3(self.x1, self.k+0.0001, self.z1),
        })
    }

}

/// Rectangle aligned along the X-axis.
pub struct YZRect<M: Material> {
    albedo: M,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,

}

impl<M: Material> YZRect<M> {
    #[allow(dead_code)]
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, albedo: M) -> Self {
        Self { y0, y1, z0, z1, k, albedo }
    }
}

impl<M: Material> Hittable for YZRect<M> {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin[X]) / r.direction[X];
        if t < t0 || t > t1 {
            return None;
        }

        let y = r.origin[Y] + t * r.direction[Y];
        let z = r.origin[Z] + t * r.direction[Z];
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = r.point_at(t);

        let front_face = true;
        let mut outward_norm = Vec3(1.0, 0.0, 0.0);
        if r.direction.dot(outward_norm) > 0.0 {
            outward_norm = -outward_norm;
        }

        Some(HitRecord::new(u, v, t, p, outward_norm, front_face, &self.albedo))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Vec3(self.k-0.0001, self.y0, self.z0),
            max: Vec3(self.k+0.0001, self.y1, self.z1),
        })
    }

}
