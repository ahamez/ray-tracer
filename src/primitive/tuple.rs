// --------------------------------------------------------------------------------------------- //

use crate::{primitive::matrix::Matrix, rtc::Transform};

// --------------------------------------------------------------------------------------------- //

pub trait Tuple {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn zero() -> Self;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}

// --------------------------------------------------------------------------------------------- //

impl<T> Transform for T
where
    T: Tuple + Copy,
{
    fn apply_transformation(&self, transformation: &Matrix) -> Self {
        *transformation * *self
    }
}

// --------------------------------------------------------------------------------------------- //