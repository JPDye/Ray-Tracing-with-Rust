use rand::distributions::{Distribution, Standard, Uniform};
use rand::prelude::*;

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    /// Generates a random Vec3 within a unit radius. Magnitude of result is between 0 and 1.
    /// Uniform<f64> must be for range 0..1
    pub fn random_in_unit_sphere(dist: &Uniform<f64>, rng: &mut impl Rng) -> Self {
        loop {
            let v =
                2.0 * Vec3(dist.sample(rng), dist.sample(rng), dist.sample(rng)) - Vec3::from(1.0);
            if v.mag_sqr() < 1.0 {
                return v;
            }
        }
    }

    /// Generates a random Vec3 within a unit disc with the radius being in the XY plane.
    /// Magnitude of result is between 0 and 1. Z component is 0.
    /// Uniform<f64> must be for range 0..1
    pub fn random_in_unit_disc(dist: &Uniform<f64>, rng: &mut impl Rng) -> Self {
        loop {
            let v = 2.0 * Vec3(dist.sample(rng), dist.sample(rng), 0.0) - Vec3(1.0, 1.0, 0.0);
            if v.mag_sqr() < 1.0 {
                return v;
            }
        }
    }

    /// Calculate the dot product between 'self' and another vector.
    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    /// Return the dot product of 'self' and 'self'. The magnitude squared.
    #[inline]
    pub fn mag_sqr(&self) -> f64 {
        self.dot(*self)
    }

    /// Return the magnitude (length) of the vector.
    #[inline]
    pub fn mag(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    /// Calculate a vector pointing in the same direction as 'self' but with a length equal to 1.
    #[inline]
    pub fn normalise(self) -> Self {
        self / self.mag()
    }

    /// Applies 'f' to each element of the vector in turn, returning a new vector.
    #[inline]
    pub fn map(self, mut f: impl FnMut(f64) -> f64) -> Self {
        Self(f(self.0), f(self.1), f(self.2))
    }

    /// Combines each corresponding element of 'self' with 'other' by passing them to 'f', returning a new Vec3.
    #[inline]
    pub fn zip_with(self, other: Vec3, mut f: impl FnMut(f64, f64) -> f64) -> Self {
        Self(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2))
    }

    #[inline]
    pub fn zip_with3(
        self,
        other1: Vec3,
        other2: Vec3,
        mut f: impl FnMut(f64, f64, f64) -> f64,
    ) -> Self {
        Self(
            f(self.0, other1.0, other2.0),
            f(self.1, other1.1, other2.1),
            f(self.2, other1.2, other2.2),
        )
    }

    /// Combines the elements of 'self' with 'f' until only one result remains.
    #[inline]
    pub fn reduce(self, f: impl Fn(f64, f64) -> f64) -> f64 {
        f(f(self.0, self.1), self.2)
    }
}

/// vector * vector
impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Mul::mul)
    }
}

/// f64 * vector
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from(self) * rhs
    }
}

/// vector * f64
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

/// vector / vector
impl std::ops::Div for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Div::div)
    }
}

/// vector / f64
impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

/// vector + vector
impl std::ops::Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Add::add)
    }
}

/// f64 + vector
impl std::ops::Add<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| self + x)
    }
}

/// vector + f64
impl std::ops::Add<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

/// vector - vector
impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Sub::sub)
    }
}

/// -vector
impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        self.map(std::ops::Neg::neg)
    }
}

/// Create a vector from a single value. Sets all channels to the value.
impl From<f64> for Vec3 {
    #[inline]
    fn from(v: f64) -> Self {
        Self(v, v, v)
    }
}

/// Names for vector components when used as a colour. Allows indexing by Channel.
/// ```
/// let v = Vec3(1.0, 2.0, 3.0)
/// assert_eq!(v[R], 1.0)
/// assert_eq!(v[G], 2.0)
/// assert_eq!(v[B], 3.0)
/// ```
#[derive(Copy, Clone, Debug)]
pub enum Channel {
    R,
    G,
    B,
}

use Channel::*;

impl std::ops::Index<Channel> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, idx: Channel) -> &Self::Output {
        match idx {
            R => &self.0,
            G => &self.1,
            B => &self.2,
        }
    }
}

impl ::std::ops::IndexMut<Channel> for Vec3 {
    #[inline]
    fn index_mut(&mut self, idx: Channel) -> &mut Self::Output {
        match idx {
            R => &mut self.0,
            G => &mut self.1,
            B => &mut self.2,
        }
    }
}

impl Distribution<Channel> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Channel {
        match rng.gen_range(0, 3) {
            0 => R,
            1 => G,
            _ => B,
        }
    }
}

/// Names for vector components when used as a co-ordinate. Allows indexing by Axis.
/// let v = Vec3(1.0, 2.0, 3.0)
/// assert_eq!(v[X], 1.0)
/// assert_eq!(v[Y], 2.0)
/// assert_eq!(v[Z], 3.0)
/// ```
#[derive(Copy, Clone, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

use Axis::*;

impl std::ops::Index<Axis> for Vec3 {
    type Output = f64;

    #[inline]
    fn index(&self, idx: Axis) -> &Self::Output {
        match idx {
            X => &self.0,
            Y => &self.1,
            Z => &self.2,
        }
    }
}

impl ::std::ops::IndexMut<Axis> for Vec3 {
    #[inline]
    fn index_mut(&mut self, idx: Axis) -> &mut Self::Output {
        match idx {
            X => &mut self.0,
            Y => &mut self.1,
            Z => &mut self.2,
        }
    }
}

impl Distribution<Axis> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        match rng.gen_range(0, 3) {
            0 => X,
            1 => Y,
            _ => Z,
        }
    }
}

/// Reflect Vector3 'v' across a surface normal 'n'.
#[inline]
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

/// Refracts 'v' into a material with surface normal 'n'.
/// 'ni_over_nt' is the ratio between the IORs of the object being left and object being entered.
/// If leaving it's the IOR of the object being left over the IOR of air (1).
/// If entering it's the IOR of air (1) over the IOR of the object being entered.
#[inline]
pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.normalise();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}
