use crate::hittable;
use crate::vec;
use crate::ray;


pub struct Sphere {
    center: vec::Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: vec::Vec3, radius: f64) -> Self {
        Self { center, radius }
    }

    /// Create HitRecord for an intersection with a ray. Helper for Hittable trait.
    fn hit_helper(&self, t: f64, r: &ray::Ray) -> hittable::HitRecord {
        let p = r.point_at(t);

        let outward_norm = (p - self.center) / self.radius;
        let mut front_face = r.direction.dot(&outward_norm) > 0.0;
        let norm: vec::Vec3;

        if front_face {
            norm = -outward_norm;
            front_face = false;
        } else {
            norm = outward_norm;
            front_face = true;
        }
        hittable::HitRecord::new(t, p, norm, front_face)
    }
}

impl hittable::Hittable for Sphere {
    /// Calculate roots for an Sphere intersection using quadratic formula.
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
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
