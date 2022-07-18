/* ---------------------------------------------------------------------------------------------- */

use crate::{
    primitive::{Point, Tuple, Vector},
    rtc::{
        rotation_x, rotation_y, rotation_z, scaling, shearing, translation, view_transform, Camera,
        Color, Light, Material, Object, Pattern, Transform,
    },
};
use std::collections::HashMap;
use yaml_rust::{yaml, Yaml, YamlLoader};

/* ---------------------------------------------------------------------------------------------- */

type Definitions<'a> = HashMap<&'a Yaml, Yaml>;

/* ---------------------------------------------------------------------------------------------- */

fn get_definitions(yaml: &Yaml) -> Definitions {
    let mut definitions = HashMap::new();

    for elem in yaml.as_vec().unwrap().iter() {
        let hash = elem.as_hash().unwrap();

        if let Some(definition_key) = hash.get(&Yaml::from_str("define")) {
            let definition_value = hash.get(&Yaml::from_str("value")).unwrap();

            // Does not handle recursive "extend"
            let definition_value = match hash.get(&Yaml::from_str("extend")) {
                Some(parent) => {
                    if let Some(definition_value_hash) = definition_value.as_hash() {
                        let mut parent_hash = get_hash(&definitions, parent).clone();
                        parent_hash.extend(definition_value_hash.clone().into_iter());

                        Yaml::Hash(parent_hash)
                    } else {
                        // To implement if encountered in the wild (like array extension)
                        panic!("Extension unsupported for {:?}", definition_value);
                    }
                }
                None => definition_value.clone(),
            };

            definitions.insert(definition_key, definition_value);
        }
    }

    definitions
}

/* ---------------------------------------------------------------------------------------------- */

fn get_hash<'a>(definitions: &'a Definitions, yaml: &'a Yaml) -> &'a yaml::Hash {
    match yaml.as_hash() {
        Some(hash) => hash,
        None => definitions
            .get(yaml)
            .unwrap_or_else(|| panic!("Definition {:?} not found", yaml))
            .as_hash()
            .unwrap(),
    }
}

/* ---------------------------------------------------------------------------------------------- */

