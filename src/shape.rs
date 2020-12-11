// --------------------------------------------------------------------------------------------- //

use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector;

// --------------------------------------------------------------------------------------------- //

pub trait Shape {
    fn intersects(&self, ray: &Ray, is: &mut Vec<Intersection>);
    fn normal_at(&self, world_point: &Point) -> Vector;
    fn material(&self) -> &Material;
    fn transformation(&self) -> &Matrix;
}

// --------------------------------------------------------------------------------------------- //
