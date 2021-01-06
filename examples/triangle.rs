use ray_tracer::{
    primitive::{Point, Tuple, Vector},
    rtc::{
        view_transform, Camera, Color, Light, Material, Object, ParallelRendering, Pattern,
        Transform, World,
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

    let pyramid = Arc::new(
        Object::new_group(vec![Arc::new(t1), Arc::new(t2), Arc::new(t3), Arc::new(t4)])
            .scale(2.0, 2.0, 2.0)
            .rotate_y(PI / 5.0)
            .translate(-1.5, 0.0, 0.0),
    );

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
        .with_objects(vec![floor, pyramid, cube])
        .with_lights(vec![light]);

    let from = Point::new(3.0, 3.0, -6.0);
    let to = Point::new(0.0, 1.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);

    let width = 5000;
    let height = 5000;
    let fov = PI / 4.5;

    let camera = Camera::new()
        .with_size(width, height)
        .with_fov(fov)
        .with_transformation(&view_transform(&from, &to, &up));

    let canvas = camera.render(&world, ParallelRendering::True);
    canvas.export("ch15_triangle.png").unwrap();
}
