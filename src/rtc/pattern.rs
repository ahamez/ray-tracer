/* ---------------------------------------------------------------------------------------------- */

use crate::{
    float::ApproxEq,
    primitive::{Matrix, Point, Tuple},
    rtc::{Color, Object, Transform},
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pattern: Patterns,
    transformation: Matrix,
    transformation_inverse: Matrix,
}

/* ---------------------------------------------------------------------------------------------- */

impl Pattern {
    pub fn new_checker(c1: Color, c2: Color) -> Self {
        Pattern {
            pattern: Patterns::Checker(CheckerPattern { c1, c2 }),
            ..Default::default()
        }
    }

    pub fn new_gradient(from: Color, to: Color) -> Self {
        Pattern {
            pattern: Patterns::Gradient(GradientPattern { from, to }),
            ..Default::default()
        }
    }

    pub fn new_plain(color: Color) -> Self {
        Pattern {
            pattern: Patterns::Plain(PlainPattern { color }),
            ..Default::default()
        }
    }

    pub fn new_ring(colors: Vec<Color>) -> Self {
        Pattern {
            pattern: Patterns::Ring(RingPattern { colors }),
            ..Default::default()
        }
    }

    pub fn new_stripe(colors: Vec<Color>) -> Self {
        Pattern {
            pattern: Patterns::Stripe(StripePattern { colors }),
            ..Default::default()
        }
    }
    pub fn new_test() -> Self {
        Pattern {
            pattern: Patterns::Test(TestPattern {}),
            ..Default::default()
        }
    }

    fn pattern_at(&self, point: &Point) -> Color {
        match &self.pattern {
            Patterns::Checker(p) => p.pattern_at(point),
            Patterns::Gradient(p) => p.pattern_at(point),
            Patterns::Plain(p) => p.pattern_at(point),
            Patterns::Ring(p) => p.pattern_at(point),
            Patterns::Stripe(p) => p.pattern_at(point),
            Patterns::Test(p) => p.pattern_at(point),
        }
    }

    pub fn pattern_at_object(&self, object: &Object, world_point: &Point) -> Color {
        let object_transformation_inv = object.transformation_inverse();
        let object_point = *object_transformation_inv * *world_point;

        let pattern_point = self.transformation_inverse * object_point;

        self.pattern_at(&pattern_point)
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Default for Pattern {
    fn default() -> Self {
        Pattern {
            pattern: Patterns::Plain(PlainPattern {
                color: Color::white(),
            }),
            transformation: Matrix::id(),
            transformation_inverse: Matrix::id(),
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Transform for Pattern {
    fn transform(self, transformation: &Matrix) -> Self {
        let new_transformation = *transformation * self.transformation;

        Pattern {
            transformation: new_transformation,
            transformation_inverse: new_transformation.invert(),
            ..self
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
enum Patterns {
    Checker(CheckerPattern),
    Gradient(GradientPattern),
    Plain(PlainPattern),
    Ring(RingPattern),
    Stripe(StripePattern),
    Test(TestPattern),
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct CheckerPattern {
    c1: Color,
    c2: Color,
}

impl CheckerPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let sum = point.x().floor() + point.y().floor() + point.z().floor();
        if (sum % 2.0).approx_eq(0.0) {
            self.c1
        } else {
            self.c2
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct GradientPattern {
    from: Color,
    to: Color,
}

impl GradientPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        self.from + point.x() * (self.to - self.from)
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct PlainPattern {
    color: Color,
}

impl PlainPattern {
    fn pattern_at(&self, _point: &Point) -> Color {
        self.color
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct RingPattern {
    colors: Vec<Color>,
}

impl RingPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let distance = (point.x() * point.x() + point.z() * point.z()).sqrt();
        let index = distance.floor() as usize % self.colors.len();

        self.colors[index]
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct StripePattern {
    colors: Vec<Color>,
}

impl StripePattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let scaled_x = point.x() * self.colors.len() as f64;
        let index = (scaled_x.floor().abs() as usize) % self.colors.len();

        self.colors[index]
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct TestPattern {}

impl TestPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        Color::new(point.x(), point.y(), point.z())
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn a_stripe_pattern_is_constant_in_y_and_z() {
        let pattern = StripePattern {
            colors: vec![Color::white(), Color::black(), Color::red()],
        };

        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 1.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 2.0, 0.0)),
            Color::white()
        );

        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 1.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 2.0)),
            Color::white()
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern {
            colors: vec![Color::white(), Color::black(), Color::red()],
        };

        assert_eq!(
            pattern.pattern_at(&Point::new(-0.2, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.4, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(pattern.pattern_at(&Point::new(0.7, 0.0, 0.0)), Color::red());
        assert_eq!(
            pattern.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Object::new_sphere().scale(2.0, 2.0, 2.0).transform();
        let pattern = Pattern::new_stripe(vec![Color::white(), Color::black()]);

        assert_eq!(
            pattern.pattern_at_object(&object, &Point::new(2.5, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Object::new_sphere();
        let pattern = Pattern::new_stripe(vec![Color::white(), Color::black()])
            .scale(2.0, 2.0, 2.0)
            .transform();

        assert_eq!(
            pattern.pattern_at_object(&object, &Point::new(2.5, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Object::new_sphere().scale(2.0, 2.0, 2.0).transform();
        let pattern = Pattern::new_stripe(vec![Color::white(), Color::black()])
            .scale(2.0, 2.0, 2.0)
            .transform();

        assert_eq!(
            pattern.pattern_at_object(&object, &Point::new(1.5, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Pattern::new_gradient(Color::white(), Color::black());

        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern::new_ring(vec![Color::white(), Color::black()]);

        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 1.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.708, 0.0, 0.708)),
            Color::black()
        );
    }
}

/* ---------------------------------------------------------------------------------------------- */
