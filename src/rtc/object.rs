/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Matrix, Point, Vector},
    rtc::{
        shapes::{Cone, Cylinder, GroupBuilder, SmoothTriangle, Sphere, TestShape, Triangle},
        BoundingBox, Intersection, IntersectionPusher, Material, Ray, Shape, Transform,
    },
};
use std::sync::Arc;

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    bounding_box: BoundingBox,
    has_shadow: bool,
    material: Material,
    shape: Shape,
    transformation: Matrix,
    transformation_inverse: Matrix,
    transformation_inverse_transpose: Matrix,
}

/* ---------------------------------------------------------------------------------------------- */

impl Object {
    pub fn new_cone(min: f64, max: f64, closed: bool) -> Self {
        let shape = Shape::Cone(Cone::new(min, max, closed));
        let bounding_box = shape.bounds();

        Object {
            shape,
            bounding_box,
            ..Default::default()
        }
    }

    pub fn new_cube() -> Self {
        let shape = Shape::Cube();
        let bounding_box = shape.bounds();

        Object {
            shape: Shape::Cube(),
            bounding_box,
            ..Default::default()
        }
    }

    pub fn new_cylinder(min: f64, max: f64, closed: bool) -> Self {
        let shape = Shape::Cylinder(Cylinder::new(min, max, closed));
        let bounding_box = shape.bounds();

        Object {
            shape,
            bounding_box,
            ..Default::default()
        }
    }

    pub(in crate::rtc) fn new_dummy() -> Self {
        Object {
            shape: Shape::Dummy(),
            ..Default::default()
        }
    }

    pub fn new_group(children: Vec<Arc<Object>>) -> Self {
        let children_group_builders = children
            .iter()
            .filter_map(|child| match child.shape() {
                Shape::Group(g) => {
                    if g.children().is_empty() {
                        None
                    } else {
                        Some(GroupBuilder::from_object(child))
                    }
                }

                _ => Some(GroupBuilder::from_object(child)),
            })
            .collect();
        let group_builder = GroupBuilder::Node(Object::new_dummy(), children_group_builders);
        let object = group_builder.build();

        Object {
            bounding_box: object.shape.bounds(),
            ..object
        }
    }

    pub fn new_plane() -> Self {
        let shape = Shape::Plane();
        let bounding_box = shape.bounds();

        Object {
            shape,
            bounding_box,
            ..Default::default()
        }
    }

    pub fn new_smooth_triangle(
        p1: Point,
        p2: Point,
        p3: Point,
        n1: Vector,
        n2: Vector,
        n3: Vector,
    ) -> Self {
        let shape = Shape::SmoothTriangle(SmoothTriangle::new(p1, p2, p3, n1, n2, n3));
        let bounding_box = shape.bounds();

        Object {
            shape,
            bounding_box,
            ..Default::default()
        }
    }

    pub fn new_sphere() -> Self {
        let shape = Shape::Sphere();
        let bounding_box = shape.bounds();

        Object {
            shape,
            bounding_box,
            ..Default::default()
        }
    }

    #[allow(dead_code)] // Actually used by tests
    pub(in crate::rtc) fn new_test_shape() -> Self {
        Object {
            shape: Shape::TestShape(TestShape::new()),
            ..Default::default()
        }
    }

    pub fn new_triangle(p1: Point, p2: Point, p3: Point) -> Self {
        let shape = Shape::Triangle(Triangle::new(p1, p2, p3));
        let bounding_box = shape.bounds();

        Object {
            shape,
            bounding_box,
            ..Default::default()
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;

        self
    }

    pub fn with_shadow(mut self, has_shadow: bool) -> Self {
        self.has_shadow = has_shadow;

        self
    }

    pub fn with_shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self.bounding_box = self.shape.bounds();

        self
    }

    pub fn with_transformation(mut self, transformation: Matrix) -> Self {
        self.transformation = transformation;
        self.transformation_inverse = self.transformation.invert();
        self.transformation_inverse_transpose = self.transformation_inverse.transpose();
        self.bounding_box = self.shape_bounds().transform(&self.transformation);

        self
    }

    pub fn intersects(&self, ray: &Ray, push: &mut impl IntersectionPusher) {
        if self.shape.skip_world_to_local() {
            self.shape.intersects(ray, push)
        } else {
            let transformed_ray = ray.transform(&self.transformation_inverse);

            self.shape.intersects(&transformed_ray, push)
        }
    }

    pub fn normal_at(&self, world_point: &Point, hit: &Intersection) -> Vector {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.shape.normal_at(&local_point, hit);

        self.normal_to_world(&local_normal)
    }

    fn world_to_object(&self, world_point: &Point) -> Point {
        self.transformation_inverse * *world_point
    }

    fn normal_to_world(&self, normal: &Vector) -> Vector {
        (self.transformation_inverse_transpose * *normal).normalize()
    }

