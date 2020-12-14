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
    let floor = Object::new_plane().with_material(
        Material::new()
            .with_pattern(Pattern::new_checker(
                Color::white(),
                Color::new(0.5, 0.5, 0.5),
            ))
            .with_reflective(0.3),
    );

    let sphere = Object::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_gradient(Color::blue(), Color::black()))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(1.0),
        )
        .translate(0.0, 2.5, -4.0);

    let light = Light {
        intensity: Color::white(),
        position: Point::new(-10.0, 10.0, -10.0),
    };

    let world = World {
        objects: vec![floor, sphere],
        lights: vec![light],
        recursion_limit: 10,
    };

    let from = Point::new(-1.0, 2.0, -9.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let factor = 100;

    let camera = Camera::new(100 * factor, 50 * factor, PI / 1.5)
        .with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.par_render(&world);

    canvas.export("./reflection.png").unwrap();
}
