// --------------------------------------------------------------------------------------------- //

use crate::{
    primitive::{Matrix, Point, Vector},
    rtc::{
        shapes::{Cone, Cylinder},
        Material, Ray, Shape, Transform,
    },
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    has_shadow: bool,
    material: Material,
    shape: Shape,
    transformation: Matrix,
    transformation_inverse: Matrix,
    transformation_inverse_transpose: Matrix,
}

// --------------------------------------------------------------------------------------------- //

impl Object {
    pub fn new_cone() -> Self {
        Object {
            shape: Shape::Cone(Cone::new()),
            ..Default::default()
        }
    }

    pub fn new_cone_truncated(min: f64, max: f64, closed: bool) -> Self {
        Object {
            shape: Shape::Cone(Cone::new_truncated(min, max, closed)),
            ..Default::default()
        }
    }

    pub fn new_cube() -> Self {
        Object {
            shape: Shape::Cube(),
            ..Default::default()
        }
    }

    pub fn new_cylinder() -> Self {
        Object {
            shape: Shape::Cylinder(Cylinder::new()),
            ..Default::default()
        }
    }

    pub fn new_cylinder_truncated(min: f64, max: f64, closed: bool) -> Self {
        Object {
            shape: Shape::Cylinder(Cylinder::new_truncated(min, max, closed)),
            ..Default::default()
        }
    }

    pub fn new_plane() -> Self {
        Object {
            shape: Shape::Plane(),
            ..Default::default()
        }
    }

    pub fn new_sphere() -> Self {
        Object {
            shape: Shape::Sphere(),
            ..Default::default()
        }
    }

    pub fn with_shadow(mut self, has_shadow: bool) -> Self {
        self.has_shadow = has_shadow;
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_transformation(mut self, transformation: Matrix) -> Self {
        self.transformation = transformation;
        self.transformation_inverse = transformation.invert().unwrap();
        self
    }

    pub fn intersects<F>(&self, ray: &Ray, push: F)
    where
        F: FnMut(f64),
    {
        let transformed_ray = ray.apply_transformation(&self.transformation_inverse);

        self.shape.intersects(&transformed_ray, push);
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let object_point = self.transformation_inverse * *world_point;
        let object_normal = self.shape.normal_at(&object_point);
        let world_normal = self.transformation_inverse_transpose * object_normal;

        world_normal.normalize()
    }

    pub fn has_shadow(&self) -> bool {
        self.has_shadow
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn transformation_inverse(&self) -> &Matrix {
        &self.transformation_inverse
    }
}

// --------------------------------------------------------------------------------------------- //

impl Default for Object {
    fn default() -> Self {
        Object {
            has_shadow: true,
            material: Material::new(),
            shape: Shape::Sphere(),
            transformation: Matrix::id(),
            transformation_inverse: Matrix::id(),
            transformation_inverse_transpose: Matrix::id(),
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl Transform for Object {
    fn apply_transformation(&self, transformation: &Matrix) -> Self {
        let transformation = *transformation * self.transformation;
        let transformation_inverse = transformation.invert().unwrap();
        let transformation_inverse_transpose = transformation_inverse.transpose();

        Object {
            transformation,
            transformation_inverse,
            transformation_inverse_transpose,
            ..self.clone()
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_object_default_transformation_is_id() {
        let s = Object::new_sphere();
        assert_eq!(s.transformation, Matrix::id());
    }
}

// --------------------------------------------------------------------------------------------- //
