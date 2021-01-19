use ray_tracer::{
    primitive::{Point, Tuple, Vector},
    rtc::{view_transform, Camera, Color, Light, Object, ParallelRendering, Transform, World},
};
use std::f64::consts::PI;

fn hexagon_corner() -> Object {
    Object::new_sphere()
        .scale(0.25, 0.25, 0.25)
        .translate(0.0, 0.0, -1.0)
        .transform()
}

fn hexagon_edge() -> Object {
    Object::new_cylinder(0.0, 1.0, false)
        .scale(0.25, 1.0, 0.25)
        .rotate_z(-PI / 2.0)
        .rotate_y(-PI / 6.0)
        .translate(0.0, 0.0, -1.0)
        .transform()
}

fn hexagon_side() -> Vec<Object> {
    vec![hexagon_corner(), hexagon_edge()]
}

fn hexagon() -> Object {
    let mut sides = vec![];

    for n in 0..=5 {
        let side = Object::new_group(hexagon_side())
            .rotate_y(n as f64 * PI / 3.0)
            .transform();
        sides.push(side);
    }

    let hex = Object::new_group(sides)
        .rotate_x(PI / 3.0)
        .translate(0.0, 0.75, 0.0)
        .transform();

    Object::new_group(vec![hex])
}

fn main() {
    let hexagon = hexagon();
    let light = Light::new_point_light(Color::white(), Point::new(-5.0, 10.0, -10.0));

    let world = World::new()
        .with_objects(vec![hexagon])
        .with_lights(vec![light]);

    let from = Point::new(0.0, 1.5, -5.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let width = 5000;
    let height = 5000;
    let fov = PI / 3.0;

    let camera = Camera::new()
        .with_size(width, height)
        .with_fov(fov)
        .with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.render(&world, ParallelRendering::True);
    canvas.export("ch14_hexagon.png").unwrap();
}
