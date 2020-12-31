use ray_tracer::{
    primitive::{Point, Tuple, Vector},
    rtc::{view_transform, Camera, Color, GroupBuilder, Light, Object, Transform, World},
};
use std::f64::consts::PI;
use std::sync::Arc;

fn hexagon_corner() -> GroupBuilder {
    let corner = Object::new_sphere()
        .scale(0.25, 0.25, 0.25)
        .translate(0.0, 0.0, -1.0);

    GroupBuilder::Leaf(corner)
}

fn hexagon_edge() -> GroupBuilder {
    let edge = Object::new_cylinder(0.0, 1.0, true)
        .scale(0.25, 1.0, 0.25)
        .rotate_z(-PI / 2.0)
        .rotate_y(-PI / 6.0)
        .translate(0.0, 0.0, -1.0);

    GroupBuilder::Leaf(edge)
}

fn hexagon_side() -> Vec<GroupBuilder> {
    vec![hexagon_corner(), hexagon_edge()]
}

fn hexagon() -> Object {
    let mut sides = vec![];

    for n in 0..=5 {
        let side = GroupBuilder::Node(
            Object::new_dummy().rotate_y(n as f64 * PI / 3.0),
            hexagon_side(),
        );

        sides.push(side);
    }

    let hex_builder = GroupBuilder::Node(Object::new_dummy(), sides);

    Object::new_group(&hex_builder)
}

fn main() {
    let hexagon = Arc::new(hexagon());
    let light = Light::new_point_light(Color::white(), Point::new(-5.0, 10.0, -10.0));

    let world = World::new()
        .with_objects(vec![hexagon])
        .with_lights(vec![light]);

    let from = Point::new(0.0, 7.0, -5.0);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 1.0);

    let width = 5000;
    let height = 5000;
    let fov = 0.4;

    let camera =
        Camera::new(width, height, fov).with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.render(&world, true);
    canvas.export("hexagon.png").unwrap();
}
