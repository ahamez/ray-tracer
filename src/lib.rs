mod float {
    pub use approx_eq::ApproxEq;
    pub use epsilon::EPSILON;

    mod approx_eq;
    mod epsilon;
}

pub mod primitive {
    pub use matrix::Matrix;
    pub use point::Point;
    pub use tuple::Tuple;
    pub use vector::Vector;

    mod matrix;
    mod point;
    mod tuple;
    mod vector;
}

pub mod io {
    pub mod obj;
    pub mod yaml;
}

pub mod rtc {
    use bounds::BoundingBox;
    pub use camera::Camera;
    pub use camera::ParallelRendering;
    pub use canvas::Canvas;
    pub use color::Color;
    pub use intersection::{Intersection, IntersectionPusher, IntersectionState, Intersections};
    pub use light::Light;
    pub use material::Material;
    pub use object::Object;
    pub use pattern::Pattern;
    pub use ray::Ray;
    pub use scene::Scene;
    pub use shape::Shape;
    pub use transformation::*;
    pub use world::World;

    mod bounds;
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

    mod lights {
        pub use area_light::AreaLight;
        pub use point_light::PointLight;

        mod area_light;
        mod point_light;
    }

    mod shapes {
        pub use cone::Cone;
        pub use cube::Cube;
        pub use cylinder::Cylinder;
        pub use group::Group;
        pub use group::GroupBuilder;
        pub use plane::Plane;
        pub use smooth_triangle::SmoothTriangle;
        pub use sphere::Sphere;
        pub use test_shape::TestShape;
        pub use triangle::Triangle;

        mod cone;
        mod cube;
        mod cylinder;
        mod group;
        mod plane;
        mod smooth_triangle;
        mod sphere;
        mod test_shape;
        mod triangle;
    }
}
