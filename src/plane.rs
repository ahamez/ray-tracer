// --------------------------------------------------------------------------------------------- //

use crate::{epsilon::EPSILON, point::Point, ray::Ray, tuple::Tuple, vector::Vector};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {}

// --------------------------------------------------------------------------------------------- //

impl Plane {
    pub fn intersects(ray: &Ray) -> Vec<f64> {
        if ray.direction.y().abs() < EPSILON {
            vec![]
        } else {
            vec![-ray.origin.y() / ray.direction.y()]
        }
    }

    pub fn normal_at(_object_point: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
        use super::*;
        use crate::{
            shape::Shape,
            vector::Vector,
        };

        #[test]
        fn the_normal_of_a_plane_is_constant_everywhere() {
            let constant = Vector::new(0.0, 1.0, 0.0);

            assert_eq!(Plane::normal_at(&Point::new(0.0, 0.0, 0.0)), constant);
        }

        #[test]
        fn intersects_with_a_ray_parallel_to_the_plane() {
            let p = Shape::new_plane();
            let ray = Ray {
                origin: Point::new(0.0, 10.0, 0.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            };

            let mut is = vec![];

            p.intersects(&ray, &mut is);
            assert!(is.len() == 0);
        }

        #[test]
        fn intersects_with_a_coplanar_ray() {
            let p = Shape::new_plane();
            let ray = Ray {
                origin: Point::new(0.0, 0.0, 0.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            };

            let mut is = vec![];

            p.intersects(&ray, &mut is);
            assert!(is.len() == 0);
        }

        #[test]
        fn a_ray_intersecting_a_plane_from_above() {
            let p = Shape::new_plane();
            let ray = Ray {
                origin: Point::new(0.0, 1.0, 0.0),
                direction: Vector::new(0.0, -1.0, 0.0),
            };

            let mut is = vec![];

            p.intersects(&ray, &mut is);
            assert!(is.len() == 1);
            assert_eq!(is[0].t, 1.0);
            assert_eq!(is[0].shape, p);
        }

        #[test]
        fn a_ray_intersecting_a_plane_from_below() {
            let p = Shape::new_plane();
            let ray = Ray {
                origin: Point::new(0.0, -1.0, 0.0),
                direction: Vector::new(0.0, 1.0, 0.0),
            };

            let mut is = vec![];

            p.intersects(&ray, &mut is);
            assert!(is.len() == 1);
            assert_eq!(is[0].t, 1.0);
            assert_eq!(is[0].shape, p);
        }
}

// --------------------------------------------------------------------------------------------- //
