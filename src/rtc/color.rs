/* ---------------------------------------------------------------------------------------------- */

use crate::float::ApproxEq;

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

/* ---------------------------------------------------------------------------------------------- */

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn red() -> Color {
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.r.approx_eq_low_precision(other.r)
            && self.g.approx_eq_low_precision(other.g)
            && self.b.approx_eq_low_precision(other.b)
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

/* ---------------------------------------------------------------------------------------------- */

// Hadamard product
impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let c1 = Color {
            r: 0.9,
            g: 0.6,
            b: 0.75,
        };

        let c2 = Color {
            r: 0.7,
            g: 0.1,
            b: 0.25,
        };

        let res = c1 + c2;
        let expected = Color {
            r: 1.6,
            g: 0.7,
            b: 1.0,
        };

        assert_eq!(res, expected);
    }
}
