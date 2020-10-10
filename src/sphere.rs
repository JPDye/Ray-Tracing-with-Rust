use crate::aabb::AABB;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;

use crate::vec::Axis::{self, *};
use crate::vec::Vec3;

use std::f64::consts::PI;

fn get_sphere_uv(p: Vec3) -> (f64, f64) {
    let phi = p[Z].atan2(p[X]);
    let theta = p[Y].asin();
    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + PI / 2.0) / PI;
    (u, v)
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere<M: Material> {
    center: Vec3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    #[allow(dead_code)]
    pub fn new(center: Vec3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    /// Create HitRecord for an intersection with a ray. Helper for Hittable trait.
    fn hit_helper(&self, t: f64, r: &Ray) -> HitRecord {
        let p = r.point_at(t);

        let outward_norm = (p - self.center) / self.radius;
        let mut front_face = r.direction.dot(outward_norm) > 0.0;
        let norm: Vec3;

        if front_face {
            norm = -outward_norm;
            front_face = false;
        } else {
            norm = outward_norm;
            front_face = true;
        }

        let (u, v) = get_sphere_uv((p - self.center) / self.radius);
        HitRecord::new(u, v, t, p, norm, front_face, &self.material)
    }
}

impl<M: Material> Hittable for Sphere<M> {
    /// Calculate roots for an Sphere intersection using quadratic formula.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;

        // Calculate discriminant
        let a = r.direction.mag_sqr();
        let half_b = oc.dot(r.direction);
        let c = oc.mag_sqr() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // Calculate roots
        if discriminant > 0.0 {
            let disc_root = discriminant.sqrt();

            // Return for one of the possible roots
            let t = (-half_b - disc_root) / a;
            if t < t_max && t > t_min {
                return Some(self.hit_helper(t, r));
            }

            // Return for the other possible root
            let t = (-half_b + disc_root) / a;
            if t < t_max && t > t_min {
                return Some(self.hit_helper(t, r));
            }
        }
        // Return if no roots
        None
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let radius = Vec3(self.radius, self.radius, self.radius);
        let aabb = AABB {
            min: self.center - radius,
            max: self.center + radius,
        };
        Some(aabb)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MovingSphere<M: Material> {
    radius: f64,
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    material: M,
}

impl<M: Material> MovingSphere<M> {
    #[allow(dead_code)]
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: M,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    /// Return the center of the sphere at a specific point in time.
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }

    /// Create HitRecord for an intersection with a ray. Helper for Hittable trait.
    fn hit_helper(&self, t: f64, r: &Ray) -> HitRecord {
        let p = r.point_at(t);

        let outward_norm = (p - self.center(r.time)) / self.radius;
        let mut front_face = r.direction.dot(outward_norm) > 0.0;
        let norm: Vec3;

        if front_face {
            norm = -outward_norm;
            front_face = false;
        } else {
            norm = outward_norm;
            front_face = true;
        }

        let (u, v) = get_sphere_uv((p - self.center(r.time)) / self.radius);
        HitRecord::new(u, v, t, p, norm, front_face, &self.material)
    }
}

impl<M: Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center(r.time);

        // Calculate discriminant
        let a = r.direction.mag_sqr();
        let half_b = oc.dot(r.direction);
        let c = oc.mag_sqr() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        // Calculate roots
        if discriminant > 0.0 {
            let disc_root = discriminant.sqrt();

            // Return for one of the possible roots
            let t = (-half_b - disc_root) / a;
            if t < t_max && t > t_min {
                return Some(self.hit_helper(t, r));
            }

            // Return for the other possible root
            let t = (-half_b + disc_root) / a;
            if t < t_max && t > t_min {
                return Some(self.hit_helper(t, r));
            }

            // Return for the other possible root
            let t = (-half_b + disc_root) / a;
            if t < t_max && t > t_min {
                return Some(self.hit_helper(t, r));
            }
        }
        None
    }

    /// Calculate the bounding box of the object. Combines the bounding box for the start and end positions.
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let aabb1 = AABB {
            min: self.center(t0) - Vec3::from(self.radius),
            max: self.center(t0) + Vec3::from(self.radius),
        };

        let aabb2 = AABB {
            min: self.center(t1) - Vec3::from(self.radius),
            max: self.center(t1) + Vec3::from(self.radius),
        };

        let output_box = aabb1.merge(aabb2);
        Some(output_box)
    }
}
