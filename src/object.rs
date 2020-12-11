// --------------------------------------------------------------------------------------------- //

use crate::{
    intersection::Intersection, material::Material, matrix::Matrix, plane, point::Point, ray::Ray,
    shape::Shape, sphere, transformation::Transform, vector::Vector,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Object {
    Plane(plane::Plane),
    Sphere(sphere::Sphere),
}

// --------------------------------------------------------------------------------------------- //

impl Object {
    pub fn intersects(&self, ray: &Ray, is: &mut Vec<Intersection>) {
        let transformation = self.transformation();
        let ray = ray.apply_transformation(&transformation.invert().unwrap());

        match self {
            Object::Plane(p) => p.intersects(&ray, is),
            Object::Sphere(s) => s.intersects(&ray, is),
        }
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let transformation = self.transformation();
        let transformation_inv = transformation.invert().unwrap();

        let object_point = transformation_inv * *world_point;

        let object_normal = match self {
            Object::Plane(p) => p.normal_at(&object_point),
            Object::Sphere(s) => s.normal_at(&object_point),
        };

        let world_normal = transformation_inv.transpose() * object_normal;

        world_normal.normalize()
    }

    pub fn material(&self) -> &Material {
        match self {
            Object::Plane(p) => p.material(),
            Object::Sphere(s) => s.material(),
        }
    }

    pub fn transformation(&self) -> &Matrix {
        match self {
            Object::Plane(p) => p.transformation(),
            Object::Sphere(s) => s.transformation(),
        }
    }
}

// --------------------------------------------------------------------------------------------- //
