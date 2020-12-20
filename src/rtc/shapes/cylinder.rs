// --------------------------------------------------------------------------------------------- //

use crate::{
    float::{ApproxEq, EPSILON},
    primitive::{Point, Tuple, Vector},
    rtc::Ray,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cylinder {
    min: f64,
    max: f64,
    closed: bool,
}

// --------------------------------------------------------------------------------------------- //

impl Cylinder {
    pub fn new() -> Self {
        Cylinder {
            ..Default::default()
        }
    }

    pub fn new_truncated(min: f64, max: f64, closed: bool) -> Self {
        let (min, max) = if min < max { (min, max) } else { (max, min) };

        Cylinder { min, max, closed }
    }

    pub fn intersects<F>(&self, ray: &Ray, mut push: F)
    where
        F: FnMut(f64),
    {
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

            let (t0, t1) = if t0 < t1 { (t0, t1) } else { (t1, t0) };

            let y0 = ray.origin.y() + t0 * ray.direction.y();
            if self.min < y0 && y0 < self.max {
                push(t0);
            }

            let y1 = ray.origin.y() + t1 * ray.direction.y();
            if self.min < y1 && y1 < self.max {
                push(t1);
            }

            self.intersects_caps(&ray, push);
        }
    }

    fn check_cap(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        (x.powi(2) + z.powi(2)) <= 1.0
    }

    fn intersects_caps<F>(&self, ray: &Ray, mut push: F)
    where
        F: FnMut(f64),
    {
        if !self.closed || ray.direction.y().approx_eq(0.0) {
            return;
        }

        let t = (self.min - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(&ray, t) {
            push(t);
        }

        let t = (self.max - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(&ray, t) {
            push(t);
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
}

// --------------------------------------------------------------------------------------------- //

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            closed: false,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn the_default_values_for_a_cylinder() {
        let c = Cylinder::new();
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
            let mut xs = vec![];

            Cylinder::new().intersects(&ray, |t| xs.push(t));

            assert_eq!(xs.len(), 2);
            assert!(xs[0].approx_eq_low_precision(t0));
            assert!(xs[1].approx_eq_low_precision(t1));
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

        let c = Cylinder::new_truncated(1.0, 2.0, false);
        for (origin, direction, count) in tests.into_iter() {
            let mut xs = vec![];
            c.intersects(
                &Ray {
                    origin,
                    direction: direction.normalize(),
                },
                |t| xs.push(t),
            );
            assert_eq!(xs.len(), count as usize);
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

        let c = Cylinder::new_truncated(1.0, 2.0, true);
        for (origin, direction, count) in tests.into_iter() {
            let mut xs = vec![];
            c.intersects(
                &Ray {
                    origin,
                    direction: direction.normalize(),
                },
                |t| xs.push(t),
            );
            assert_eq!(xs.len(), count as usize);
        }
    }

    #[test]
    fn a_ray_misses_a_cylinder() {
        fn test(origin: Point, direction: Vector) {
            let ray = Ray {
                origin,
                direction: direction.normalize(),
            };
            let mut xs = vec![];
            Cylinder::new().intersects(&ray, |t| xs.push(t));
            assert_eq!(xs.len(), 0);
        }

        test(Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        test(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        test(Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let c = Cylinder::new();
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

        let c = Cylinder::new_truncated(1.0, 2.0, true);
        for (point, normal) in tests.into_iter() {
            assert_eq!(c.normal_at(&point), normal);
        }
    }
}

// --------------------------------------------------------------------------------------------- //
