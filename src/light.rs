// --------------------------------------------------------------------------------------------- //

use crate::color::Color;
use crate::point::Point;

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug)]
pub struct Light {
    pub intensity: Color,
    pub position: Point,
}

// --------------------------------------------------------------------------------------------- //

impl Light {
    pub fn new(intensity: Color, position: Point) -> Self {
        Light {
            intensity,
            position,
        }
    }
}

// --------------------------------------------------------------------------------------------- //
