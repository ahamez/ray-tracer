// --------------------------------------------------------------------------------------------- //

pub use self::default::default_scenes;
pub use camera::Camera;

mod camera;
mod canvas;
mod ch09_plane;
mod ch10_pattern;
mod ch11_reflection;
mod ch11_refraction;
mod default;

// --------------------------------------------------------------------------------------------- //

#[derive(Debug)]
pub struct Scene {
    pub world: crate::rtc::World,
    pub view_transform: crate::primitive::Matrix,
    pub fov: f64,
}

// --------------------------------------------------------------------------------------------- //
