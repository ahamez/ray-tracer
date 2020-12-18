// --------------------------------------------------------------------------------------------- //

use crate::{
    float::ApproxEq,
    primitive::{Point, Tuple, Vector},
    rtc::Ray,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cylinder {}

// --------------------------------------------------------------------------------------------- //

impl Cylinder {
    pub fn intersects<F>(ray: &Ray, mut push: F)
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

        push(t0);
        push(t1);
    }

    pub fn normal_at(object_point: &Point) -> Vector {
        Vector::new(object_point.x(), 0.0, object_point.z())
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn a_ray_strikes_a_cylinder() {
        fn test(origin: Point, direction: Vector, t0: f64, t1: f64) {
            let ray = Ray {
                origin,
                direction: direction.normalize(),
            };
            let mut xs = vec![];

            Cylinder::intersects(&ray, |t| xs.push(t));

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
    fn a_ray_misses_a_cylinder() {
        fn test(origin: Point, direction: Vector) {
            let ray = Ray {
                origin,
                direction: direction.normalize(),
            };
            let mut xs = vec![];
            Cylinder::intersects(&ray, |t| xs.push(t));
            assert_eq!(xs.len(), 0);
        }

        test(Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        test(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        test(Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        assert_eq!(
            Cylinder::normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Cylinder::normal_at(&Point::new(0.0, 5.0, -1.0)),
            Vector::new(0.0, 0.0, -1.0)
        );
        assert_eq!(
            Cylinder::normal_at(&Point::new(0.0, -2.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );
        assert_eq!(
            Cylinder::normal_at(&Point::new(-1.0, 1.0, 0.0)),
            Vector::new(-1.0, 0.0, 0.0)
        );
    }
}

// --------------------------------------------------------------------------------------------- //
