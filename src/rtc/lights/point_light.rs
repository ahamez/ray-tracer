// --------------------------------------------------------------------------------------------- //

use crate::{
    primitive::Point,
    rtc::{Color, World},
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    intensity: Color,
    position: Point,
}

// --------------------------------------------------------------------------------------------- //

impl PointLight {
    pub fn new(intensity: Color, position: Point) -> Self {
        PointLight {
            intensity,
            position,
        }
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }

    pub fn intensity_at(&self, world: &World, point: &Point) -> f64 {
        if world.is_shadowed(&self.position, point) {
            0.0
        } else {
            1.0
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }
}

// --------------------------------------------------------------------------------------------- //
