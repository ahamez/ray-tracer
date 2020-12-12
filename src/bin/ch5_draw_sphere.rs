// --------------------------------------------------------------------------------------------- //

use std::f64::consts::PI;

use ray_tracer::{
    canvas::Canvas, color::Color, point::Point, ray::Ray, shape::Shape,
    transformation::Transform, tuple::Tuple,
};

// --------------------------------------------------------------------------------------------- //

fn main() {
    const CANVAS_PIXELS: usize = 100;

    let mut canvas = Canvas::new_with_color(CANVAS_PIXELS, CANVAS_PIXELS, Color::black());

    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / CANVAS_PIXELS as f64;
    let half = wall_size / 2.0;

    let color: Color = Color::red();
    let shape = Shape::new_sphere()
        .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        .rotate_z(PI / 4.0)
        // .rotate_y(PI / 4.0)
        .scale(1.0, 0.5, 1.0)
        .scale(0.7, 1.0, 1.0)
        .translate(0.0, 0.0, 1.0);

    for y in 0..CANVAS_PIXELS {
        let target_y = half - pixel_size * y as f64;

        for x in 0..CANVAS_PIXELS {
            let target_x = -half + pixel_size * x as f64;
            let target = Point::new(target_x, target_y, wall_z);

            let direction = (target - ray_origin).normalize();
            let ray = Ray {
                origin: ray_origin,
                direction,
            };

            let intersections = ray.intersects(&[shape]);
            if !intersections.is_empty() {
                canvas[x][y] = color;
            }
        }
    }

    canvas.export("./sphere_projection.png").unwrap();
}
