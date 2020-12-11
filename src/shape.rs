// --------------------------------------------------------------------------------------------- //

use crate::{
    intersection::Intersection, material::Material, matrix::Matrix, point::Point, ray::Ray,
    vector::Vector,
};

// --------------------------------------------------------------------------------------------- //

pub trait Shape {
    fn intersects(&self, ray: &Ray, is: &mut Vec<Intersection>);
    fn normal_at(&self, world_point: &Point) -> Vector;
    fn material(&self) -> &Material;
    fn transformation(&self) -> &Matrix;
}

// --------------------------------------------------------------------------------------------- //
