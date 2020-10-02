use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Vec3 reperesents a 3D Vector with each field being an f64.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Create a Vec3 with each field (x, y, z) being a randomised f64. Distribution is from 0 to 1.
    pub fn random(distribution: &Uniform<f64>, rng_thread: &mut ThreadRng) -> Self {
        Self {
            x: (distribution.sample(rng_thread) - 0.5) * 2.0, // turn num from dist into -1..1 range
            y: (distribution.sample(rng_thread) - 0.5) * 2.0,
            z: (distribution.sample(rng_thread) - 0.5) * 2.0,
        }
    }

    /// Create  random unit vector for true lambertian reflection.
    pub fn random_lambert(distribution: &Uniform<f64>, rng_thread: &mut ThreadRng) -> Self {
        let a = distribution.sample(rng_thread) * 2.0 * std::f64::consts::PI; // random f64 between 0 and 2pi
        let z = (distribution.sample(rng_thread) - 0.5) * 2.0; // random f64 between -1 and 1
        let r = (1.0 - z * z).sqrt();

        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }

    /// Create a random Vec3 within a unit_sphere (for an approximation at lambertian reflection).
    pub fn random_in_unit_sphere(distribution: &Uniform<f64>, rng_thread: &mut ThreadRng) -> Self {
        loop {
            let p = Self::random(distribution, rng_thread);
            if p.mag_sqr() < 1.0 {
                return p;
            }
        }
    }

    /// The square of the magnitude of the vector. Used to calculate the magnitude. x*x + y*y + z*z.
    pub fn mag_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// The magnitude (size/length) of the vector. Square root of mag_sqr().
    pub fn mag(&self) -> f64 {
        self.mag_sqr().sqrt()
    }

    /// Set the values of the vector such that the magnitude is 1.
    pub fn norm(&self) -> Self {
        *self / self.mag()
    }

    /// Calculate the cross product. If |A| and |B| are 1, the dot product is the Cosine of the angle between them.
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Calculate a vector perpendicular to the two given vectors.
    pub fn cross(&self, rhs: &Vec3) -> Self {
        Self {
            x: self.y * rhs.z - rhs.y * self.z,
            y: self.z * rhs.x - rhs.z * self.x,
            z: self.x * rhs.y - rhs.x * self.y,
        }
    }
}

/// Negation trait for Vec3.
impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
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
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Addition with an f64 trait.
impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
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
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
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
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
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
