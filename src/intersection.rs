// --------------------------------------------------------------------------------------------- //

use std::{cmp::Ordering, sync::Arc};

use crate::{epsilon::EPSILON, object::Object, point::Point, ray::Ray, vector::Vector};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Arc<Object>,
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
        is.sort_unstable();
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

    pub fn hit_index(&self) -> Option<usize> {
        self.0.iter().position(|i| i.t >= 0.0)
    }

    pub fn iter(&self) -> std::slice::Iter<Intersection> {
        self.0.iter()
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
    cos_i: f64,
    eye_v: Vector,
    inside: bool,
    n1: f64,
    n2: f64,
    normal_v: Vector,
    object: Arc<Object>,
    over_point: Point,
    point: Point,
    reflect_v: Vector,
    t: f64,
    under_point: Point,
}

// --------------------------------------------------------------------------------------------- //

impl IntersectionState {
    pub fn new(intersections: &Intersections, intersection_index: usize, ray: &Ray) -> Self {
        let intersection = &intersections[intersection_index];

        let mut containers = Vec::<&Arc<Object>>::with_capacity(intersections.len());

        let mut n1 = None;
        let mut n2 = None;

        for (index, i) in intersections.iter().enumerate() {
            let is_intersection = index == intersection_index;

            if is_intersection {
                n1 = containers
                    .last()
                    .map(|object| object.material().refractive_index);
            }

            match containers
                .iter()
                .position(|object| Arc::ptr_eq(object, &i.object))
            {
                Some(pos) => {
                    containers.remove(pos);
                }
                None => {
                    containers.push(&i.object);
                }
            }

            if is_intersection {
                n2 = containers
                    .last()
                    .map(|object| object.material().refractive_index);

                break;
            }
        }

        let point = ray.position(intersection.t);

        let eye_v = -ray.direction;
        let normal_v = intersection.object.normal_at(&point);
        let (inside, normal_v) = if normal_v ^ eye_v < 0.0 {
            (true, -normal_v)
        } else {
            (false, normal_v)
        };
        let reflect_v = ray.direction.reflect(&normal_v);
        let over_point = point + normal_v * EPSILON;
        let under_point = point - normal_v * EPSILON;

        IntersectionState {
            cos_i: normal_v ^ eye_v,
            eye_v,
            inside,
            n1: n1.unwrap_or(1.0),
            n2: n2.unwrap_or(1.0),
            normal_v,
            object: intersection.object.clone(),
            over_point,
            point,
            reflect_v,
            t: intersection.t,
            under_point,
        }
    }

    pub fn schlick(&self) -> f64 {
        let mut cos = self.cos_i;

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }

    pub fn cos_i(&self) -> f64 {
        self.cos_i
    }

    pub fn eye_v(&self) -> Vector {
        self.eye_v
    }

    pub fn n(&self) -> (f64, f64) {
        (self.n1, self.n2)
    }

    pub fn normal_v(&self) -> Vector {
        self.normal_v
    }

    pub fn object(&self) -> &Object {
        &self.object
    }

    pub fn over_point(&self) -> Point {
        self.over_point
    }

    pub fn reflect_v(&self) -> Vector {
        self.reflect_v
    }

    pub fn under_point(&self) -> Point {
        self.under_point
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{approx_eq::ApproxEq, object::Object, transformation::Transform, tuple::Tuple};

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let object = Arc::new(Object::new_sphere());
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
            object: Arc::new(Object::new_sphere()),
        };
        let i1 = Intersection {
            t: -1.0,
            object: Arc::new(Object::new_sphere()),
        };
        let i2 = Intersection {
            t: 0.0,
            object: Arc::new(Object::new_sphere()),
        };

        let mut vec = vec![i0.clone(), i1.clone(), i2.clone()];
        vec.sort();

        assert_eq!(vec, vec![i1, i2, i0]);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let object = Arc::new(Object::new_sphere());

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
        let object = Arc::new(Object::new_sphere());

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
        let object = Arc::new(Object::new_sphere());

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
        let object = Arc::new(Object::new_sphere());

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
            object: Arc::new(Object::new_sphere()),
        };
        let comps = IntersectionState::new(&Intersections::new(vec![i.clone()]), 0, &r);

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
            object: Arc::new(Object::new_sphere()),
        };
        let comps = IntersectionState::new(&Intersections::new(vec![i]), 0, &r);

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

        let object = Arc::new(Object::new_sphere().translate(0.0, 0.0, 1.0));

        let i = Intersection { t: 5.0, object };

        let comps = IntersectionState::new(&Intersections::new(vec![i]), 0, &r);

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
        let object = Arc::new(Object::new_plane());
        let i = Intersection { t: sqrt2, object };

        let comps = IntersectionState::new(&Intersections::new(vec![i]), 0, &ray);

        assert_eq!(comps.reflect_v, Vector::new(0.0, half_sqrt2, half_sqrt2));
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let object = Arc::new(crate::sphere::tests::glassy_sphere().translate(0.0, 0.0, 1.0));

        let i = Intersection { t: 5.0, object };

        let comps = IntersectionState::new(&Intersections::new(vec![i]), 0, &ray);

        assert!(comps.under_point.z() > EPSILON / 2.0);
        assert!(comps.point.z() < comps.under_point.z());
    }

    #[test]
    fn the_schlick_apporimixation_under_toal_internal_reflection() {
        let object = Arc::new(crate::sphere::tests::glassy_sphere());
        let ray = Ray {
            origin: Point::new(0.0, 0.0, f64::sqrt(2.0) / 2.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = Intersections::new(vec![
            Intersection {
                t: -f64::sqrt(2.0) / 2.0,
                object: object.clone(),
            },
            Intersection {
                t: f64::sqrt(2.0) / 2.0,
                object,
            },
        ]);

        let comps = IntersectionState::new(&xs, 1, &ray);

        assert!(comps.schlick().approx_eq(1.0));
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let object = Arc::new(crate::sphere::tests::glassy_sphere());
        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = Intersections::new(vec![
            Intersection {
                t: -1.0,
                object: object.clone(),
            },
            Intersection { t: 1.0, object },
        ]);

        let comps = IntersectionState::new(&xs, 1, &ray);

        assert!(comps.schlick().approx_eq(0.04));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let object = Arc::new(crate::sphere::tests::glassy_sphere());
        let ray = Ray {
            origin: Point::new(0.0, 0.99, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = Intersections::new(vec![Intersection { t: 1.8589, object }]);

        let comps = IntersectionState::new(&xs, 0, &ray);

        assert!(comps.schlick().approx_eq(0.48873));
    }
}

// --------------------------------------------------------------------------------------------- //
