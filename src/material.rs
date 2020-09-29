use rand::distributions::Uniform;
use rand::rngs::ThreadRng;

use crate::hittable::HitRecord;
use crate::colour::Colour;
use crate::vec::Vec3;
use crate::ray::Ray;


pub trait Material {
    /// Given an input ray and a record of a collision, calculate the reflected ray and the Colour of the point.
    fn scatter(&self, rec: &HitRecord, ray: &Ray, dist: &Uniform<f64>, rng: &mut ThreadRng) -> Option<(Ray, Colour)>;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self { Self { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, rec: &HitRecord, _ray: &Ray, dist: &Uniform<f64>, rng: &mut ThreadRng) -> Option<(Ray, Colour)> {
        let scattered_ray = Ray::new(rec.p, rec.p + rec.norm + Vec3::random_lambert(dist, rng));
        Some((scattered_ray, self.albedo))
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self { Self { albedo, fuzz } }
}

impl Material for Metal {
    fn scatter(&self, rec: &HitRecord, ray: &Ray, dist: &Uniform<f64>, rng: &mut ThreadRng) -> Option<(Ray, Colour)> {
        let reflected_ray = Vec3::reflect(&ray.direction.norm(), &rec.norm);

        if reflected_ray.dot(&rec.norm) > 0.0 {
            let scattered_ray = Ray::new(rec.p, reflected_ray + Vec3::random_in_unit_sphere(dist, rng) * self.fuzz);
            Some((scattered_ray, self.albedo))
        } else {
            None
        }
    }
}


