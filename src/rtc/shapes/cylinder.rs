/* ---------------------------------------------------------------------------------------------- */

use crate::{
    float::{ApproxEq, EPSILON},
    primitive::{Point, Tuple, Vector},
    rtc::{BoundingBox, IntersectionPusher, Ray},
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cylinder {
    min: f64,
    max: f64,
    closed: bool,
}

/* ---------------------------------------------------------------------------------------------- */

impl Cylinder {
    pub fn new(min: f64, max: f64, closed: bool) -> Self {
        let (min, max) = if min < max { (min, max) } else { (max, min) };

        Cylinder { min, max, closed }
    }

    pub fn intersects(&self, ray: &Ray, push: &mut impl IntersectionPusher) {
        let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);

        if a.approx_eq(0.0) {
            self.intersects_caps(&ray, push);
        } else {
            let b = 2.0 * (ray.origin.x() * ray.direction.x() + ray.origin.z() * ray.direction.z());
            let c = ray.origin.x().powi(2) + ray.origin.z().powi(2) - 1.0;

            let discriminant = b.powi(2) - 4.0 * a * c;

            if discriminant < 0.0 {
                return;
            }

            let double_a = 2.0 * a;
            let t0 = (-b - discriminant.sqrt()) / double_a;
            let t1 = (-b + discriminant.sqrt()) / double_a;

            let y0 = ray.origin.y() + t0 * ray.direction.y();
            if self.min < y0 && y0 < self.max {
                push.t(t0);
            }

            let y1 = ray.origin.y() + t1 * ray.direction.y();
            if self.min < y1 && y1 < self.max {
                push.t(t1);
            }

            self.intersects_caps(&ray, push);
        }
    }

    fn check_cap(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        (x.powi(2) + z.powi(2)) <= 1.0
    }

    pub fn intersects_caps(&self, ray: &Ray, push: &mut impl IntersectionPusher) {
        if !self.closed || ray.direction.y().approx_eq(0.0) {
            return;
        }

        let t = (self.min - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(&ray, t) {
            push.t(t);
        }

        let t = (self.max - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(&ray, t) {
            push.t(t);
        }
    }

    pub fn normal_at(&self, object_point: &Point) -> Vector {
        let dist = object_point.x().powi(2) + object_point.z().powi(2);

        if dist < 1.0 && object_point.y() >= (self.max - EPSILON) {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && object_point.y() <= (self.min + EPSILON) {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(object_point.x(), 0.0, object_point.z())
        }
    }

    pub fn bounds(&self) -> BoundingBox {
        BoundingBox::new()
            .with_min(Point::new(-1.0, self.min, -1.0))
            .with_max(Point::new(1.0, self.max, 1.0))
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            closed: false,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::rtc::{IntersectionPusher, Object};
    use std::sync::Arc;

    struct Push {
        pub xs: Vec<f64>,
    }

    impl IntersectionPusher for Push {
        fn t(&mut self, t: f64) {
            self.xs.push(t);
        }
        fn set_object(&mut self, _object: Arc<Object>) {
            panic!();
        }
    }

    #[test]
    fn the_default_values_for_a_cylinder() {
        let c: Cylinder = Default::default();
        assert_eq!(c.min, f64::NEG_INFINITY);
        assert_eq!(c.max, f64::INFINITY);
        assert_eq!(c.closed, false);
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        fn test(origin: Point, direction: Vector, t0: f64, t1: f64) {
            let ray = Ray {
                origin,
                direction: direction.normalize(),
            };
            let mut push = Push { xs: vec![] };

            let c: Cylinder = Default::default();
            c.intersects(&ray, &mut push);

            assert_eq!(push.xs.len(), 2);
            assert!(push.xs[0].approx_eq_low_precision(t0));
            assert!(push.xs[1].approx_eq_low_precision(t1));
        }

        test(
            Point::new(1.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            5.0,
            5.0,
        );
        test(
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            4.0,
            6.0,
        );

        test(
            Point::new(0.5, 0.0, -5.0),
            Vector::new(0.1, 1.0, 1.0),
            6.80798,
            7.08872,
        );
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let tests = vec![
            (Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
            (Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
            (Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
        ];

        let c = Cylinder::new(1.0, 2.0, false);
        for (origin, direction, count) in tests.into_iter() {
            let mut push = Push { xs: vec![] };
            c.intersects(
                &Ray {
                    origin,
                    direction: direction.normalize(),
                },
                &mut push,
            );
            assert_eq!(push.xs.len(), count as usize);
        }
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let tests = vec![
            (Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 2),
            (Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 2),
            (Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 2.0), 2),
            (Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 2),
        ];

        let c = Cylinder::new(1.0, 2.0, true);
        for (origin, direction, count) in tests.into_iter() {
            let mut push = Push { xs: vec![] };
            c.intersects(
                &Ray {
                    origin,
                    direction: direction.normalize(),
                },
                &mut push,
            );
            assert_eq!(push.xs.len(), count as usize);
        }
    }

    #[test]
    fn a_ray_misses_a_cylinder() {
        fn test(origin: Point, direction: Vector) {
            let ray = Ray {
                origin,
                direction: direction.normalize(),
            };
            let mut push = Push { xs: vec![] };
            let c: Cylinder = Default::default();
            c.intersects(&ray, &mut push);
            assert_eq!(push.xs.len(), 0);
        }

        test(Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        test(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        test(Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let c: Cylinder = Default::default();
        assert_eq!(
            c.normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            c.normal_at(&Point::new(0.0, 5.0, -1.0)),
            Vector::new(0.0, 0.0, -1.0)
        );
        assert_eq!(
            c.normal_at(&Point::new(0.0, -2.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );
        assert_eq!(
            c.normal_at(&Point::new(-1.0, 1.0, 0.0)),
            Vector::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn the_normal_vector_on_a_cylinder_s_end_caps() {
        let tests = vec![
            (Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.5, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0)),
            (Point::new(0.0, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.5, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
            (Point::new(0.0, 2.0, 0.5), Vector::new(0.0, 1.0, 0.0)),
        ];

        let c = Cylinder::new(1.0, 2.0, true);
        for (point, normal) in tests.into_iter() {
            assert_eq!(c.normal_at(&point), normal);
        }
    }

    #[test]
    fn an_unbounded_cylinder_has_a_bounding_box() {
        let c = Object::new_cylinder(f64::NEG_INFINITY, f64::INFINITY, false);
        assert_eq!(c.bounds().min(), Point::new(-1.0, f64::NEG_INFINITY, -1.0));
        assert_eq!(c.bounds().max(), Point::new(1.0, f64::INFINITY, 1.0));
    }

    #[test]
    fn a_bounded_cylinder_has_a_bounding_box() {
        let c = Object::new_cylinder(-5.0, 3.0, false);
        assert_eq!(c.bounds().min(), Point::new(-1.0, -5.0, -1.0));
        assert_eq!(c.bounds().max(), Point::new(1.0, 3.0, 1.0));
    }
}

/* ---------------------------------------------------------------------------------------------- */
