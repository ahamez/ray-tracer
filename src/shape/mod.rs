// --------------------------------------------------------------------------------------------- //

mod plane;
mod sphere;

// --------------------------------------------------------------------------------------------- //

use crate::{
    primitive::{Point, Vector},
    rtc::Ray,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Shape {
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
