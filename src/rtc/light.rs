/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Point, Vector},
    rtc::{
        lights::{AreaLight, PointLight},
        Color, World,
    },
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub enum LightType {
    AreaLight(AreaLight),
    PointLight(PointLight),
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct Light {
    light: LightType,
}

/* ---------------------------------------------------------------------------------------------- */

impl Light {
    pub fn new_area_light(
        intensity: Color,
        corner: Point,
        uvec: Vector,
        usteps: u32,
        vvec: Vector,
        vsteps: u32,
    ) -> Self {
        Light {
            light: LightType::AreaLight(AreaLight::new(
                intensity, corner, uvec, usteps, vvec, vsteps,
            )),
        }
    }

    pub fn new_point_light(intensity: Color, position: Point) -> Self {
        Light {
            light: LightType::PointLight(PointLight::new(intensity, position)),
        }
    }

    pub fn intensity(&self) -> Color {
        match &self.light {
            LightType::AreaLight(l) => l.intensity(),
            LightType::PointLight(l) => l.intensity(),
        }
    }

    pub fn positions(&self) -> &[Point] {
        match &self.light {
            LightType::AreaLight(l) => l.positions(),
            LightType::PointLight(l) => l.positions(),
        }
    }

    pub fn intensity_at(&self, world: &World, point: &Point) -> f64 {
        match &self.light {
            LightType::AreaLight(l) => l.intensity_at(world, point),
            LightType::PointLight(l) => l.intensity_at(world, point),
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Tuple;

    #[test]
    fn point_lights_evaluate_the_light_intensity_at_given_point() {
        let w = crate::rtc::world::tests::default_world();
        let light = &w.lights()[0];

        let tests = vec![
            (Point::new(0.0, 1.0001, 0.0), 1.0),
            (Point::new(-1.0001, 0.0, 0.0), 1.0),
            (Point::new(0.0, 0.0, -1.0001), 1.0),
            (Point::new(0.0, 0.0, 1.0001), 0.0),
            (Point::new(1.0001, 0.0, 0.0), 0.0),
            (Point::new(0.0, -1.0001, 0.0), 0.0),
            (Point::new(0.0, 0.0, 0.0), 0.0),
        ];

        for (point, result) in tests.into_iter() {
            assert_eq!(light.intensity_at(&w, &point), result);
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */
