// --------------------------------------------------------------------------------------------- //

use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::object::Object;
use crate::point::Point;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::transformation::Transform;
use crate::tuple::Tuple;
use crate::vector::Vector;

// --------------------------------------------------------------------------------------------- //

// We assume a sphere is always at Position{0, 0 , 0}, thus the absence of coordinates.
// Intersection with rays will be computed reversing the sphere's transformation (which
// includes the translation).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    transformation: Matrix,
    material: Material,
}

// --------------------------------------------------------------------------------------------- //

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transformation: Matrix::id(),
            material: Material::new(),
        }
    }

    pub fn with_transformation(mut self, transformation: Matrix) -> Self {
        self.transformation = transformation;
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn transformation(&self) -> &Matrix {
        &self.transformation
    }

    pub fn set_transformation(&mut self, transformation: Matrix) -> &mut Self {
        self.transformation = transformation;
        self
    }

    pub fn set_material(&mut self, material: Material) -> &mut Self {
        self.material = material;
        self
    }
}

// --------------------------------------------------------------------------------------------- //

impl Shape for Sphere {
    fn intersects(&self, ray: &Ray, is: &mut Vec<Intersection>) {
        let ray = ray.apply_transformation(&self.transformation.invert().unwrap());

        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);

        let a = ray.direction ^ ray.direction;
        let b = 2.0 * ray.direction ^ sphere_to_ray;
        let c = (sphere_to_ray ^ sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - (4.0 * a * c);

        if discriminant >= 0.0 {
            let sqrt_discriminant = f64::sqrt(discriminant);
            let double_a = 2.0 * a;

            let t1 = (-b - sqrt_discriminant) / double_a;
            let t2 = (-b + sqrt_discriminant) / double_a;

            is.push(Intersection {
                t: t1,
                object: Object::Sphere(*self),
            });
            is.push(Intersection {
                t: t2,
                object: Object::Sphere(*self),
            });
        }
    }

    fn normal_at(&self, world_point: &Point) -> Vector {
        let object_point = self.transformation.invert().unwrap() * *world_point;
        let object_normal = object_point - Point::zero();
        let world_normal = self.transformation.invert().unwrap().transpose() * object_normal;

        world_normal.normalize()
    }

    fn material(&self) -> &Material {
        &self.material
    }
}

// --------------------------------------------------------------------------------------------- //

impl Transform for Sphere {
    fn apply_transformation(&self, transformation: &Matrix) -> Self {
        Sphere {
            transformation: self.transformation * *transformation,
            ..*self
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformation::scaling;
    use crate::vector::Vector;

    #[test]
    fn a_sphere_default_transformation_is_id() {
        let s = Sphere::new();
        assert_eq!(s.transformation(), &Matrix::id());
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let mut s = Sphere::new();
        let transformation = Matrix::new(4);
        s.set_transformation(transformation);
        assert_eq!(s.transformation(), &transformation);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::new();

        let mut xs = vec![];
        s.intersects(&r, &mut xs);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray {
            origin: Point::new(0.0, 1.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::new();

        let mut xs = vec![];
        s.intersects(&r, &mut xs);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray {
            origin: Point::new(0.0, 2.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::new();

        let mut xs = vec![];
        s.intersects(&r, &mut xs);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::new();

        let mut xs = vec![];
        s.intersects(&r, &mut xs);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::new();

        let mut xs = vec![];
        s.intersects(&r, &mut xs);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Sphere::new().with_transformation(scaling(2.0, 2.0, 2.0));

        let mut xs = vec![];
        s.intersects(&r, &mut xs);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_x_axis() {
        let s = Sphere::new();
        assert_eq!(
            s.normal_at(&Point::new(1.0, 0.0, 0.0)),
            Vector::new(1.0, 0.0, 0.0)
        )
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_y_axis() {
        let s = Sphere::new();
        assert_eq!(
            s.normal_at(&Point::new(0.0, 1.0, 0.0)),
            Vector::new(0.0, 1.0, 0.0)
        )
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_z_axis() {
        let s = Sphere::new();
        assert_eq!(
            s.normal_at(&Point::new(0.0, 0.0, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        )
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let x = f64::sqrt(3.0) / 3.0;
        assert_eq!(s.normal_at(&Point::new(x, x, x)), Vector::new(x, x, x))
    }

    #[test]
    fn normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let x = f64::sqrt(3.0) / 3.0;
        let n = s.normal_at(&Point::new(x, x, x));
        assert_eq!(n.normalize(), n);
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let s = Sphere::new().translate(0.0, 1.0, 0.0);
        assert_eq!(
            s.normal_at(&Point::new(0.0, 1.70711, -0.70711)),
            Vector::new(0.0, 0.70711, -0.70711)
        );
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let s = Sphere::new()
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
}

// --------------------------------------------------------------------------------------------- //
