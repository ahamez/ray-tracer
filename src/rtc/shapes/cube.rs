/* ---------------------------------------------------------------------------------------------- */

use crate::{
    float::ApproxEq,
    primitive::{Point, Tuple, Vector},
    rtc::{BoundingBox, IntersectionPusher, Ray},
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {}

/* ---------------------------------------------------------------------------------------------- */

impl Cube {
    pub fn intersects(ray: &Ray, push: &mut impl IntersectionPusher) {
        let (xtmin, xtmax) = Cube::check_axis(ray.origin.x(), ray.direction.x());
        let (ytmin, ytmax) = Cube::check_axis(ray.origin.y(), ray.direction.y());
        let (ztmin, ztmax) = Cube::check_axis(ray.origin.z(), ray.direction.z());

        let tmax = xtmax.min(ytmax.min(ztmax));
        if tmax < 0.0 {
            return;
        }

        let tmin = xtmin.max(ytmin.max(ztmin));

        if tmin <= tmax {
            push.t(tmin);
            push.t(tmax);
        }
    }

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        // The book proposes the following code when the language does not handle
        // division by zero.
        //
        // let (tmin, tmax) = if direction.abs() >= f64::EPSILON {
        //     (tmin_numerator / direction, tmax_numerator / direction)
        // } else {
        //     (tmin_numerator * f64::INFINITY, tmax_numerator * f64::INFINITY)
        // };
        //
        // But, what about the NaN case (which can occur with 0.0/0.0)?
        // The following code gives us confidence that a NaN, should it occur,
        // would be discarded
        //
        // f64::NAN.min(0.0))             == 0.0
        // 0.0f64.min(f64::NAN))          == 0.0
        // 0.0f64.min(f64::INFINITY))     == 0.0
        // f64::INFINITY.min(0.0))        == 0.0
        // 0.0f64.min(f64::NEG_INFINITY)) == -inf
        // f64::NEG_INFINITY.min(0.0))    == -inf
        // f64::NAN.max(0.0))             == 0.0
        // 0.0f64.max(f64::NAN))          == 0.0
        // 0.0f64.max(f64::INFINITY))     == inf
        // f64::INFINITY.max(0.0))        == inf
        // 0.0f64.max(f64::NEG_INFINITY)) == 0.0
        // f64::NEG_INFINITY.max(0.0))    == 0.0

        // So, in the end, we can use the more straighforward following code:
        // (However, we would have a problem if all axis return NaN as Nan.min(NaN) == NaN.NaN
        //  But is it possible?)
        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }

    pub fn normal_at(object_point: &Point) -> Vector {
        let x = object_point.x();
        let y = object_point.y();
        let z = object_point.z();

        let max_c = x.abs().max(y.abs()).max(z.abs());

        if max_c.approx_eq(x.abs()) {
            Vector::new(x, 0.0, 0.0)
        } else if max_c.approx_eq(y.abs()) {
            Vector::new(0.0, y, 0.0)
        } else {
            Vector::new(0.0, 0.0, z)
        }
    }

    pub fn bounds() -> BoundingBox {
        BoundingBox::new()
            .with_min(Point::new(-1.0, -1.0, -1.0))
            .with_max(Point::new(1.0, 1.0, 1.0))
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
    fn a_ray_intersects_a_cube() {
        fn test(origin: Point, direction: Vector, t1: f64, t2: f64) {
            let ray = Ray { origin, direction };
            let mut push = Push { xs: vec![] };
            Cube::intersects(&ray, &mut push);
            assert_eq!(push.xs.len(), 2);
            assert!(push.xs[0].approx_eq(t1));
            assert!(push.xs[1].approx_eq(t2));
        }

        test(
            Point::new(5.0, 0.5, 0.0),
            Vector::new(-1.0, 0.0, 0.0),
            4.0,
            6.0,
        );
        test(
            Point::new(-5.0, 0.5, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            4.0,
            6.0,
        );
        test(
            Point::new(0.5, 5.0, 0.0),
            Vector::new(0.0, -1.0, 0.0),
            4.0,
            6.0,
        );
        test(
            Point::new(0.5, -5.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            4.0,
            6.0,
        );
        test(
            Point::new(0.5, 0.0, 5.0),
            Vector::new(0.0, 0.0, -1.0),
            4.0,
            6.0,
        );
        test(
            Point::new(0.5, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            4.0,
            6.0,
        );
        test(
            Point::new(0.0, 0.5, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            -1.0,
            1.0,
        );
    }

    #[test]
    fn a_ray_misses_a_cube() {
        fn test(origin: Point, direction: Vector) {
            let ray = Ray { origin, direction };
            let mut push = Push { xs: vec![] };
            Cube::intersects(&ray, &mut push);
            assert_eq!(push.xs.len(), 0);
        }

        test(
            Point::new(-2.0, 0.0, 0.0),
            Vector::new(0.2673, 0.5345, 0.8018),
        );
        test(
            Point::new(0.0, -2.0, 0.0),
            Vector::new(0.8018, 0.2673, 0.5345),
        );
        test(
            Point::new(0.0, 0.0, -2.0),
            Vector::new(0.5345, 0.8018, 0.2673),
        );
        test(Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0));
        test(Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0));
        test(Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0));
        test(Point::new(0.0, 0.0, 2.0), Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        assert_eq!(
            Cube::normal_at(&Point::new(1.0, 0.5, -0.8)),
            Vector::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Cube::normal_at(&Point::new(-1.0, -0.2, -0.9)),
            Vector::new(-1.0, 0.0, 0.0)
        );
        assert_eq!(
            Cube::normal_at(&Point::new(-0.4, 1.0, -0.1)),
            Vector::new(0.0, 1.0, 0.0)
        );
        assert_eq!(
            Cube::normal_at(&Point::new(0.3, -1.0, -0.7)),
            Vector::new(0.0, -1.0, 0.0)
        );
        assert_eq!(
            Cube::normal_at(&Point::new(-0.6, 0.3, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );
        assert_eq!(
            Cube::normal_at(&Point::new(0.4, 0.4, -1.0)),
            Vector::new(0.0, 0.0, -1.0)
        );
        assert_eq!(
            Cube::normal_at(&Point::new(1.0, 1.0, 1.0)),
            Vector::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Cube::normal_at(&Point::new(-1.0, -1.0, -1.0)),
            Vector::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn a_cube_has_a_bounding_box() {
        let c = Object::new_cube();
        assert_eq!(c.bounds().min(), Point::new(-1.0, -1.0, -1.0));
        assert_eq!(c.bounds().max(), Point::new(1.0, 1.0, 1.0));
    }
}

/* ---------------------------------------------------------------------------------------------- */
