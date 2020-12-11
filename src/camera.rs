// --------------------------------------------------------------------------------------------- //

use crate::{
    canvas::Canvas, matrix::Matrix, point::Point, ray::Ray, transformation::Transform,
    tuple::Tuple, world::World,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    h_size: usize,
    v_size: usize,
    fov: f64,
    transformation: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

// --------------------------------------------------------------------------------------------- //

impl Camera {
    pub fn new(h_size: usize, v_size: usize, fov: f64) -> Self {
        let half_view = (fov / 2.0).tan();
        let aspect = h_size as f64 / v_size as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / h_size as f64;

        Camera {
            h_size,
            v_size,
            fov,
            transformation: Matrix::id(),
            pixel_size,
            half_width,
            half_height,
        }
    }

    pub fn with_transformation(mut self, transformation: &Matrix) -> Self {
        self.transformation = *transformation;
        self
    }

    fn ray_for_pixel(&self, px: f64, py: f64) -> Ray {
        let x_offset = (px + 0.5) * self.pixel_size;
        let y_offset = (py + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transformation_inv = self.transformation.invert().unwrap();
        let pixel = transformation_inv * Point::new(world_x, world_y, -1.0);

        let origin = transformation_inv * Point::zero();
        let direction = (pixel - origin).normalize();

        Ray { origin, direction }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.h_size, self.v_size);

        for x in 0..self.h_size {
            for y in 0..self.v_size {
                let ray = self.ray_for_pixel(x as f64, y as f64);
                let color = world.color_at(&ray);

                image[y][x] = color;
            }
        }

        image
    }
}

// --------------------------------------------------------------------------------------------- //

impl Transform for Camera {
    fn apply_transformation(&self, transformation: &Matrix) -> Self {
        Camera {
            transformation: self.transformation * *transformation,
            ..*self
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use std::f64::consts::PI;

    use super::*;
    use crate::{color::Color, point::Point, transformation::view_transform, tuple::Tuple, vector::Vector};

    #[test]
    fn pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(approx_eq!(f64, c.pixel_size, 0.01, epsilon = 0.00001));
    }

    #[test]
    fn pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(approx_eq!(f64, c.pixel_size, 0.01, epsilon = 0.00001));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100.0, 50.0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0.0, 0.0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let c = Camera::new(201, 101, PI / 2.0)
            .rotate_y(PI / 4.0)
            .translate(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100.0, 50.0);

        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World {
            ..Default::default()
        };
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let c = Camera::new(11, 11, PI / 2.0).with_transformation(&view_transform(&from, &to, &up));

        let image = c.render(&w);

        assert_eq!(image[5][5], Color::new(0.38066, 0.47583, 0.2855));
    }
}

// --------------------------------------------------------------------------------------------- //
