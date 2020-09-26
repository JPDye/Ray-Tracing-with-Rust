use crate::vec;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: vec::Vec3,
    pub direction: vec::Vec3,
}

impl Ray {
    pub fn new(origin: vec::Vec3, direction: vec::Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn point_at(&self, t: f64) -> vec::Vec3 {
        self.origin + self.direction * t
    }
}
