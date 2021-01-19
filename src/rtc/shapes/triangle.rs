/* ---------------------------------------------------------------------------------------------- */

use crate::{
    float::EPSILON,
    primitive::{Point, Vector},
    rtc::{BoundingBox, IntersectionPusher, Ray},
};
use serde::{Deserialize, Serialize};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    e1: Vector,
    e2: Vector,
    normal: Vector,
}

/* ---------------------------------------------------------------------------------------------- */

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = (e2 * e1).normalize();

        Self {
            p1,
            p2,
            p3,
            e1,
            e2,
            normal,
        }
    }

    #[allow(clippy::manual_range_contains)]
    pub fn intersects<'a>(&self, ray: &Ray, push: &mut impl IntersectionPusher<'a>) {
        let dir_cross_e2 = ray.direction * self.e2;
        let det = self.e1 ^ dir_cross_e2;

        if det.abs() < EPSILON {
            return;
        }

        let f = 1.0 / det;
        let p1_to_origin = ray.origin - self.p1;
        let u = f * (p1_to_origin ^ dir_cross_e2);

        if u < 0.0 || u > 1.0 {
            return;
        }

        let origin_cross_e1 = p1_to_origin * self.e1;
        let v = f * (ray.direction ^ origin_cross_e1);

        if v < 0.0 || (u + v) > 1.0 {
            return;
        }

        let t = f * (self.e2 ^ origin_cross_e1);

        // As SmoothTriangle delegates its intersection to Triangle, we need to take care
        // of pushing u and v as well here.
        // It's not very satisfying in regard to the code architecture, but it avoids code
        // duplication or a lot of boiler plate (like returning Option<f64, f64, f64> that would be
        // used differently by SmoothTriangle::intersects and Triangle::intersects)
        push.t_u_v(t, u, v);
    }

    pub fn normal_at(&self, _object_point: &Point) -> Vector {
        self.normal
    }

    pub fn bounds(&self) -> BoundingBox {
        BoundingBox::new()
            .add_point(self.p1)
            .add_point(self.p2)
            .add_point(self.p3)
    }

    pub fn p1(&self) -> Point {
        self.p1
    }

    pub fn p2(&self) -> Point {
        self.p2
    }

    pub fn p3(&self) -> Point {
        self.p3
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{primitive::Tuple, rtc::Object};

    struct Push {
        pub xs: Vec<f64>,
    }

    impl IntersectionPusher<'_> for Push {
        fn t(&mut self, t: f64) {
            self.xs.push(t);
        }
        fn t_u_v(&mut self, t: f64, _u: f64, _v: f64) {
            self.xs.push(t);
        }
        fn set_object(&mut self, _object: &'_ Object) {
            panic!();
        }
    }

    #[test]
    fn constructing_a_triangle() {
        let p1 = Point::new(0.0, 1.0, 0.0);
        let p2 = Point::new(-1.0, 0.0, 0.0);
        let p3 = Point::new(1.0, 0.0, 0.0);
        let t = Triangle::new(p1, p2, p3);

        assert_eq!(t.p1, p1);
        assert_eq!(t.p2, p2);
        assert_eq!(t.p3, p3);
        assert_eq!(t.e1, Vector::new(-1., -1., 0.));
        assert_eq!(t.e2, Vector::new(1., -1., 0.));
        assert_eq!(t.normal, Vector::new(0., 0., -1.));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        assert_eq!(t.normal_at(&Point::new(0.0, 0.5, 0.0)), t.normal);
        assert_eq!(t.normal_at(&Point::new(-0.5, 0.75, 0.0)), t.normal);
        assert_eq!(t.normal_at(&Point::new(0.5, 0.25, 0.0)), t.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        let ray = Ray {
            origin: Point::new(0.0, -1.0, -2.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let mut push = Push { xs: vec![] };

        t.intersects(&ray, &mut push);

        assert_eq!(push.xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p1_p3_edge() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        let ray = Ray {
            origin: Point::new(1.0, 1.0, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut push = Push { xs: vec![] };

        t.intersects(&ray, &mut push);

        assert_eq!(push.xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        let ray = Ray {
            origin: Point::new(-1.0, 1.0, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut push = Push { xs: vec![] };

        t.intersects(&ray, &mut push);

        assert_eq!(push.xs.len(), 0);
    }

    #[test]
    fn a_ray_misses_the_p2_p3_edge() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        let ray = Ray {
            origin: Point::new(0.0, -1.0, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut push = Push { xs: vec![] };

        t.intersects(&ray, &mut push);

        assert_eq!(push.xs.len(), 0);
    }

    #[test]
    fn a_strikes_a_triangle() {
        let t = Triangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
        );

        let ray = Ray {
            origin: Point::new(0.0, 0.5, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut push = Push { xs: vec![] };

        t.intersects(&ray, &mut push);

        assert_eq!(push.xs.len(), 1);
        assert_eq!(push.xs[0], 2.0);
    }

    #[test]
    fn a_triangle_has_a_bounding_box() {
        let t = Triangle::new(
            Point::new(-3.0, 7.0, 2.0),
            Point::new(6.0, 2.0, -4.0),
            Point::new(2.0, -1.0, -1.0),
        );

        assert_eq!(t.bounds().min(), Point::new(-3.0, -1.0, -4.0));
        assert_eq!(t.bounds().max(), Point::new(6.0, 7.0, 2.0));
    }
}

/* ---------------------------------------------------------------------------------------------- */
