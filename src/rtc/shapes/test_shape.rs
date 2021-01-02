/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Point, Tuple, Vector},
    rtc::{BoundingBox, IntersectionPusher, Ray},
};
use std::sync::{Arc, Mutex};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub struct TestShape {
    ray: Arc<Mutex<Option<Ray>>>,
}

/* ---------------------------------------------------------------------------------------------- */

impl TestShape {
    pub fn new() -> Self {
        Self {
            ray: Arc::new(Mutex::new(None)),
        }
    }

    pub fn intersects(&self, ray: &Ray, _push: &mut impl IntersectionPusher) {
        let mut reference = self.ray.lock().unwrap();
        *reference = Some(ray.clone());
    }

    pub fn normal_at(&self, _object_point: &Point) -> Vector {
        unreachable!()
    }

    pub fn bounds(&self) -> BoundingBox {
        BoundingBox::new()
            .with_min(Point::new(-1.0, -1.0, -1.0))
            .with_max(Point::new(1.0, 1.0, 1.0))
    }

    pub fn ray(&self) -> Option<Ray> {
        *self.ray.lock().unwrap()
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl PartialEq for TestShape {
    fn eq(&self, _other: &TestShape) -> bool {
        unreachable!()
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rtc::{Object, Shape, Transform};

    struct Push {
        pub xs: Vec<f64>,
    }

    impl IntersectionPusher for Push {
        fn t(&mut self, t: f64) {
            self.xs.push(t);
        }
        fn set_object(&mut self, _object: Arc<Object>) {
            panic!();
        }
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Object::new_test_shape().scale(2.0, 2.0, 2.0);

        let mut push = Push { xs: vec![] };
        s.intersects(&ray, &mut push);
        let s = match s.shape() {
            Shape::TestShape(s) => s,
            _ => panic!(),
        };

        assert_eq!(s.ray().unwrap().origin, Point::new(0.0, 0.0, -2.5));
        assert_eq!(s.ray().unwrap().direction, Vector::new(0.0, 0.0, 0.5));
    }

    #[test]
    fn a_test_shape_has_a_bounding_box() {
        let t = Object::new_test_shape();
        assert_eq!(t.bounds().min(), Point::new(-1.0, -1.0, -1.0));
        assert_eq!(t.bounds().max(), Point::new(1.0, 1.0, 1.0));
    }
}

/* ---------------------------------------------------------------------------------------------- */
