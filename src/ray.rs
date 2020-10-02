use crate::vec::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self { origin, direction, time }
    }

    pub fn point_at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
