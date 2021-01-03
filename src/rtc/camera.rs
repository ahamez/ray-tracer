/* ---------------------------------------------------------------------------------------------- */

use rayon::prelude::*;

use crate::{
    primitive::{Matrix, Point, Tuple},
    rtc::{Canvas, Ray, Transform, World},
};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    h_size: usize,
    v_size: usize,
    fov: f64,
    transformation: Matrix,
    transformation_inverse: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

/* ---------------------------------------------------------------------------------------------- */

impl Camera {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_size(mut self, h_size: usize, v_size: usize) -> Self {
        let (pixel_size, half_width, half_height) = Camera::pixel_size(h_size, v_size, self.fov);
        self.h_size = h_size;
        self.v_size = v_size;
        self.pixel_size = pixel_size;
        self.half_width = half_width;
        self.half_height = half_height;

        self
    }

    pub fn with_fov(mut self, fov: f64) -> Self {
        let (pixel_size, half_width, half_height) =
            Camera::pixel_size(self.h_size, self.v_size, fov);
        self.fov = fov;
        self.pixel_size = pixel_size;
        self.half_width = half_width;
        self.half_height = half_height;

        self
    }

    fn pixel_size(h_size: usize, v_size: usize, fov: f64) -> (f64, f64, f64) {
        let half_view = (fov / 2.0).tan();
        let aspect = h_size as f64 / v_size as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = (half_width * 2.0) / h_size as f64;

        (pixel_size, half_width, half_height)
    }

    pub fn with_transformation(mut self, transformation: &Matrix) -> Self {
        self.transformation = *transformation;
        self.transformation_inverse = transformation.invert();
        self
    }

    fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let px = px as f64;
        let py = py as f64;

        let x_offset = (px + 0.5) * self.pixel_size;
        let y_offset = (py + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let transformation_inv = self.transformation_inverse;
        let pixel = transformation_inv * Point::new(world_x, world_y, -1.0);

        let origin = transformation_inv * Point::zero();
        let direction = (pixel - origin).normalize();

        Ray { origin, direction }
    }

    pub fn render(&self, world: &World, parallel: bool) -> Canvas {
        if parallel {
            self.parallel_render(world)
        } else {
            self.sequential_render(world)
        }
    }

    pub fn sequential_render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.h_size, self.v_size);

        for row in 0..self.v_size {
            for col in 0..self.h_size {
                let ray = self.ray_for_pixel(col, row);
                let color = world.color_at(&ray);

                image[row][col] = color;
            }
        }

        image
    }

    pub fn parallel_render(&self, world: &World) -> Canvas {
        const BAND_SIZE: usize = 10;
        let mut image = Canvas::new(self.h_size, self.v_size);

        image
            .pixels()
            .par_chunks_mut(self.h_size * BAND_SIZE)
            .enumerate()
            .for_each(|(i, band)| {
                for row in 0..BAND_SIZE {
                    for col in 0..self.h_size {
                        let ray = self.ray_for_pixel(col, row + i * BAND_SIZE);
                        let color = world.color_at(&ray);

                        band[row * self.h_size + col] = color;
                    }
                }
            });

        image
    }

    pub fn size(&self) -> (usize, usize) {
        (self.h_size, self.v_size)
    }

    pub fn fov(&self) -> f64 {
        self.fov
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Default for Camera {
    fn default() -> Self {
        let h_size = 100;
        let v_size = 100;
        let fov = std::f64::consts::PI / 2.0;

        let (pixel_size, half_width, half_height) = Camera::pixel_size(h_size, v_size, fov);

        Camera {
            h_size,
            v_size,
            fov,
            transformation: Matrix::id(),
            transformation_inverse: Matrix::id(),
            pixel_size,
            half_width,
            half_height,
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

impl Transform for Camera {
    fn transform(&self, transformation: &Matrix) -> Self {
        let new_transformation = *transformation * self.transformation;
        Camera {
            transformation: new_transformation,
            transformation_inverse: new_transformation.invert(),
            ..*self
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::{
        float::ApproxEq,
        primitive::{Point, Tuple, Vector},
        rtc::{view_transform, Color},
    };

    #[test]
    fn pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new().with_size(200, 125).with_fov(PI / 2.0);
        assert!(c.pixel_size.approx_eq(0.01));
    }

    #[test]
    fn pixel_size_for_a_vertical_canvas() {
        let c = Camera::new().with_size(200, 125).with_fov(PI / 2.0);
        assert!(c.pixel_size.approx_eq(0.01));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new().with_size(201, 101).with_fov(PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new().with_size(201, 101).with_fov(PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let c = Camera::new()
            .with_size(201, 101)
            .with_fov(PI / 2.0)
            .translate(0.0, -2.0, 5.0)
            .rotate_y(PI / 4.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = crate::rtc::world::tests::default_world();
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let c = Camera::new()
            .with_size(11, 11)
            .with_fov(PI / 2.0)
            .with_transformation(&view_transform(&from, &to, &up));

        let image = c.sequential_render(&w);

        assert_eq!(image[5][5], Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn parallel_rendering_a_world_with_a_camera() {
        let w = crate::rtc::world::tests::default_world();
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let c = Camera::new()
            .with_size(100, 100)
            .with_fov(PI / 2.0)
            .with_transformation(&view_transform(&from, &to, &up));

        let image = c.sequential_render(&w);
        let par_image = c.parallel_render(&w);

        assert_eq!(image, par_image);
    }
}

/* ---------------------------------------------------------------------------------------------- */
