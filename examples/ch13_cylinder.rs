/* ---------------------------------------------------------------------------------------------- */

use ray_tracer::{
    primitive::{Point, Tuple, Vector},
    rtc::{
        view_transform, Camera, Color, Light, Material, Object, ParallelRendering, Pattern,
        Transform, World,
    },
};
use std::{f64::consts::PI, sync::Arc};

/* ---------------------------------------------------------------------------------------------- */

fn main() {
    let wall_left = Arc::new(
        Object::new_plane()
            .with_material(
                Material::new()
                    .with_pattern(Pattern::new_checker(
                        Color::white(),
                        Color::new(0.5, 0.5, 0.5),
                    ))
                    .with_reflective(0.0),
            )
            .rotate_z(PI / 2.0)
            .translate(-15.0, 0.0, 0.0)
            .transform(),
    );

    let wall_right = Arc::new(
        Object::new_plane()
            .with_material(
                Material::new()
                    .with_pattern(Pattern::new_checker(
                        Color::white(),
                        Color::new(0.5, 0.5, 0.5),
                    ))
                    .with_reflective(0.0),
            )
            .rotate_x(PI / 2.0)
            .translate(0.0, 0.0, 15.0)
            .transform(),
    );

    let cylinder_x = Arc::new(
        Object::new_cylinder(f64::NEG_INFINITY, f64::INFINITY, true)
            .with_material(
                Material::new()
                    .with_color(Color::red())
                    .with_diffuse(0.7)
                    .with_specular(0.5)
                    .with_reflective(0.1),
            )
            .rotate_z(PI / 2.0)
            .translate(0.0, 0.0, 0.0)
            .transform(),
    );

    let cylinder_y = Arc::new(
        Object::new_cylinder(f64::NEG_INFINITY, f64::INFINITY, true)
            .with_material(
                Material::new()
                    .with_color(Color::blue())
                    .with_diffuse(0.7)
                    .with_specular(0.5)
                    .with_reflective(0.1),
            )
            .translate(0.0, 0.0, 0.0)
            .transform(),
    );

    let cylinder_z = Arc::new(
        Object::new_cylinder(f64::NEG_INFINITY, f64::INFINITY, true)
            .with_material(
                Material::new()
                    .with_color(Color::green())
                    .with_diffuse(0.7)
                    .with_specular(0.5)
                    .with_reflective(0.1),
            )
            .rotate_x(PI / 2.0)
            .translate(0.0, 0.0, 0.0)
            .transform(),
    );

    let shallow_cylinder = Arc::new(
        Object::new_cylinder(-2.0, 2.0, false)
            .with_material(
                Material::new()
                    .with_pattern(Pattern::new_checker(Color::white(), Color::red()))
                    .with_diffuse(0.3)
                    .with_specular(0.2)
                    .with_reflective(0.00),
            )
            .rotate_x(PI / 2.0)
            .translate(-3.0, 3.0, -4.0)
            .transform(),
    );

    let cylinder = Arc::new(
        Object::new_cylinder(-2.0, 2.0, true)
            .with_material(
                Material::new()
                    .with_pattern(Pattern::new_checker(Color::white(), Color::red()))
                    .with_diffuse(0.3)
                    .with_specular(0.2)
                    .with_reflective(0.00),
            )
            .rotate_z(PI / 2.0)
            .translate(-3.0, 6.0, -4.0)
            .transform(),
    );

    let refractive_cylinder = Arc::new(
        Object::new_cylinder(-2.0, 2.0, true)
            .with_material(
                Material::new()
                    .with_color(Color::new(0.1, 0.1, 0.1))
                    .with_diffuse(0.3)
                    .with_specular(0.2)
                    .with_reflective(0.00)
                    .with_transparency(1.0)
                    .with_refractive_index(1.5),
            )
            .rotate_z(PI / 2.0)
            .translate(5.0, 2.0, -4.0)
            .transform(),
    );

    let light = Light::new_point_light(Color::white(), Point::new(-5.0, 10.0, -10.0));

    let world = World::new()
        .with_objects(vec![
            wall_left,
            wall_right,
            cylinder_x,
            cylinder_y,
            cylinder_z,
            shallow_cylinder,
            cylinder,
            refractive_cylinder,
        ])
        .with_lights(vec![light])
        .with_recursion_limit(5);

    let from = Point::new(5.0, 2.5, -10.5);
    let to = Point::new(1.5, 3.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let width = 5000;
    let height = 2500;
    let fov = PI / 1.5;

    let camera = Camera::new()
        .with_size(width, height)
        .with_fov(fov)
        .with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.render(&world, ParallelRendering::True);
    canvas.export("ch13_cylinder.png").unwrap();
}
