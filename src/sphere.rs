use crate::material::Material;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Sphere<M: Material> {
    center: Vec3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f64, material: M) -> Self {
        Self { center, radius, material }
    }

    /// Create HitRecord for an intersection with a ray. Helper for Hittable trait.
    fn hit_helper(&self, t: f64, r: &Ray) -> HitRecord {
        let p = r.point_at(t);

        let outward_norm = (p - self.center) / self.radius;
        let mut front_face = r.direction.dot(&outward_norm) > 0.0;
        let norm: Vec3;

        if front_face {
            norm = -outward_norm;
            front_face = false;
        } else {
            norm = outward_norm;
            front_face = true;
        }
        HitRecord::new(t, p, norm, front_face, &self.material)
    }
}

impl<M: Material> Hittable for Sphere<M> {
    /// Calculate roots for an Sphere intersection using quadratic formula.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;

        // Calculate discriminant
        let a = r.direction.mag_sqr();
        let half_b = oc.dot(&r.direction);
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
}
