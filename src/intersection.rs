// --------------------------------------------------------------------------------------------- //

use std::cmp::Ordering;

use crate::{epsilon::EPSILON, object::Object, point::Point, ray::Ray, vector::Vector};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Object,
}

// --------------------------------------------------------------------------------------------- //

impl std::cmp::PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::cmp::Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.t.is_nan() {
            Ordering::Greater
        } else if other.t.is_nan() {
            Ordering::Less
        } else if self.t > other.t {
            Ordering::Greater
        } else if self.t < other.t {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::cmp::Eq for Intersection {}

// --------------------------------------------------------------------------------------------- //

#[derive(Debug)]
pub struct Intersections(Vec<Intersection>);

// --------------------------------------------------------------------------------------------- //

impl Intersections {
    pub fn new(mut is: Vec<Intersection>) -> Self {
        is.sort();
        Intersections(is)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.0.iter().find(|i| i.t >= 0.0)
    }
}

// --------------------------------------------------------------------------------------------- //

impl std::ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, i: usize) -> &Intersection {
        let &Intersections(vec) = &self;
        &vec[i]
    }
}

// --------------------------------------------------------------------------------------------- //

#[derive(Debug)]
pub struct IntersectionState {
    t: f64,
    object: Object,
    point: Point,
    over_point: Point,
    eye_v: Vector,
    normal_v: Vector,
    reflect_v: Vector,
    inside: bool,
}

// --------------------------------------------------------------------------------------------- //

impl IntersectionState {
    pub fn new(intersection: &Intersection, ray: &Ray) -> Self {
        let point = ray.position(intersection.t);

        let eye_v = -ray.direction;
        let normal_v = intersection.object.normal_at(&point);
        let (inside, normal_v) = if (normal_v ^ eye_v) < 0.0 {
            (true, -normal_v)
        } else {
            (false, normal_v)
        };
        let reflect_v = ray.direction.reflect(&normal_v);
        let over_point = point + normal_v * EPSILON;

        IntersectionState {
            t: intersection.t,
            object: intersection.object.clone(),
            point,
            over_point,
            eye_v,
            normal_v,
            reflect_v,
            inside,
        }
    }

    pub fn object(&self) -> &Object {
        &self.object
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn over_point(&self) -> Point {
        self.over_point
    }

    pub fn eye_v(&self) -> Vector {
        self.eye_v
    }

    pub fn normal_v(&self) -> Vector {
        self.normal_v
    }

    pub fn reflect_v(&self) -> Vector {
        self.reflect_v
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {

    use super::*;

    use crate::{object::Object, transformation::Transform, tuple::Tuple};

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let object = Object::new_sphere();
        let t = 3.5;
        let i = Intersection {
            t,
            object: object.clone(),
        };

        assert_eq!(i.t, t);
        assert_eq!(i.object, object);
    }

    #[test]
    fn sort_intersections() {
        let i0 = Intersection {
            t: 1.0,
            object: Object::new_sphere(),
        };
        let i1 = Intersection {
            t: -1.0,
            object: Object::new_sphere(),
        };
        let i2 = Intersection {
            t: 0.0,
            object: Object::new_sphere(),
        };

        let mut vec = vec![i0.clone(), i1.clone(), i2.clone()];
        vec.sort();

        assert_eq!(vec, vec![i1, i2, i0]);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let object = Object::new_sphere();

        let i0 = Intersection {
            t: 1.0,
            object: object.clone(),
        };
        let i1 = Intersection { t: 2.0, object };
        let is = Intersections::new(vec![i0.clone(), i1]);

        let i = is.hit();

        assert_eq!(i, Some(&i0));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let object = Object::new_sphere();

        let i0 = Intersection {
            t: -1.0,
            object: object.clone(),
        };
        let i1 = Intersection { t: 2.0, object };
        let is = Intersections::new(vec![i0, i1.clone()]);

        let i = is.hit();

        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let object = Object::new_sphere();

        let i0 = Intersection {
            t: -1.0,
            object: object.clone(),
        };
        let i1 = Intersection { t: -1.0, object };
        let is = Intersections::new(vec![i0.clone(), i1.clone()]);

        let i = is.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let object = Object::new_sphere();

        let i0 = Intersection {
            t: 5.0,
            object: object.clone(),
        };
        let i1 = Intersection {
            t: 7.0,
            object: object.clone(),
        };
        let i2 = Intersection {
            t: -3.0,
            object: object.clone(),
        };
        let i3 = Intersection { t: 2.0, object };
        let is = Intersections::new(vec![i0, i1, i2, i3.clone()]);

        let i = is.hit();

        assert_eq!(i, Some(&i3));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };
        let i = Intersection {
            t: 4.0,
            object: Object::new_sphere(),
        };
        let comps = IntersectionState::new(&i, &r);

        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eye_v, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };
        let i = Intersection {
            t: 1.0,
            object: Object::new_sphere(),
        };
        let comps = IntersectionState::new(&i, &r);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eye_v, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal_v, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let object = Object::new_sphere().translate(0.0, 0.0, 1.0);

        let i = Intersection { t: 5.0, object };

        let comps = IntersectionState::new(&i, &r);

        assert!(comps.over_point.z() < EPSILON / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let sqrt2 = f64::sqrt(2.0);
        let half_sqrt2 = sqrt2 / 2.0;

        let ray = Ray {
            origin: Point::new(0.0, 1.0, -1.0),
            direction: Vector::new(0.0, -half_sqrt2, half_sqrt2),
        };
        let object = Object::new_plane();
        let i = Intersection { t: sqrt2, object };

        let comps = IntersectionState::new(&i, &ray);

        assert_eq!(comps.reflect_v, Vector::new(0.0, half_sqrt2, half_sqrt2));
    }
}

// --------------------------------------------------------------------------------------------- //
