// --------------------------------------------------------------------------------------------- //

use std::{f64::consts::PI, rc::Rc, sync::Arc};

use crate::{
    primitive::{Point, Tuple, Vector},
    rtc::{view_transform, Color, Light, Material, Object, Pattern, Scene, Transform, World},
};

// --------------------------------------------------------------------------------------------- //

pub fn make_scene() -> Rc<Scene> {
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
            .translate(-15.0, 0.0, 0.0)
            .rotate_z(PI / 2.0),
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
            .translate(0.0, 0.0, 15.0)
            .rotate_x(PI / 2.0),
    );

    let cylinder_x = Arc::new(
        Object::new_cylinder()
            .with_material(
                Material::new()
                    .with_color(Color::red())
                    .with_diffuse(0.7)
                    .with_specular(0.5)
                    .with_reflective(0.1),
            )
            .translate(0.0, 0.0, 0.0)
            .rotate_z(PI / 2.0),
    );

    let cylinder_y = Arc::new(
        Object::new_cylinder()
            .with_material(
                Material::new()
                    .with_color(Color::blue())
                    .with_diffuse(0.7)
                    .with_specular(0.5)
                    .with_reflective(0.1),
            )
            .translate(0.0, 0.0, 0.0),
    );

    let cylinder_z = Arc::new(
        Object::new_cylinder()
            .with_material(
                Material::new()
                    .with_color(Color::green())
                    .with_diffuse(0.7)
                    .with_specular(0.5)
                    .with_reflective(0.1),
            )
            .translate(0.0, 0.0, 0.0)
            .rotate_x(PI / 2.0),
    );

    let light = Light {
        intensity: Color::white(),
        position: Point::new(-5.0, 10.0, -10.0),
    };

    let world = World::new()
        .with_objects(vec![
            wall_left, wall_right, cylinder_x, cylinder_y, cylinder_z,
        ])
        .with_lights(vec![light])
        .with_recursion_limit(5);

    let from = Point::new(5.0, 2.5, -10.5);
    let to = Point::new(1.5, 3.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    Rc::new(Scene {
        world,
        view_transform: view_transform(&from, &to, &up),
        fov: PI / 1.5,
    })
}
