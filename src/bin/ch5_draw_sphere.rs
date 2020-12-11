// --------------------------------------------------------------------------------------------- //

use std::fs::File;
use std::io::Write;
use std::path::Path;

use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;

use ray_tracer::object::Object;
use ray_tracer::point::Point;
use ray_tracer::ray::Ray;
use ray_tracer::sphere::Sphere;
use ray_tracer::transformation::Transform;
use ray_tracer::tuple::Tuple;

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
    let shape = Sphere::new()
        .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        .rotate_z(3.14 / 4.0)
        // .rotate_y(3.14 / 4.0)
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

            let intersections = ray.intersects(&[Object::Sphere(shape)]);
            if !intersections.is_empty() {
                canvas[x][y] = color;
            }
        }
    }

    let ppm = canvas.ppm();
    let path = Path::new("./sphere_projection.ppm");
    let mut file = File::create(&path).unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
