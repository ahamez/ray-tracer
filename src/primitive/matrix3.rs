/* ---------------------------------------------------------------------------------------------- */

use crate::{float::ApproxEq, primitive::matrix2::Matrix2};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug)]
pub struct Matrix3 {
    data: [[f64; 3]; 3],
}

/* ---------------------------------------------------------------------------------------------- */

impl Matrix3 {
    pub fn new() -> Self {
        Self {
            data: [[0.0; 3]; 3],
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut res = 0.0;
        for col in 0..3 {
            res += self[0][col] * self.cofactor(0, col);
        }

        res
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let mut res = Matrix2::new();

        let mut new_row = 0;
        for i in 0..3 {
            let mut new_col = 0;
            if i != row {
                for j in 0..3 {
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
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Matrix3) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !self[i][j].approx_eq_low_precision(other[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Index<usize> for Matrix3 {
    type Output = [f64; 3];

    fn index(&self, row: usize) -> &[f64; 3] {
        &self.data[row]
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::IndexMut<usize> for Matrix3 {
    fn index_mut(&mut self, row: usize) -> &mut [f64; 3] {
        &mut self.data[row]
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submatrix() {
        let mut m = Matrix3::new();
        m[0][0] = 1.0;
        m[0][1] = 5.0;
        m[0][2] = 0.0;

        m[1][0] = -3.0;
        m[1][1] = 2.0;
        m[1][2] = 7.0;

        m[2][0] = 0.0;
        m[2][1] = 6.0;
        m[2][2] = -3.0;

        let mut expected = Matrix2::new();
        expected[0][0] = -3.0;
        expected[0][1] = 2.0;

        expected[1][0] = 0.0;
        expected[1][1] = 6.0;

        assert_eq!(m.submatrix(0, 2), expected);
    }

    #[test]
    fn determinant() {
        let mut m = Matrix3::new();
        m[0][0] = 1.0;
        m[0][1] = 2.0;
        m[0][2] = 6.0;

        m[1][0] = -5.0;
        m[1][1] = 8.0;
        m[1][2] = -4.0;

        m[2][0] = 2.0;
        m[2][1] = 6.0;
        m[2][2] = 4.0;

        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn minor() {
        let mut m = Matrix3::new();
        m[0][0] = 3.0;
        m[0][1] = 5.0;
        m[0][2] = 0.0;

        m[1][0] = 2.0;
        m[1][1] = -1.0;
        m[1][2] = -7.0;

        m[2][0] = 6.0;
        m[2][1] = -1.0;
        m[2][2] = 5.0;

        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor() {
        let mut m = Matrix3::new();
        m[0][0] = 3.0;
        m[0][1] = 5.0;
        m[0][2] = 0.0;

        m[1][0] = 2.0;
        m[1][1] = -1.0;
        m[1][2] = -7.0;

        m[2][0] = 6.0;
        m[2][1] = -1.0;
        m[2][2] = 5.0;

        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }
}

/* ---------------------------------------------------------------------------------------------- */
