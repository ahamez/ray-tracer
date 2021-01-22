/* ---------------------------------------------------------------------------------------------- */

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

/* ---------------------------------------------------------------------------------------------- */

use clap::{App, AppSettings, Arg};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use ray_tracer::{
    io::{obj, yaml},
    primitive::{Point, Tuple, Vector},
    rtc::{
        view_transform, Camera, Color, Light, Material, Object, ParallelRendering, Pattern,
        Transform, World,
    },
};
use sha3::{Digest, Sha3_256};
use std::{
    f64::consts::PI,
    fs::File,
    io::{Read, Write},
    time::Instant,
};

/* ---------------------------------------------------------------------------------------------- */

fn output_path(path: &std::path::Path) -> Result<String, Box<dyn std::error::Error>> {
    let file_name = path
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or(format!("Can't get the file name of {:?}", path))?;

    let extension = path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or(format!("Can't get the extension of {:?}", path))?;

    let output_path = file_name
        .strip_suffix(extension)
        .map_or(file_name, |stripped| stripped);

    Ok(format!("./{}png", output_path))
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
                .help("The minimal number of shapes to create a sub group. Deactivates BVH if 0.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("aa-level")
                .long("aa-level")
                .value_name("INTEGER")
                .help("The antialiasing level. From 1 to 5. Default to 1.")
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
            Arg::with_name("sequential")
                .short("s")
                .long("sequential")
                .help("Deactivate parallel rendering")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("soft-shadows")
                .long("soft-shadows")
                .help("Use soft shadows (takes much more time)")
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
    let aa_level = clap::value_t!(matches.value_of("aa-level"), usize).unwrap_or(1);
    let fov = clap::value_t!(matches.value_of("fov"), f64).unwrap_or(1.0);
    let rotate_x = clap::value_t!(matches.value_of("rotate-x"), f64).unwrap_or(0.0);
    let rotate_y = clap::value_t!(matches.value_of("rotate-y"), f64).unwrap_or(0.0);
    let rotate_z = clap::value_t!(matches.value_of("rotate-z"), f64).unwrap_or(0.0);
    let parallel: ParallelRendering = matches.is_present("sequential").into();
    let soft_shadows = matches.is_present("soft-shadows");
    let path_str = matches.value_of("INPUT").expect("Invalid INPUT");

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
            let hash = Sha3_256::new()
                .chain(path_str)
                .chain(rotate_x.to_le_bytes())
                .chain(rotate_y.to_le_bytes())
                .chain(rotate_z.to_le_bytes())
                .chain(bvh_threshold.to_le_bytes())
                .finalize();

            let cache_path = format!(".rtc_{:x}.gz", hash);

            let group = if File::open(&cache_path).is_err() {
                let object = obj::parse_file(path)?
                    .rotate_x(rotate_x)
                    .rotate_y(rotate_y)
                    .rotate_z(rotate_z)
                    .transform();

                let bbox = object.bounding_box();
                // Translate the object to touch the floor at 0.0.
                let object = object.translate(0.0, -bbox.min().y(), 0.0).transform();

                let object = if bvh_threshold == 0 {
                    object
                } else {
                    object.divide(bvh_threshold)
                };

                println!("Writing cached object");

                let serialized = bincode::serialize(&object)?;
                let mut gz = GzEncoder::new(Vec::new(), Compression::default());
                gz.write_all(&serialized)?;
                let compressed = gz.finish()?;
                std::fs::write(&cache_path, &compressed)?;

                object
            } else {
                println!("Using cached object");

                let compressed = std::fs::read(&cache_path)?;
                let mut gz = GzDecoder::new(&compressed[..]);
                let mut serialized = vec![];
                gz.read_to_end(&mut serialized)?;
                bincode::deserialize(&serialized)?
            };

            let floor = Object::new_plane().with_material(
                Material::new()
                    .with_pattern(Pattern::new_checker(
                        Color::white(),
                        Color::new(0.5, 0.5, 0.5),
                    ))
                    .with_reflective(0.0),
            );

            let wall_left = Object::new_plane()
                .with_material(
                    Material::new()
                        .with_pattern(Pattern::new_checker(
                            Color::white(),
                            Color::new(0.5, 0.5, 0.5),
                        ))
                        .with_reflective(0.0),
                )
                .rotate_z(PI / 2.0)
                .translate(-7.0, 0.0, 0.0)
                .transform();

            let wall_right = Object::new_plane()
                .with_material(Material::new().with_pattern(Pattern::new_checker(
                    Color::white(),
                    Color::new(0.5, 0.5, 0.5),
                )))
                .rotate_x(PI / 2.0)
                .translate(0.0, 0.0, 7.0)
                .transform();

            let light = if soft_shadows {
                Light::new_area_light(
                    Color::new(0.9, 0.9, 0.9),
                    Point::new(-5.0, 25.0, -15.0),
                    Vector::new(2.0, 0.0, 0.0),
                    8,
                    Vector::new(2.0, 0.0, 0.0),
                    8,
                )
            } else {
                Light::new_point_light(Color::new(0.9, 0.9, 0.9), Point::new(-5.0, 25.0, -15.0))
            };

            let world = World::new()
                .with_objects(vec![group, wall_left, wall_right, floor])
                .with_lights(vec![light]);

            let from = Point::new(1.0, 1.0, -3.0);
            let to = Point::new(0.0, 1.0, 0.0);
            let up = Vector::new(0.0, 1.0, 0.0);

            let width = 100;
            let height = 100;

            let camera = Camera::new()
                .with_size(width, height)
                .with_fov(fov)
                .with_transformation(&view_transform(&from, &to, &up));

            (world, camera)
        }
        Some(_) => panic!(),
        None => panic!(),
    };

    let camera_h_size = camera.h_size();
    let camera_v_size = camera.v_size();

    let camera = camera.with_size(camera_h_size * factor, camera_v_size * factor);
    let construction_duration = construction_start.elapsed();

    println!("Time elapsed in construction: {:?}", construction_duration);

    let rendering_start = Instant::now();
    let canvas = camera.with_anti_aliasing(aa_level).render(&world, parallel);
    let rendering_duration = rendering_start.elapsed();
    println!("Time elapsed in rendering: {:?}", rendering_duration);

    canvas.export(&output_path(path)?)?;

    Ok(())
}

/* ---------------------------------------------------------------------------------------------- */
