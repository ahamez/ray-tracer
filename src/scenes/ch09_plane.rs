// --------------------------------------------------------------------------------------------- //

use std::{f64::consts::PI, rc::Rc, sync::Arc};

use crate::{
    primitive::{Point, Tuple, Vector},
    rtc::{view_transform, Color, Light, Material, Object, Scene, Transform, World},
};

// --------------------------------------------------------------------------------------------- //

pub fn make_scene() -> Rc<Scene> {
    let material = Material::new()
        .with_color(Color::new(1.0, 0.9, 0.9))
        .with_specular(0.0);

    let floor = Arc::new(Object::new_plane().with_material(material));

    let left = Arc::new(
        Object::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(1.0, 0.8, 0.1))
                    .with_diffuse(0.7)
                    .with_specular(0.3),
            )
            .translate(-1.5, 0.33, -0.75)
            .scale(0.33, 0.33, 0.33),
    );

    let middle = Arc::new(
        Object::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(0.1, 1.0, 0.5))
                    .with_diffuse(0.7)
                    .with_specular(0.3),
            )
            .translate(-0.5, 1.0, 0.5),
    );

    let middle2 = Arc::new(
        Object::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(0.1, 1.0, 0.5))
                    .with_diffuse(0.7)
                    .with_specular(0.3),
            )
            .translate(-0.5, 0.5, 0.5),
    );

    let right = Arc::new(
        Object::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(0.5, 1.0, 0.1))
                    .with_diffuse(0.7)
                    .with_specular(0.3),
            )
            .translate(1.5, 0.5, -0.5)
            .rotate_y(PI / 4.0)
            .scale(0.5, 0.5, 0.5)
            .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    );

    let light1 = Light {
        intensity: Color::white(),
        position: Point::new(-10.0, 10.0, -10.0),
    };

    let light2 = Light {
        intensity: Color::new(0.2, 0.2, 0.2),
        position: Point::new(-50.0, 10.0, -50.0),
    };

    let light3 = Light {
        intensity: Color::new(0.2, 0.2, 0.2),
        position: Point::new(30.0, 10.0, -30.0),
    };

    let world = World::new()
        .with_objects(vec![floor, left, middle, middle2, right])
        .with_lights(vec![light1, light2, light3]);

    let from = Point::new(0.0, 1.5, -5.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    Rc::new(Scene {
        world,
        view_transform: view_transform(&from, &to, &up),
        fov: PI / 2.0,
    })
}
