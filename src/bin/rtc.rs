// --------------------------------------------------------------------------------------------- //

use ray_tracer::{rtc::Camera, scenes::default_scenes};

// --------------------------------------------------------------------------------------------- //

fn main() {
    let scene_name = "ch12_cube";
    let scenes = default_scenes();
    let scene = scenes.get(scene_name).unwrap();

    let width = 100;
    let height = 50;
    let factor = 20;

    let camera = Camera::new(width * factor, height * factor, scene.fov)
        .with_transformation(&scene.view_transform);

    let canvas = camera.par_render(&scene.world);

    canvas.export(&format!("./{}.png", scene_name)).unwrap();
}

// --------------------------------------------------------------------------------------------- //
