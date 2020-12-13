// --------------------------------------------------------------------------------------------- //

#![allow(unused_variables)]

use std::f64::consts::PI;

use ray_tracer::{
    camera::Camera,
    color::Color,
    light::Light,
    material::Material,
    object::Object,
    pattern::Pattern,
    point::Point,
    transformation::{view_transform, Transform},
    tuple::Tuple,
    vector::Vector,
    world::World,
};

// --------------------------------------------------------------------------------------------- //

fn main() {
    let floor =
        Object::new_plane().with_material(Material::new().with_pattern(Pattern::new_stripe(vec![
            Color::new(0.5, 0.5, 0.5),
            Color::white(),
            Color::new(0.7, 0.6, 0.7),
        ])));

    let wall = Object::new_plane()
        .with_material(Material::new().with_pattern(Pattern::new_ring(vec![
            Color::new(0.5, 0.5, 0.5),
            Color::white(),
            Color::new(0.7, 0.6, 0.7),
        ])))
        .translate(0.0, 0.0, 5.0)
        .rotate_x(PI / 2.0);

    let sphere = Object::new_sphere()
        .with_material(
            Material::new()
                // .with_pattern(Pattern::new_ring(vec![
                .with_pattern(Pattern::new_stripe(vec![
                    Color::white(),
                    Color::blue(),
                    Color::new(0.5, 0.0, 0.9),
                ]))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(0.0, 1.5, -4.0)
        .scale(1.5, 1.5, 1.5);

    let light1 = Light {
        intensity: Color::white(),
        position: Point::new(-10.0, 10.0, -10.0),
    };

    let world = World {
        objects: vec![floor, wall, sphere],
        lights: vec![light1],
    };

    let from = Point::new(0.0, 1.5, -8.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let factor = 10;

    let camera = Camera::new(100 * factor, 50 * factor, PI / 1.5)
        .with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.render(&world);

    canvas.export("./pattern.png").unwrap();
}
