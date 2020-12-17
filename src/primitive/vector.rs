// --------------------------------------------------------------------------------------------- //

use crate::{float::ApproxEq, primitive::tuple::Tuple};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

// --------------------------------------------------------------------------------------------- //

impl Vector {
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn normalize(&self) -> Vector {
        *self / self.magnitude()
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        *self - (*normal * 2.0) * (*self ^ *normal)
    }
}

// --------------------------------------------------------------------------------------------- //

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    fn zero() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        0.0
    }
}

// --------------------------------------------------------------------------------------------- //

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

// --------------------------------------------------------------------------------------------- //

// "Cross" product
impl std::ops::Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

// "Dot" product (or "scalar" product)
impl std::ops::BitXor for Vector {
    type Output = f64;

    fn bitxor(self, rhs: Vector) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let v1 = Vector {
            x: 1.1,
            y: 2.2,
            z: 3.3,
        };

        let v2 = Vector {
            x: 1.0,
            y: 5.0,
            z: 10.0,
        };

        let res = v1 + v2;
        let expected = Vector {
            x: 2.1,
            y: 7.2,
            z: 13.3,
        };

        assert_eq!(res, expected);
    }

    #[test]
    fn sub() {
        let v1 = Vector {
            x: 1.1,
            y: 2.2,
            z: 1.3,
        };

        let v2 = Vector {
            x: 1.0,
            y: 5.0,
            z: 1.0,
        };

        let res: Vector = v1 - v2;

        let expected = Vector {
            x: 0.1,
            y: -2.8,
            z: 0.3,
        };

        assert_eq!(res, expected);
    }

    #[test]
    fn neg() {
        let v = Vector {
            x: 1.1,
            y: 2.2,
            z: 1.3,
        };

        let res: Vector = -v;

        let expected = Vector {
            x: -1.1,
            y: -2.2,
            z: -1.3,
        };

        assert_eq!(res, expected);
    }

    #[test]
    fn scale() {
        let v = Vector {
            x: 1.1,
            y: 2.2,
            z: 1.3,
        };

        let res1: Vector = v * 2.0;
        let res2: Vector = 2.0 * v;

        let expected = Vector {
            x: 2.2,
            y: 4.4,
            z: 2.6,
        };

        assert_eq!(res1, expected);
        assert_eq!(res2, expected);
    }

    #[test]
    fn divide() {
        let v = Vector {
            x: 1.1,
            y: 2.2,
            z: 1.3,
        };

        let res: Vector = v / 2.0;

        let expected = Vector {
            x: 0.55,
            y: 1.1,
            z: 0.65,
        };

        assert_eq!(res, expected);
    }

    #[test]
    fn dot() {
        let v1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let v2 = Vector {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        assert_eq!(v1 ^ v2, 20.0);
    }

    #[test]
    fn cross() {
        let v1 = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let v2 = Vector {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };

        {
            let res: Vector = v1 * v2;
            let expected = Vector {
                x: -1.0,
                y: 2.0,
                z: -1.0,
            };

            assert_eq!(res, expected);
        }
        {
            let res: Vector = v2 * v1;
            let expected = Vector {
                x: 1.0,
                y: -2.0,
                z: 1.0,
            };

            assert_eq!(res, expected);
        }
    }

    #[test]
    fn magnitude() {
        {
            let v = Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
            assert_eq!(v.magnitude(), 1.0);
        }
        {
            let v = Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            };
            assert_eq!(v.magnitude(), 1.0);
        }
        {
            let v = Vector {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            };
            assert_eq!(v.magnitude(), 1.0);
        }
        {
            let v = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };
            assert_eq!(v.magnitude(), f64::sqrt(14.0));
        }
        {
            let v = Vector {
                x: -1.0,
                y: -2.0,
                z: -3.0,
            };
            assert_eq!(v.magnitude(), f64::sqrt(14.0));
        }
    }

    #[test]
    fn normalize() {
        {
            let v = Vector {
                x: 4.0,
                y: 0.0,
                z: 0.0,
            };

            let res = v.normalize();
            let expected = Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };

            assert_eq!(res, expected);
        }
        {
            let v = Vector {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            };

            let res = v.normalize();
            let expected = Vector {
                x: 0.26726,
                y: 0.53452,
                z: 0.80178,
            };

            assert_eq!(res, expected);
        }
    }

    #[test]
    fn magnitude_of_a_normalized_vector() {
        let v = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(v.reflect(&n), Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);

        assert_eq!(v.reflect(&n), Vector::new(1.0, 0.0, 0.0));
    }
}
