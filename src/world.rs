#![allow(unused_variables)]

// --------------------------------------------------------------------------------------------- //

use crate::{
    color::Color,
    intersection::{IntersectionState, Intersections},
    light::Light,
    material::Material,
    object::Object,
    point::Point,
    ray::Ray,
    sphere::Sphere,
    transformation::Transform,
    tuple::Tuple,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Debug)]

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

// --------------------------------------------------------------------------------------------- //

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersects(ray);

        match intersections.hit() {
            Some(hit) => {
                let comps = IntersectionState::new(&hit, &ray);
                self.shade_hit(&comps)
            }
            None => Color::black(),
        }
    }

    fn intersects(&self, ray: &Ray) -> Intersections {
        ray.intersects(&self.objects)
    }

    fn shade_hit(&self, comps: &IntersectionState) -> Color {
        self.lights.iter().fold(Color::black(), |acc, &light| {
            let is_shadowed = self.is_shadowed(&comps.over_point());

            let color = comps.object().material().lighting(
                &light,
                &comps.over_point(),
                &comps.eye_v(),
                &comps.normal_v(),
                is_shadowed,
            );

            acc + color
        })
    }

    fn is_shadowed(&self, point: &Point) -> bool {
        for light in &self.lights {
            let v = light.position - *point;
            let distance = v.magnitude();
            let direction = v.normalize();

            let ray = Ray {
                origin: *point,
                direction,
            };
            let intersections = self.intersects(&ray);

            if let Some(hit) = intersections.hit() {
                if hit.t < distance {
                    return true;
                }
            }
        }

        false
    }
}

// --------------------------------------------------------------------------------------------- //

impl Default for World {
    fn default() -> Self {
        World {
            objects: vec![
                Object::Sphere(
                    Sphere::new().with_material(
                        Material::new()
                            .with_color(Color::new(0.8, 1.0, 0.6))
                            .with_diffuse(0.7)
                            .with_specular(0.2),
                    ),
                ),
                Object::Sphere(Sphere::new().scale(0.5, 0.5, 0.5)),
            ],
            lights: vec![Light::new(Color::white(), Point::new(-10.0, 10.0, -10.0))],
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::intersection::Intersection;
    use crate::shape::Shape;
    use crate::vector::Vector;

    #[test]
    fn intersects_a_world_with_a_ray() {
        let w = World {
            ..Default::default()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = w.intersects(&ray);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World {
            ..Default::default()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = w.objects[0];
        let i = Intersection {
            t: 4.0,
            object: shape,
        };

        let comps = IntersectionState::new(&i, &ray);
        let color = w.shade_hit(&comps);

        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            lights: vec![Light {
                intensity: Color::white(),
                position: Point::new(0.0, 0.25, 0.0),
            }],
            ..Default::default()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = w.objects[1];
        let i = Intersection {
            t: 0.5,
            object: shape,
        };

        let comps = IntersectionState::new(&i, &ray);

        assert_eq!(w.shade_hit(&comps), Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn shade_hit_is_given_an_intesection_in_shadow() {
        let s1 = Sphere::new();
        let s2 = Sphere::new().translate(0.0, 0.0, 10.0);

        let w = World {
            lights: vec![Light {
                intensity: Color::white(),
                position: Point::new(0.0, 0.0, -10.0),
            }],
            objects: vec![Object::Sphere(s1)],
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.5),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 4.0,
            object: Object::Sphere(s2),
        };

        let comps = IntersectionState::new(&i, &ray);

        assert_eq!(w.shade_hit(&comps), Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World {
            ..Default::default()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        assert_eq!(w.color_at(&ray), Color::black());
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World {
            ..Default::default()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        assert_eq!(w.color_at(&ray), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let outer = Sphere::new().with_material(
            Material::new()
                .with_color(Color::new(0.8, 1.0, 0.6))
                .with_diffuse(0.7)
                .with_specular(0.2)
                .with_ambient(1.0),
        );

        let inner = Sphere::new()
            .with_material(Material::new().with_ambient(1.0))
            .scale(0.5, 0.5, 0.5);

        let w = World {
            objects: vec![Object::Sphere(outer), Object::Sphere(inner)],
            ..Default::default()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.75),
            direction: Vector::new(0.0, 0.0, -1.0),
        };

        assert_eq!(w.color_at(&ray), inner.material().color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World {
            ..Default::default()
        };

        assert_eq!(w.is_shadowed(&Point::new(0.0, 10.0, 0.0)), false);
    }

    #[test]
    fn shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World {
            ..Default::default()
        };

        assert_eq!(w.is_shadowed(&Point::new(10.0, -10.0, 10.0)), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World {
            ..Default::default()
        };

        assert_eq!(w.is_shadowed(&Point::new(-20.0, 20.0, -20.0)), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World {
            ..Default::default()
        };

        assert_eq!(w.is_shadowed(&Point::new(-2.0, 2.0, -2.0)), false);
    }
}

// --------------------------------------------------------------------------------------------- //
