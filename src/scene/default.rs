// --------------------------------------------------------------------------------------------- //

use std::{collections::HashMap, rc::Rc};

use crate::{
    scene::ch09_plane, scene::ch10_pattern, scene::ch11_reflection, scene::ch11_refraction,
    scene::ch12_cube, scene::Scene,
};

// --------------------------------------------------------------------------------------------- //

pub fn default_scenes() -> HashMap<&'static str, Rc<Scene>> {
    [
        ("ch09_plane", ch09_plane::make_scene()),
        ("ch10_pattern", ch10_pattern::make_scene()),
        ("ch11_reflection", ch11_reflection::make_scene()),
        ("ch11_refraction", ch11_refraction::make_scene()),
        ("ch12_cube", ch12_cube::make_scene()),
    ]
    .iter()
    .cloned()
    .collect()
}

// --------------------------------------------------------------------------------------------- //
