#![allow(unused_variables)]

// --------------------------------------------------------------------------------------------- //

use crate::{
    color::Color,
    intersection::{IntersectionState, Intersections},
    light::Light,
    object::Object,
    point::Point,
    ray::Ray,
};

// --------------------------------------------------------------------------------------------- //

#[derive(Debug)]

pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub recursion_limit: u8,
}

// --------------------------------------------------------------------------------------------- //

impl World {
    pub fn color_at(&self, ray: &Ray) -> Color {
        let recursion_limit = if self.recursion_limit == 0 {
            1
        } else {
            self.recursion_limit
        };

        self.color_at_impl(ray, recursion_limit)
    }

    fn color_at_impl(&self, ray: &Ray, remaining_recursions: u8) -> Color {
        let intersections = self.intersects(ray);

        match intersections.hit_index() {
            Some(hit_index) => {
                let comps = IntersectionState::new(&intersections, hit_index, &ray);
                self.shade_hit(&comps, remaining_recursions)
            }
            None => Color::black(),
        }
    }

    fn intersects(&self, ray: &Ray) -> Intersections {
        ray.intersects(&self.objects)
    }

    fn shade_hit(&self, comps: &IntersectionState, remaining_recursions: u8) -> Color {
        self.lights.iter().fold(Color::black(), |acc, &light| {
            let is_shadowed = self.is_shadowed(&comps.over_point());

            let surface_color = comps.object().material().lighting(
                &comps.object(),
                &light,
                &comps.over_point(),
                &comps.eye_v(),
                &comps.normal_v(),
                is_shadowed,
            );

            acc + surface_color
                + self.reflected_color(comps, remaining_recursions)
                + self.refracted_color(comps, remaining_recursions)
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

    fn reflected_color(&self, comps: &IntersectionState, remaining_recursions: u8) -> Color {
        if remaining_recursions == 0 || comps.object().material().reflective == 0.0 {
            Color::black()
        } else {
            let reflect_ray = Ray {
                origin: comps.over_point(),
                direction: comps.reflect_v(),
            };

            let color = self.color_at_impl(&reflect_ray, remaining_recursions - 1);

            color * comps.object().material().reflective
        }
    }

    fn refracted_color(&self, comps: &IntersectionState, remaining_recursions: u8) -> Color {
        if remaining_recursions == 0 || comps.object().material().transparency == 0.0 {
            Color::black()
        } else {
            let (n1, n2) = comps.n();
            let n_ratio = n1 / n2;
            let cos_i = comps.eye_v() ^ comps.normal_v();
            let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);

            if sin2_t > 1.0 {
                Color::black()
            } else {
                let cos_t = f64::sqrt(1.0 - sin2_t);

                let direction =
                    comps.normal_v() * (n_ratio * cos_i - cos_t) - comps.eye_v() * n_ratio;

                let refract_ray = Ray {
                    origin: comps.under_point(),
                    direction,
                };

                self.color_at_impl(&refract_ray, remaining_recursions - 1)
                    * comps.object().material().transparency
            }
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl Default for World {
    fn default() -> Self {
        World {
            objects: vec![],
            lights: vec![],
            recursion_limit: 4,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        intersection::Intersection, material::Material, pattern::Pattern,
        transformation::Transform, tuple::Tuple, vector::Vector,
    };

    pub fn default_world() -> World {
        World {
            objects: vec![
                Object::new_sphere().with_material(
                    Material::new()
                        .with_pattern(Pattern::new_plain(Color::new(0.8, 1.0, 0.6)))
                        .with_diffuse(0.7)
                        .with_specular(0.2),
                ),
                Object::new_sphere().scale(0.5, 0.5, 0.5),
            ],
            lights: vec![Light::new(Color::white(), Point::new(-10.0, 10.0, -10.0))],
            ..Default::default()
        }
    }

    #[test]
    fn intersects_a_world_with_a_ray() {
        let w = default_world();

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
        let w = default_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let object = w.objects[0].clone();
        let i = Intersection { t: 4.0, object };

        let comps = IntersectionState::new(&Intersections::new(vec![i]), 0, &ray);
        let color = w.shade_hit(&comps, 1);

        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            lights: vec![Light {
                intensity: Color::white(),
                position: Point::new(0.0, 0.25, 0.0),
            }],
            ..default_world()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let object = w.objects[1].clone();
        let i = Intersection { t: 0.5, object };

        let comps = IntersectionState::new(&Intersections::new(vec![i]), 0, &ray);

        assert_eq!(
            w.shade_hit(&comps, 1),
            Color::new(0.90498, 0.90498, 0.90498)
        );
    }

    #[test]
    fn shade_hit_is_given_an_intesection_in_shadow() {
        let s1 = Object::new_sphere();
        let s2 = Object::new_sphere().translate(0.0, 0.0, 10.0);

        let w = World {
            lights: vec![Light {
                intensity: Color::white(),
                position: Point::new(0.0, 0.0, -10.0),
            }],
            objects: vec![s1],
            ..Default::default()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.5),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection { t: 4.0, object: s2 };

        let comps = IntersectionState::new(&Intersections::new(vec![i]), 0, &ray);

        assert_eq!(w.shade_hit(&comps, 1), Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        assert_eq!(w.color_at(&ray), Color::black());
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        assert_eq!(w.color_at(&ray), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let outer = Object::new_sphere().with_material(
            Material::new()
                .with_pattern(Pattern::new_plain(Color::new(0.8, 1.0, 0.6)))
                .with_diffuse(0.7)
                .with_specular(0.2)
                .with_ambient(1.0),
        );

        let inner = Object::new_sphere()
            .with_material(Material::new().with_ambient(1.0))
            .scale(0.5, 0.5, 0.5);

        let w = World {
            objects: vec![outer, inner],
            ..default_world()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.75),
            direction: Vector::new(0.0, 0.0, -1.0),
        };

        assert_eq!(w.color_at(&ray), Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();

        assert_eq!(w.is_shadowed(&Point::new(0.0, 10.0, 0.0)), false);
    }

    #[test]
    fn shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();

        assert_eq!(w.is_shadowed(&Point::new(10.0, -10.0, 10.0)), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();

        assert_eq!(w.is_shadowed(&Point::new(-20.0, 20.0, -20.0)), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();

        assert_eq!(w.is_shadowed(&Point::new(-2.0, 2.0, -2.0)), false);
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let w = default_world();

        let mut object = w.objects[1].clone();
        object.material_mut().ambient = 1.0;

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection { t: 1.0, object };

        let comps = IntersectionState::new(&Intersections::new(vec![i]), 0, &ray);

        assert_eq!(w.reflected_color(&comps, 1), Color::black());
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let sqrt2 = f64::sqrt(2.0);

        let mut w = default_world();

        let object = Object::new_plane()
            .with_material(Material::new().with_reflective(0.5))
            .translate(0.0, -1.0, 0.0);

        w.objects.push(object.clone());

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -sqrt2 / 2.0, sqrt2 / 2.0),
        };

        let i = Intersection { t: sqrt2, object };

        let comps = IntersectionState::new(&Intersections::new(vec![i]), 0, &ray);

        assert_eq!(
            w.reflected_color(&comps, 1),
            Color::new(0.19032, 0.2379, 0.14274)
        );
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let sqrt2 = f64::sqrt(2.0);

        let mut w = default_world();

        let object = Object::new_plane()
            .with_material(Material::new().with_reflective(0.5))
            .translate(0.0, -1.0, 0.0);

        w.objects.push(object.clone());

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -sqrt2 / 2.0, sqrt2 / 2.0),
        };

        let i = Intersection { t: sqrt2, object };

        let comps = IntersectionState::new(&Intersections::new(vec![i]), 0, &ray);

        assert_eq!(
            w.shade_hit(&comps, 1),
            Color::new(0.87677, 0.92436, 0.82918)
        );
    }

    #[test]
    fn color_at_with_mutually_reflexive_surfaces() {
        let w = World {
            lights: vec![Light::new(Color::white(), Point::new(0.0, 0.0, 0.0))],
            objects: vec![
                Object::new_plane()
                    .with_material(Material::new().with_reflective(1.0))
                    .translate(0.0, -1.0, 0.0),
                Object::new_plane()
                    .with_material(Material::new().with_reflective(1.0))
                    .translate(0.0, 1.0, 0.0),
            ],
            ..Default::default()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        // The color doesn't really matter here, what we want is to make sure that
        // the call doesn't end in an infinite recursion.
        w.color_at(&ray);
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = default_world();
        let object = w.objects[0].clone();
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = Intersections::new(vec![
            Intersection {
                t: 4.0,
                object: object.clone(),
            },
            Intersection {
                t: 6.0,
                object: object.clone(),
            },
        ]);

        let comps = IntersectionState::new(&xs, 0, &ray);

        assert_eq!(w.refracted_color(&comps, 5), Color::black());
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let w = default_world();
        let mut object = w.objects[0].clone();
        object.material_mut().transparency = 1.0;
        object.material_mut().refractive_index = 1.5;

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = Intersections::new(vec![
            Intersection {
                t: 4.0,
                object: object.clone(),
            },
            Intersection {
                t: 6.0,
                object: object.clone(),
            },
        ]);

        let comps = IntersectionState::new(&xs, 0, &ray);

        assert_eq!(w.refracted_color(&comps, 0), Color::black());
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let w = default_world();
        let mut object = w.objects[0].clone();
        object.material_mut().transparency = 1.0;
        object.material_mut().refractive_index = 1.5;

        let ray = Ray {
            origin: Point::new(0.0, 0.0, f64::sqrt(2.0) / 2.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = Intersections::new(vec![
            Intersection {
                t: -f64::sqrt(2.0) / 2.0,
                object: object.clone(),
            },
            Intersection {
                t: f64::sqrt(2.0) / 2.0,
                object: object.clone(),
            },
        ]);

        let comps = IntersectionState::new(&xs, 1, &ray);

        assert_eq!(w.refracted_color(&comps, 5), Color::black());
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = default_world();

        let mut a = w.objects[0].clone();
        a.material_mut().ambient = 1.0;
        a.material_mut().pattern = Pattern::new_test();

        let mut b = w.objects[1].clone();
        b.material_mut().transparency = 1.0;
        b.material_mut().refractive_index = 1.5;

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.1),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        w.objects = vec![a.clone(), b.clone()];

        let xs = Intersections::new(vec![
            Intersection {
                t: -0.9899,
                object: a.clone(),
            },
            Intersection {
                t: -0.4899,
                object: b.clone(),
            },
            Intersection {
                t: 0.4899,
                object: b.clone(),
            },
            Intersection {
                t: 0.9899,
                object: a.clone(),
            },
        ]);

        let comps = IntersectionState::new(&xs, 2, &ray);

        assert_eq!(
            w.refracted_color(&comps, 5),
            // The book gives the following values: (0.0, 0.99888, 0.04725)
            Color::new(0.0, 0.99787, 0.04747)
        );
    }

    #[test]
    fn shade_hot_with_a_transparent_material() {
        let mut w = default_world();

        let floor = Object::new_plane()
            .with_material(
                Material::new()
                    .with_transparency(0.5)
                    .with_refractive_index(1.5),
            )
            .translate(0.0, -1.0, 0.0);

        let ball = Object::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(1.0, 0.0, 0.0))
                    .with_ambient(0.5),
            )
            .translate(0.0, -3.5, -0.5);

        w.objects = vec![floor.clone(), ball];

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        };

        let xs = Intersections::new(vec![Intersection {
            t: f64::sqrt(2.0),
            object: floor,
        }]);

        let comps = IntersectionState::new(&xs, 0, &ray);

        assert_eq!(
            w.shade_hit(&comps, 5),
            Color::new(0.93642, 0.68642, 0.68642)
        );
    }
}

// --------------------------------------------------------------------------------------------- //
