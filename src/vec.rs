use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::cmp::PartialEq;

/// Vec3 reperesents a 3D Vector with each field being an f64.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// The square of the magnitude of the vector. Used to calculate the magnitude. x*x + y*y + z*z.
    pub fn mag_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// The magnitude (size/length) of the vector. Square root of mag_sqr().
    pub fn mag(&self) -> f64 {
        self.mag_sqr().sqrt()
    }
}

/// Negation trait for Vec3.
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 { x: -self.x, y: -self.y, z: -self.z }
    }
}

/// Add Assign trait for Vec. Modifies LHS.
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}


/// Addition trait for Vec. Creates a new Vec.
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

/// Sub Assign trait for Vec. Modifies LHS.
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

/// Subtraction trait for Vec.
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

/// Mul Assign trait for Vec. Modifies LHs.
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

/// Multiplication (with a f64) trait for Vec. Creates a new Vec.
impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

/// Div Assign trait for Vec. Multiplies by inverse of RHs. Modifies LHS.
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        let inv = 1.0 / rhs;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

/// Division (with a f64) trait for Vec. Multiplies by inverse of RHS. Creates a new Vec.
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
