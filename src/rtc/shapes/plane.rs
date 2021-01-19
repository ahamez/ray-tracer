/* ---------------------------------------------------------------------------------------------- */

use crate::{
    float::EPSILON,
    primitive::{Point, Tuple, Vector},
    rtc::{BoundingBox, IntersectionPusher, Ray},
};
use serde::{Deserialize, Serialize};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plane {}

/* ---------------------------------------------------------------------------------------------- */

impl Plane {
    pub fn intersects<'a>(ray: &Ray, push: &mut impl IntersectionPusher<'a>) {
        if ray.direction.y().abs() >= EPSILON {
            push.t(-ray.origin.y() / ray.direction.y())
        }
    }

    pub fn normal_at(_object_point: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    pub fn bounds() -> BoundingBox {
        BoundingBox::new()
            .with_min(Point::new(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY))
            .with_max(Point::new(f64::INFINITY, 0.0, f64::INFINITY))
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        primitive::Vector,
        rtc::{IntersectionPusher, Object},
    };

    struct Push {
        pub xs: Vec<f64>,
    }

    impl IntersectionPusher<'_> for Push {
        fn t(&mut self, t: f64) {
            self.xs.push(t);
        }
        fn t_u_v(&mut self, _t: f64, _u: f64, _v: f64) {
            panic!();
        }
        fn set_object(&mut self, _object: &'_ Object) {
            panic!();
        }
    }

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let constant = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(Plane::normal_at(&Point::new(0.0, 0.0, 0.0)), constant);
    }

    #[test]
    fn intersects_with_a_ray_parallel_to_the_plane() {
        let p = Object::new_plane();
        let ray = Ray {
            origin: Point::new(0.0, 10.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut push = Push { xs: vec![] };

        p.intersects(&ray, &mut push);
        assert!(push.xs.len() == 0);
    }

    #[test]
    fn intersects_with_a_coplanar_ray() {
        let p = Object::new_plane();
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut push = Push { xs: vec![] };

        p.intersects(&ray, &mut push);
        assert!(push.xs.len() == 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Object::new_plane();
        let ray = Ray {
            origin: Point::new(0.0, 1.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0),
        };

        let mut push = Push { xs: vec![] };

        p.intersects(&ray, &mut push);
        assert!(push.xs.len() == 1);
        assert_eq!(push.xs[0], 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Object::new_plane();
        let ray = Ray {
            origin: Point::new(0.0, -1.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let mut push = Push { xs: vec![] };

        p.intersects(&ray, &mut push);
        assert!(push.xs.len() == 1);
        assert_eq!(push.xs[0], 1.0);
    }

    #[test]
    fn a_plane_has_a_bounding_box() {
        let p = Object::new_plane();
        assert_eq!(
            p.shape_bounds().min(),
            Point::new(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY)
        );
        assert_eq!(
            p.shape_bounds().max(),
            Point::new(f64::INFINITY, 0.0, f64::INFINITY)
        );
    }
}

/* ---------------------------------------------------------------------------------------------- */
