/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Point, Vector},
    rtc::{shapes::Triangle, BoundingBox, Intersection, IntersectionPusher, Ray},
};
use serde::{Deserialize, Serialize};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmoothTriangle {
    triangle: Triangle,
    n1: Vector,
    n2: Vector,
    n3: Vector,
}

/* ---------------------------------------------------------------------------------------------- */

impl SmoothTriangle {
    pub fn new(p1: Point, p2: Point, p3: Point, n1: Vector, n2: Vector, n3: Vector) -> Self {
        Self {
            triangle: Triangle::new(p1, p2, p3),
            n1,
            n2,
            n3,
        }
    }

    #[allow(clippy::manual_range_contains)]
    pub fn intersects<'a>(&self, ray: &Ray, push: &mut impl IntersectionPusher<'a>) {
        self.triangle.intersects(ray, push);
    }

    pub fn normal_at(&self, _object_point: &Point, hit: &Intersection) -> Vector {
        self.n2 * hit.u() + self.n3 * hit.v() + self.n1 * (1.0 - hit.u() - hit.v())
    }

    pub fn bounds(&self) -> BoundingBox {
        self.triangle.bounds()
    }

    pub fn p1(&self) -> Point {
        self.triangle.p1()
    }

    pub fn p2(&self) -> Point {
        self.triangle.p2()
    }

    pub fn p3(&self) -> Point {
        self.triangle.p3()
    }

    pub fn n1(&self) -> Vector {
        self.n1
    }

    pub fn n2(&self) -> Vector {
        self.n2
    }

    pub fn n3(&self) -> Vector {
        self.n3
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        float::ApproxEq,
        primitive::Tuple,
        rtc::{IntersectionState, Intersections, Object},
    };
    use std::f64::INFINITY;

    struct Push {
        pub xs: Vec<(f64, f64, f64)>,
    }

    impl IntersectionPusher<'_> for Push {
        fn t(&mut self, t: f64) {
            self.xs.push((t, INFINITY, INFINITY));
        }
        fn t_u_v(&mut self, t: f64, u: f64, v: f64) {
            self.xs.push((t, u, v));
        }
        fn set_object(&mut self, _object: &'_ Object) {
            panic!();
        }
    }

    fn mk_test_smooth_triangle() -> SmoothTriangle {
        SmoothTriangle::new(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(-1.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
        )
    }

    #[test]
    fn constructing_a_smooth_triangle() {
        let t = mk_test_smooth_triangle();

        assert_eq!(t.p1(), Point::new(0.0, 1.0, 0.0));
        assert_eq!(t.p2(), Point::new(-1.0, 0.0, 0.0));
        assert_eq!(t.p3(), Point::new(1.0, 0.0, 0.0));
        assert_eq!(t.n1(), Vector::new(0.0, 1.0, 0.0));
        assert_eq!(t.n2(), Vector::new(-1.0, 0.0, 0.0));
        assert_eq!(t.n3(), Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn an_intersection_with_a_smooth_triangle_stores_u_v() {
        let t = mk_test_smooth_triangle();
        let ray = Ray {
            origin: Point::new(-0.2, 0.3, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut push = Push { xs: vec![] };

        t.intersects(&ray, &mut push);
        assert_eq!(push.xs.len(), 1);
        assert!(push.xs[0].1.approx_eq(0.45));
        assert!(push.xs[0].2.approx_eq(0.25));
    }

    #[test]
    fn a_smooth_triangle_uses_u_v_to_interpolate_the_normal() {
        let t = Object::new_smooth_triangle(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(-1.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
        );

        let i = Intersection::new(1.0, &t).with_u_and_v(0.45, 0.25);

        assert_eq!(
            t.normal_at(&Point::zero(), &i),
            Vector::new(-0.5547, 0.83205, 0.0)
        );
    }

    #[test]
    fn preparing_the_normal_on_a_smooth_triangle() {
        struct Push<'a> {
            pub is: Vec<Intersection<'a>>,
            pub object: &'a Object,
        }

        impl IntersectionPusher<'_> for Push<'_> {
            fn t(&mut self, _t: f64) {
                panic!();
            }
            fn t_u_v(&mut self, t: f64, u: f64, v: f64) {
                self.is
                    .push(Intersection::new(t, self.object).with_u_and_v(u, v));
            }
            fn set_object(&mut self, _object: &'_ Object) {
                panic!();
            }
        }

        let t = Object::new_smooth_triangle(
            Point::new(0.0, 1.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(-1.0, 0.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
        );

        let ray = Ray {
            origin: Point::new(-0.2, 0.3, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut push = Push {
            is: vec![],
            object: &t,
        };
        t.intersects(&ray, &mut push);

        assert_eq!(push.is.len(), 1);
        assert!(push.is[0].u().approx_eq(0.45));
        assert!(push.is[0].v().approx_eq(0.25));

        let comps = IntersectionState::new(&Intersections::new(push.is), 0, &ray);
        assert_eq!(comps.normal_v(), Vector::new(-0.5547, 0.83205, 0.0));
    }
}

/* ---------------------------------------------------------------------------------------------- */
