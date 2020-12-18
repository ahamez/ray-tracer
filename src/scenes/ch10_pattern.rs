// --------------------------------------------------------------------------------------------- //

use std::{f64::consts::PI, rc::Rc, sync::Arc};

use crate::{
    primitive::{Point, Tuple, Vector},
    rtc::{view_transform, Color, Light, Material, Object, Pattern, Scene, Transform, World},
};

// --------------------------------------------------------------------------------------------- //

pub fn make_scene() -> Rc<Scene> {
    let floor = Arc::new(
        Object::new_plane().with_material(Material::new().with_pattern(Pattern::new_checker(
            Color::white(),
            Color::new(0.5, 0.5, 0.5),
        ))),
    );

    let wall = Arc::new(
        Object::new_plane()
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
            .rotate_x(PI / 2.0),
    );

    let sphere = Arc::new(
        Object::new_sphere()
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
            .scale(1.5, 1.5, 1.5),
    );

    let sphere2 = Arc::new(
        Object::new_sphere()
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
            .scale(1.5, 1.5, 1.5),
    );

    let sphere3 = Arc::new(
        Object::new_sphere()
            .with_material(
                Material::new()
                    .with_pattern(Pattern::new_gradient(
                        Color::new(0.7, 0.6, 0.7),
                        Color::black(),
                    ))
                    .with_diffuse(0.7)
                    .with_specular(0.3),
            )
            .translate(0.0, 1.0, -7.0)
            .scale(0.33, 0.33, 0.33),
    );

    let light = Light {
        intensity: Color::white(),
        position: Point::new(-10.0, 10.0, -10.0),
    };

    let world = World::new()
        .with_objects(vec![floor, wall, sphere, sphere2, sphere3])
        .with_lights(vec![light]);

    let from = Point::new(-1.0, 2.0, -9.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    Rc::new(Scene {
        world,
        view_transform: view_transform(&from, &to, &up),
        fov: PI / 1.5,
    })
}
