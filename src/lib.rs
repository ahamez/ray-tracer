mod float {
    pub use approx_eq::ApproxEq;
    pub use epsilon::EPSILON;

    mod approx_eq;
    mod epsilon;
}

mod primitive {
    pub use matrix::Matrix;
    pub use point::Point;
    pub use tuple::Tuple;
    pub use vector::Vector;

    mod matrix;
    mod point;
    mod tuple;
    mod vector;
}

pub mod rtc {
    pub use camera::Camera;
    pub use canvas::Canvas;
    pub use color::Color;
    pub use intersection::{Intersection, IntersectionState, Intersections};
    pub use light::Light;
    pub use material::Material;
    pub use object::Object;
    pub use pattern::Pattern;
    pub use ray::Ray;
    pub use scene::Scene;
    pub use shape::Shape;
    pub use transformation::*;
    pub use world::World;

    mod camera;
    mod canvas;
    mod color;
    mod intersection;
    mod light;
    mod material;
    mod object;
    mod pattern;
    mod ray;
    mod scene;
    mod shape;
    mod transformation;
    pub mod world;

    mod shapes {
        pub use cube::Cube;
        pub use cylinder::Cylinder;
        pub use plane::Plane;
        pub use sphere::Sphere;

        mod cube;
        mod cylinder;
        mod plane;
        mod sphere;
    }
}

pub mod scenes {
    pub use self::default::default_scenes;

    mod ch09_plane;
    mod ch10_pattern;
    mod ch11_reflection;
    mod ch11_refraction;
    mod ch12_cube;
    mod ch13_cylinder;
    mod default;
}
