// --------------------------------------------------------------------------------------------- //

use crate::{
    primitive::{Point, Tuple, Vector},
    rtc::Ray,
};

// --------------------------------------------------------------------------------------------- //

// We assume a sphere is always at Position{0, 0 , 0}, thus the absence of coordinates.
// Intersection with rays will be computed reversing the sphere's transformation (which
// includes the translation).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {}

// --------------------------------------------------------------------------------------------- //

impl Sphere {
    #[allow(clippy::eq_op)]
    pub fn intersects<F>(ray: &Ray, mut push: F)
    where
        F: FnMut(f64),
    {
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);

        let a = ray.direction ^ ray.direction;
        let b = 2.0 * (ray.direction ^ sphere_to_ray);
        let c = (sphere_to_ray ^ sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - (4.0 * a * c);

        if discriminant >= 0.0 {
            let sqrt_discriminant = f64::sqrt(discriminant);
            let double_a = 2.0 * a;

            let t1 = (-b - sqrt_discriminant) / double_a;
            let t2 = (-b + sqrt_discriminant) / double_a;

            push(t1);
            push(t2);
        }
    }

    pub fn normal_at(object_point: &Point) -> Vector {
        *object_point - Point::zero()
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use super::*;

    use crate::rtc::{
        scaling, Intersection, IntersectionState, Intersections, Material, Object, Transform,
    };

    fn glassy_sphere() -> Object {
        Object::new_sphere().with_material(
            Material::new()
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        )
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut xs = vec![];
        Sphere::intersects(&r, |t| xs.push(t));

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray {
            origin: Point::new(0.0, 1.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut xs = vec![];
        Sphere::intersects(&r, |t| xs.push(t));

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray {
            origin: Point::new(0.0, 2.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut xs = vec![];
        Sphere::intersects(&r, |t| xs.push(t));

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut xs = vec![];
        Sphere::intersects(&r, |t| xs.push(t));

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut xs = vec![];
        Sphere::intersects(&r, |t| xs.push(t));

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Object::new_sphere().with_transformation(scaling(2.0, 2.0, 2.0));

        let mut xs = vec![];
        s.intersects(&r, |t: f64| xs.push(t));

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 3.0);
        assert_eq!(xs[1], 7.0);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_x_axis() {
        assert_eq!(
            Sphere::normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        )
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_y_axis() {
        assert_eq!(
            Sphere::normal_at(&Point::new(0.0, 1.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        )
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_z_axis() {
        assert_eq!(
            Sphere::normal_at(&Point::new(0.0, 0.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        )
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let x = f64::sqrt(3.0) / 3.0;
        assert_eq!(
            Sphere::normal_at(&Point::new(x, x, x)),
            Vector::new(x, x, x)
        )
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let x = f64::sqrt(3.0) / 3.0;
        let n = Sphere::normal_at(&Point::new(x, x, x));
        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let s = Object::new_sphere().translate(0.0, 1.0, 0.0);
        assert_eq!(
            s.normal_at(&Point::new(0.0, 1.70711, -0.70711)),
            Vector::new(0.0, 0.70711, -0.70711)
        );
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let s = Object::new_sphere()
            .scale(1.0, 0.5, 1.0)
            .rotate_z(std::f64::consts::PI / 5.0);

        assert_eq!(
            s.normal_at(&Point::new(
                0.0,
                f64::sqrt(2.0) / 2.0,
                -f64::sqrt(2.0) / 2.0
            )),
            Vector::new(0.0, 0.97014, -0.24254)
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = Arc::new(glassy_sphere().scale(2.0, 2.0, 2.0));
        let b = Arc::new(
            glassy_sphere().translate(0.0, 0.0, -0.25).with_material(
                glassy_sphere()
                    .material()
                    .clone()
                    .with_refractive_index(2.0),
            ),
        );
        let c = Arc::new(
            glassy_sphere().translate(0.0, 0.0, 0.25).with_material(
                glassy_sphere()
                    .material()
                    .clone()
                    .with_refractive_index(2.5),
            ),
        );

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -4.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = Intersections::new(vec![
            Intersection {
                t: 2.0,
                object: a.clone(),
            },
            Intersection {
                t: 2.75,
                object: b.clone(),
            },
            Intersection {
                t: 3.25,
                object: c.clone(),
            },
            Intersection { t: 4.75, object: b },
            Intersection { t: 5.25, object: c },
            Intersection { t: 6.0, object: a },
        ]);

        assert_eq!(IntersectionState::new(&xs, 0, &ray).n(), (1.0, 1.5));
        assert_eq!(IntersectionState::new(&xs, 1, &ray).n(), (1.5, 2.0));
        assert_eq!(IntersectionState::new(&xs, 2, &ray).n(), (2.0, 2.5));
        assert_eq!(IntersectionState::new(&xs, 3, &ray).n(), (2.5, 2.5));
        assert_eq!(IntersectionState::new(&xs, 4, &ray).n(), (2.5, 1.5));
        assert_eq!(IntersectionState::new(&xs, 5, &ray).n(), (1.5, 1.0));
    }
}

// --------------------------------------------------------------------------------------------- //
