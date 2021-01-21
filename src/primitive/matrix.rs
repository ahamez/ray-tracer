/* ---------------------------------------------------------------------------------------------- */

use crate::{
    float::ApproxEq,
    primitive::{matrix3::Matrix3, tuple::Tuple},
};
use serde::{Deserialize, Serialize};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Matrix {
    data: [f64; 16],
}

/* ---------------------------------------------------------------------------------------------- */

impl Matrix {
    pub fn new() -> Matrix {
        Matrix {
            data: [0.0; 16],
        }
    }

    pub fn id() -> Matrix {
        Matrix {
            data: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut res = Matrix::new();
        for i in 0..4 {
            for j in 0..4 {
                res[(j, i)] = self[(i, j)];
            }
        }

        res
    }

    pub fn invert(&self) -> Matrix {
        let determinant = self.determinant();

        if determinant.approx_eq(0.0) {
            panic!("Non invertible matrix")
        } else {
            let mut res = Matrix::new();

            for row in 0..4 {
                for col in 0..4 {
                    let c = self.cofactor(row, col);
                    res[(col, row)] = c / determinant;
                }
            }

            res
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut res = 0.0;
        for col in 0..4 {
            res += self[(0, col)] * self.cofactor(0, col);
        }

        res
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        let mut res = Matrix3::new();

        let mut new_row = 0;
        for i in 0..4 {
            let mut new_col = 0;
            if i != row {
                for j in 0..4 {
                    if j != col {
                        res[new_row][new_col] = self[(i, j)];
                        new_col += 1;
                    }
                }
                new_row += 1;
            }
        }

        res
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 != 0 {
            -minor
        } else {
            minor
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !self[(i, j)].approx_eq_low_precision(other[(i, j)]) {
                    return false;
                }
            }
        }
        true
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        debug_assert!(row < 4);
        debug_assert!(col < 4);
        // Just for a thrill 😅
        unsafe { self.data.get_unchecked(row * 4 + col) }
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        debug_assert!(row < 4);
        debug_assert!(col < 4);
        unsafe { self.data.get_unchecked_mut(row * 4 + col) }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut res = Matrix::new();

        for row in 0..4 {
            for col in 0..4 {
                res[(row, col)] = self[(row, 0)] * rhs[(0, col)]
                    + self[(row, 1)] * rhs[(1, col)]
                    + self[(row, 2)] * rhs[(2, col)]
                    + self[(row, 3)] * rhs[(3, col)];
            }
        }

        res
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl<T> std::ops::Mul<T> for Matrix
where
    T: Tuple,
{
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(
            self[(0, 0)] * rhs.x()
                + self[(0, 1)] * rhs.y()
                + self[(0, 2)] * rhs.z()
                + self[(0, 3)] * rhs.w(),
            self[(1, 0)] * rhs.x()
                + self[(1, 1)] * rhs.y()
                + self[(1, 2)] * rhs.z()
                + self[(1, 3)] * rhs.w(),
            self[(2, 0)] * rhs.x()
                + self[(2, 1)] * rhs.y()
                + self[(2, 2)] * rhs.z()
                + self[(2, 3)] * rhs.w(),
        )
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use crate::primitive::{point::Point, vector::Vector};

    use super::*;

    #[test]
    fn create() {
        let mut m = Matrix::new();

        m[(0, 0)] = 1.0;
        m[(0, 1)] = 2.0;
        m[(0, 2)] = 3.0;
        m[(0, 3)] = 4.0;

        m[(1, 0)] = 5.5;
        m[(1, 1)] = 6.5;
        m[(1, 2)] = 7.5;
        m[(1, 3)] = 8.5;

        m[(2, 0)] = 9.0;
        m[(2, 1)] = 10.0;
        m[(2, 2)] = 11.0;
        m[(2, 3)] = 12.0;

        m[(3, 0)] = 13.5;
        m[(3, 1)] = 14.5;
        m[(3, 2)] = 15.5;
        m[(3, 3)] = 16.5;

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    fn mult() {
        let m0 = {
            let mut m = Matrix::new();

            m[(0, 0)] = 1.0;
            m[(0, 1)] = 2.0;
            m[(0, 2)] = 3.0;
            m[(0, 3)] = 4.0;

            m[(1, 0)] = 5.0;
            m[(1, 1)] = 6.0;
            m[(1, 2)] = 7.0;
            m[(1, 3)] = 8.0;

            m[(2, 0)] = 9.0;
            m[(2, 1)] = 8.0;
            m[(2, 2)] = 7.0;
            m[(2, 3)] = 6.0;

            m[(3, 0)] = 5.0;
            m[(3, 1)] = 4.0;
            m[(3, 2)] = 3.0;
            m[(3, 3)] = 2.0;

            m
        };

        let m1 = {
            let mut m = Matrix::new();

            m[(0, 0)] = -2.0;
            m[(0, 1)] = 1.0;
            m[(0, 2)] = 2.0;
            m[(0, 3)] = 3.0;

            m[(1, 0)] = 3.0;
            m[(1, 1)] = 2.0;
            m[(1, 2)] = 1.0;
            m[(1, 3)] = -1.0;

            m[(2, 0)] = 4.0;
            m[(2, 1)] = 3.0;
            m[(2, 2)] = 6.0;
            m[(2, 3)] = 5.0;

            m[(3, 0)] = 1.0;
            m[(3, 1)] = 2.0;
            m[(3, 2)] = 7.0;
            m[(3, 3)] = 8.0;

            m
        };

        let expected = {
            let mut m = Matrix::new();

            m[(0, 0)] = 20.0;
            m[(0, 1)] = 22.0;
            m[(0, 2)] = 50.0;
            m[(0, 3)] = 48.0;

            m[(1, 0)] = 44.0;
            m[(1, 1)] = 54.0;
            m[(1, 2)] = 114.0;
            m[(1, 3)] = 108.0;

            m[(2, 0)] = 40.0;
            m[(2, 1)] = 58.0;
            m[(2, 2)] = 110.0;
            m[(2, 3)] = 102.0;

            m[(3, 0)] = 16.0;
            m[(3, 1)] = 26.0;
            m[(3, 2)] = 46.0;
            m[(3, 3)] = 42.0;

            m
        };

        assert_eq!(m0 * m1, expected);
    }

    #[test]
    fn mult_with_vector() {
        let mut m = Matrix::new();

        m[(0, 0)] = 1.0;
        m[(0, 1)] = 2.0;
        m[(0, 2)] = 3.0;
        m[(0, 3)] = 4.0;

        m[(1, 0)] = 2.0;
        m[(1, 1)] = 4.0;
        m[(1, 2)] = 4.0;
        m[(1, 3)] = 2.0;

        m[(2, 0)] = 8.0;
        m[(2, 1)] = 6.0;
        m[(2, 2)] = 4.0;
        m[(2, 3)] = 1.0;

        m[(3, 0)] = 0.0;
        m[(3, 1)] = 0.0;
        m[(3, 2)] = 0.0;
        m[(3, 3)] = 1.0;

        let v = Vector::new(1.0, 2.0, 3.0);
        let expected = Vector::new(14.0, 22.0, 32.0);

        assert_eq!(m * v, expected);
    }

    #[test]
    fn mult_with_point() {
        let mut m = Matrix::new();

        m[(0, 0)] = 1.0;
        m[(0, 1)] = 2.0;
        m[(0, 2)] = 3.0;
        m[(0, 3)] = 4.0;

        m[(1, 0)] = 2.0;
        m[(1, 1)] = 4.0;
        m[(1, 2)] = 4.0;
        m[(1, 3)] = 2.0;

        m[(2, 0)] = 8.0;
        m[(2, 1)] = 6.0;
        m[(2, 2)] = 4.0;
        m[(2, 3)] = 1.0;

        m[(3, 0)] = 0.0;
        m[(3, 1)] = 0.0;
        m[(3, 2)] = 0.0;
        m[(3, 3)] = 1.0;

        let v = Point::new(1.0, 2.0, 3.0);
        let expected = Point::new(18.0, 24.0, 33.0);

        assert_eq!(m * v, expected);
    }

    #[test]
    fn id() {
        {
            let mut m = Matrix::new();

            m[(0, 0)] = 1.0;
            m[(0, 1)] = 2.0;
            m[(0, 2)] = 3.0;
            m[(0, 3)] = 4.0;

            m[(1, 0)] = 2.0;
            m[(1, 1)] = 4.0;
            m[(1, 2)] = 4.0;
            m[(1, 3)] = 2.0;

            m[(2, 0)] = 8.0;
            m[(2, 1)] = 6.0;
            m[(2, 2)] = 4.0;
            m[(2, 3)] = 1.0;

            m[(3, 0)] = 0.0;
            m[(3, 1)] = 0.0;
            m[(3, 2)] = 0.0;
            m[(3, 3)] = 1.0;

            let clone = m.clone();
            assert_eq!(m * Matrix::id(), clone);
        }
        {
            let v = Vector::new(1.0, 2.0, 3.0);
            assert_eq!(Matrix::id() * v, v);
        }
    }

    #[test]
    fn transpose() {
        {
            assert_eq!(Matrix::id().transpose(), Matrix::id());
        }
        {
            let mut m = Matrix::new();

            m[(0, 0)] = 0.0;
            m[(0, 1)] = 9.0;
            m[(0, 2)] = 3.0;
            m[(0, 3)] = 0.0;

            m[(1, 0)] = 9.0;
            m[(1, 1)] = 8.0;
            m[(1, 2)] = 0.0;
            m[(1, 3)] = 8.0;

            m[(2, 0)] = 1.0;
            m[(2, 1)] = 8.0;
            m[(2, 2)] = 5.0;
            m[(2, 3)] = 3.0;

            m[(3, 0)] = 0.0;
            m[(3, 1)] = 0.0;
            m[(3, 2)] = 5.0;
            m[(3, 3)] = 8.0;

            let mut expected = Matrix::new();

            expected[(0, 0)] = 0.0;
            expected[(0, 1)] = 9.0;
            expected[(0, 2)] = 1.0;
            expected[(0, 3)] = 0.0;

            expected[(1, 0)] = 9.0;
            expected[(1, 1)] = 8.0;
            expected[(1, 2)] = 8.0;
            expected[(1, 3)] = 0.0;

            expected[(2, 0)] = 3.0;
            expected[(2, 1)] = 0.0;
            expected[(2, 2)] = 5.0;
            expected[(2, 3)] = 5.0;

            expected[(3, 0)] = 0.0;
            expected[(3, 1)] = 8.0;
            expected[(3, 2)] = 3.0;
            expected[(3, 3)] = 8.0;

            assert_eq!(m.transpose(), expected);
        }
    }

    #[test]
    fn determinant() {
        let mut m = Matrix::new();

        m[(0, 0)] = -2.0;
        m[(0, 1)] = -8.0;
        m[(0, 2)] = 3.0;
        m[(0, 3)] = 5.0;

        m[(1, 0)] = -3.0;
        m[(1, 1)] = 1.0;
        m[(1, 2)] = 7.0;
        m[(1, 3)] = 3.0;

        m[(2, 0)] = 1.0;
        m[(2, 1)] = 2.0;
        m[(2, 2)] = -9.0;
        m[(2, 3)] = 6.0;

        m[(3, 0)] = -6.0;
        m[(3, 1)] = 7.0;
        m[(3, 2)] = 7.0;
        m[(3, 3)] = -9.0;

        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn submatrix() {
        let mut m = Matrix::new();

        m[(0, 0)] = -6.0;
        m[(0, 1)] = 1.0;
        m[(0, 2)] = 1.0;
        m[(0, 3)] = 6.0;

        m[(1, 0)] = -8.0;
        m[(1, 1)] = 5.0;
        m[(1, 2)] = 8.0;
        m[(1, 3)] = 6.0;

        m[(2, 0)] = -1.0;
        m[(2, 1)] = 0.0;
        m[(2, 2)] = 8.0;
        m[(2, 3)] = 2.0;

        m[(3, 0)] = -7.0;
        m[(3, 1)] = 1.0;
        m[(3, 2)] = -1.0;
        m[(3, 3)] = 1.0;

        let mut expected = Matrix3::new();
        expected[0][0] = -6.0;
        expected[0][1] = 1.0;
        expected[0][2] = 6.0;

        expected[1][0] = -8.0;
        expected[1][1] = 8.0;
        expected[1][2] = 6.0;

        expected[2][0] = -7.0;
        expected[2][1] = -1.0;
        expected[2][2] = 1.0;

        assert_eq!(m.submatrix(2, 1), expected);
    }

    #[test]
    #[should_panic]
    fn inversion_impossible() {
        let mut m = Matrix::new();

        m[(0, 0)] = -4.0;
        m[(0, 1)] = 2.0;
        m[(0, 2)] = -2.0;
        m[(0, 3)] = -3.0;

        m[(1, 0)] = 9.0;
        m[(1, 1)] = 6.0;
        m[(1, 2)] = 2.0;
        m[(1, 3)] = 6.0;

        m[(2, 0)] = 0.0;
        m[(2, 1)] = -5.0;
        m[(2, 2)] = 1.0;
        m[(2, 3)] = -5.0;

        m[(3, 0)] = 0.0;
        m[(3, 1)] = 0.0;
        m[(3, 2)] = 0.0;
        m[(3, 3)] = 0.0;

        m.invert();
    }

    #[test]
    fn invert() {
        {
            let mut m = Matrix::new();

            m[(0, 0)] = -5.0;
            m[(0, 1)] = 2.0;
            m[(0, 2)] = 6.0;
            m[(0, 3)] = -8.0;

            m[(1, 0)] = 1.0;
            m[(1, 1)] = -5.0;
            m[(1, 2)] = 1.0;
            m[(1, 3)] = 8.0;

            m[(2, 0)] = 7.0;
            m[(2, 1)] = 7.0;
            m[(2, 2)] = -6.0;
            m[(2, 3)] = -7.0;

            m[(3, 0)] = 1.0;
            m[(3, 1)] = -3.0;
            m[(3, 2)] = 7.0;
            m[(3, 3)] = 4.0;

            let mut expected = Matrix::new();

            expected[(0, 0)] = 0.21805;
            expected[(0, 1)] = 0.45133;
            expected[(0, 2)] = 0.24060;
            expected[(0, 3)] = -0.04511;

            expected[(1, 0)] = -0.80827;
            expected[(1, 1)] = -1.45677;
            expected[(1, 2)] = -0.44361;
            expected[(1, 3)] = 0.52068;

            expected[(2, 0)] = -0.07895;
            expected[(2, 1)] = -0.22368;
            expected[(2, 2)] = -0.05263;
            expected[(2, 3)] = 0.19737;

            expected[(3, 0)] = -0.52256;
            expected[(3, 1)] = -0.81391;
            expected[(3, 2)] = -0.30075;
            expected[(3, 3)] = 0.30639;

            assert_eq!(m.invert(), expected);
        }
        {
            let mut a = Matrix::new();

            a[(0, 0)] = 3.0;
            a[(0, 1)] = -9.0;
            a[(0, 2)] = 7.0;
            a[(0, 3)] = 3.0;

            a[(1, 0)] = 3.0;
            a[(1, 1)] = -8.0;
            a[(1, 2)] = 2.0;
            a[(1, 3)] = -9.0;

            a[(2, 0)] = -4.0;
            a[(2, 1)] = 4.0;
            a[(2, 2)] = 4.0;
            a[(2, 3)] = 1.0;

            a[(3, 0)] = -6.0;
            a[(3, 1)] = 5.0;
            a[(3, 2)] = -1.0;
            a[(3, 3)] = 1.0;

            let mut b = Matrix::new();

            b[(0, 0)] = 8.0;
            b[(0, 1)] = 2.0;
            b[(0, 2)] = 2.0;
            b[(0, 3)] = 2.0;

            b[(1, 0)] = 3.0;
            b[(1, 1)] = -1.0;
            b[(1, 2)] = 7.0;
            b[(1, 3)] = 0.0;

            b[(2, 0)] = 7.0;
            b[(2, 1)] = 0.0;
            b[(2, 2)] = 5.0;
            b[(2, 3)] = 4.0;

            b[(3, 0)] = 6.0;
            b[(3, 1)] = -2.0;
            b[(3, 2)] = 0.0;
            b[(3, 3)] = 5.0;

            let c = a * b;

            assert_eq!(c * b.invert(), a);
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */
