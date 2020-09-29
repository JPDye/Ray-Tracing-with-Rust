use crate::ray::Ray;
use crate::vec::Vec3;


pub fn deg_to_rad(deg: f64) -> f64 {
    deg / 180.0 * std::f64::consts::PI
}

pub struct Camera {
    origin: Vec3,
    vertical: Vec3,
    horizontal: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = deg_to_rad(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).norm();
        let u = vup.cross(&w).norm();
        let v = w.cross(&u);

        let origin = look_from;
        let vertical = v * viewport_height;
        let horizontal = u * viewport_width;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

        Self {
            origin,
            vertical,
            horizontal,
            lower_left_corner,
        }
    }


    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
