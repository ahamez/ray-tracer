// --------------------------------------------------------------------------------------------- //

use std::{f64::consts::PI, rc::Rc, sync::Arc};

use crate::{
    primitive::{Point, Tuple, Vector},
    rtc::{view_transform, Color, Light, Material, Object, Pattern, Scene, Transform, World},
};

// --------------------------------------------------------------------------------------------- //

pub fn make_scene() -> Rc<Scene> {
    let floor = Arc::new(
        Object::new_plane().with_material(
            Material::new()
                .with_pattern(Pattern::new_checker(
                    Color::white(),
                    Color::new(0.5, 0.5, 0.5),
                ))
                .with_reflective(0.3),
        ),
    );

    let sphere = Arc::new(
        Object::new_sphere()
            .with_material(
                Material::new()
                    .with_pattern(Pattern::new_gradient(Color::blue(), Color::black()))
                    .with_diffuse(0.7)
                    .with_specular(0.3)
                    .with_reflective(1.0),
            )
            .translate(-1.3, 1.5, -4.0),
    );

    let transparent_sphere = Arc::new(
        Object::new_sphere()
            .with_material(
                Material::new()
                    .with_diffuse(0.7)
                    .with_specular(0.3)
                    .with_transparency(0.5),
            )
            .translate(0.0, 2.0, -6.0),
    );

    let light = Light::new_point_light(Color::white(), Point::new(-5.0, 10.0, -10.0));

    let world = World::new()
        .with_objects(vec![floor, sphere, transparent_sphere])
        .with_lights(vec![light])
        .with_recursion_limit(5);

    let from = Point::new(-1.0, 2.0, -9.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    Rc::new(Scene {
        world,
        view_transform: view_transform(&from, &to, &up),
        fov: PI / 1.5,
    })
}
