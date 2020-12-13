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
        // Object::new_plane().with_material(Material::new().with_pattern(Pattern::new_stripe(vec![
        //     Color::new(0.5, 0.5, 0.5),
        //     Color::white(),
        //     Color::new(0.7, 0.6, 0.7),
        // ])));

        Object::new_plane().with_material(Material::new().
            with_pattern(Pattern::new_checker(Color::white(), Color::new(0.5, 0.5, 0.5))));

    let wall = Object::new_plane()
        .with_material(
            Material::new().with_pattern(
                Pattern::new_ring(vec![
                    Color::new(0.5, 0.5, 0.5),
                    Color::white(),
                    Color::new(0.7, 0.6, 0.7),
                ])
                .shear(1.0, 1.0, 0.0, 0.0, 0.0, 0.0),
            ),
        )
        .translate(0.0, 0.0, 5.0)
        .rotate_x(PI / 2.0);

    let sphere = Object::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(
                    Pattern::new_ring(vec![
                        Color::new(0.5, 0.5, 0.5),
                        Color::white(),
                        Color::black(),
                        Color::new(0.7, 0.6, 0.7),
                    ])
                    .scale(0.2, 0.2, 0.2),
                )
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(-3.0, 1.5, -4.0)
        .scale(1.5, 1.5, 1.5);

    let sphere2 = Object::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_stripe(vec![
                    Color::new(0.5, 0.5, 0.5),
                    Color::white(),
                    Color::new(0.7, 0.6, 0.7),
                ]))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(3.0, 1.5, -4.0)
        .scale(1.5, 1.5, 1.5);

    let sphere3 = Object::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_gradient(
                    Color::new(0.5, 0.5, 0.5),
                    Color::new(0.7, 0.6, 0.7),
                ))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(0.0, 1.0, -7.0)
        .scale(0.33, 0.33, 0.33);

    let light1 = Light {
        intensity: Color::white(),
        position: Point::new(-10.0, 10.0, -10.0),
    };

    let world = World {
        objects: vec![floor, wall, sphere, sphere2, sphere3],
        lights: vec![light1],
    };

    let from = Point::new(-2.0, 1.5, -8.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let factor = 30;

    let camera = Camera::new(100 * factor, 50 * factor, PI / 1.5)
        .with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.render(&world);

    canvas.export("./pattern.png").unwrap();
}
