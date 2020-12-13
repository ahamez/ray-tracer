// --------------------------------------------------------------------------------------------- //

use crate::{
    color::Color, matrix::Matrix, object::Object, point::Point, transformation::Transform,
    tuple::Tuple,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pattern: Patterns,
    transformation: Matrix,
}

// --------------------------------------------------------------------------------------------- //

impl Pattern {
    pub fn new_gradient(from: Color, to: Color) -> Self {
        Pattern {
            pattern: Patterns::Gradient(GradientPattern { from, to }),
            transformation: Matrix::id(),
        }
    }

    pub fn new_plain(color: Color) -> Self {
        Pattern {
            pattern: Patterns::Plain(PlainPattern { color }),
            transformation: Matrix::id(),
        }
    }

    pub fn new_stripe(colors: Vec<Color>) -> Self {
        Pattern {
            pattern: Patterns::Stripe(StripePattern { colors }),
            transformation: Matrix::id(),
        }
    }

    pub fn new_ring(colors: Vec<Color>) -> Self {
        Pattern {
            pattern: Patterns::Ring(RingPattern { colors }),
            transformation: Matrix::id(),
        }
    }

    fn pattern_at(&self, point: &Point) -> Color {
        match &self.pattern {
            Patterns::Gradient(p) => p.pattern_at(point),
            Patterns::Plain(p) => p.pattern_at(point),
            Patterns::Ring(p) => p.pattern_at(point),
            Patterns::Stripe(p) => p.pattern_at(point),
        }
    }

    pub fn pattern_at_object(&self, object: &Object, world_point: &Point) -> Color {
        let object_transformation_inv = object.transformation().invert().unwrap();
        let object_point = object_transformation_inv * *world_point;

        let pattern_transformation_inv = self.transformation.invert().unwrap();
        let pattern_point = pattern_transformation_inv * object_point;

        self.pattern_at(&pattern_point)
    }
}

// --------------------------------------------------------------------------------------------- //

impl Transform for Pattern {
    fn apply_transformation(&self, transformation: &Matrix) -> Self {
        Pattern {
            transformation: self.transformation * *transformation,
            ..self.clone()
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
enum Patterns {
    Gradient(GradientPattern),
    Plain(PlainPattern),
    Ring(RingPattern),
    Stripe(StripePattern),
}

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct GradientPattern {
    from: Color,
    to: Color,
}

impl GradientPattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let distance = self.to - self.from;
        let fraction = point.x() - point.x().floor();

        self.from + distance * fraction
    }
}

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct PlainPattern {
    color: Color,
}

impl PlainPattern {
    fn pattern_at(&self, _point: &Point) -> Color {
        self.color
    }
}

// --------------------------------------------------------------------------------------------- //

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

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct StripePattern {
    colors: Vec<Color>,
}

impl StripePattern {
    fn pattern_at(&self, point: &Point) -> Color {
        let index = point.x().floor().abs() as usize % self.colors.len();

        self.colors[index]
    }
}

// --------------------------------------------------------------------------------------------- //

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
            pattern.pattern_at(&Point::new(-2.0, 0.0, 0.0)),
            Color::red()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(-1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(0.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(1.0, 0.0, 0.0)),
            Color::black()
        );
        assert_eq!(pattern.pattern_at(&Point::new(2.0, 0.0, 0.0)), Color::red());
        assert_eq!(
            pattern.pattern_at(&Point::new(3.0, 0.0, 0.0)),
            Color::white()
        );
        assert_eq!(
            pattern.pattern_at(&Point::new(4.0, 0.0, 0.0)),
            Color::black()
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Object::new_sphere().scale(2.0, 2.0, 2.0);
        let pattern = Pattern::new_stripe(vec![Color::white(), Color::black()]);

        assert_eq!(
            pattern.pattern_at_object(&object, &Point::new(1.5, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Object::new_sphere();
        let pattern =
            Pattern::new_stripe(vec![Color::white(), Color::black()]).scale(2.0, 2.0, 2.0);

        assert_eq!(
            pattern.pattern_at_object(&object, &Point::new(1.5, 0.0, 0.0)),
            Color::white()
        );
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Object::new_sphere().scale(2.0, 2.0, 2.0);
        let pattern =
            Pattern::new_stripe(vec![Color::white(), Color::black()]).scale(2.0, 2.0, 2.0);

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

// --------------------------------------------------------------------------------------------- //
