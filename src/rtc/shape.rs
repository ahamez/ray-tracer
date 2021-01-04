/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Point, Vector},
    rtc::{
        shapes::{Cone, Cube, Cylinder, Group, Plane, SmoothTriangle, Sphere, TestShape, Triangle},
        BoundingBox, Intersection, IntersectionPusher, Ray,
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
    SmoothTriangle(SmoothTriangle),
    Sphere(),
    TestShape(TestShape),
    Triangle(Triangle),
}

/* ---------------------------------------------------------------------------------------------- */

impl Shape {
    pub fn intersects(&self, ray: &Ray, push: &mut impl IntersectionPusher) {
        match self {
            Shape::Cone(c) => c.intersects(&ray, push),
            Shape::Cube() => Cube::intersects(&ray, push),
            Shape::Cylinder(c) => c.intersects(&ray, push),
            Shape::Dummy() => unreachable!("Dummy::intersects() should never be called"),
            Shape::Group(g) => g.intersects(&ray, push),
            Shape::Plane() => Plane::intersects(&ray, push),
            Shape::SmoothTriangle(t) => t.intersects(&ray, push),
            Shape::Sphere() => Sphere::intersects(&ray, push),
            Shape::TestShape(t) => t.intersects(&ray, push),
            Shape::Triangle(t) => t.intersects(&ray, push),
        }
    }

    pub fn normal_at(&self, object_point: &Point, hit: &Intersection) -> Vector {
        match self {
            Shape::Cone(c) => c.normal_at(&object_point),
            Shape::Cube() => Cube::normal_at(&object_point),
            Shape::Cylinder(c) => c.normal_at(&object_point),
            Shape::Dummy() => unreachable!("Dummy::normal_at() should never be called"),
            Shape::Group(g) => g.normal_at(&object_point),
            Shape::Plane() => Plane::normal_at(&object_point),
            Shape::SmoothTriangle(t) => t.normal_at(&object_point, hit),
            Shape::Sphere() => Sphere::normal_at(&object_point),
            Shape::TestShape(t) => t.normal_at(&object_point),
            Shape::Triangle(t) => t.normal_at(&object_point),
        }
    }

    pub fn bounds(&self) -> BoundingBox {
        match self {
            Shape::Cone(c) => c.bounds(),
            Shape::Cube() => Cube::bounds(),
            Shape::Cylinder(c) => c.bounds(),
            Shape::Dummy() => BoundingBox::new(),
            Shape::Group(g) => g.bounds(),
            Shape::Plane() => Plane::bounds(),
            Shape::SmoothTriangle(t) => t.bounds(),
            Shape::Sphere() => Sphere::bounds(),
            Shape::TestShape(t) => t.bounds(),
            Shape::Triangle(t) => t.bounds(),
        }
    }

    pub fn skip_world_to_local(&self) -> bool {
        // Skip world to local conversion for groups, since the transformation matrix
        // has been propagated to children at build time via GroupBuilder.
        // TODO. Dispatch this to shapes to further decouple from concrete types.
        matches!(self, Shape::Group(_))
    }

    pub fn as_group(&self) -> Option<&Group> {
        match self {
            Shape::Group(g) => Some(g),
            _ => None,
        }
    }

    pub fn as_smooth_triangle(&self) -> Option<&SmoothTriangle> {
        match self {
            Shape::SmoothTriangle(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_test_shape(&self) -> Option<&TestShape> {
        match self {
            Shape::TestShape(ts) => Some(ts),
            _ => None,
        }
    }

    pub fn as_triangle(&self) -> Option<&Triangle> {
        match self {
            Shape::Triangle(t) => Some(t),
            _ => None,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */
