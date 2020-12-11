use std::fs::File;
use std::io::Write;
use std::path::Path;

use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::point::Point;
use ray_tracer::transformation::Transform;
use ray_tracer::tuple::Tuple;

// --------------------------------------------------------------------------------------------- //

fn main() {
    const CANVAS_SIZE: usize = 600;
    const RADIUS: f64 = CANVAS_SIZE as f64 * 3.0 / 8.0;

    let mut canvas = Canvas::new_with_color(CANVAS_SIZE, CANVAS_SIZE, Color::black());

    let initial = Point::new(0.0, 0.0, 1.0);
    for i in 0..12 {
        let point = initial.rotate_y(i as f64 * std::f64::consts::PI / 6.0);
        let point_on_radius = point * RADIUS;
        let point_around_center = point_on_radius + (CANVAS_SIZE / 2) as f64;
        println!("{:?}", point_around_center);

        for i in -10isize..11 {
            for j in -10isize..11 {
                let x = point_around_center.x() as isize + i;
                let y = point_around_center.z() as isize + j;
                canvas[x as usize][y as usize] = Color::white();
            }
        }
    }

    let ppm = canvas.ppm();

    let path = Path::new("./clock.ppm");
    let mut file = File::create(&path).unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