    pub fn has_shadow(&self) -> bool {
        self.has_shadow
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn transformation(&self) -> &Matrix {
        &self.transformation
    }

    pub fn transformation_inverse(&self) -> &Matrix {
        &self.transformation_inverse
    }

    pub fn shape_bounds(&self) -> BoundingBox {
        self.shape.bounds()
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }

    pub fn divide(self, threshold: usize) -> Self {
        Self {
            shape: self.shape.divide(threshold),
            ..self
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Default for Object {
    fn default() -> Self {
        Object {
            bounding_box: Sphere::bounds(),
            has_shadow: true,
            material: Material::new(),
            shape: Shape::Sphere(),
            transformation: Matrix::id(),
            transformation_inverse: Matrix::id(),
            transformation_inverse_transpose: Matrix::id(),
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Transform for Object {
    fn transform(self, new_transformation: &Matrix) -> Self {
        match self.shape() {
            Shape::Group(g) => {
                // Each time a Group is transformed, we convert it back to a GroupBuilder,
                // which is easier to manipulate. It's not the most efficient, but as this
                // is only peformed when constructing objects of a world, it has no impact on
                // the rendering itself.
                let children_group_builders = g
                    .children()
                    .iter()
                    .map(|child| GroupBuilder::from_object(child))
                    .collect();

                // We then create a new top GroupBuilder Node from which the new transformation is
                // applied.
                let group_builder = GroupBuilder::Node(
                    Object::new_dummy().with_transformation(*new_transformation),
                    children_group_builders,
                );

                // Convert back to a Group.
                group_builder.build()
            }
            _other_shape => {
                let new_transformation = *new_transformation * self.transformation;
                self.with_transformation(new_transformation)
            }
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Tuple;
    use std::sync::Arc;

    #[test]
    fn an_object_default_transformation_is_id() {
        let s = Object::new_sphere();
        assert_eq!(s.transformation, Matrix::id());
    }

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        // Without a group
        {
            let s = Object::new_sphere()
                .translate(5.0, 0.0, 0.0)
                .scale(2.0, 2.0, 2.0)
                .rotate_y(std::f64::consts::PI / 2.0)
                .transform();

            assert_eq!(
                s.world_to_object(&Point::new(-2.0, 0.0, -10.0)),
                Point::new(0.0, 0.0, -1.0)
            );
        }
        // With two nested groups with transformations in both
        {
            let s = Object::new_sphere().translate(5.0, 0.0, 0.0).transform();
            let g2 = Object::new_group(vec![Arc::new(s)])
                .scale(2.0, 2.0, 2.0)
                .rotate_y(std::f64::consts::PI / 2.0)
                .transform();
            let g1 = Object::new_group(vec![Arc::new(g2.clone())]);

            // Retrieve the s with the baked-in group transform.
            let group_g2 = g1.shape().as_group().unwrap().children()[0].clone();
            let group_s = group_g2.shape().as_group().unwrap().children()[0].clone();

            assert_eq!(
                group_s.world_to_object(&Point::new(-2.0, 0.0, -10.0)),
                Point::new(0.0, 0.0, -1.0)
            );
        }
        {
            let s = Object::new_sphere().translate(5.0, 0.0, 0.0).transform();
            let g2 = Object::new_group(vec![Arc::new(s)])
                .scale(2.0, 2.0, 2.0)
                .transform();
            let g1 = Object::new_group(vec![Arc::new(g2)])
                .rotate_y(std::f64::consts::PI / 2.0)
                .transform();

            // Retrieve the s with the baked-in group transform.
            let group_g2 = g1.shape().as_group().unwrap().children()[0].clone();
            let group_s = group_g2.shape().as_group().unwrap().children()[0].clone();

            assert_eq!(
                group_s.world_to_object(&Point::new(-2.0, 0.0, -10.0)),
                Point::new(0.0, 0.0, -1.0)
            );
        }
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let s = Object::new_sphere().translate(5.0, 0.0, 0.0).transform();
        let g2 = Object::new_group(vec![Arc::new(s)])
            .scale(1.0, 2.0, 3.0)
            .transform();
        let g1 = Object::new_group(vec![Arc::new(g2)])
            .rotate_y(std::f64::consts::PI / 2.0)
            .transform();

        // Retrieve the s with the baked-in group transform.
        let group_g2 = g1.shape().as_group().unwrap().children()[0].clone();
        let group_s = group_g2.shape().as_group().unwrap().children()[0].clone();

        let sqrt3div3 = 3.0_f64.sqrt() / 3.0;

        assert_eq!(
            group_s.normal_to_world(&Vector::new(sqrt3div3, sqrt3div3, sqrt3div3)),
            Vector::new(0.2857, 0.4286, -0.8571)
        );
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let s = Object::new_sphere().translate(5.0, 0.0, 0.0).transform();
        let g2 = Object::new_group(vec![Arc::new(s)])
            .scale(1.0, 2.0, 3.0)
            .transform();
        let g1 = Object::new_group(vec![Arc::new(g2)])
            .rotate_y(std::f64::consts::PI / 2.0)
            .transform();

        // Retrieve the s with the baked-in group transform.
        let group_g2 = g1.shape().as_group().unwrap().children()[0].clone();
        let group_s = group_g2.shape().as_group().unwrap().children()[0].clone();

        let dummy_intersection =
            Intersection::new(std::f64::INFINITY, Arc::new(Object::new_test_shape()));

        assert_eq!(
            group_s.normal_at(&Point::new(1.7321, 1.1547, -5.5774), &dummy_intersection),
            Vector::new(0.2857, 0.4286, -0.8571)
        );
    }
}

/* ---------------------------------------------------------------------------------------------- */
