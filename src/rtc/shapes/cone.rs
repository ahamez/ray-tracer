// --------------------------------------------------------------------------------------------- //

use crate::{
    float::{ApproxEq, EPSILON},
    primitive::{Point, Tuple, Vector},
    rtc::Ray,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cone {
    min: f64,
    max: f64,
    closed: bool,
}

// --------------------------------------------------------------------------------------------- //

impl Cone {
    pub fn new() -> Self {
        Cone {
            ..Default::default()
        }
    }

    pub fn new_truncated(min: f64, max: f64, closed: bool) -> Self {
        let (min, max) = if min < max { (min, max) } else { (max, min) };

        Cone { min, max, closed }
    }

    pub fn intersects<F>(&self, ray: &Ray, mut push: F)
    where
        F: FnMut(f64),
    {
        let a = ray.direction.x().powi(2) - ray.direction.y().powi(2) + ray.direction.z().powi(2);

        let b = 2.0
            * (ray.origin.x() * ray.direction.x() - ray.origin.y() * ray.direction.y()
                + ray.origin.z() * ray.direction.z());

        // Delayed evaluation.
        let c = || ray.origin.x().powi(2) - ray.origin.y().powi(2) + ray.origin.z().powi(2);

        if a.approx_eq(0.0) && !b.approx_eq(0.0) {
            let t = c() / (-2.0 * b);
            push(t);
        } else {
            let discriminant = b.powi(2) - 4.0 * a * c();

            if discriminant < 0.0 {
                return;
            }

            let double_a = 2.0 * a;
            let t0 = (-b - discriminant.sqrt()) / double_a;
            let t1 = (-b + discriminant.sqrt()) / double_a;

            let (t0, t1) = if t0 < t1 { (t0, t1) } else { (t1, t0) };

            let y0 = ray.origin.y() + t0 * ray.direction.y();
            if self.min < y0 && y0 < self.max {
                push(t0);
            }

            let y1 = ray.origin.y() + t1 * ray.direction.y();
            if self.min < y1 && y1 < self.max {
                push(t1);
            }
        }
        self.intersects_caps(&ray, push);
    }

    fn check_cap(ray: &Ray, t: f64, radius: f64) -> bool {
        let x = ray.origin.x() + t * ray.direction.x();
        let z = ray.origin.z() + t * ray.direction.z();

        (x.powi(2) + z.powi(2)) <= radius.powi(2)
    }

    fn intersects_caps<F>(&self, ray: &Ray, mut push: F)
    where
        F: FnMut(f64),
    {
        if !self.closed || ray.direction.y().approx_eq(0.0) {
            return;
        }

        let t = (self.min - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(&ray, t, self.min) {
            push(t);
        }

        let t = (self.max - ray.origin.y()) / ray.direction.y();
        if Self::check_cap(&ray, t, self.max) {
            push(t);
        }
    }

    pub fn normal_at(&self, object_point: &Point) -> Vector {
        let dist = object_point.x().powi(2) + object_point.z().powi(2);

        if dist < 1.0 && object_point.y() >= (self.max - EPSILON) {
            Vector::new(0.0, 1.0, 0.0)
        } else if dist < 1.0 && object_point.y() <= (self.min + EPSILON) {
            Vector::new(0.0, -1.0, 0.0)
        } else {
            Vector::new(
                object_point.x(),
                if object_point.y() > 0.0 {
                    -dist.sqrt()
                } else {
                    dist.sqrt()
                },
                object_point.z(),
            )
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl Default for Cone {
    fn default() -> Self {
        Cone {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            closed: false,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let c = Cone::new();
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -1.0),
            direction: Vector::new(0.0, 1.0, 1.0).normalize(),
        };
        let mut xs = vec![];
        c.intersects(&ray, |t| xs.push(t));
        assert_eq!(xs.len(), 1);
        assert!(xs[0].approx_eq_low_precision(0.35355));
    }

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let tests = vec![
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new(0.0, 0.0, -5.0),
                Vector::new(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                Point::new(1.0, 1.0, -5.0),
                Vector::new(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ];

        let c = Cone::new();
        for (origin, direction, t0, t1) in tests.into_iter() {
            let mut xs = vec![];
            c.intersects(
                &Ray {
                    origin,
                    direction: direction.normalize(),
                },
                |t| xs.push(t),
            );
            assert_eq!(xs.len(), 2);
            assert!(xs[0].approx_eq_low_precision(t0));
            assert!(xs[1].approx_eq_low_precision(t1));
        }
    }

    #[test]
    fn intersecting_a_cone_s_end_caps() {
        let tests = vec![
            (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
            (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
        ];

        let c = Cone::new_truncated(-0.5, 0.5, true);
        for (origin, direction, count) in tests.into_iter() {
            let mut xs = vec![];
            c.intersects(
                &Ray {
                    origin,
                    direction: direction.normalize(),
                },
                |t| xs.push(t),
            );
            assert_eq!(xs.len(), count as usize);
        }
    }

    #[test]
    fn normal_vector_on_a_conee() {
        let c = Cone::new();
        assert_eq!(
            c.normal_at(&Point::new(0.0, 0.0, 0.0)),
            Vector::new(0.0, 0.0, 0.0)
        );
        assert_eq!(
            c.normal_at(&Point::new(1.0, 1.0, 1.0)),
            Vector::new(1.0, -f64::sqrt(2.0), 1.0)
        );
        assert_eq!(
            c.normal_at(&Point::new(-1.0, -1.0, 0.0)),
            Vector::new(-1.0, 1.0, 0.0)
        );
    }
}

// --------------------------------------------------------------------------------------------- //
