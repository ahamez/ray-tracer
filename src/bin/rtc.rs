#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

// --------------------------------------------------------------------------------------------- //

use std::sync::Arc;

#[macro_use]
extern crate clap;

use clap::{App, Arg};
use yaml_rust::{Yaml, YamlLoader};

use ray_tracer::{
    primitive::{Matrix, Point, Tuple, Vector},
    rtc::{
        rotation_x, rotation_y, rotation_z, scaling, shearing, translation, view_transform, Camera,
        Color, Light, Material, Object, Pattern, Scene, Transform, World,
    },
};

// --------------------------------------------------------------------------------------------- //

fn mk_bool(yaml: &Yaml) -> bool {
    match yaml.as_bool() {
        None => panic!("Expected boolean, got: {:?}", yaml),
        Some(value) => value,
    }
}

// --------------------------------------------------------------------------------------------- //

fn mk_bool_from_key(hash: &yaml_rust::yaml::Hash, key: &str) -> Option<bool> {
    match hash.get(&Yaml::from_str(key)) {
        None => None,
        Some(x) => Some(mk_bool(x)),
    }
}

// --------------------------------------------------------------------------------------------- //

fn mk_usize(yaml: &Yaml) -> usize {
    match yaml.as_i64() {
        None => panic!("Expected integer, got: {:?}", yaml),
        Some(value) => value as usize,
    }
}

// --------------------------------------------------------------------------------------------- //

fn mk_usize_from_key(hash: &yaml_rust::yaml::Hash, key: &str) -> Option<usize> {
    match hash.get(&Yaml::from_str(key)) {
        None => None,
        Some(x) => Some(mk_usize(x)),
    }
}

// --------------------------------------------------------------------------------------------- //

fn mk_f64(yaml: &Yaml) -> f64 {
    match yaml.as_f64() {
        None => match yaml.as_i64() {
            None => panic!("Expected a scalar, got: {:?}", yaml),
            Some(value) => value as f64,
        },
        Some(value) => value,
    }
}

// --------------------------------------------------------------------------------------------- //

fn mk_f64_from_key(hash: &yaml_rust::yaml::Hash, key: &str) -> Option<f64> {
    match hash.get(&Yaml::from_str(key)) {
        None => None,
        Some(x) => Some(mk_f64(x)),
    }
}

// --------------------------------------------------------------------------------------------- //

fn mk_color(yaml: &Yaml) -> Color {
    let rgb = yaml.as_vec().unwrap();
    assert_eq!(rgb.len(), 3);

    Color::new(mk_f64(&rgb[0]), mk_f64(&rgb[1]), mk_f64(&rgb[2]))
}

// --------------------------------------------------------------------------------------------- //

fn mk_color_from_key(hash: &yaml_rust::yaml::Hash, key: &str) -> Option<Color> {
    match hash.get(&Yaml::from_str(key)) {
        None => None,
        Some(x) => Some(mk_color(x)),
    }
}

// --------------------------------------------------------------------------------------------- //

fn mk_point(yaml: &Yaml) -> Point {
    let xyz = yaml.as_vec().unwrap();
    assert_eq!(xyz.len(), 3);

    Point::new(mk_f64(&xyz[0]), mk_f64(&xyz[1]), mk_f64(&xyz[2]))
}

// --------------------------------------------------------------------------------------------- //

fn mk_point_from_key(hash: &yaml_rust::yaml::Hash, key: &str) -> Option<Point> {
    match hash.get(&Yaml::from_str(key)) {
        None => None,
        Some(x) => Some(mk_point(x)),
    }
}

// --------------------------------------------------------------------------------------------- //

fn mk_vector(yaml: &Yaml) -> Vector {
    let xyz = yaml.as_vec().unwrap();
    assert_eq!(xyz.len(), 3);

    Vector::new(mk_f64(&xyz[0]), mk_f64(&xyz[1]), mk_f64(&xyz[2]))
}

// --------------------------------------------------------------------------------------------- //

fn mk_vector_from_key(hash: &yaml_rust::yaml::Hash, key: &str) -> Option<Vector> {
    match hash.get(&Yaml::from_str(key)) {
        None => None,
        Some(x) => Some(mk_vector(x)),
    }
}

// --------------------------------------------------------------------------------------------- //

