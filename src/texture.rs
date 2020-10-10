use crate::colour::Colour;

use crate::vec::Axis::*;
use crate::vec::Vec3;

pub trait Texture: Sync {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour;
}

/// A 'Texture' made from a single colour.
#[derive(Copy, Clone)]
pub struct SolidColour {
    albedo: Colour,
}

impl SolidColour {
    pub fn new(c: Colour) -> Self {
        Self { albedo: c }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Colour {
        self.albedo
    }
}

/// A checkered 'Texture' made from two alternating colours.
#[derive(Copy, Clone)]
pub struct CheckeredTexture<T: Texture> {
    odd: T,
    even: T,
}

impl<T: Texture> CheckeredTexture<T> {
    pub fn new(t0: T, t1: T) -> Self {
        Self { odd: t0, even: t1 }
    }
}

impl<T: Texture> Texture for CheckeredTexture<T> {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Colour {
        let sines = (10.0 * p[X]).sin() * (10.0 * p[Y]).sin() * (10.0 * p[Z]).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
