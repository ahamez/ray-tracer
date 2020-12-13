// --------------------------------------------------------------------------------------------- //

use crate::{
    material::Material, matrix::Matrix, plane, point::Point, ray::Ray, sphere,
    transformation::Transform, vector::Vector,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    shape: Shape,
    transformation: Matrix,
    transformation_inverse: Matrix,
    material: Material,
}

// --------------------------------------------------------------------------------------------- //

impl Object {
    pub fn new_sphere() -> Self {
        Object {
            shape: Shape::Sphere(),
            ..Default::default()
        }
    }

    pub fn new_plane() -> Self {
        Object {
            shape: Shape::Plane(),
            ..Default::default()
        }
    }

    pub fn with_transformation(mut self, transformation: Matrix) -> Self {
        self.transformation = transformation;
        self.transformation_inverse = transformation.invert().unwrap();
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
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
        let world_normal = self.transformation_inverse.transpose() * object_normal;

        world_normal.normalize()
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn transformation(&self) -> &Matrix {
        &self.transformation
    }

    pub fn transformation_inverse(&self) -> &Matrix {
        &self.transformation_inverse
    }
}

// --------------------------------------------------------------------------------------------- //

impl Default for Object {
    fn default() -> Self {
        Object {
            shape: Shape::Sphere(),
            transformation: Matrix::id(),
            transformation_inverse: Matrix::id(),
            material: Material::new(),
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl Transform for Object {
    fn apply_transformation(&self, transformation: &Matrix) -> Self {
        let new_transformation = self.transformation * *transformation;
        Object {
            transformation: new_transformation,
            transformation_inverse: new_transformation.invert().unwrap(),
            ..self.clone()
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
enum Shape {
    Plane(),
    Sphere(),
}

// --------------------------------------------------------------------------------------------- //

impl Shape {
    pub fn intersects<F>(&self, ray: &Ray, push: F)
    where
        F: FnMut(f64),
    {
        match self {
            Shape::Plane() => plane::Plane::intersects(&ray, push),
            Shape::Sphere() => sphere::Sphere::intersects(&ray, push),
        }
    }

    pub fn normal_at(&self, object_point: &Point) -> Vector {
        match self {
            Shape::Plane() => plane::Plane::normal_at(&object_point),
            Shape::Sphere() => sphere::Sphere::normal_at(&object_point),
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
