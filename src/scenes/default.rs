// --------------------------------------------------------------------------------------------- //

use std::{collections::HashMap, rc::Rc};

use crate::{
    rtc::Scene,
    scenes::{ch09_plane, ch10_pattern, ch11_reflection, ch11_refraction, ch12_cube, ch13_cylinder},
};

// --------------------------------------------------------------------------------------------- //

pub fn default_scenes() -> HashMap<&'static str, Rc<Scene>> {
    [
        ("ch09_plane", ch09_plane::make_scene()),
        ("ch10_pattern", ch10_pattern::make_scene()),
        ("ch11_reflection", ch11_reflection::make_scene()),
        ("ch11_refraction", ch11_refraction::make_scene()),
        ("ch12_cube", ch12_cube::make_scene()),
        ("ch13_cylinder", ch13_cylinder::make_scene()),
    ]
    .iter()
    .cloned()
    .collect()
}

// --------------------------------------------------------------------------------------------- //
