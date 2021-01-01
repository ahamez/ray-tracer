/* ---------------------------------------------------------------------------------------------- */

use crate::primitive::{Matrix, Point, Tuple, Vector};

/* ---------------------------------------------------------------------------------------------- */

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut res = Matrix::id();
    res[0][3] = x;
    res[1][3] = y;
    res[2][3] = z;
    res
}

/* ---------------------------------------------------------------------------------------------- */

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut res = Matrix::id();
    res[0][0] = x;
    res[1][1] = y;
    res[2][2] = z;
    res
}

/* ---------------------------------------------------------------------------------------------- */

pub fn rotation_x(angle: f64) -> Matrix {
    let mut res = Matrix::id();
    res[1][1] = f64::cos(angle);
    res[1][2] = -f64::sin(angle);
    res[2][1] = f64::sin(angle);
    res[2][2] = f64::cos(angle);
    res
}

/* ---------------------------------------------------------------------------------------------- */

pub fn rotation_y(angle: f64) -> Matrix {
    let mut res = Matrix::id();
    res[0][0] = f64::cos(angle);
    res[0][2] = f64::sin(angle);
    res[2][0] = -f64::sin(angle);
    res[2][2] = f64::cos(angle);
    res
}

/* ---------------------------------------------------------------------------------------------- */

pub fn rotation_z(angle: f64) -> Matrix {
    let mut res = Matrix::id();
    res[0][0] = f64::cos(angle);
    res[0][1] = -f64::sin(angle);
    res[1][0] = f64::sin(angle);
    res[1][1] = f64::cos(angle);
    res
}

/* ---------------------------------------------------------------------------------------------- */

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut res = Matrix::id();
    res[0][1] = xy;
    res[0][2] = xz;
    res[1][0] = yx;
    res[1][2] = yz;
    res[2][0] = zx;
    res[2][1] = zy;
    res
}

/* ---------------------------------------------------------------------------------------------- */

pub fn view_transform(from: &Point, to: &Point, up: &Vector) -> Matrix {
    let forward = (*to - *from).normalize();
    let left = forward * up.normalize();
    let true_up = left * forward;

    let orientation = {
        let mut m = Matrix::new(4);

        m[0][0] = left.x();
        m[0][1] = left.y();
        m[0][2] = left.z();
        m[0][3] = 0.0;

        m[1][0] = true_up.x();
        m[1][1] = true_up.y();
        m[1][2] = true_up.z();
        m[1][3] = 0.0;

        m[2][0] = -forward.x();
        m[2][1] = -forward.y();
        m[2][2] = -forward.z();
        m[2][3] = 0.0;

        m[3][0] = 0.0;
        m[3][1] = 0.0;
        m[3][2] = 0.0;
        m[3][3] = 1.0;

        m
    };

    let translation = translation(-from.x(), -from.y(), -from.z());

    orientation * translation
}

/* ---------------------------------------------------------------------------------------------- */

pub trait Transform {
    fn transform(&self, transformation: &Matrix) -> Self;

    fn translate(&self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let transformation = translation(x, y, z);
        self.transform(&transformation)
    }

    fn scale(&self, x: f64, y: f64, z: f64) -> Self
    where
        Self: Sized,
    {
        let transformation = scaling(x, y, z);
        self.transform(&transformation)
    }

    fn rotate_x(&self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let transformation = rotation_x(angle);
        self.transform(&transformation)
    }

    fn rotate_y(&self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let transformation = rotation_y(angle);
        self.transform(&transformation)
    }

    fn rotate_z(&self, angle: f64) -> Self
    where
        Self: Sized,
    {
        let transformation = rotation_z(angle);
        self.transform(&transformation)
    }

    fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self
    where
        Self: Sized,
    {
        let transformation = shearing(xy, xz, yx, yz, zx, zy);
        self.transform(&transformation)
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::primitive::{Point, Tuple, Vector};

    #[test]
    fn translate_point() {
        {
            let p = Point::new(-3.0, 4.0, 5.0);
            let transform = translation(5.0, -3.0, 2.0);
            let expected = Point::new(2.0, 1.0, 7.0);

            assert_eq!(transform * p, expected);
        }
        {
            let p = Point::new(-3.0, 4.0, 5.0);
            let transform = translation(5.0, -3.0, 2.0);
            let inv = transform.invert();
            let expected = Point::new(-8.0, 7.0, 3.0);

            assert_eq!(inv * p, expected);
        }
        {
            let p = Point::new(-3.0, 4.0, 5.0);
            let expected = Point::new(2.0, 1.0, 7.0);
            assert_eq!(p.translate(5.0, -3.0, 2.0), expected);
        }
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn scale_point() {
        {
            let transformation = scaling(2.0, 3.0, 4.0);
            let p = Point::new(-4.0, 6.0, 8.0);
            let expected = Point::new(-8.0, 18.0, 32.0);
            assert_eq!(transformation * p, expected);
        }
        {
            let p = Point::new(-4.0, 6.0, 8.0);
            let expected = Point::new(-8.0, 18.0, 32.0);
            assert_eq!(p.scale(2.0, 3.0, 4.0), expected);
        }
        {
            // Same as a reflection along x axis.
            let transformation = scaling(-1.0, 1.0, 1.0);
            let p = Point::new(2.0, 3.0, 4.0);
            let expected = Point::new(-2.0, 3.0, 4.0);
            assert_eq!(transformation * p, expected);
        }
    }

    #[test]
    fn scale_vector() {
        let transformation = scaling(2.0, 3.0, 4.0);
        let v = Vector::new(-4.0, 6.0, 8.0);
        let expected = Vector::new(-8.0, 18.0, 32.0);
        assert_eq!(transformation * v, expected);
    }

    #[test]
    fn rotation() {
        // X axis
        {
            let p = Point::new(0.0, 1.0, 0.0);
            let half_quarter = rotation_x(PI / 4.0);
            let full_quarter = rotation_x(PI / 2.0);

            assert_eq!(
                half_quarter * p,
                Point::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
            );
            assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0));
        }
        {
            let p = Point::new(0.0, 1.0, 0.0);
            let half_quarter = rotation_x(PI / 4.0);
            let inv = half_quarter.invert();

            assert_eq!(
                inv * p,
                Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0)
            );
        }
        {
            let p = Point::new(0.0, 1.0, 0.0);

            assert_eq!(
                p.rotate_x(PI / 4.0),
                Point::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
            );
            assert_eq!(p.rotate_x(PI / 2.0), Point::new(0.0, 0.0, 1.0));
        }
        {
            let p = Vector::new(0.0, 1.0, 0.0);

            assert_eq!(
                p.rotate_x(PI / 4.0),
                Vector::new(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
            );
            assert_eq!(p.rotate_x(PI / 2.0), Vector::new(0.0, 0.0, 1.0));
        }
        {
            let p = Point::new(0.0, 1.0, 0.0);
            assert_eq!(
                p.rotate_x(-PI / 4.0),
                Point::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0)
            );
        }
        // Y axis
        {
            let p = Point::new(0.0, 0.0, 1.0);
            assert_eq!(
                p.rotate_y(PI / 4.0),
                Point::new(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0)
            );
            assert_eq!(p.rotate_y(PI / 2.0), Point::new(1.0, 0.0, 0.0));
        }
        {
            let p = Vector::new(0.0, 0.0, 1.0);
            assert_eq!(
                p.rotate_y(PI / 4.0),
                Vector::new(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0)
            );
            assert_eq!(p.rotate_y(PI / 2.0), Vector::new(1.0, 0.0, 0.0));
        }
        // Z axis
        {
            let p = Point::new(0.0, 1.0, 0.0);
            assert_eq!(
                p.rotate_z(PI / 4.0),
                Point::new(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0)
            );
            assert_eq!(p.rotate_z(PI / 2.0), Point::new(-1.0, 0.0, 0.0));
        }
        {
            let p = Vector::new(0.0, 1.0, 0.0);
            assert_eq!(
                p.rotate_z(PI / 4.0),
                Vector::new(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0)
            );
            assert_eq!(p.rotate_z(PI / 2.0), Vector::new(-1.0, 0.0, 0.0));
        }
    }

    #[test]
    fn shearing() {
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(
            p.shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            Point::new(5.0, 3.0, 4.0)
        );
        assert_eq!(
            p.shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0),
            Point::new(6.0, 3.0, 4.0)
        );
        assert_eq!(
            p.shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0),
            Point::new(2.0, 5.0, 4.0)
        );
        assert_eq!(
            p.shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            Point::new(2.0, 7.0, 4.0)
        );
        assert_eq!(
            p.shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0),
            Point::new(2.0, 3.0, 6.0)
        );
        assert_eq!(
            p.shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            Point::new(2.0, 3.0, 7.0)
        );
    }

    #[test]
    fn chaining() {
        {
            let p0 = Point::new(1.0, 0.0, 1.0);
            let p1 = p0.rotate_x(PI / 2.0);
            let p2 = p1.scale(5.0, 5.0, 5.0);
            let p3 = p2.translate(10.0, 5.0, 7.0);

            assert_eq!(p1, Point::new(1.0, -1.0, 0.0));
            assert_eq!(p2, Point::new(5.0, -5.0, 0.0));
            assert_eq!(p3, Point::new(15.0, 0.0, 7.0));
        }
        {
            let p = Point::new(1.0, 0.0, 1.0)
                .rotate_x(PI / 2.0)
                .scale(5.0, 5.0, 5.0)
                .translate(10.0, 5.0, 7.0);

            assert_eq!(p, Point::new(15.0, 0.0, 7.0));
        }
    }

    #[test]
    fn identity_is_the_default_transformation_matrix() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(view_transform(&from, &to, &up), Matrix::id());
    }

    #[test]
    fn a_view_transformation_matrix_looking_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(view_transform(&from, &to, &up), scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(view_transform(&from, &to, &up), translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);

        let mut transform = Matrix::new(4);

        transform[0][0] = -0.50709;
        transform[0][1] = 0.50709;
        transform[0][2] = 0.67612;
        transform[0][3] = -2.36643;

        transform[1][0] = 0.76772;
        transform[1][1] = 0.60609;
        transform[1][2] = 0.12111;
        transform[1][3] = -2.82843;

        transform[2][0] = -0.35857;
        transform[2][1] = 0.59761;
        transform[2][2] = -0.71714;
        transform[2][3] = 0.0;

        transform[3][0] = 0.0;
        transform[3][1] = 0.0;
        transform[3][2] = 0.0;
        transform[3][3] = 1.0;

        assert_eq!(view_transform(&from, &to, &up), transform);
    }
}

/* ---------------------------------------------------------------------------------------------- */
