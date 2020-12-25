/* ---------------------------------------------------------------------------------------------- */

use crate::{
    float::EPSILON,
    primitive::{Point, Tuple, Vector},
    rtc::Ray,
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {}

/* ---------------------------------------------------------------------------------------------- */

impl Plane {
    pub fn intersects<F>(ray: &Ray, push: F)
    where
        F: FnOnce(f64),
    {
        if ray.direction.y().abs() >= EPSILON {
            push(-ray.origin.y() / ray.direction.y())
        }
    }

    pub fn normal_at(_object_point: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{primitive::Vector, rtc::Object};

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

        let mut is = vec![];

        p.intersects(&ray, |t: f64| is.push(t));
        assert!(is.len() == 0);
    }

    #[test]
    fn intersects_with_a_coplanar_ray() {
        let p = Object::new_plane();
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut is = vec![];

        p.intersects(&ray, |t: f64| is.push(t));
        assert!(is.len() == 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Object::new_plane();
        let ray = Ray {
            origin: Point::new(0.0, 1.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0),
        };

        let mut is = vec![];

        p.intersects(&ray, |t: f64| is.push(t));
        assert!(is.len() == 1);
        assert_eq!(is[0], 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Object::new_plane();
        let ray = Ray {
            origin: Point::new(0.0, -1.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let mut is = vec![];

        p.intersects(&ray, |t: f64| is.push(t));
        assert!(is.len() == 1);
        assert_eq!(is[0], 1.0);
    }
}

/* ---------------------------------------------------------------------------------------------- */