fn mk_pattern(hash: &yaml_rust::yaml::Hash) -> Option<Pattern> {
    if let Some(color) = hash.get(&Yaml::from_str(&"color")) {
        Some(Pattern::new_plain(mk_color(color)))
    } else if let Some(pattern) = hash.get(&Yaml::from_str(&"pattern")) {
        let pattern_hash = pattern.clone().into_hash().unwrap();
        let ty = pattern_hash
            .get(&Yaml::from_str(&"type"))
            .unwrap()
            .as_str()
            .unwrap();

        let pattern = match ty {
            "checkers" => {
                let colors = pattern_hash
                    .get(&Yaml::from_str(&"colors"))
                    .unwrap()
                    .as_vec()
                    .unwrap();

                Pattern::new_checker(mk_color(&colors[0]), mk_color(&colors[1]))
            }

            "gradient" => {
                let colors = pattern_hash
                    .get(&Yaml::from_str(&"colors"))
                    .unwrap()
                    .as_vec()
                    .unwrap();

                Pattern::new_gradient(mk_color(&colors[0]), mk_color(&colors[1]))
            }

            "ring" => {
                let colors = pattern_hash
                    .get(&Yaml::from_str(&"colors"))
                    .unwrap()
                    .as_vec()
                    .unwrap();

                let v: Vec<_> = colors.iter().map(|c| mk_color(&c)).collect();

                Pattern::new_ring(v)
            }

            "stripes" => {
                let colors = pattern_hash
                    .get(&Yaml::from_str(&"colors"))
                    .unwrap()
                    .as_vec()
                    .unwrap();

                let v: Vec<_> = colors.iter().map(|c| mk_color(&c)).collect();

                Pattern::new_stripe(v)
            }
            _ => unimplemented!(),
        };

        Some(transform(pattern, &pattern_hash))
    } else {
        None
    }
}

// --------------------------------------------------------------------------------------------- //

fn mk_material(hash: &yaml_rust::yaml::Hash) -> Material {
    let default = Material::new();

    match hash.get(&Yaml::from_str(&"material")) {
        Some(material_yaml) => {
            let material_hash = material_yaml.clone().into_hash().unwrap();

            let pattern = mk_pattern(&material_hash).unwrap_or(default.pattern);

            Material::new()
                .with_ambient(mk_f64_from_key(&material_hash, "ambient").unwrap_or(default.ambient))
                .with_diffuse(mk_f64_from_key(&material_hash, "diffuse").unwrap_or(default.diffuse))
                .with_reflective(
                    mk_f64_from_key(&material_hash, "reflective").unwrap_or(default.reflective),
                )
                .with_refractive_index(
                    mk_f64_from_key(&material_hash, "refractive-index")
                        .unwrap_or(default.refractive_index),
                )
                .with_shininess(
                    mk_f64_from_key(&material_hash, "shininess").unwrap_or(default.shininess),
                )
                .with_specular(
                    mk_f64_from_key(&material_hash, "specular").unwrap_or(default.specular),
                )
                .with_transparency(
                    mk_f64_from_key(&material_hash, "transparency").unwrap_or(default.transparency),
                )
                .with_pattern(pattern)
        }
        None => default,
    }
}

// --------------------------------------------------------------------------------------------- //

fn transform<T>(mut x: T, hash: &yaml_rust::yaml::Hash) -> T
where
    T: Transform,
{
    if let Some(transform_array) = hash.get(&Yaml::from_str(&"transform")) {
        let transform_array = transform_array.clone().into_vec().unwrap();
        for transform in transform_array {
            let operation = transform[0].as_str().unwrap();

            let transformation = match operation {
                "rotate-x" => rotation_x(mk_f64(&transform[1])),
                "rotate-y" => rotation_y(mk_f64(&transform[1])),
                "rotate-z" => rotation_z(mk_f64(&transform[1])),
                "scale" => scaling(
                    mk_f64(&transform[1]),
                    mk_f64(&transform[2]),
                    mk_f64(&transform[3]),
                ),
                "shear" => shearing(
                    mk_f64(&transform[1]),
                    mk_f64(&transform[2]),
                    mk_f64(&transform[3]),
                    mk_f64(&transform[4]),
                    mk_f64(&transform[5]),
                    mk_f64(&transform[6]),
                ),
                "translate" => translation(
                    mk_f64(&transform[1]),
                    mk_f64(&transform[2]),
                    mk_f64(&transform[3]),
                ),
                _ => unimplemented!(),
            };
            x = x.apply_transformation(&transformation);
        }
    }

    x
}

