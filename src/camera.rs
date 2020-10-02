use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use crate::ray::Ray;
use crate::vec::Vec3;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg / 180.0 * std::f64::consts::PI
}

pub fn random_in_unit_disk(dist: &Uniform<f64>, rng: &mut ThreadRng) -> Vec3 {
    loop {
        let x = (dist.sample(rng) - 0.5) * 2.0;
        let y = (dist.sample(rng) - 0.5) * 2.0;

        let p = Vec3::new(x, y, 0.0);
        if p.mag_sqr() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub struct Camera {
    origin: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    lower_left_corner: Vec3,
    u: Vec3,
    w: Vec3,
    v: Vec3,
    lens_radius: f64,
    dist: Uniform<f64>,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = deg_to_rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).norm();
        let u = vup.cross(&w).norm();
        let v = w.cross(&u);

        let origin = look_from;
        let vertical = v * viewport_height * focus_dist;
        let horizontal = u * viewport_width * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius = aperture / 2.0;

        let dist = Uniform::from(time0..time1);

        Self {
            origin,
            vertical,
            horizontal,
            lower_left_corner,
            u,
            w,
            v,
            lens_radius,
            dist,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut ThreadRng) -> Ray {
        let rd = random_in_unit_disk(&self.dist, rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.dist.sample(rng);

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            time,
        )
    }
}
