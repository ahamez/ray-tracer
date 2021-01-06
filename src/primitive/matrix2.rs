/* ---------------------------------------------------------------------------------------------- */

use crate::float::ApproxEq;

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug)]
pub struct Matrix2 {
    data: [[f64; 2]; 2],
}

/* ---------------------------------------------------------------------------------------------- */

impl Matrix2 {
    pub fn new() -> Self {
        Self {
            data: [[0.0; 2]; 2],
        }
    }

    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Matrix2) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                if !self[i][j].approx_eq_low_precision(other[i][j]) {
                    return false;
                }
            }
        }
        true
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Index<usize> for Matrix2 {
    type Output = [f64; 2];

    fn index(&self, row: usize) -> &[f64; 2] {
        &self.data[row]
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::IndexMut<usize> for Matrix2 {
    fn index_mut(&mut self, row: usize) -> &mut [f64; 2] {
        &mut self.data[row]
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let mut m = Matrix2::new();

        m[0][0] = -3.0;
        m[0][1] = 5.0;

        m[1][0] = 1.0;
        m[1][1] = -2.0;

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    #[test]
    fn eq() {
        {
            let mut m0 = Matrix2::new();
            m0[0][0] = -3.0;
            m0[0][1] = 5.0;
            m0[1][0] = 1.0;
            m0[1][1] = -2.0;

            let m1 = m0.clone();

            assert_eq!(m0, m1);
        }
        {
            let mut m0 = Matrix2::new();
            m0[0][0] = -3.0;
            m0[0][1] = 5.0;
            m0[1][0] = 1.0;
            m0[1][1] = -2.0;

            let mut m1 = Matrix2::new();
            m1[0][0] = -3.1;
            m1[0][1] = 5.0;
            m1[1][0] = 1.0;
            m1[1][1] = -2.0;

            assert_ne!(m0, m1);
        }
    }

    #[test]
    fn determinant() {
        let mut m = Matrix2::new();
        m[0][0] = 1.0;
        m[0][1] = 5.0;
        m[1][0] = -3.0;
        m[1][1] = 2.0;

        assert_eq!(m.determinant(), 17.0);
    }
}

/* ---------------------------------------------------------------------------------------------- */
