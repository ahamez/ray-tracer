// --------------------------------------------------------------------------------------------- //

use crate::{
    intersection::Intersection, material::Material, matrix::Matrix, plane, point::Point, ray::Ray,
    sphere, transformation::Transform, vector::Vector,
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

    pub fn intersects(&self, ray: &Ray, is: &mut Vec<Intersection>) {
        let ray = ray.apply_transformation(&self.transformation_inverse);

        self.shape.intersects(&ray).iter().for_each(|t| {
            is.push(Intersection {
                t: *t,
                object: self.clone(),
            })
        });
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
    pub fn intersects(&self, ray: &Ray) -> Vec<f64> {
        match self {
            Shape::Plane() => plane::Plane::intersects(&ray),
            Shape::Sphere() => sphere::Sphere::intersects(&ray),
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
