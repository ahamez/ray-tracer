// --------------------------------------------------------------------------------------------- //

use ray_tracer::{
    canvas::Canvas, color::Color, light::Light, material::Material, point::Point, ray::Ray,
    shape::Shape, transformation::Transform, tuple::Tuple,
};

// --------------------------------------------------------------------------------------------- //

fn main() {
    const CANVAS_PIXELS: usize = 1000;

    let mut canvas = Canvas::new_with_color(CANVAS_PIXELS, CANVAS_PIXELS, Color::black());

    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / CANVAS_PIXELS as f64;
    let half = wall_size / 2.0;

    let material = Material::new().with_color(Color::new(1.0, 0.2, 1.0));

    let shape = Shape::new_sphere()
        .with_material(material)
        // .rotate_z(3.14 / 4.0)
        // .rotate_y(3.14 / 2.0)
        .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        .scale(1.0, 0.5, 1.0)
        .scale(0.7, 1.0, 1.0)
        // .translate(0.0, 0.0, 1.0)
        ;

    let light_position = Point::new(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = Light::new(light_color, light_position);

    for y in 0..CANVAS_PIXELS {
        let target_y = half - pixel_size * y as f64;

        for x in 0..CANVAS_PIXELS {
            let target_x = -half + pixel_size * x as f64;
            let target = Point::new(target_x, target_y, wall_z);

            let direction = (target - ray_origin).normalize();
            let ray = Ray {
                origin: ray_origin,
                direction,
            };

            let intersections = ray.intersects(&[shape.clone()]);
            if let Some(hit) = intersections.hit() {
                let point = ray.position(hit.t);
                let normal_v = hit.shape.normal_at(&point);
                let eye_v = -ray.direction;
                let color = hit
                    .shape
                    .material()
                    .lighting(&Shape::new_sphere(), &light, &point, &eye_v, &normal_v, false);

                canvas[x][y] = color;
            }
        }
    }

    canvas
        .export("./sphere_projection_with_lighting.png")
        .unwrap();
}
