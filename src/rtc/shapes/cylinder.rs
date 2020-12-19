// --------------------------------------------------------------------------------------------- //

use crate::{
    float::ApproxEq,
    primitive::{Point, Tuple, Vector},
    rtc::Ray,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cylinder {
    min: f64,
    max: f64,
}

// --------------------------------------------------------------------------------------------- //

impl Cylinder {
    pub fn new() -> Self {
        Cylinder {
            ..Default::default()
        }
    }

    pub fn new_truncated(min: f64, max: f64) -> Self {
        let (min, max) = if min < max { (min, max) } else { (max, min) };

        Cylinder { min, max }
    }

    pub fn intersects<F>(&self, ray: &Ray, mut push: F)
    where
        F: FnMut(f64),
    {
        let a = ray.direction.x().powi(2) + ray.direction.z().powi(2);

        if a.approx_eq(0.0) {
            return;
        }

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
    }

    pub fn normal_at(&self, object_point: &Point) -> Vector {
        Vector::new(object_point.x(), 0.0, object_point.z())
    }
}

// --------------------------------------------------------------------------------------------- //

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn the_default_min_and_max_for_a_cylinder() {
        let c = Cylinder::new();
        assert_eq!(c.min, f64::NEG_INFINITY);
        assert_eq!(c.max, f64::INFINITY);
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
            assert!(xs[0].approx_eq(t0));
            assert!(xs[1].approx_eq(t1));
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

        let c = Cylinder::new_truncated(1.0, 2.0);
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
}

// --------------------------------------------------------------------------------------------- //
