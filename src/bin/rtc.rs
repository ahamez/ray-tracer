/* ---------------------------------------------------------------------------------------------- */

use clap::{App, Arg};
use ray_tracer::{
    io::{obj, yaml},
    primitive::{Point, Tuple, Vector},
    rtc::{view_transform, Camera, Color, Light, ParallelRendering, World},
};
use std::{f64::consts::PI, sync::Arc};

/* ---------------------------------------------------------------------------------------------- */

fn output_path(path: &std::path::Path) -> String {
    let file_name = path.file_name().and_then(|p| p.to_str()).unwrap();
    let extension = path.extension().and_then(|p| p.to_str()).unwrap();

    let path = if let Some(stripped) = file_name.strip_suffix(extension) {
        stripped
    } else {
        file_name
    };

    format!("./{}png", path)
}

/* ---------------------------------------------------------------------------------------------- */

fn main() {
    let matches = App::new("Ray Tracer")
        .arg(
            Arg::with_name("factor")
                .short("f")
                .long("factor")
                .value_name("FACTOR")
                .help("Sets a factor to apply on width/height")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fov")
                .long("fov")
                .value_name("FOV")
                .help("Sets the field of view")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sequential")
                .short("s")
                .long("sequential")
                .help("Deactivate parallel rendering")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input YAML or OBJ file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let factor = clap::value_t!(matches.value_of("factor"), usize).unwrap_or(1);
    let fov = clap::value_t!(matches.value_of("fov"), f64).unwrap_or(PI / 2.0);
    let parallel = if matches.is_present("sequential") {
        ParallelRendering::False
    } else {
        ParallelRendering::True
    };
    let path_str = matches.value_of("INPUT").unwrap();

    let path = std::path::Path::new(&path_str);
    let ext = match path.extension() {
        Some(ext) => ext,
        None => todo!(),
    };

    println!("Factor: {}", factor);
    println!("FoV: {}", fov);
    println!("Input file: {}", path_str);
    println!("Parallel rendering: {}", parallel);

    let (world, camera) = match ext.to_str() {
        Some("yml") => yaml::parse(&path),
        Some("obj") => {
            let group = Arc::new(obj::parse_file(&path).unwrap());

            let light = Light::new_point_light(Color::white(), Point::new(-5.0, 10.0, -10.0));

            let world = World::new()
                .with_objects(vec![group])
                .with_lights(vec![light]);

            let from = Point::new(0.0, 1.5, -5.0);
            let to = Point::new(0.0, 1.0, 0.0);
            let up = Vector::new(0.0, 1.0, 0.0);

            let width = 500;
            let height = 500;

            let camera = Camera::new()
                .with_size(width, height)
                .with_fov(fov)
                .with_transformation(&view_transform(&from, &to, &up));

            (world, camera)
        }
        Some(_) => panic!(),
        None => panic!(),
    };

    let camera = camera.with_size(camera.h_size() * factor, camera.v_size() * factor);
    let canvas = camera.render(&world, parallel);

    println!("Computed intersections: {}", world.nb_intersections());

    canvas.export(&output_path(&path)).unwrap();
}

/* ---------------------------------------------------------------------------------------------- */
