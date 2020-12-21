// --------------------------------------------------------------------------------------------- //

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::{
    primitive::{Point, Tuple, Vector},
    rtc::{Color, World},
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug)]
pub struct AreaLight {
    intensity: Color,
    position: Point,
    corner: Point,
    uvec: Vector,
    usteps: u32,
    vvec: Vector,
    vsteps: u32,
    samples: u32,
}

// --------------------------------------------------------------------------------------------- //

impl AreaLight {
    pub fn new(
        intensity: Color,
        corner: Point,
        uvec: Vector,
        usteps: u32,
        vvec: Vector,
        vsteps: u32,
    ) -> Self {
        AreaLight {
            intensity,
            position: Point::new(
                corner.x() + uvec.x() / 2.0,
                corner.y() + vvec.y() / 2.0,
                corner.z() + vvec.z() / 2.0,
            ),
            corner,
            uvec: uvec / usteps as f64,
            usteps,
            vvec: vvec / vsteps as f64,
            vsteps,
            samples: usteps * vsteps,
        }
    }
    pub fn intensity(&self) -> Color {
        self.intensity
    }

    pub fn intensity_at(&self, world: &World, point: &Point) -> f64 {
        let mut rng = SmallRng::from_entropy();
        self.intensity_at_impl(world, point, || rng.gen())
    }

    pub fn position(&self) -> Point {
        self.position
    }

    fn point_on_light<T>(&self, u: u32, v: u32, mut random: T) -> Point
    where
        T: FnMut() -> f64,
    {
        let random = random();
        self.corner + self.uvec * (u as f64 + random) + self.vvec * (v as f64 + random)
    }

    fn intensity_at_impl<T>(&self, world: &World, point: &Point, mut random: T) -> f64
    where
        T: FnMut() -> f64,
    {
        let mut total = 0.0;

        for v in 0..self.vsteps {
            for u in 0..self.usteps {
                let light_position = self.point_on_light(u, v, || random());
                if !world.is_shadowed(&light_position, &point) {
                    total += 1.0;
                }
            }
        }

        total / self.samples as f64
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_an_area_light() {
        let corner = Point::zero();
        let v1 = Vector::new(2.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 0.0, 1.0);
        let light = AreaLight::new(Color::white(), corner, v1, 4, v2, 2);

        assert_eq!(light.corner, corner);
        assert_eq!(light.uvec, Vector::new(0.5, 0.0, 0.0));
        assert_eq!(light.usteps, 4);
        assert_eq!(light.vvec, Vector::new(0.0, 0.0, 0.5));
        assert_eq!(light.vsteps, 2);
        assert_eq!(light.samples, 8);
        assert_eq!(light.position, Point::new(1.0, 0.0, 0.5));
    }

    #[test]
    fn finding_a_single_point_on_an_area_light() {
        let corner = Point::zero();
        let v1 = Vector::new(2.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 0.0, 1.0);
        let light = AreaLight::new(Color::white(), corner, v1, 4, v2, 2);

        let tests = vec![
            (0, 0, Point::new(0.25, 0.0, 0.25)),
            (1, 0, Point::new(0.75, 0.0, 0.25)),
            (0, 1, Point::new(0.25, 0.0, 0.75)),
            (2, 0, Point::new(1.25, 0.0, 0.25)),
            (3, 1, Point::new(1.75, 0.0, 0.75)),
        ];

        for (u, v, point) in tests.into_iter() {
            assert_eq!(light.point_on_light(u, v, || 0.5), point);
        }
    }

    #[test]
    fn the_area_light_intensity_function() {
        let w = crate::rtc::world::tests::default_world();

        let corner = Point::new(-0.5, -0.5, -5.0);
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let light = AreaLight::new(Color::white(), corner, v1, 2, v2, 2);

        let tests = vec![
            (Point::new(0.0, 0.0, 2.0), 0.0),
            (Point::new(1.0, -1.0, 2.0), 0.25),
            (Point::new(1.5, 0.0, 2.0), 0.5),
            (Point::new(1.25, 1.25, 3.0), 0.75),
            (Point::new(0.0, 0.0, -2.0), 1.0),
        ];

        for (point, result) in tests.into_iter() {
            assert_eq!(light.intensity_at_impl(&w, &point, || 0.5), result);
        }
    }
}

// --------------------------------------------------------------------------------------------- //
