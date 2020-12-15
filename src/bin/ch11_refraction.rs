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
            .with_reflective(0.0),
    );

    let wall_left = Object::new_plane()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_checker(
                    Color::white(),
                    Color::new(0.5, 0.5, 0.5),
                ))
                .with_reflective(0.0),
        )
        .translate(-15.0, 0.0, 0.0)
        .rotate_z(PI / 2.0);

    let wall_right = Object::new_plane()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_checker(
                    Color::white(),
                    Color::new(0.5, 0.5, 0.5),
                ))
                .with_reflective(0.0),
        )
        .translate(0.0, 0.0, 15.0)
        .rotate_x(PI / 2.0);

    let refractive_sphere = Object::new_sphere()
        .with_material(
            Material::new()
                .with_color(Color::new(0.1, 0.1, 0.1))
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        )
        .translate(0.0, 1.0, 0.0);

    let blue_ball = Object::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_gradient(Color::blue(), Color::black()))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(0.2),
        )
        .translate(-8.0, 1.0, 5.0);

    let red_ball = Object::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_gradient(Color::red(), Color::black()))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(0.2),
        )
        .translate(-1.0, 0.5, 5.0)
        .scale(0.5, 0.5, 0.5);

    let green_ball = Object::new_sphere()
        .with_material(
            Material::new()
                .with_pattern(Pattern::new_gradient(Color::green(), Color::black()))
                .with_diffuse(0.7)
                .with_specular(0.3)
                .with_reflective(0.2),
        )
        .translate(-2.3, 0.5, 0.77)
        .scale(0.5, 0.5, 0.5);

    let light = Light {
        intensity: Color::white(),
        position: Point::new(-5.0, 10.0, -10.0),
    };

    let world = World {
        objects: vec![
            floor,
            wall_left,
            wall_right,
            refractive_sphere,
            blue_ball,
            red_ball,
            green_ball,
        ],
        lights: vec![light],
        recursion_limit: 5,
    };

    let from = Point::new(5.0, 1.5, -5.5);
    let to = Point::new(0.0, 0.7, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let factor = 100;

    let camera = Camera::new(100 * factor, 50 * factor, PI / 3.0)
        .with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.par_render(&world);

    canvas.export("./refraction.png").unwrap();
}
