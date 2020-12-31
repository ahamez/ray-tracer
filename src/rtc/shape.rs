/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Point, Vector},
    rtc::{
        shapes::{Cone, Cube, Cylinder, Group, Plane, Sphere},
        IntersectionPusher, Ray,
    },
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Cone(Cone),
    Cube(),
    Dummy(),
    Cylinder(Cylinder),
    Group(Group),
    Plane(),
    Sphere(),
}

/* ---------------------------------------------------------------------------------------------- */

impl Shape {
    pub fn intersects(&self, ray: &Ray, push: &mut impl IntersectionPusher) {
        match self {
            Shape::Cone(c) => c.intersects(&ray, push),
            Shape::Cube() => Cube::intersects(&ray, push),
            Shape::Cylinder(c) => c.intersects(&ray, push),
            Shape::Dummy() => panic!("Dummy::intersects() should never be called"),
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
            Shape::Dummy() => panic!("Dummy::normal_at() should never be called"),
            Shape::Group(_) => panic!("Group::normal_at() should never be called"),
            Shape::Plane() => Plane::normal_at(&object_point),
            Shape::Sphere() => Sphere::normal_at(&object_point),
        }
    }

    pub fn as_group(&self) -> Option<&Group> {
        match self {
            Shape::Group(g) => Some(g),
            _ => None,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */
