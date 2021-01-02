use ray_tracer::{
    primitive::{Point, Tuple, Vector},
    rtc::{
        view_transform, Camera, Color, GroupBuilder, Light, Material, Object, Pattern, Transform,
        World,
    },
};
use std::f64::consts::PI;
use std::sync::Arc;

fn main() {
    let floor = Arc::new(
        Object::new_plane().with_material(
            Material::new()
                .with_pattern(Pattern::new_checker(
                    Color::white(),
                    Color::new(0.5, 0.5, 0.5),
                ))
                .with_reflective(0.0),
        ),
    );

    let t1 = Object::new_triangle(
        Point::new(0.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 0.0),
        Point::new(0.5, 1.0, 0.5),
    )
    .with_material(Material::new().with_color(Color::red()));

    let t2 = Object::new_triangle(
        Point::new(1.0, 0.0, 0.0),
        Point::new(1.0, 0.0, 1.0),
        Point::new(0.5, 1.0, 0.5),
    )
    .with_material(Material::new().with_color(Color::red()));

    let t3 = Object::new_triangle(
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.0, 0.0, 1.0),
        Point::new(0.5, 1.0, 0.5),
    )
    .with_material(Material::new().with_color(Color::green()));

    let t4 = Object::new_triangle(
        Point::new(0.0, 0.0, 1.0),
        Point::new(1.0, 0.0, 1.0),
        Point::new(0.5, 1.0, 0.5),
    )
    .with_material(Material::new().with_color(Color::blue()));

    let pyramid_group_builder = GroupBuilder::Node(
        Object::new_dummy()
            .scale(2.0, 2.0, 2.0)
            .rotate_y(PI / 5.0)
            .translate(-1.5, 0.0, 0.0),
        vec![
            GroupBuilder::Leaf(t1),
            GroupBuilder::Leaf(t2),
            GroupBuilder::Leaf(t3),
            GroupBuilder::Leaf(t4),
        ],
    );
    let pyramid_group = Arc::new(Object::new_group(&pyramid_group_builder));

    let cube = Arc::new(
        Object::new_cube()
            .with_material(
                Material::new()
                    .with_reflective(1.0)
                    .with_ambient(0.0)
                    .with_diffuse(0.3)
                    .with_specular(0.1)
                    .with_shininess(100.0),
            )
            .scale(100.0, 100.0, 0.00001)
            .translate(0.0, 1.0, 4.0),
    );

    let light = Light::new_point_light(Color::white(), Point::new(-20.0, 6.0, -7.0));

    let world = World::new()
        .with_objects(vec![floor, pyramid_group, cube])
        .with_lights(vec![light]);

    let from = Point::new(3.0, 3.0, -6.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let width = 5000;
    let height = 5000;
    let fov = PI / 4.5;

    let camera =
        Camera::new(width, height, fov).with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.render(&world, true);
    canvas.export("ch15_triangle.png").unwrap();
}
