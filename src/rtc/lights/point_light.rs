/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::Point,
    rtc::{Color, World},
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    intensity: Color,
    position: [Point; 1],
}

/* ---------------------------------------------------------------------------------------------- */

impl PointLight {
    pub fn new(intensity: Color, position: Point) -> Self {
        PointLight {
            intensity,
            position: [position],
        }
    }

    pub fn intensity(&self) -> Color {
        self.intensity
    }

    pub fn intensity_at(&self, world: &World, point: &Point) -> f64 {
        if world.is_shadowed(&self.position[0], point) {
            0.0
        } else {
            1.0
        }
    }

    pub fn positions(&self) -> &[Point] {
        &self.position
    }
}

/* ---------------------------------------------------------------------------------------------- */
