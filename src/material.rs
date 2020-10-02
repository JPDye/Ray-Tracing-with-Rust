use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;

/// Reflect an input vector V across the normal N.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (*n * v.dot(&n) * 2.0)
}

/// Refraction
fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.norm();
    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted = (uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

/// Schlick's approximation
fn schlick(cos: f64, ior: f64) -> f64 {
    let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

pub trait Material: Sync {
    /// Given an input ray and a record of a collision, calculate the reflected ray and the Colour of the point.
    fn scatter(
        &self,
        rec: &HitRecord,
        ray: &Ray,
        dist: &Uniform<f64>,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Colour)>;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        rec: &HitRecord,
        ray: &Ray,
        dist: &Uniform<f64>,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Colour)> {
        let scattered_ray = Ray::new(rec.p, rec.norm + Vec3::random_in_unit_sphere(dist, rng), ray.time);
        Some((scattered_ray, self.albedo))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        rec: &HitRecord,
        ray: &Ray,
        dist: &Uniform<f64>,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Colour)> {
        let reflected_ray = reflect(&ray.direction.norm(), &rec.norm);

        if reflected_ray.dot(&rec.norm) > 0.0 {
            let scattered_ray = Ray::new(
                rec.p,
                reflected_ray + Vec3::random_in_unit_sphere(dist, rng) * self.fuzz,
                ray.time,
            );
            Some((scattered_ray, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    ior: f64,
}

impl Dielectric {
    pub fn new(ior: f64) -> Self {
        Self { ior }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        rec: &HitRecord,
        ray: &Ray,
        dist: &Uniform<f64>,
        rng: &mut ThreadRng,
    ) -> Option<(Ray, Colour)> {
        let attenuation = Colour::new(1.0, 1.0, 1.0);

        let ni_over_nt = if rec.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        let unit_direction = ray.direction.norm();
        let cos = (-unit_direction).dot(&rec.norm).min(1.0);

        // Match on whether or not refraction is possible given the ratio between ior's
        match refract(&unit_direction, &rec.norm, ni_over_nt) {
            // If refraction is possible, use Shclick's approximation to choose between reflection and refraction
            Some(refracted) => {
                // If Shlick's approximation says reflection is highly likely, use Uniform dist decide whether to reflect
                let reflect_prob = schlick(cos, ni_over_nt);
                if dist.sample(rng) < reflect_prob {
                    let reflected = reflect(&unit_direction, &rec.norm);
                    let scattered = Ray::new(rec.p, reflected, ray.time);
                    return Some((scattered, attenuation));
                }

                // Otherwise refract the ray
                let scattered = Ray::new(rec.p, refracted, ray.time);
                Some((scattered, attenuation))
            }

            // Reflect the ray if no refraction is possible
            None => {
                let reflected = reflect(&unit_direction, &rec.norm);
                let scattered = Ray::new(rec.p, reflected, ray.time);
                Some((scattered, attenuation))
            }
        }
    }
}
