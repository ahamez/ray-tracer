/* ---------------------------------------------------------------------------------------------- */

use clap::{App, Arg};
use ray_tracer::io::yaml;

/* ---------------------------------------------------------------------------------------------- */

fn output_path(path_str: &str) -> String {
    let path = std::path::Path::new(&path_str);
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
            Arg::with_name("sequential")
                .short("s")
                .long("sequential")
                .help("Deactivate parallel rendering")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input YAML file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let factor = clap::value_t!(matches.value_of("factor"), usize).unwrap_or(1);
    let sequential = matches.is_present("sequential");
    let path_str = matches.value_of("INPUT").unwrap();
    let path = std::path::Path::new(&path_str);

    println!("Using factor: {}", factor);
    println!("Using input file: {}", path_str);
    println!("Parallel rendering: {}", !sequential);

    let (world, camera) = yaml::parse(&path, factor);
    let canvas = camera.render(&world, !sequential);

    println!("Computed intersections: {}", world.nb_intersections());

    canvas.export(&output_path(&path)).unwrap();
}

/* ---------------------------------------------------------------------------------------------- */