// --------------------------------------------------------------------------------------------- //

fn mk_object(hash: &yaml_rust::yaml::Hash, ty: &str) -> Arc<Object> {
    let object = match ty {
        "cube" => Object::new_cube(),
        "plane" => Object::new_plane(),
        "sphere" => Object::new_sphere(),
        _ => panic!("Unexpected object type: {:?}", ty),
    }
    .with_material(mk_material(&hash))
    .with_shadow(mk_bool_from_key(&hash, "shadow").unwrap_or(true));

    let object = transform(object, &hash);

    Arc::new(object)
}

// --------------------------------------------------------------------------------------------- //

fn mk_camera(hash: &yaml_rust::yaml::Hash, factor: usize) -> Camera {
    Camera::new(
        mk_usize_from_key(&hash, "width").unwrap() * factor,
        mk_usize_from_key(&hash, "height").unwrap() * factor,
        mk_f64_from_key(&hash, "field-of-view").unwrap(),
    )
    .with_transformation(&view_transform(
        &mk_point_from_key(&hash, "from").unwrap(),
        &mk_point_from_key(&hash, "to").unwrap(),
        &mk_vector_from_key(&hash, "up").unwrap(),
    ))
}

// --------------------------------------------------------------------------------------------- //

fn mk_area_light(hash: &yaml_rust::yaml::Hash) -> Light {
    Light::new_area_light(
        mk_color_from_key(hash, "intensity").unwrap(),
        mk_point_from_key(hash, "corner").unwrap(),
        mk_vector_from_key(hash, "uvec").unwrap(),
        mk_usize_from_key(hash, "usteps").unwrap() as u32,
        mk_vector_from_key(hash, "vvec").unwrap(),
        mk_usize_from_key(hash, "vsteps").unwrap() as u32,
    )
}

// --------------------------------------------------------------------------------------------- //

fn mk_point_light(hash: &yaml_rust::yaml::Hash) -> Light {
    Light::new_point_light(
        mk_color_from_key(hash, "intensity").unwrap(),
        mk_point_from_key(hash, "at").unwrap(),
    )
}

// --------------------------------------------------------------------------------------------- //

fn mk_light(hash: &yaml_rust::yaml::Hash) -> Light {
    if hash.get(&Yaml::from_str("corner")).is_some() {
        mk_area_light(hash)
    } else if hash.get(&Yaml::from_str("at")).is_some() {
        mk_point_light(hash)
    } else {
        panic!("Unexpected light type, got: {:?}", hash);
    }
}

// --------------------------------------------------------------------------------------------- //

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

// --------------------------------------------------------------------------------------------- //

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
            Arg::with_name("INPUT")
                .help("Sets the input YAML file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let factor = clap::value_t!(matches.value_of("factor"), usize).unwrap_or(1);
    let path_str = matches.value_of("INPUT").unwrap();

    println!("Using factor: {}", factor);
    println!("Using input file: {}", path_str);

    let path = std::path::Path::new(&path_str);

    let yaml = std::fs::read_to_string(path).unwrap();
    let docs = YamlLoader::load_from_str(&yaml).unwrap();
    let doc = &docs[0];

    let mut objects = vec![];
    let mut lights = vec![];
    let mut camera = None;

    for elem in doc.as_vec().unwrap().iter() {
        let hash = elem.clone().into_hash().unwrap();

        if let Some(x) = hash.get(&Yaml::from_str(&"add")) {
            let ty = x.as_str().unwrap().as_ref();

            match ty {
                "camera" => {
                    camera = Some(mk_camera(&hash, factor));
                }
                "light" => {
                    lights.push(mk_light(&hash));
                }
                "cube" | "plane" | "sphere" => {
                    objects.push(mk_object(&hash, ty));
                }
                _ => unimplemented!(),
            }
        }
    }

    let world = World::new().with_objects(objects).with_lights(lights);
    let canvas = camera.unwrap().par_render(&world);

    canvas.export(&output_path(&path)).unwrap();
}

// --------------------------------------------------------------------------------------------- //
