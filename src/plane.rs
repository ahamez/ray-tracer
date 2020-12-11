// --------------------------------------------------------------------------------------------- //

use crate::{
    epsilon::EPSILON, intersection::Intersection, material::Material, matrix::Matrix,
    object::Object, point::Point, ray::Ray, shape::Shape, transformation::Transform, tuple::Tuple,
    vector::Vector,
};


// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    transformation: Matrix,
    material: Material,
}

// --------------------------------------------------------------------------------------------- //

impl Plane {
    pub fn new() -> Self {
        Plane {
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
}

// --------------------------------------------------------------------------------------------- //

impl Default for Plane {
    fn default() -> Self {
        Plane::new()
    }
}

// --------------------------------------------------------------------------------------------- //

impl Shape for Plane {
    fn intersects(&self, ray: &Ray, is: &mut Vec<Intersection>) {
        if ray.direction.y().abs() < EPSILON {
            return;
        }

        is.push(Intersection {
            t: -ray.origin.y() / ray.direction.y(),
            object: Object::Plane(*self),
        });
    }

    fn normal_at(&self, _object_point: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transformation(&self) -> &Matrix {
        &self.transformation
    }
}

// --------------------------------------------------------------------------------------------- //

impl Transform for Plane {
    fn apply_transformation(&self, transformation: &Matrix) -> Self {
        Plane {
            transformation: self.transformation * *transformation,
            ..*self
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::Vector;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();

        let constant = Vector::new(0.0, 1.0, 0.0);

        assert_eq!(p.normal_at(&Point::new(0.0, 0.0, 0.0)), constant);
    }

    #[test]
    fn intersects_with_a_ray_parallel_to_the_plane() {
        let p = Object::Plane(Plane::new());
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
        let p = Plane::new();
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
        let p = Plane::new();
        let ray = Ray {
            origin: Point::new(0.0, 1.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0),
        };

        let mut is = vec![];

        p.intersects(&ray, &mut is);
        assert!(is.len() == 1);
        assert_eq!(is[0].t, 1.0);
        assert_eq!(is[0].object, Object::Plane(p));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let ray = Ray {
            origin: Point::new(0.0, -1.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let mut is = vec![];

        p.intersects(&ray, &mut is);
        assert!(is.len() == 1);
        assert_eq!(is[0].t, 1.0);
        assert_eq!(is[0].object, Object::Plane(p));
    }
}

// --------------------------------------------------------------------------------------------- //
