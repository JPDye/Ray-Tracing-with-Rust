use crate::vec;
use crate::ray;

pub struct Camera {
    origin: vec::Vec3,
    vertical: vec::Vec3,
    horizontal: vec::Vec3,
    lower_left_corner: vec::Vec3,
}

impl Camera {
    pub fn default() -> Self {
        let focal_length = 1.0;
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = vec::Vec3::new(0.0, 0.0, 0.0);
        let vertical = vec::Vec3::new(0.0, viewport_height, 0.0);
        let horizontal = vec::Vec3::new(viewport_width, 0.0, 0.0);
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - vec::Vec3::new(0.0, 0.0, focal_length);

        Self { origin, vertical, horizontal, lower_left_corner }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::Ray::new(self.origin, self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin)
    }
}
