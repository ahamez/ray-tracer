/* ---------------------------------------------------------------------------------------------- */

use std::sync::Arc;

use crate::{
    primitive::{Matrix, Point, Vector},
    rtc::{Intersection, Intersections, Object, Transform},
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

/* ---------------------------------------------------------------------------------------------- */

impl Ray {
    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn intersects(&self, objects: &[Arc<Object>]) -> Intersections {
        let mut is = Vec::<Intersection>::with_capacity(16);
        objects.iter().for_each(|object| {
            if object.is_group() {
                object.group_intersects(self, &mut |t: f64, object: Arc<Object>| {
                    is.push(Intersection { t, object })
                })
            } else {
                object.intersects(self, &mut |t: f64| {
                    is.push(Intersection {
                        t,
                        object: object.clone(),
                    })
                });
            }
        });

        Intersections::new(is)
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Transform for Ray {
    fn apply_transformation(&self, transformation: &Matrix) -> Ray {
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

        let r1 = r0.translate(3.0, 4.0, 5.0);

        assert_eq!(r1.origin, Point::new(4.0, 6.0, 8.0));
        assert_eq!(r1.direction, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r0 = Ray {
            origin: Point::new(1.0, 2.0, 3.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let r1 = r0.scale(2.0, 3.0, 4.0);

        assert_eq!(r1.origin, Point::new(2.0, 6.0, 12.0));
        assert_eq!(r1.direction, Vector::new(0.0, 3.0, 0.0));
    }
}

/* ---------------------------------------------------------------------------------------------- */
