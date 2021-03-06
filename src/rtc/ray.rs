/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Matrix, Point, Vector},
    rtc::{Intersection, IntersectionPusher, Intersections, Object, Transform},
};
use serde::{Deserialize, Serialize};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

/* ---------------------------------------------------------------------------------------------- */

pub struct RayIntersectionPusher<'a> {
    pub intersections: Intersections<'a>,
    pub object: &'a Object,
}

impl<'a> IntersectionPusher<'a> for RayIntersectionPusher<'a> {
    fn t(&mut self, t: f64) {
        self.intersections.push(Intersection::new(t, self.object));
    }

    fn t_u_v(&mut self, t: f64, u: f64, v: f64) {
        self.intersections
            .push(Intersection::new(t, self.object).with_u_and_v(u, v));
    }

    fn set_object(&mut self, object: &'a Object) {
        self.object = object;
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Ray {
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn intersects<'a>(
        &self,
        objects: &'a [Object],
        intersections: Intersections<'a>,
    ) -> Intersections<'a> {
        objects
            .iter()
            .fold(intersections, |acc, object| {
                let mut pusher = RayIntersectionPusher {
                    intersections: acc,
                    object,
                };
                object.intersects(self, &mut pusher);

                pusher.intersections
            })
            .sort()
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Transform for Ray {
    fn transform(self, transformation: &Matrix) -> Ray {
        Ray {
            origin: *transformation * self.origin,
            direction: *transformation * self.direction,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Tuple;

    #[test]
    fn position() {
        let r = Ray {
            origin: Point::new(2.0, 3.0, 4.0),
            direction: Vector::new(1.0, 0.0, 0.0),
        };

        assert_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r0 = Ray {
            origin: Point::new(1.0, 2.0, 3.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let r1 = r0.translate(3.0, 4.0, 5.0).transform();

        assert_eq!(r1.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r1.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r0 = Ray {
            origin: Point::new(1.0, 2.0, 3.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let r1 = r0.scale(2.0, 3.0, 4.0).transform();

        assert_eq!(r1.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r1.direction, Vector::new(0.0, 3.0, 0.0));
    }
}

/* ---------------------------------------------------------------------------------------------- */
