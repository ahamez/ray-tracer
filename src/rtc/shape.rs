/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Point, Vector},
    rtc::{
        shapes::{Cone, Cube, Cylinder, Group, Plane, Sphere},
        Ray,
    },
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Cone(Cone),
    Cube(),
    Cylinder(Cylinder),
    Group(Group),
    Plane(),
    Sphere(),
}

/* ---------------------------------------------------------------------------------------------- */

impl Shape {
    pub fn intersects(&self, ray: &Ray, push: &mut impl FnMut(f64)) {
        match self {
            Shape::Cone(c) => c.intersects(&ray, push),
            Shape::Cube() => Cube::intersects(&ray, push),
            Shape::Cylinder(c) => c.intersects(&ray, push),
            Shape::Group(g) => g.intersects(&ray, push),
            Shape::Plane() => Plane::intersects(&ray, push),
            Shape::Sphere() => Sphere::intersects(&ray, push),
        }
    }

    pub fn normal_at(&self, object_point: &Point) -> Vector {
        match self {
            Shape::Cone(c) => c.normal_at(&object_point),
            Shape::Cube() => Cube::normal_at(&object_point),
            Shape::Cylinder(c) => c.normal_at(&object_point),
            Shape::Group(g) => g.normal_at(&object_point),
            Shape::Plane() => Plane::normal_at(&object_point),
            Shape::Sphere() => Sphere::normal_at(&object_point),
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */
