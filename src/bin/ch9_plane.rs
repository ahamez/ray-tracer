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
    let material = Material::new()
        // .with_color(Color::new(1.0, 0.9, 0.9))
        .with_pattern(Pattern::new_stripe(vec![
            Color::new(1.0, 0.9, 0.9),
            Color::blue(),
            Color::new(0.5, 0.7, 0.9),
        ]))
        .with_specular(0.0);

    let floor = Object::new_plane().with_material(material.clone());

    let wall = Object::new_plane()
        .with_material(Material::new().with_pattern(Pattern::new_ring(vec![
            Color::new(0.5, 0.5, 0.5),
            Color::white(),
            Color::new(0.7, 0.6, 0.7),
        ])))
        .translate(0.0, 0.0, 5.0)
        .rotate_x(PI / 2.0);

    let left = Object::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_ring(vec![
                    Color::white(),
                    Color::black(),
                    Color::white(),
                ]))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(-2.5, 0.33, -0.75)
        .scale(0.7, 0.7, 0.7);

    let center = Object::new_sphere().scale(0.1, 0.1, 0.1);

    let middle = Object::new_sphere()
        .with_material(
            Material::new()
                .with_color(Color::new(0.1, 1.0, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(-0.5, 1.0, 0.5);

    let middle2 = Object::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_stripe(vec![Color::green(), Color::red()]))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(-0.5, 0.5, 0.5);

    let right = Object::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_gradient(Color::blue(), Color::white()))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(1.5, 0.5, -0.5)
        .scale(0.5, 0.5, 0.5)
        .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    let right2 = Object::new_sphere()
        .with_material(
            Material::new()
                // .with_color(Color::new(0.5, 1.0, 0.1))
                .with_pattern(Pattern::new_gradient(Color::white(), Color::blue()))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(2.5, 0.0, 0.0)
        .rotate_y(2.0 * PI);

    let light1 = Light {
        intensity: Color::white(),
        position: Point::new(-10.0, 10.0, -10.0),
    };

    let light2 = Light {
        intensity: Color::red(),
        position: Point::new(-50.0, 10.0, -50.0),
    };

    let light3 = Light {
        intensity: Color::blue(),
        position: Point::new(30.0, 10.0, -30.0),
    };

    let world = World {
        objects: vec![left, middle, right2],
        lights: vec![light1],
    };

    let from = Point::new(0.0, 1.5, -5.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let factor = 10;

    let camera = Camera::new(100 * factor, 50 * factor, PI / 2.0)
        .with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.render(&world);

    canvas.export("./plane.png").unwrap();
}
