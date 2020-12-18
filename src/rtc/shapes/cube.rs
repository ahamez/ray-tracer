// --------------------------------------------------------------------------------------------- //

use crate::{
    float::ApproxEq,
    primitive::{Point, Tuple, Vector},
    rtc::Ray,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cube {}

// --------------------------------------------------------------------------------------------- //

impl Cube {
    pub fn intersects<F>(ray: &Ray, mut push: F)
    where
        F: FnMut(f64),
    {
        let (xtmin, xtmax) = Cube::check_axis(ray.origin.x(), ray.direction.x());
        let (ytmin, ytmax) = Cube::check_axis(ray.origin.y(), ray.direction.y());
        let (ztmin, ztmax) = Cube::check_axis(ray.origin.z(), ray.direction.z());

        let tmax = xtmax.min(ytmax.min(ztmax));
        if tmax < 0.0 {
            return;
        }

        let tmin = xtmin.max(ytmin.max(ztmin));

        if tmin <= tmax {
            push(tmin);
            push(tmax);
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
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        fn test(origin: Point, direction: Vector, t1: f64, t2: f64) {
            let ray = Ray { origin, direction };
            let mut xs = vec![];
            Cube::intersects(&ray, |t| xs.push(t));
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], t1);
            assert_eq!(xs[1], t2);
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
            let mut xs = vec![];
            Cube::intersects(&ray, |t| xs.push(t));
            assert_eq!(xs.len(), 0);
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
}

// --------------------------------------------------------------------------------------------- //
