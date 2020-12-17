// --------------------------------------------------------------------------------------------- //

use crate::{
    float::ApproxEq,
    primitive::{tuple::Tuple, vector::Vector},
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

// --------------------------------------------------------------------------------------------- //

impl Tuple for Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    fn zero() -> Point {
        Point {
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
        1.0
    }
}

// --------------------------------------------------------------------------------------------- //

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x(),
            y: self.y + rhs.y(),
            z: self.z + rhs.z(),
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x() + rhs.x,
            y: self.y() + rhs.y,
            z: self.z() + rhs.z,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x - rhs.x(),
            y: self.y - rhs.y(),
            z: self.z - rhs.z(),
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Add<f64> for Point {
    type Output = Point;

    fn add(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let p = Point {
            x: 1.1,
            y: 2.2,
            z: 3.3,
        };
        let v = Vector::new(1.0, 5.0, 10.0);

        let res = p + v;
        let expected = Point {
            x: 2.1,
            y: 7.2,
            z: 13.3,
        };
        assert_eq!(res, expected);
    }

    #[test]
    fn sub() {
        let p = Point {
            x: 1.1,
            y: 2.2,
            z: 1.3,
        };
        let v = Vector::new(1.0, 5.0, 1.0);

        let res = p - v;
        let expected = Point {
            x: 0.1,
            y: -2.8,
            z: 0.3,
        };
        assert_eq!(res, expected);
    }
}

// --------------------------------------------------------------------------------------------- //
