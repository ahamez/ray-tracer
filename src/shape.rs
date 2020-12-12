// --------------------------------------------------------------------------------------------- //

use crate::{
    intersection::Intersection, material::Material, matrix::Matrix, plane, point::Point, ray::Ray,
    sphere, transformation::Transform, vector::Vector,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    object: Object,
    transformation: Matrix,
    material: Material,
}

// --------------------------------------------------------------------------------------------- //

impl Shape {
    pub fn new_sphere() -> Self {
        Shape {
            object: Object::Sphere(),
            ..Default::default()
        }
    }

    pub fn new_plane() -> Self {
        Shape {
            object: Object::Plane(),
            ..Default::default()
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

    pub fn intersects(&self, ray: &Ray, is: &mut Vec<Intersection>) {
        let ray = ray.apply_transformation(&self.transformation.invert().unwrap());

        self.object.intersects(&ray).iter().for_each(|t| {
            is.push(Intersection {
                t: *t,
                shape: self.clone(),
            })
        });
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let transformation_inv = self.transformation.invert().unwrap();
        let object_point = transformation_inv * *world_point;
        let object_normal = self.object.normal_at(&object_point);
        let world_normal = transformation_inv.transpose() * object_normal;

        world_normal.normalize()
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn transformation(&self) -> &Matrix {
        &self.transformation
    }
}

// --------------------------------------------------------------------------------------------- //

impl Default for Shape {
    fn default() -> Self {
        Shape {
            object: Object::Sphere(),
            transformation: Matrix::id(),
            material: Material::new(),
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl Transform for Shape {
    fn apply_transformation(&self, transformation: &Matrix) -> Self {
        Shape {
            transformation: self.transformation * *transformation,
            ..self.clone()
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
enum Object {
    Plane(),
    Sphere(),
}

// --------------------------------------------------------------------------------------------- //

impl Object {
    pub fn intersects(&self, ray: &Ray) -> Vec<f64> {
        match self {
            Object::Plane() => plane::Plane::intersects(&ray),
            Object::Sphere() => sphere::Sphere::intersects(&ray),
        }
    }

    pub fn normal_at(&self, object_point: &Point) -> Vector {
        match self {
            Object::Plane() => plane::Plane::normal_at(&object_point),
            Object::Sphere() => sphere::Sphere::normal_at(&object_point),
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shape_default_transformation_is_id() {
        let s = Shape::new_sphere();
        assert_eq!(s.transformation, Matrix::id());
    }
}

// --------------------------------------------------------------------------------------------- //
