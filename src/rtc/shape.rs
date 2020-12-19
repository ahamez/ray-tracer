// --------------------------------------------------------------------------------------------- //

use crate::{
    primitive::{Point, Vector},
    rtc::{
        shapes::{Cube, Cylinder, Plane, Sphere},
        Ray,
    },
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
    Cube(),
    Cylinder(Cylinder),
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
            Shape::Cube() => Cube::intersects(&ray, push),
            Shape::Cylinder(c) => c.intersects(&ray, push),
            Shape::Plane() => Plane::intersects(&ray, push),
            Shape::Sphere() => Sphere::intersects(&ray, push),
        }
    }

    pub fn normal_at(&self, object_point: &Point) -> Vector {
        match self {
            Shape::Cube() => Cube::normal_at(&object_point),
            Shape::Cylinder(c) => c.normal_at(&object_point),
            Shape::Plane() => Plane::normal_at(&object_point),
            Shape::Sphere() => Sphere::normal_at(&object_point),
        }
    }
}

// --------------------------------------------------------------------------------------------- //
