/* ---------------------------------------------------------------------------------------------- */

use std::sync::Arc;

use crate::{
    primitive::{Matrix, Point, Vector},
    rtc::{BoundingBox, IntersectionPusher, Object, Ray, Shape, Transform},
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    bounding_box: BoundingBox,
    children: Vec<Arc<Object>>,
}

/* ---------------------------------------------------------------------------------------------- */

impl Group {
    fn new(children: Vec<Arc<Object>>) -> Self {
        let bounding_box = Group::mk_bounding_box(&children);

        Self {
            children,
            bounding_box,
        }
    }

    pub fn intersects(&self, ray: &Ray, push: &mut impl IntersectionPusher) {
        if self.bounds().is_intersected(ray) {
            for child in &self.children {
                push.set_object(child.clone());
                child.intersects(ray, push);
            }
        }
    }

    pub fn normal_at(&self, _object_point: &Point) -> Vector {
        unreachable!()
    }

    pub fn children(&self) -> &Vec<Arc<Object>> {
        &self.children
    }

    pub fn bounds(&self) -> BoundingBox {
        self.bounding_box
    }

    fn mk_bounding_box(children: &[Arc<Object>]) -> BoundingBox {
        let mut bbox = BoundingBox::new();
        for child in children {
            bbox = bbox + child.bounding_box();
        }

        bbox
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub enum GroupBuilder {
    Leaf(Object),
    Node(Object, Vec<GroupBuilder>),
}

impl GroupBuilder {
    pub fn build(&self) -> Object {
        GroupBuilder::rec(self, &Matrix::id())
    }

    fn rec(gb: &Self, transform: &Matrix) -> Object {
        match gb {
            GroupBuilder::Leaf(o) => o.transform(transform),
            GroupBuilder::Node(group, children) => {
                let child_transform = *transform * *group.transformation();
                let mut new_children = vec![];
                for c in children {
                    new_children.push(Arc::new(GroupBuilder::rec(&c, &child_transform)));
                }

                group
                    .clone()
                    .with_shape(Shape::Group(Group::new(new_children)))
                    .with_transformation(Matrix::id())
            }
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        primitive::{Point, Tuple, Vector},
        rtc::IntersectionPusher,
    };

    struct Push {
        pub xs: Vec<f64>,
        pub object: Arc<Object>,
    }

    impl Push {
        pub fn new() -> Self {
            Self {
                xs: vec![],
                object: Arc::new(Object::new_dummy()),
            }
        }
    }

    impl IntersectionPusher for Push {
        fn t(&mut self, t: f64) {
            self.xs.push(t);
        }
        fn set_object(&mut self, object: Arc<Object>) {
            self.object = object
        }
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let group_builder = GroupBuilder::Node(Object::new_dummy(), vec![]);
        let group = Object::new_group(&group_builder);
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut push = Push::new();

        group.intersects(&ray, &mut push);

        assert_eq!(push.xs.len(), 0);
    }

    #[test]
    fn intersecting_a_ray_with_an_non_empty_group() {
        let s1 = Object::new_sphere();
        let s2 = Object::new_sphere().translate(0.0, 0.0, -3.0);
        let s3 = Object::new_sphere().translate(5.0, 0.0, 0.0);

        let group_builder = GroupBuilder::Node(
            Object::new_dummy(),
            vec![
                GroupBuilder::Leaf(s1.clone()),
                GroupBuilder::Leaf(s2.clone()),
                GroupBuilder::Leaf(s3),
            ],
        );
        let group = Object::new_group(&group_builder);

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = ray.intersects(&(vec![Arc::new(group)][..]));

        assert_eq!(xs.len(), 4);
        assert_eq!(*xs[0].object(), Arc::new(s2.clone()));
        assert_eq!(*xs[1].object(), Arc::new(s2));
        assert_eq!(*xs[2].object(), Arc::new(s1.clone()));
        assert_eq!(*xs[3].object(), Arc::new(s1));
    }

    #[test]
    fn intersecting_a_ray_with_a_nested_non_empty_group() {
        {
            let s1 = Object::new_sphere();
            let s2 = Object::new_sphere().translate(0.0, 0.0, -3.0);
            let s3 = Object::new_sphere().translate(5.0, 0.0, 0.0);

            let group_builder_1 = GroupBuilder::Node(
                Object::new_dummy(),
                vec![
                    GroupBuilder::Leaf(s1.clone()),
                    GroupBuilder::Leaf(s2.clone()),
                    GroupBuilder::Leaf(s3),
                ],
            );

            let group_builder_2 = GroupBuilder::Node(Object::new_dummy(), vec![group_builder_1]);
            let group_2 = Object::new_group(&group_builder_2);

            let ray = Ray {
                origin: Point::new(0.0, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            };

            let xs = ray.intersects(&(vec![Arc::new(group_2)][..]));

            assert_eq!(xs.len(), 4);
            assert_eq!(*xs[0].object(), Arc::new(s2.clone()));
            assert_eq!(*xs[1].object(), Arc::new(s2));
            assert_eq!(*xs[2].object(), Arc::new(s1.clone()));
            assert_eq!(*xs[3].object(), Arc::new(s1));
        }
        // s1 and s2 in different groups
        {
            let s1 = Object::new_sphere();
            let s2 = Object::new_sphere().translate(0.0, 0.0, -3.0);
            let s3 = Object::new_sphere().translate(5.0, 0.0, 0.0);

            let group_builder_1 = GroupBuilder::Node(
                Object::new_dummy(),
                vec![GroupBuilder::Leaf(s1.clone()), GroupBuilder::Leaf(s3)],
            );

            let group_builder_2 = GroupBuilder::Node(
                Object::new_dummy(),
                vec![group_builder_1, GroupBuilder::Leaf(s2.clone())],
            );
            let group_2 = Object::new_group(&group_builder_2);

            let ray = Ray {
                origin: Point::new(0.0, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            };

            let xs = ray.intersects(&(vec![Arc::new(group_2)][..]));

            assert_eq!(xs.len(), 4);
            assert_eq!(*xs[0].object(), Arc::new(s2.clone()));
            assert_eq!(*xs[1].object(), Arc::new(s2));
            assert_eq!(*xs[2].object(), Arc::new(s1.clone()));
            assert_eq!(*xs[3].object(), Arc::new(s1));
        }
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let s = Object::new_sphere().translate(5.0, 0.0, 0.0);

        let group_builder = GroupBuilder::Node(
            Object::new_dummy().scale(2.0, 2.0, 2.0),
            vec![GroupBuilder::Leaf(s.clone())],
        );
        let group = Object::new_group(&group_builder);

        let ray = Ray {
            origin: Point::new(10.0, 0.0, -10.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = ray.intersects(&(vec![Arc::new(group)][..]));

        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_a_nested_transformed_group() {
        {
            let s = Object::new_sphere().translate(5.0, 0.0, 0.0);

            let group_builder_1 =
                GroupBuilder::Node(Object::new_dummy(), vec![GroupBuilder::Leaf(s.clone())]);

            let group_builder_2 = GroupBuilder::Node(
                Object::new_dummy().scale(2.0, 2.0, 2.0),
                vec![group_builder_1],
            );
            let group_2 = Object::new_group(&group_builder_2);

            let ray = Ray {
                origin: Point::new(10.0, 0.0, -10.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            };

            let xs = ray.intersects(&(vec![Arc::new(group_2)][..]));

            assert_eq!(xs.len(), 2);
        }
        {
            let s = Object::new_sphere().translate(5.0, 0.0, 0.0);

            let group_builder_1 = GroupBuilder::Node(
                Object::new_dummy().scale(2.0, 2.0, 2.0),
                vec![GroupBuilder::Leaf(s.clone())],
            );

            let group_builder_2 = GroupBuilder::Node(Object::new_dummy(), vec![group_builder_1]);
            let group_2 = Object::new_group(&group_builder_2);

            let ray = Ray {
                origin: Point::new(10.0, 0.0, -10.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            };

            let xs = ray.intersects(&(vec![Arc::new(group_2)][..]));

            assert_eq!(xs.len(), 2);
        }
    }

    #[test]
    fn transformations_are_propagated() {
        let s = Object::new_sphere()
            .translate(5.0, 0.0, 0.0)
            .scale(2.0, 2.0, 2.0)
            .rotate_y(std::f64::consts::PI / 2.0);

        let expected_transformation = s.transformation();

        // With one group
        {
            let s = Object::new_sphere().translate(5.0, 0.0, 0.0);

            let g2_builder = GroupBuilder::Node(
                Object::new_dummy()
                    .scale(2.0, 2.0, 2.0)
                    .rotate_y(std::f64::consts::PI / 2.0),
                vec![GroupBuilder::Leaf(s.clone())],
            );
            let g2 = Object::new_group(&g2_builder);

            // Retrieve the s with the baked-in group transform.
            let group_s = g2.shape().as_group().unwrap().children[0].clone();

            assert_eq!(group_s.transformation(), expected_transformation);
        }
        {
            let s = Object::new_sphere().translate(5.0, 0.0, 0.0);

            let g2_builder = GroupBuilder::Node(
                Object::new_dummy()
                    .rotate_y(std::f64::consts::PI / 2.0)
                    .scale(2.0, 2.0, 2.0),
                vec![GroupBuilder::Leaf(s.clone())],
            );

            let g1_builder = GroupBuilder::Node(Object::new_dummy(), vec![g2_builder]);
            let g1 = Object::new_group(&g1_builder);

            // Retrieve the s with the baked-in group transform.
            let group_g2 = g1.shape().as_group().unwrap().children[0].clone();
            let group_s = group_g2.shape().as_group().unwrap().children[0].clone();

            assert_eq!(group_s.transformation(), expected_transformation);
        }
        // With three nested groups, only one being transformed
        {
            let s = Object::new_sphere().translate(5.0, 0.0, 0.0);

            let g2_builder = GroupBuilder::Node(
                Object::new_dummy()
                    .rotate_y(std::f64::consts::PI / 2.0)
                    .scale(2.0, 2.0, 2.0),
                vec![GroupBuilder::Leaf(s.clone())],
            );

            let g1_builder = GroupBuilder::Node(Object::new_dummy(), vec![g2_builder]);

            let g0_builder = GroupBuilder::Node(Object::new_dummy(), vec![g1_builder]);
            let g0 = Object::new_group(&g0_builder);

            // Retrieve the s with the baked-in group transform.
            let group_g1 = g0.shape().as_group().unwrap().children[0].clone();
            let group_g2 = group_g1.shape().as_group().unwrap().children[0].clone();
            let group_s = group_g2.shape().as_group().unwrap().children[0].clone();

            assert_eq!(group_s.transformation(), expected_transformation);
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
            let group_g2 = g1.shape().as_group().unwrap().children[0].clone();
            let group_s = group_g2.shape().as_group().unwrap().children[0].clone();

            assert_eq!(group_s.transformation(), expected_transformation);
        }
    }

    #[test]
    fn a_group_has_a_bounding_box_that_contains_its_children() {
        let s = Object::new_sphere()
            .scale(2.0, 2.0, 2.0)
            .translate(2.0, 5.0, -3.0);
        let c = Object::new_cylinder(-2.0, 2.0, true)
            .scale(0.5, 1.0, 0.5)
            .translate(-4.0, -1.0, 4.0);

        let group_builder = GroupBuilder::Node(
            Object::new_dummy(),
            vec![GroupBuilder::Leaf(s), GroupBuilder::Leaf(c)],
        );
        let g = Object::new_group(&group_builder);

        assert_eq!(g.bounding_box().min(), Point::new(-4.5, -3.0, -5.0));
        assert_eq!(g.bounding_box().max(), Point::new(4.0, 7.0, 4.5));
    }

    #[test]
    fn intersecting_a_ray_with_doesnt_test_children_if_bbox_is_missed() {
        let ts = Object::new_test_shape();

        let group_builder = GroupBuilder::Node(Object::new_dummy(), vec![GroupBuilder::Leaf(ts)]);
        let g = Object::new_group(&group_builder);

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let mut push = Push::new();
        g.intersects(&ray, &mut push);

        let ts = g.shape().as_group().unwrap().children()[0]
            .shape()
            .as_test_shape()
            .unwrap();

        assert!(ts.ray().is_none());
    }

    #[test]
    fn intersecting_a_ray_with_tests_children_if_bbox_is_hit() {
        let ts = Object::new_test_shape();

        let group_builder = GroupBuilder::Node(Object::new_dummy(), vec![GroupBuilder::Leaf(ts)]);
        let g = Object::new_group(&group_builder);

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut push = Push::new();
        g.intersects(&ray, &mut push);

        let ts = g.shape().as_group().unwrap().children()[0]
            .shape()
            .as_test_shape()
            .unwrap();

        assert!(ts.ray().is_some());
    }
}

/* ---------------------------------------------------------------------------------------------- */
