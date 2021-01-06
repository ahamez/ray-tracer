/* ---------------------------------------------------------------------------------------------- */

use crate::{
    float::ApproxEq,
    primitive::{Matrix, Point, Tuple},
    rtc::{Ray, Transform},
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox {
    min: Point,
    max: Point,
}

/* ---------------------------------------------------------------------------------------------- */

impl BoundingBox {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn min(&self) -> Point {
        self.min
    }

    pub fn max(&self) -> Point {
        self.max
    }

    pub fn with_min(mut self, point: Point) -> Self {
        self.min = point;

        self
    }

    pub fn with_max(mut self, point: Point) -> Self {
        self.max = point;

        self
    }

    pub fn add_point(mut self, point: Point) -> Self {
        self.min = Point::new(
            f64::min(self.min.x(), point.x()),
            f64::min(self.min.y(), point.y()),
            f64::min(self.min.z(), point.z()),
        );

        self.max = Point::new(
            f64::max(self.max.x(), point.x()),
            f64::max(self.max.y(), point.y()),
            f64::max(self.max.z(), point.z()),
        );

        self
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        self.min.x() <= point.x()
            && point.x() <= self.max.x()
            && self.min.y() <= point.y()
            && point.y() <= self.max.y()
            && self.min.z() <= point.z()
            && point.z() <= self.max.z()
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.contains_point(&other.min) && self.contains_point(&other.max)
    }

    pub fn is_intersected(&self, ray: &Ray) -> bool {
        let (xtmin, xtmax) = BoundingBox::check_axis(
            ray.origin.x(),
            ray.direction.x(),
            self.min.x(),
            self.max.x(),
        );
        let (ytmin, ytmax) = BoundingBox::check_axis(
            ray.origin.y(),
            ray.direction.y(),
            self.min.y(),
            self.max.y(),
        );
        let (ztmin, ztmax) = BoundingBox::check_axis(
            ray.origin.z(),
            ray.direction.z(),
            self.min.z(),
            self.max.z(),
        );

        let tmax = xtmax.min(ytmax.min(ztmax));
        if tmax < 0.0 {
            false
        } else {
            let tmin = xtmin.max(ytmin.max(ztmin));

            tmin <= tmax
        }
    }

    fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;

        let tmin = tmin_numerator / direction;
        let tmax = tmax_numerator / direction;

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }

    pub fn split(&self) -> (BoundingBox, BoundingBox) {
        let dx = self.max.x() - self.min.x();
        let dy = self.max.y() - self.min.y();
        let dz = self.max.z() - self.min.z();

        let greatest = dx.max(dy.max(dz));

        let (mut x0, mut y0, mut z0) = (self.min.x(), self.min.y(), self.min.z());
        let (mut x1, mut y1, mut z1) = (self.max.x(), self.max.y(), self.max.z());

        if greatest.approx_eq(dx) {
            x0 += dx / 2.0;
            x1 = x0;
        } else if greatest.approx_eq(dy) {
            y0 += dy / 2.0;
            y1 = y0;
        } else {
            z0 += dz / 2.0;
            z1 = z0;
        }

        let mid_min = Point::new(x0, y0, z0);
        let mid_max = Point::new(x1, y1, z1);

        (
            BoundingBox::new().with_min(self.min).with_max(mid_max),
            BoundingBox::new().with_min(mid_min).with_max(self.max),
        )
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Default for BoundingBox {
    fn default() -> Self {
        BoundingBox {
            min: Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl std::ops::Add for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: BoundingBox) -> Self::Output {
        self.add_point(rhs.min()).add_point(rhs.max())
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Transform for BoundingBox {
    fn transform(&self, matrix: &Matrix) -> Self {
        let p1 = self.min;
        let p2 = Point::new(self.min.x(), self.min.y(), self.max.z());
        let p3 = Point::new(self.min.x(), self.max.y(), self.min.z());
        let p4 = Point::new(self.min.x(), self.max.y(), self.max.z());
        let p5 = Point::new(self.max.x(), self.min.y(), self.min.z());
        let p6 = Point::new(self.max.x(), self.min.y(), self.max.z());
        let p7 = Point::new(self.max.x(), self.max.y(), self.min.z());
        let p8 = self.max;

        Self::new()
            .add_point(*matrix * p1)
            .add_point(*matrix * p2)
            .add_point(*matrix * p3)
            .add_point(*matrix * p4)
            .add_point(*matrix * p5)
            .add_point(*matrix * p6)
            .add_point(*matrix * p7)
            .add_point(*matrix * p8)
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Vector;

    #[test]
    fn adding_points_to_an_empty_bounding_box() {
        let bbox = BoundingBox::new()
            .add_point(Point::new(-5.0, 2.0, 0.0))
            .add_point(Point::new(7.0, 0.0, -3.0));

        assert_eq!(bbox.min, Point::new(-5.0, 0.0, -3.0));
        assert_eq!(bbox.max, Point::new(7.0, 2.0, 0.0));
    }

    #[test]
    fn adding_one_bounding_box_to_the_other() {
        let bbox1 = BoundingBox::new()
            .with_min(Point::new(-5.0, -2.0, 0.0))
            .with_max(Point::new(7.0, 4.0, 4.0));

        let bbox2 = BoundingBox::new()
            .with_min(Point::new(8.0, -7.0, -2.0))
            .with_max(Point::new(14.0, 2.0, 8.0));

        let bbox = bbox1 + bbox2;

        assert_eq!(bbox.min, Point::new(-5.0, -7.0, -2.0));
        assert_eq!(bbox.max, Point::new(14.0, 4.0, 8.0));
    }

    #[test]
    fn a_box_contains_a_given_point() {
        let bbox = BoundingBox::new()
            .with_min(Point::new(5.0, -2.0, 0.0))
            .with_max(Point::new(11.0, 4.0, 7.0));

        let tests = vec![
            (Point::new(5.0, -2.0, 0.0), true),
            (Point::new(11.0, 4.0, 7.0), true),
            (Point::new(8.0, 1.0, 3.0), true),
            (Point::new(3.0, 0.0, 3.0), false),
            (Point::new(8.0, -4.0, 3.0), false),
            (Point::new(8.0, 1.0, -1.0), false),
            (Point::new(13.0, 1.0, 3.0), false),
            (Point::new(8.0, 5.0, 3.0), false),
            (Point::new(8.0, 1.0, 8.0), false),
        ];

        for (point, result) in tests {
            assert_eq!(bbox.contains_point(&point), result);
        }
    }

    #[test]
    fn a_box_contains_a_box() {
        let bbox1 = BoundingBox::new()
            .with_min(Point::new(5.0, -2.0, 0.0))
            .with_max(Point::new(11.0, 4.0, 7.0));

        let tests = vec![
            (Point::new(5.0, -2.0, 0.0), Point::new(11.0, 4.0, 7.0), true),
            (Point::new(6.0, -1.0, 1.0), Point::new(10.0, 3.0, 6.0), true),
            (
                Point::new(4.0, -3.0, -1.0),
                Point::new(10.0, 3.0, 6.0),
                false,
            ),
            (
                Point::new(6.0, -1.0, 1.0),
                Point::new(12.0, 5.0, 8.0),
                false,
            ),
        ];

        for (min, max, result) in tests {
            let bbox2 = BoundingBox::new().with_min(min).with_max(max);
            assert_eq!(bbox1.contains(&bbox2), result);
        }
    }

    #[test]
    fn transforming_a_bounding_box() {
        let bbox1 = BoundingBox::new()
            .with_min(Point::new(-1.0, -1.0, -1.0))
            .with_max(Point::new(1.0, 1.0, 1.0));

        let bbox2 = bbox1
            .rotate_y(std::f64::consts::PI / 4.0)
            .rotate_x(std::f64::consts::PI / 4.0);

        assert_eq!(bbox2.min, Point::new(-1.4142135, -1.7071067, -1.7071067));
        assert_eq!(bbox2.max, Point::new(1.4142135, 1.7071067, 1.7071067));
    }

    #[test]
    fn intersecting_a_ray_with_a_non_cubic_bounding_box() {
        let bbox = BoundingBox::new()
            .with_min(Point::new(5.0, -2.0, 0.0))
            .with_max(Point::new(11.0, 4.0, 7.0));

        let tests = vec![
            (
                Point::new(15.0, 1.0, 2.0),
                Vector::new(-1.0, 0.0, 0.0),
                true,
            ),
            (
                Point::new(-5.0, -1.0, 4.0),
                Vector::new(1.0, 0.0, 0.0),
                true,
            ),
            (Point::new(7.0, 6.0, 5.0), Vector::new(0.0, -1.0, 0.0), true),
            (Point::new(9.0, -5.0, 6.0), Vector::new(0.0, 1.0, 0.0), true),
            (
                Point::new(8.0, 2.0, 12.0),
                Vector::new(0.0, 0.0, -1.0),
                true,
            ),
            (Point::new(6.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), true),
            (Point::new(8.0, 1.0, 3.5), Vector::new(0.0, 0.0, 1.0), true),
            (
                Point::new(9.0, -1.0, -8.0),
                Vector::new(2.0, 4.0, 6.0),
                false,
            ),
            (
                Point::new(8.0, 3.0, -4.0),
                Vector::new(6.0, 2.0, 4.0),
                false,
            ),
            (
                Point::new(9.0, -1.0, -2.0),
                Vector::new(4.0, 6.0, 2.0),
                false,
            ),
            (
                Point::new(4.0, 0.0, 9.0),
                Vector::new(0.0, 0.0, -1.0),
                false,
            ),
            (
                Point::new(12.0, 5.0, 4.0),
                Vector::new(-1.0, 0.0, 0.0),
                false,
            ),
        ];

        for (origin, direction, result) in tests {
            let ray = Ray {
                origin,
                direction: direction.normalize(),
            };
            assert_eq!(bbox.is_intersected(&ray), result);
        }
    }

    #[test]
    fn splitting_a_perfect_cube() {
        let bbox = BoundingBox::new()
            .with_min(Point::new(-1.0, -4.0, -5.0))
            .with_max(Point::new(9.0, 6.0, 5.0));

        let (left, right) = bbox.split();

        assert_eq!(left.min, Point::new(-1.0, -4.0, -5.0));
        assert_eq!(left.max, Point::new(4.0, 6.0, 5.0));

        assert_eq!(right.min, Point::new(4.0, -4.0, -5.0));
        assert_eq!(right.max, Point::new(9.0, 6.0, 5.0));
    }

    #[test]
    fn splitting_a_x_wide_box() {
        let bbox = BoundingBox::new()
            .with_min(Point::new(-1.0, -2.0, -3.0))
            .with_max(Point::new(9.0, 5.5, 3.0));

        let (left, right) = bbox.split();

        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(4.0, 5.5, 3.0));

        assert_eq!(right.min, Point::new(4.0, -2.0, -3.0));
        assert_eq!(right.max, Point::new(9.0, 5.5, 3.0));
    }

    #[test]
    fn splitting_a_y_wide_box() {
        let bbox = BoundingBox::new()
            .with_min(Point::new(-1.0, -2.0, -3.0))
            .with_max(Point::new(5.0, 8.0, 3.0));

        let (left, right) = bbox.split();

        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(5.0, 3.0, 3.0));

        assert_eq!(right.min, Point::new(-1.0, 3.0, -3.0));
        assert_eq!(right.max, Point::new(5.0, 8.0, 3.0));
    }

    #[test]
    fn splitting_a_z_wide_box() {
        let bbox = BoundingBox::new()
            .with_min(Point::new(-1.0, -2.0, -3.0))
            .with_max(Point::new(5.0, 3.0, 7.0));

        let (left, right) = bbox.split();

        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(5.0, 3.0, 2.0));

        assert_eq!(right.min, Point::new(-1.0, -2.0, 2.0));
        assert_eq!(right.max, Point::new(5.0, 3.0, 7.0));
    }
}

/* ---------------------------------------------------------------------------------------------- */
