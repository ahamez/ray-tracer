/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Point, Vector},
    rtc::{
        IntersectionPusher, Ray,
        shapes::{Cone, Cube, Cylinder, Group, Plane, Sphere, TestShape},
    },
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub enum Shape {
    Cone(Cone),
    Cube(),
    Dummy(), // Does not exist on its own
    Cylinder(Cylinder),
    Group(Group),
    Plane(),
    Sphere(),
    TestShape(TestShape),
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
            Shape::TestShape(t) => t.intersects(&ray, push),
        }
    }

    pub fn normal_at(&self, object_point: &Point) -> Vector {
        match self {
            Shape::Cone(c) => c.normal_at(&object_point),
            Shape::Cube() => Cube::normal_at(&object_point),
            Shape::Cylinder(c) => c.normal_at(&object_point),
            Shape::Dummy() => panic!("Dummy::normal_at() should never be called"),
            Shape::Group(g) => g.normal_at(&object_point),
            Shape::Plane() => Plane::normal_at(&object_point),
            Shape::Sphere() => Sphere::normal_at(&object_point),
            Shape::TestShape(t) => t.normal_at(&object_point),
        }
    }

    pub fn skip_world_to_local(&self) -> bool {
        // Skip world to local conversion for groups, since the transformation matrix
        // has been propagated to children at build time via GroupBuilder.
        // TODO. Dispatch this to shapes to further decouple from concrete types.
        matches!(self, Shape::Group(_))
    }

    // TODO. Remove this. Quite ugly as it's only used for tests.
    pub fn as_group(&self) -> Option<&Group> {
        match self {
            Shape::Group(g) => Some(g),
            _ => None,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */
