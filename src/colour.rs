use std::fmt;

use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

/// Colour represents an RGB Colour with each field being an f64 between 0 and 1.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

/// Print Colour as an RGB tuple with each field a U8 between 0 and 255.
impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = self.r.sqrt();
        let g = self.g.sqrt();
        let b = self.b.sqrt();

        let ir = std::cmp::min((255.0 * r) as u8, 255);
        let ig = std::cmp::min((255.0 * g) as u8, 255);
        let ib = std::cmp::min((255.0 * b) as u8, 255);

        write!(f, "{} {} {}", ir, ig, ib)
    }
}

/// Add Assign trait for Colour. Modifies LHS.
impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

/// Addition trait for Colour. Creates new Colour.
impl Add for Colour {
    type Output = Colour;
    fn add(self, rhs: Self) -> Self::Output {
        Colour {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

/// Mul Assign trait for Colour. Modifies LHS.
impl MulAssign<f64> for Colour {
    fn mul_assign(&mut self, rhs: f64) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

/// Multiplication (with a f64) trait for Colour. Creates new Colour.
impl Mul<f64> for Colour {
    type Output = Colour;
    fn mul(self, rhs: f64) -> Self::Output {
        Colour {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

/// Div Assign trait for Colour. Multiplies by inverse of RHs. Modifies LHS.
impl DivAssign<f64> for Colour {
    fn div_assign(&mut self, rhs: f64) {
        let inv = 1.0 / rhs;
        self.r *= inv;
        self.g *= inv;
        self.b *= inv;
    }
}

/// Division (with a f64) trait for Colour. Multiplies by inverse of RHS. Creates new Colour.
impl Div<f64> for Colour {
    type Output = Colour;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
