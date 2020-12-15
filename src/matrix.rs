// --------------------------------------------------------------------------------------------- //

use crate::{approx_eq::ApproxEq, tuple::Tuple};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy)]
pub struct Matrix {
    size: usize,
    data: [f64; 4 * 4],
}

// --------------------------------------------------------------------------------------------- //

impl Matrix {
    pub fn new(size: usize) -> Matrix {
        assert!(size > 1 && size <= 4);

        Matrix {
            size,
            data: [0.0; 4 * 4],
        }
    }

    pub fn id() -> Matrix {
        Matrix {
            size: 4,
            data: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn transpose(&self) -> Matrix {
        assert_eq!(self.size, 4);

        let mut res = Matrix::new(4);
        for i in 0..4 {
            for j in 0..4 {
                res[j][i] = self[i][j];
            }
        }

        res
    }

    pub fn invert(&self) -> Option<Matrix> {
        let determinant = self.determinant();

        if determinant.approx_eq(0.0) {
            None
        } else {
            let mut res = Matrix::new(self.size);

            for row in 0..self.size {
                for col in 0..self.size {
                    let c = self.cofactor(row, col);
                    res[col][row] = c / determinant;
                }
            }

            Some(res)
        }
    }

    pub fn determinant(&self) -> f64 {
        assert!(self.size >= 2);
        assert!(self.size <= 4);

        if self.size == 2 {
            (self[0][0] * self[1][1]) - (self[0][1] * self[1][0])
        } else {
            let mut res = 0.0;
            for col in 0..self.size {
                res += self[0][col] * self.cofactor(0, col);
            }

            res
        }
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix {
        assert!(self.size > 2);

        let mut res = Matrix::new(self.size - 1);

        let mut new_row = 0;
        for i in 0..self.size {
            let mut new_col = 0;
            if i != row {
                for j in 0..self.size {
                    if j != col {
                        res[new_row][new_col] = self[i][j];
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

// --------------------------------------------------------------------------------------------- //

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.size != other.size {
            return false;
        }

        for i in 0..self.size {
            for j in 0..self.size {
                if !self[i][j].approx_eq(other[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Index<usize> for Matrix {
    type Output = [f64];

    fn index(&self, row: usize) -> &[f64] {
        let start = row * self.size;
        &self.data[start..start + self.size]
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut [f64] {
        let start = row * self.size;
        &mut self.data[start..start + self.size]
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.size, 4);
        assert_eq!(rhs.size, 4);

        let mut res = Matrix::new(self.size);

        for row in 0..4 {
            for col in 0..4 {
                res[row][col] = self[row][0] * rhs[0][col]
                    + self[row][1] * rhs[1][col]
                    + self[row][2] * rhs[2][col]
                    + self[row][3] * rhs[3][col];
            }
        }

        res
    }
}

// --------------------------------------------------------------------------------------------- //

impl<T> std::ops::Mul<T> for Matrix
where
    T: Tuple,
{
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        assert_eq!(self.size, 4);

        Self::Output::new(
            self[0][0] * rhs.x()
                + self[0][1] * rhs.y()
                + self[0][2] * rhs.z()
                + self[0][3] * rhs.w(),
            self[1][0] * rhs.x()
                + self[1][1] * rhs.y()
                + self[1][2] * rhs.z()
                + self[1][3] * rhs.w(),
            self[2][0] * rhs.x()
                + self[2][1] * rhs.y()
                + self[2][2] * rhs.z()
                + self[2][3] * rhs.w(),
        )
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{} ", self[i][j])?;
            }
            writeln!(f)?;
        }

        writeln!(f)
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::vector::Vector;

    use super::*;

    #[test]
    fn create() {
        {
            let mut m = Matrix::new(4);

            m[0][0] = 1.0;
            m[0][1] = 2.0;
            m[0][2] = 3.0;
            m[0][3] = 4.0;

            m[1][0] = 5.5;
            m[1][1] = 6.5;
            m[1][2] = 7.5;
            m[1][3] = 8.5;

            m[2][0] = 9.0;
            m[2][1] = 10.0;
            m[2][2] = 11.0;
            m[2][3] = 12.0;

            m[3][0] = 13.5;
            m[3][1] = 14.5;
            m[3][2] = 15.5;
            m[3][3] = 16.5;

            assert_eq!(m[0][0], 1.0);
            assert_eq!(m[0][3], 4.0);
            assert_eq!(m[1][0], 5.5);
            assert_eq!(m[1][2], 7.5);
            assert_eq!(m[2][2], 11.0);
            assert_eq!(m[3][0], 13.5);
            assert_eq!(m[3][2], 15.5);
        }
        {
            let mut m = Matrix::new(2);

            m[0][0] = -3.0;
            m[0][1] = 5.0;

            m[1][0] = 1.0;
            m[1][1] = -2.0;

            assert_eq!(m[0][0], -3.0);
            assert_eq!(m[0][1], 5.0);
            assert_eq!(m[1][0], 1.0);
            assert_eq!(m[1][1], -2.0);
        }
    }

    #[test]
    fn eq() {
        {
            let m0 = Matrix::new(2);
            let m1 = Matrix::new(3);
            assert_ne!(m0, m1);
        }
        {
            let mut m0 = Matrix::new(2);
            m0[0][0] = -3.0;
            m0[0][1] = 5.0;
            m0[1][0] = 1.0;
            m0[1][1] = -2.0;

            let m1 = m0.clone();

            assert_eq!(m0, m1);
        }
        {
            let mut m0 = Matrix::new(2);
            m0[0][0] = -3.0;
            m0[0][1] = 5.0;
            m0[1][0] = 1.0;
            m0[1][1] = -2.0;

            let mut m1 = Matrix::new(2);
            m1[0][0] = -3.1;
            m1[0][1] = 5.0;
            m1[1][0] = 1.0;
            m1[1][1] = -2.0;

            assert_ne!(m0, m1);
        }
    }

    #[test]
    fn mult() {
        let m0 = {
            let mut m = Matrix::new(4);

            m[0][0] = 1.0;
            m[0][1] = 2.0;
            m[0][2] = 3.0;
            m[0][3] = 4.0;

            m[1][0] = 5.0;
            m[1][1] = 6.0;
            m[1][2] = 7.0;
            m[1][3] = 8.0;

            m[2][0] = 9.0;
            m[2][1] = 8.0;
            m[2][2] = 7.0;
            m[2][3] = 6.0;

            m[3][0] = 5.0;
            m[3][1] = 4.0;
            m[3][2] = 3.0;
            m[3][3] = 2.0;

            m
        };

        let m1 = {
            let mut m = Matrix::new(4);

            m[0][0] = -2.0;
            m[0][1] = 1.0;
            m[0][2] = 2.0;
            m[0][3] = 3.0;

            m[1][0] = 3.0;
            m[1][1] = 2.0;
            m[1][2] = 1.0;
            m[1][3] = -1.0;

            m[2][0] = 4.0;
            m[2][1] = 3.0;
            m[2][2] = 6.0;
            m[2][3] = 5.0;

            m[3][0] = 1.0;
            m[3][1] = 2.0;
            m[3][2] = 7.0;
            m[3][3] = 8.0;

            m
        };

        let expected = {
            let mut m = Matrix::new(4);

            m[0][0] = 20.0;
            m[0][1] = 22.0;
            m[0][2] = 50.0;
            m[0][3] = 48.0;

            m[1][0] = 44.0;
            m[1][1] = 54.0;
            m[1][2] = 114.0;
            m[1][3] = 108.0;

            m[2][0] = 40.0;
            m[2][1] = 58.0;
            m[2][2] = 110.0;
            m[2][3] = 102.0;

            m[3][0] = 16.0;
            m[3][1] = 26.0;
            m[3][2] = 46.0;
            m[3][3] = 42.0;

            m
        };

        assert_eq!(m0 * m1, expected);
    }

    #[test]
    fn mult_with_vector() {
        let m = {
            let mut m = Matrix::new(4);

            m[0][0] = 1.0;
            m[0][1] = 2.0;
            m[0][2] = 3.0;
            m[0][3] = 4.0;

            m[1][0] = 2.0;
            m[1][1] = 4.0;
            m[1][2] = 4.0;
            m[1][3] = 2.0;

            m[2][0] = 8.0;
            m[2][1] = 6.0;
            m[2][2] = 4.0;
            m[2][3] = 1.0;

            m[3][0] = 0.0;
            m[3][1] = 0.0;
            m[3][2] = 0.0;
            m[3][3] = 1.0;

            m
        };

        let v = Vector::new(1.0, 2.0, 3.0);

        let expected = Vector::new(14.0, 22.0, 32.0);

        assert_eq!(m * v, expected);
    }

    #[test]
    fn mult_with_point() {
        let m = {
            let mut m = Matrix::new(4);

            m[0][0] = 1.0;
            m[0][1] = 2.0;
            m[0][2] = 3.0;
            m[0][3] = 4.0;

            m[1][0] = 2.0;
            m[1][1] = 4.0;
            m[1][2] = 4.0;
            m[1][3] = 2.0;

            m[2][0] = 8.0;
            m[2][1] = 6.0;
            m[2][2] = 4.0;
            m[2][3] = 1.0;

            m[3][0] = 0.0;
            m[3][1] = 0.0;
            m[3][2] = 0.0;
            m[3][3] = 1.0;

            m
        };

        let v = Point::new(1.0, 2.0, 3.0);

        let expected = Point::new(18.0, 24.0, 33.0);

        assert_eq!(m * v, expected);
    }

    #[test]
    fn id() {
        {
            let m = {
                let mut m = Matrix::new(4);

                m[0][0] = 1.0;
                m[0][1] = 2.0;
                m[0][2] = 3.0;
                m[0][3] = 4.0;

                m[1][0] = 2.0;
                m[1][1] = 4.0;
                m[1][2] = 4.0;
                m[1][3] = 2.0;

                m[2][0] = 8.0;
                m[2][1] = 6.0;
                m[2][2] = 4.0;
                m[2][3] = 1.0;

                m[3][0] = 0.0;
                m[3][1] = 0.0;
                m[3][2] = 0.0;
                m[3][3] = 1.0;

                m
            };

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
            let m = {
                let mut m = Matrix::new(4);

                m[0][0] = 0.0;
                m[0][1] = 9.0;
                m[0][2] = 3.0;
                m[0][3] = 0.0;

                m[1][0] = 9.0;
                m[1][1] = 8.0;
                m[1][2] = 0.0;
                m[1][3] = 8.0;

                m[2][0] = 1.0;
                m[2][1] = 8.0;
                m[2][2] = 5.0;
                m[2][3] = 3.0;

                m[3][0] = 0.0;
                m[3][1] = 0.0;
                m[3][2] = 5.0;
                m[3][3] = 8.0;

                m
            };

            let expected = {
                let mut m = Matrix::new(4);

                m[0][0] = 0.0;
                m[0][1] = 9.0;
                m[0][2] = 1.0;
                m[0][3] = 0.0;

                m[1][0] = 9.0;
                m[1][1] = 8.0;
                m[1][2] = 8.0;
                m[1][3] = 0.0;

                m[2][0] = 3.0;
                m[2][1] = 0.0;
                m[2][2] = 5.0;
                m[2][3] = 5.0;

                m[3][0] = 0.0;
                m[3][1] = 8.0;
                m[3][2] = 3.0;
                m[3][3] = 8.0;

                m
            };

            assert_eq!(m.transpose(), expected);
        }
    }

    #[test]
    fn determinant() {
        {
            let mut m = Matrix::new(2);
            m[0][0] = 1.0;
            m[0][1] = 5.0;
            m[1][0] = -3.0;
            m[1][1] = 2.0;

            assert_eq!(m.determinant(), 17.0);
        }
        {
            let m = {
                let mut m = Matrix::new(3);
                m[0][0] = 1.0;
                m[0][1] = 2.0;
                m[0][2] = 6.0;

                m[1][0] = -5.0;
                m[1][1] = 8.0;
                m[1][2] = -4.0;

                m[2][0] = 2.0;
                m[2][1] = 6.0;
                m[2][2] = 4.0;

                m
            };

            assert_eq!(m.determinant(), -196.0);
        }
        {
            let m = {
                let mut m = Matrix::new(4);

                m[0][0] = -2.0;
                m[0][1] = -8.0;
                m[0][2] = 3.0;
                m[0][3] = 5.0;

                m[1][0] = -3.0;
                m[1][1] = 1.0;
                m[1][2] = 7.0;
                m[1][3] = 3.0;

                m[2][0] = 1.0;
                m[2][1] = 2.0;
                m[2][2] = -9.0;
                m[2][3] = 6.0;

                m[3][0] = -6.0;
                m[3][1] = 7.0;
                m[3][2] = 7.0;
                m[3][3] = -9.0;

                m
            };

            assert_eq!(m.determinant(), -4071.0);
        }
    }

    #[test]
    fn submatrix() {
        {
            let m = {
                let mut m = Matrix::new(3);
                m[0][0] = 1.0;
                m[0][1] = 5.0;
                m[0][2] = 0.0;

                m[1][0] = -3.0;
                m[1][1] = 2.0;
                m[1][2] = 7.0;

                m[2][0] = 0.0;
                m[2][1] = 6.0;
                m[2][2] = -3.0;

                m
            };

            let expected = {
                let mut m = Matrix::new(2);
                m[0][0] = -3.0;
                m[0][1] = 2.0;

                m[1][0] = 0.0;
                m[1][1] = 6.0;

                m
            };

            assert_eq!(m.submatrix(0, 2), expected);
        }
        {
            let m = {
                let mut m = Matrix::new(4);

                m[0][0] = -6.0;
                m[0][1] = 1.0;
                m[0][2] = 1.0;
                m[0][3] = 6.0;

                m[1][0] = -8.0;
                m[1][1] = 5.0;
                m[1][2] = 8.0;
                m[1][3] = 6.0;

                m[2][0] = -1.0;
                m[2][1] = 0.0;
                m[2][2] = 8.0;
                m[2][3] = 2.0;

                m[3][0] = -7.0;
                m[3][1] = 1.0;
                m[3][2] = -1.0;
                m[3][3] = 1.0;

                m
            };

            let expected = {
                let mut m = Matrix::new(3);
                m[0][0] = -6.0;
                m[0][1] = 1.0;
                m[0][2] = 6.0;

                m[1][0] = -8.0;
                m[1][1] = 8.0;
                m[1][2] = 6.0;

                m[2][0] = -7.0;
                m[2][1] = -1.0;
                m[2][2] = 1.0;

                m
            };

            assert_eq!(m.submatrix(2, 1), expected);
        }
    }

    #[test]
    fn minor() {
        let m = {
            let mut m = Matrix::new(3);
            m[0][0] = 3.0;
            m[0][1] = 5.0;
            m[0][2] = 0.0;

            m[1][0] = 2.0;
            m[1][1] = -1.0;
            m[1][2] = -7.0;

            m[2][0] = 6.0;
            m[2][1] = -1.0;
            m[2][2] = 5.0;

            m
        };

        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor() {
        let m = {
            let mut m = Matrix::new(3);
            m[0][0] = 3.0;
            m[0][1] = 5.0;
            m[0][2] = 0.0;

            m[1][0] = 2.0;
            m[1][1] = -1.0;
            m[1][2] = -7.0;

            m[2][0] = 6.0;
            m[2][1] = -1.0;
            m[2][2] = 5.0;

            m
        };

        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn invert() {
        {
            let m = {
                let mut m = Matrix::new(4);

                m[0][0] = -4.0;
                m[0][1] = 2.0;
                m[0][2] = -2.0;
                m[0][3] = -3.0;

                m[1][0] = 9.0;
                m[1][1] = 6.0;
                m[1][2] = 2.0;
                m[1][3] = 6.0;

                m[2][0] = 0.0;
                m[2][1] = -5.0;
                m[2][2] = 1.0;
                m[2][3] = -5.0;

                m[3][0] = 0.0;
                m[3][1] = 0.0;
                m[3][2] = 0.0;
                m[3][3] = 0.0;

                m
            };

            assert_eq!(m.invert(), None);
        }
        {
            let m = {
                let mut m = Matrix::new(4);

                m[0][0] = -5.0;
                m[0][1] = 2.0;
                m[0][2] = 6.0;
                m[0][3] = -8.0;

                m[1][0] = 1.0;
                m[1][1] = -5.0;
                m[1][2] = 1.0;
                m[1][3] = 8.0;

                m[2][0] = 7.0;
                m[2][1] = 7.0;
                m[2][2] = -6.0;
                m[2][3] = -7.0;

                m[3][0] = 1.0;
                m[3][1] = -3.0;
                m[3][2] = 7.0;
                m[3][3] = 4.0;

                m
            };

            let expected = {
                let mut m = Matrix::new(4);

                m[0][0] = 0.21805;
                m[0][1] = 0.45133;
                m[0][2] = 0.24060;
                m[0][3] = -0.04511;

                m[1][0] = -0.80827;
                m[1][1] = -1.45677;
                m[1][2] = -0.44361;
                m[1][3] = 0.52068;

                m[2][0] = -0.07895;
                m[2][1] = -0.22368;
                m[2][2] = -0.05263;
                m[2][3] = 0.19737;

                m[3][0] = -0.52256;
                m[3][1] = -0.81391;
                m[3][2] = -0.30075;
                m[3][3] = 0.30639;

                m
            };

            assert_eq!(m.invert(), Some(expected));
        }
        {
            let a = {
                let mut m = Matrix::new(4);

                m[0][0] = 3.0;
                m[0][1] = -9.0;
                m[0][2] = 7.0;
                m[0][3] = 3.0;

                m[1][0] = 3.0;
                m[1][1] = -8.0;
                m[1][2] = 2.0;
                m[1][3] = -9.0;

                m[2][0] = -4.0;
                m[2][1] = 4.0;
                m[2][2] = 4.0;
                m[2][3] = 1.0;

                m[3][0] = -6.0;
                m[3][1] = 5.0;
                m[3][2] = -1.0;
                m[3][3] = 1.0;

                m
            };

            let b = {
                let mut m = Matrix::new(4);

                m[0][0] = 8.0;
                m[0][1] = 2.0;
                m[0][2] = 2.0;
                m[0][3] = 2.0;

                m[1][0] = 3.0;
                m[1][1] = -1.0;
                m[1][2] = 7.0;
                m[1][3] = 0.0;

                m[2][0] = 7.0;
                m[2][1] = 0.0;
                m[2][2] = 5.0;
                m[2][3] = 4.0;

                m[3][0] = 6.0;
                m[3][1] = -2.0;
                m[3][2] = 0.0;
                m[3][3] = 5.0;

                m
            };

            let c = a * b;

            assert_eq!(c * b.invert().unwrap(), a);
        }
    }
}

// --------------------------------------------------------------------------------------------- //
