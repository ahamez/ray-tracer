// --------------------------------------------------------------------------------------------- //

#![allow(unused_variables)]

use core::f64::consts::PI;

use ray_tracer::{
    camera::Camera,
    color::Color,
    light::Light,
    material::Material,
    object::Object,
    point::Point,
    sphere::Sphere,
    transformation::{view_transform, Transform},
    tuple::Tuple,
    vector::Vector,
    world::World,
};

// --------------------------------------------------------------------------------------------- //

fn main() {
    let material = Material::new()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let floor = Sphere::new()
        .with_material(material)
        .scale(10.0, 0.01, 10.0);

    let left_wall = Sphere::new()
        .with_material(material)
        .translate(0.0, 0.0, 5.0)
        .rotate_y(-PI / 4.0)
        .rotate_x(PI / 2.0)
        .scale(10.0, 0.01, 10.0);

    let right_wall = Sphere::new()
        .with_material(material)
        .translate(0.0, 0.0, 5.0)
        .rotate_y(PI / 4.0)
        .rotate_x(PI / 2.0)
        .scale(10.0, 0.01, 10.0);

    let left = Sphere::new()
        .with_material(
            Material::new()
                .with_color(Color::new(1.0, 0.8, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(-1.5, 0.33, -0.75)
        .scale(0.33, 0.33, 0.33);

    let middle = Sphere::new()
        .with_material(
            Material::new()
                .with_color(Color::new(0.1, 1.0, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(-0.5, 1.0, 0.5);

    let middle2 = Sphere::new()
        .with_material(
            Material::new()
                .with_color(Color::new(0.1, 1.0, 0.5))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(-0.5, 0.5, 0.5);

    let right = Sphere::new()
        .with_material(
            Material::new()
                .with_color(Color::new(0.5, 1.0, 0.1))
                .with_diffuse(0.7)
                .with_specular(0.3),
        )
        .translate(1.5, 0.5, -0.5)
        .scale(0.5, 0.5, 0.5)
        .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);

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
        objects: vec![
            Object::Sphere(floor),
            Object::Sphere(left_wall),
            Object::Sphere(right_wall),
            Object::Sphere(left),
            Object::Sphere(middle),
            Object::Sphere(middle2),
            Object::Sphere(right),
        ],
        lights: vec![light1, light3],
    };

    let from = Point::new(0.0, 1.5, -5.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let factor = 20;

    let camera = Camera::new(100 * factor, 50 * factor, PI / 3.0)
        .with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.render(&world);

    canvas.export("./camera.png").unwrap();
}
