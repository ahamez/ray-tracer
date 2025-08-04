/* ---------------------------------------------------------------------------------------------- */

use crate::{float::ApproxEq, primitive::tuple::Tuple};
use glam::DVec3;
use serde::{Deserialize, Serialize};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Vector(pub DVec3);

/* ---------------------------------------------------------------------------------------------- */

impl Vector {
    #[inline]
    pub fn magnitude(&self) -> f64 {
        self.0.length()
    }

    #[inline]
    pub fn normalize(&self) -> Vector {
        Vector(self.0.normalize())
    }

    #[inline]
    pub fn reflect(&self, normal: &Vector) -> Vector {
        *self - *normal * 2.0 * (*self ^ *normal)
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Tuple for Vector {
    #[inline]
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector(DVec3::new(x, y, z))
    }

    #[inline]
    fn zero() -> Vector {
        Vector(DVec3::ZERO)
    }

    #[inline]
    fn x(&self) -> f64 {
        self.0.x
    }

    #[inline]
    fn y(&self) -> f64 {
        self.0.y
    }

    #[inline]
    fn z(&self) -> f64 {
        self.0.z
    }

    #[inline]
    fn w(&self) -> f64 {
        0.0
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl PartialEq for Vector {
    #[inline]
    fn eq(&self, other: &Vector) -> bool {
        self.x().approx_eq_low_precision(other.x())
            && self.y().approx_eq_low_precision(other.y())
            && self.z().approx_eq_low_precision(other.z())
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Neg for Vector {
    type Output = Vector;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector(-self.0)
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Add for Vector {
    type Output = Vector;

    #[inline]
    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0)
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Sub for Vector {
    type Output = Vector;

    #[inline]
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector(self.0 - rhs.0)
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vector(self.0 * rhs)
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

/* ---------------------------------------------------------------------------------------------- */

// "Cross" product
impl std::ops::Mul for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self.0.cross(rhs.0))
    }
}

/* ---------------------------------------------------------------------------------------------- */

// "Dot" product (or "scalar" product)
impl std::ops::BitXor for Vector {
    type Output = f64;

    #[inline]
    fn bitxor(self, rhs: Vector) -> Self::Output {
        self.0.dot(rhs.0)
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vector(self.0 / rhs)
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let v1 = Vector::new(1.1, 2.2, 3.3);
        let v2 = Vector::new(1.0, 5.0, 10.0);

        let res = v1 + v2;
        let expected = Vector::new(2.1, 7.2, 13.3);

        assert_eq!(res, expected);
    }

    #[test]
    fn sub() {
        let v1 = Vector::new(1.1, 2.2, 1.3);
        let v2 = Vector::new(1.0, 5.0, 1.0);

        let res: Vector = v1 - v2;

        let expected = Vector::new(0.1, -2.8, 0.3);

        assert_eq!(res, expected);
    }

    #[test]
    fn neg() {
        let v = Vector::new(1.1, 2.2, 1.3);

        let res = -v;
        let expected = Vector::new(-1.1, -2.2, -1.3);

        assert_eq!(res, expected);
    }

    #[test]
    fn mul() {
        let v = Vector::new(1.0, -2.0, 3.0);

        let res = v * 3.5;
        let expected = Vector::new(3.5, -7.0, 10.5);

        assert_eq!(res, expected);
    }

    #[test]
    fn mul_with_scalar_first() {
        let v = Vector::new(1.0, -2.0, 3.0);

        let res = 3.5 * v;
        let expected = Vector::new(3.5, -7.0, 10.5);

        assert_eq!(res, expected);
    }

    #[test]
    fn div() {
        let v = Vector::new(1.0, -2.0, 3.0);

        let res = v / 2.0;
        let expected = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(res, expected);
    }

    #[test]
    fn magnitude() {
        let v = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Vector::new(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Vector::new(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), f64::sqrt(14.0));

        let v = Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn normalize() {
        let v = Vector::new(4.0, 0.0, 0.0);

        let res = v.normalize();
        let expected = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(res, expected);

        let v = Vector::new(1.0, 2.0, 3.0);

        let res = v.normalize();
        let expected = Vector::new(0.26726, 0.53452, 0.80178);

        assert_eq!(res, expected);
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn dot() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1 ^ v2, 20.0);
    }

    #[test]
    fn cross() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        assert_eq!(v1 * v2, Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2 * v1, Vector::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflect_vector_approaching_at_45() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(v.reflect(&n), Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_vector_off_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);

        assert_eq!(v.reflect(&n), Vector::new(1.0, 0.0, 0.0));
    }
}

/* ---------------------------------------------------------------------------------------------- */
