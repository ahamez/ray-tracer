/* ---------------------------------------------------------------------------------------------- */

use clap::{App, AppSettings, Arg};
use ray_tracer::{
    io::{obj, yaml},
    primitive::{Point, Tuple, Vector},
    rtc::{view_transform, Camera, Color, Light, ParallelRendering, Transform, World},
};
use std::{f64::consts::PI, sync::Arc, time::Instant};

/* ---------------------------------------------------------------------------------------------- */

fn output_path(path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    let file_name = path
        .file_name()
        .and_then(|p| p.to_str())
        .ok_or(format!("Can't get the file name of {:?}", path))?;

    let extension = path
        .extension()
        .and_then(|p| p.to_str())
        .ok_or(format!("Can't get the extension of {:?}", path))?;

    let path = if let Some(stripped) = file_name.strip_suffix(extension) {
        stripped
    } else {
        file_name
    };

    Ok(format!("./{}png", path))
}

/* ---------------------------------------------------------------------------------------------- */

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Ray Tracer")
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(
            Arg::with_name("factor")
                .short("f")
                .long("factor")
                .value_name("INTEGER")
                .help("Sets a factor to apply on width/height")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("bvh-threshold")
                .long("bvh-threshold")
                .value_name("INTEGER")
                .help("The minimal number of shapes to create a sub group. Deactivates BVH is 0.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fov")
                .long("fov")
                .value_name("FLOAT")
                .help("Sets the field of view")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rotate-x")
                .long("rotate-x")
                .value_name("FLOAT")
                .help("Rotate along the X axis")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rotate-y")
                .long("rotate-y")
                .value_name("FLOAT")
                .help("Rotate along the Y axis")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rotate-z")
                .long("rotate-z")
                .value_name("FLOAT")
                .help("Rotate along the Z axis")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("translate-x")
                .long("translate-x")
                .value_name("FLOAT")
                .help("Rotate along the X axis")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("translate-y")
                .long("translate-y")
                .value_name("FLOAT")
                .help("Rotate along the Y axis")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("translate-z")
                .long("translate-z")
                .value_name("FLOAT")
                .help("Rotate along the Z axis")
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
    let bvh_threshold = clap::value_t!(matches.value_of("bvh-threshold"), usize).unwrap_or(4);
    let fov = clap::value_t!(matches.value_of("fov"), f64).unwrap_or(PI / 2.0);
    let rotate_x = clap::value_t!(matches.value_of("rotate-x"), f64).unwrap_or(0.0);
    let rotate_y = clap::value_t!(matches.value_of("rotate-y"), f64).unwrap_or(0.0);
    let rotate_z = clap::value_t!(matches.value_of("rotate-z"), f64).unwrap_or(0.0);
    let translate_x = clap::value_t!(matches.value_of("translate-x"), f64).unwrap_or(0.0);
    let translate_y = clap::value_t!(matches.value_of("translate-y"), f64).unwrap_or(0.0);
    let translate_z = clap::value_t!(matches.value_of("translate-z"), f64).unwrap_or(0.0);
    let parallel: ParallelRendering = matches.is_present("sequential").into();
    let path_str = matches.value_of("INPUT").unwrap();

    let path = std::path::Path::new(&path_str);
    let ext = match path.extension() {
        Some(ext) => ext,
        None => todo!(),
    };

    println!("Input file: {}", path_str);
    println!("Factor: {}", factor);
    println!("BVH: {}", bvh_threshold != 0);
    println!("FoV: {}", fov);
    println!("Parallel rendering: {}", parallel);

    let construction_start = Instant::now();
    let (world, camera) = match ext.to_str() {
        Some("yml") => yaml::parse(&path),
        Some("obj") => {
            let object = obj::parse_file(&path)?
                .rotate_x(rotate_x)
                .rotate_y(rotate_y)
                .rotate_z(rotate_z)
                .translate(translate_x, translate_y, translate_z)
                .transform();

            let object = if bvh_threshold != 0 {
                object.divide(bvh_threshold)
            } else {
                object
            };

            let group = Arc::new(object);

            let light = Light::new_point_light(Color::white(), Point::new(20.0, -0.0, 40.0));

            let world = World::new()
                .with_objects(vec![group])
                .with_lights(vec![light]);

            let from = Point::new(20.0, -10.0, 40.0);
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
    let construction_duration = construction_start.elapsed();

    println!("Time elapsed in construction: {:?}", construction_duration);

    let rendering_start = Instant::now();
    let canvas = camera.render(&world, parallel);
    let rendering_duration = rendering_start.elapsed();
    println!("Time elapsed in rendering: {:?}", rendering_duration);

    println!("Computed intersections: {}", world.nb_intersections());

    canvas.export(&output_path(&path)?)?;

    Ok(())
}

/* ---------------------------------------------------------------------------------------------- */
