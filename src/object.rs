// --------------------------------------------------------------------------------------------- //

use crate::intersection::Intersection;
use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::sphere;
use crate::vector::Vector;

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Object {
    Sphere(sphere::Sphere),
}

// --------------------------------------------------------------------------------------------- //

impl Object {
    pub fn intersects(&self, ray: &Ray, is: &mut Vec<Intersection>) {
        match self {
            Object::Sphere(s) => s.intersects(ray, is),
        }
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        match self {
            Object::Sphere(s) => s.normal_at(world_point),
        }
    }

    pub fn material(&self) -> &Material {
        match self {
            Object::Sphere(s) => s.material(),
        }
    }
}

// --------------------------------------------------------------------------------------------- //
