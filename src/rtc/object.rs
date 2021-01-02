/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Matrix, Point, Vector},
    rtc::{
        shapes::{Cone, Cylinder, GroupBuilder, Sphere, TestShape, Triangle},
        BoundingBox, IntersectionPusher, Material, Ray, Shape, Transform,
    },
};

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

    pub fn new_dummy() -> Self {
        Object {
            shape: Shape::Dummy(),
            ..Default::default()
        }
    }

    pub fn new_group(builder: &GroupBuilder) -> Self {
        builder.build().transform(&Matrix::id())
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

    pub fn new_sphere() -> Self {
        let shape = Shape::Sphere();
        let bounding_box = shape.bounds();

        Object {
            shape,
            bounding_box,
            ..Default::default()
        }
    }

    pub fn new_test_shape() -> Self {
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
        self
    }

    pub fn with_transformation(mut self, transformation: Matrix) -> Self {
        self.transformation = transformation;
        self.transformation_inverse = self.transformation.invert();
        self.transformation_inverse_transpose = self.transformation_inverse.transpose();
        self.bounding_box = self.bounds().transform(&self.transformation);

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

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.shape.normal_at(&local_point);

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

    pub fn bounds(&self) -> BoundingBox {
        self.shape.bounds()
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
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
    fn transform(&self, transformation: &Matrix) -> Self {
        let transformation = *transformation * self.transformation;
        let transformation_inverse = transformation.invert();
        let transformation_inverse_transpose = transformation_inverse.transpose();
        let bounding_box = self.shape.bounds().transform(&transformation);

        Object {
            bounding_box,
            transformation,
            transformation_inverse,
            transformation_inverse_transpose,
            ..self.clone()
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Tuple;

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
                .rotate_y(std::f64::consts::PI / 2.0);

            assert_eq!(
                s.world_to_object(&Point::new(-2.0, 0.0, -10.0)),
                Point::new(0.0, 0.0, -1.0)
            );
        }
        // With two nested groups with transformations in both
        {
            let s = Object::new_sphere().translate(5.0, 0.0, 0.0);

            let g2_builder = GroupBuilder::Node(
                Object::new_dummy().scale(2.0, 2.0, 2.0),
                vec![GroupBuilder::Leaf(s.clone())],
            );

            let g1_builder = GroupBuilder::Node(
                Object::new_dummy().rotate_y(std::f64::consts::PI / 2.0),
                vec![g2_builder],
            );
            let g1 = Object::new_group(&g1_builder);

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
        let s = Object::new_sphere().translate(5.0, 0.0, 0.0);

        let g2_builder = GroupBuilder::Node(
            Object::new_dummy().scale(1.0, 2.0, 3.0),
            vec![GroupBuilder::Leaf(s.clone())],
        );

        let g1_builder = GroupBuilder::Node(
            Object::new_dummy().rotate_y(std::f64::consts::PI / 2.0),
            vec![g2_builder],
        );
        let g1 = Object::new_group(&g1_builder);

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
        let s = Object::new_sphere().translate(5.0, 0.0, 0.0);

        let g2_builder = GroupBuilder::Node(
            Object::new_dummy().scale(1.0, 2.0, 3.0),
            vec![GroupBuilder::Leaf(s.clone())],
        );

        let g1_builder = GroupBuilder::Node(
            Object::new_dummy().rotate_y(std::f64::consts::PI / 2.0),
            vec![g2_builder],
        );
        let g1 = Object::new_group(&g1_builder);

        // Retrieve the s with the baked-in group transform.
        let group_g2 = g1.shape().as_group().unwrap().children()[0].clone();
        let group_s = group_g2.shape().as_group().unwrap().children()[0].clone();

        assert_eq!(
            group_s.normal_at(&Point::new(1.7321, 1.1547, -5.5774)),
            Vector::new(0.2857, 0.4286, -0.8571)
        );
    }
}

/* ---------------------------------------------------------------------------------------------- */