fn get_array<'a>(definitions: &'a Definitions, yaml: &'a Yaml) -> &'a yaml::Array {
    match yaml.as_vec() {
        Some(hash) => hash,
        None => definitions
            .get(yaml)
            .unwrap_or_else(|| panic!("Definition {:?} not found", yaml))
            .as_vec()
            .unwrap(),
    }
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_bool(yaml: &Yaml) -> bool {
    match yaml.as_bool() {
        None => panic!("Expected boolean, got: {:?}", yaml),
        Some(value) => value,
    }
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_bool_from_key(hash: &yaml::Hash, key: &str) -> Option<bool> {
    hash.get(&Yaml::from_str(key)).map(mk_bool)
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_usize(yaml: &Yaml) -> usize {
    match yaml.as_i64() {
        None => panic!("Expected integer, got: {:?}", yaml),
        Some(value) => value as usize,
    }
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_usize_from_key(hash: &yaml::Hash, key: &str) -> Option<usize> {
    hash.get(&Yaml::from_str(key)).map(mk_usize)
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_f64(yaml: &Yaml) -> f64 {
    match yaml.as_f64() {
        None => match yaml.as_i64() {
            None => panic!("Expected scalar, got: {:?}", yaml),
            Some(value) => value as f64,
        },
        Some(value) => value,
    }
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_f64_from_key(hash: &yaml::Hash, key: &str) -> Option<f64> {
    hash.get(&Yaml::from_str(key)).map(mk_f64)
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_color(yaml: &Yaml) -> Color {
    let rgb = yaml.as_vec().unwrap();
    assert_eq!(rgb.len(), 3);

    Color::new(mk_f64(&rgb[0]), mk_f64(&rgb[1]), mk_f64(&rgb[2]))
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_color_from_key(hash: &yaml::Hash, key: &str) -> Option<Color> {
    hash.get(&Yaml::from_str(key)).map(mk_color)
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_point(yaml: &Yaml) -> Point {
    let xyz = yaml.as_vec().unwrap();
    assert_eq!(xyz.len(), 3);

    Point::new(mk_f64(&xyz[0]), mk_f64(&xyz[1]), mk_f64(&xyz[2]))
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_point_from_key(hash: &yaml::Hash, key: &str) -> Option<Point> {
    hash.get(&Yaml::from_str(key)).map(mk_point)
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_vector(yaml: &Yaml) -> Vector {
    let xyz = yaml.as_vec().unwrap();
    assert_eq!(xyz.len(), 3);

    Vector::new(mk_f64(&xyz[0]), mk_f64(&xyz[1]), mk_f64(&xyz[2]))
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_vector_from_key(hash: &yaml::Hash, key: &str) -> Option<Vector> {
    hash.get(&Yaml::from_str(key)).map(mk_vector)
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_pattern(defs: &Definitions, hash: &yaml::Hash) -> Option<Pattern> {
    if let Some(color) = hash.get(&Yaml::from_str("color")) {
        Some(Pattern::new_plain(mk_color(color)))
    } else if let Some(pattern) = hash.get(&Yaml::from_str("pattern")) {
        let pattern_hash = pattern.as_hash().unwrap();
        let ty = pattern_hash
            .get(&Yaml::from_str("type"))
            .unwrap()
            .as_str()
            .unwrap();

        let pattern = match ty {
            "checkers" => {
                let colors = pattern_hash
                    .get(&Yaml::from_str("colors"))
                    .unwrap()
                    .as_vec()
                    .unwrap();

                Pattern::new_checker(mk_color(&colors[0]), mk_color(&colors[1]))
            }

            "gradient" => {
                let colors = pattern_hash
                    .get(&Yaml::from_str("colors"))
                    .unwrap()
                    .as_vec()
                    .unwrap();

                Pattern::new_gradient(mk_color(&colors[0]), mk_color(&colors[1]))
            }

            "ring" => {
                let colors = pattern_hash
                    .get(&Yaml::from_str("colors"))
                    .unwrap()
                    .as_vec()
                    .unwrap();

                let v: Vec<_> = colors.iter().map(mk_color).collect();

                Pattern::new_ring(v)
            }

            "stripes" => {
                let colors = pattern_hash
                    .get(&Yaml::from_str("colors"))
                    .unwrap()
                    .as_vec()
                    .unwrap();

                let v: Vec<_> = colors.iter().map(mk_color).collect();

                Pattern::new_stripe(v)
            }
            _ => panic!("Unknown pattern: {:?}", pattern),
        };

        Some(transform(defs, pattern, pattern_hash))
    } else {
        None
    }
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_material(defs: &Definitions, hash: &yaml::Hash) -> Material {
    let default = Material::new();

    match hash.get(&Yaml::from_str("material")) {
        Some(material_yaml) => {
            let material_hash = get_hash(defs, material_yaml);

            Material::new()
                .with_ambient(mk_f64_from_key(material_hash, "ambient").unwrap_or(default.ambient))
                .with_diffuse(mk_f64_from_key(material_hash, "diffuse").unwrap_or(default.diffuse))
                .with_reflective(
                    mk_f64_from_key(material_hash, "reflective").unwrap_or(default.reflective),
                )
                .with_refractive_index(
                    mk_f64_from_key(material_hash, "refractive-index")
                        .unwrap_or(default.refractive_index),
                )
                .with_shininess(
                    mk_f64_from_key(material_hash, "shininess").unwrap_or(default.shininess),
                )
                .with_specular(
                    mk_f64_from_key(material_hash, "specular").unwrap_or(default.specular),
                )
                .with_transparency(
                    mk_f64_from_key(material_hash, "transparency").unwrap_or(default.transparency),
                )
                .with_pattern(mk_pattern(defs, material_hash).unwrap_or(default.pattern))
        }
        None => default,
    }
}

/* ---------------------------------------------------------------------------------------------- */

fn transform<T>(defs: &Definitions, mut x: T, hash: &yaml::Hash) -> T
where
    T: Transform,
{
    // Recursively find transformations that are referenced via a "define" statement.
    fn get_transformations(
        defs: &Definitions,
        array: &[Yaml],
        transformations: &mut Vec<Yaml>,
    ) {
        for transform in array {
            match transform[0].as_str() {
                Some(_) => transformations.push(transform.clone()),
                None => {
                    let embedded_transformations = get_array(defs, transform);
                    get_transformations(defs, embedded_transformations, transformations);
                }
            }
        }
    }

    if let Some(transform_array) = hash.get(&Yaml::from_str("transform")) {
        let transform_array = transform_array.as_vec().unwrap();

        let mut transformations_yaml = vec![];
        get_transformations(defs, transform_array, &mut transformations_yaml);

        for transform in transformations_yaml {
            let transform = get_array(defs, &transform);
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
                other => panic!("Unexpected transformation {:?}", other),
            };

            x = x.transform(&transformation);
        }
    }

    x
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_object(defs: &Definitions, hash: &yaml::Hash, ty: &str) -> Object {
    let object = match ty {
        "cube" => Object::new_cube(),
        "plane" => Object::new_plane(),
        "sphere" => Object::new_sphere(),
        _ => panic!("Unexpected object type: {:?}", ty),
    }
    .with_material(mk_material(defs, hash))
    .with_shadow(mk_bool_from_key(hash, "shadow").unwrap_or(true));

    transform(defs, object, hash)
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_camera(hash: &yaml::Hash) -> Camera {
    Camera::new()
        .with_size(
            mk_usize_from_key(hash, "width").unwrap(),
            mk_usize_from_key(hash, "height").unwrap(),
        )
        .with_fov(mk_f64_from_key(hash, "field-of-view").unwrap())
        .with_transformation(&view_transform(
            &mk_point_from_key(hash, "from").unwrap(),
            &mk_point_from_key(hash, "to").unwrap(),
            &mk_vector_from_key(hash, "up").unwrap(),
        ))
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_area_light(hash: &yaml::Hash) -> Light {
    Light::new_area_light(
        mk_color_from_key(hash, "intensity").unwrap(),
        mk_point_from_key(hash, "corner").unwrap(),
        mk_vector_from_key(hash, "uvec").unwrap(),
        mk_usize_from_key(hash, "usteps").unwrap() as u32,
        mk_vector_from_key(hash, "vvec").unwrap(),
        mk_usize_from_key(hash, "vsteps").unwrap() as u32,
    )
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_point_light(hash: &yaml::Hash) -> Light {
    Light::new_point_light(
        mk_color_from_key(hash, "intensity").unwrap(),
        mk_point_from_key(hash, "at").unwrap(),
    )
}

/* ---------------------------------------------------------------------------------------------- */

fn mk_light(hash: &yaml::Hash) -> Light {
    if hash.get(&Yaml::from_str("corner")).is_some() {
        mk_area_light(hash)
    } else if hash.get(&Yaml::from_str("at")).is_some() {
        mk_point_light(hash)
    } else {
        panic!("Unexpected light type, got: {:?}", hash);
    }
}

/* ---------------------------------------------------------------------------------------------- */

// TODO: don't unwrap() everywhere...
pub fn parse(path: &std::path::Path) -> (Vec<Object>, Vec<Light>, Camera) {
    let yaml = std::fs::read_to_string(path).unwrap();
    let docs = YamlLoader::load_from_str(&yaml).unwrap();
    let doc = &docs[0];

    let mut objects = vec![];
    let mut lights = vec![];
    let mut camera = None;

    // First, look for all definitions
    let definitions = get_definitions(doc);

    for elem in doc.as_vec().unwrap().iter() {
        let hash = elem.as_hash().unwrap();

        if let Some(x) = hash.get(&Yaml::from_str("add")) {
            let ty = x.as_str().unwrap().as_ref();

            match ty {
                "camera" => {
                    camera = Some(mk_camera(hash));
                }
                "light" => {
                    lights.push(mk_light(hash));
                }
                "cube" | "plane" | "sphere" => {
                    objects.push(mk_object(&definitions, hash, ty));
                }
                _ => unimplemented!(),
            }
        }
    }

    (objects, lights, camera.unwrap())
}

/* ---------------------------------------------------------------------------------------------- */
