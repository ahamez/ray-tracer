/* ---------------------------------------------------------------------------------------------- */

use crate::{
    float::ApproxEq,
    primitive::Point,
    rtc::{Color, IntersectionState, Intersections, Light, Object, Ray},
};
use atomic_counter::{AtomicCounter, RelaxedCounter};
use serde::{Deserialize, Serialize};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    objects: Vec<Object>,
    lights: Vec<Light>,
    recursion_limit: u8,
    #[serde(skip)]
    nb_intersections: RelaxedCounter,
}

/* ---------------------------------------------------------------------------------------------- */

impl World {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_recursion_limit(mut self, limit: u8) -> Self {
        self.recursion_limit = if limit == 0 { 1 } else { limit };

        self
    }

    pub fn with_objects(mut self, objects: Vec<Object>) -> Self {
        self.objects = objects;

        self
    }

    pub fn with_lights(mut self, lights: Vec<Light>) -> Self {
        self.lights = lights;

        self
    }

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn nb_intersections(&self) -> usize {
        self.nb_intersections.get()
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        self.color_at_impl(ray, self.recursion_limit)
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
        let is = ray.intersects(&self.objects);
        self.nb_intersections.add(is.len());

        is
    }

    fn shade_hit(&self, comps: &IntersectionState, remaining_recursions: u8) -> Color {
        self.lights.iter().fold(Color::black(), |acc, light| {
            let light_intensity = light.intensity_at(&self, &comps.over_point());

            let surface_color = comps.object().material().lighting(
                &comps.object(),
                &light,
                &comps.over_point(),
                &comps.eye_v(),
                &comps.normal_v(),
                light_intensity,
            );

            let reflected_color = self.reflected_color(comps, remaining_recursions);
            let refracted_color = self.refracted_color(comps, remaining_recursions);

            if comps.object().material().reflective > 0.0
                && comps.object().material().transparency > 0.0
            {
                let reflectance = comps.schlick();

                acc + surface_color
                    + reflected_color * reflectance
                    + refracted_color * (1.0 - reflectance)
            } else {
                acc + surface_color + reflected_color + refracted_color
            }
        })
    }

    pub fn is_shadowed(&self, light_position: &Point, point: &Point) -> bool {
        let v = *light_position - *point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray {
            origin: *point,
            direction,
        };
        let intersections = self.intersects(&ray);

        if let Some(hit) = intersections.hit() {
            if hit.object().has_shadow() && hit.t() < distance {
                return true;
            }
        }

        false
    }

    fn reflected_color(&self, comps: &IntersectionState, remaining_recursions: u8) -> Color {
        if remaining_recursions == 0 || comps.object().material().reflective.approx_eq(0.0) {
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
        if remaining_recursions == 0 || comps.object().material().transparency.approx_eq(0.0) {
            Color::black()
        } else {
            let (n1, n2) = comps.n();
            let n_ratio = n1 / n2;
            let cos_i = comps.cos_i();
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

/* ---------------------------------------------------------------------------------------------- */

impl Default for World {
    fn default() -> Self {
        World {
            objects: vec![],
            lights: vec![],
            recursion_limit: 4,
            nb_intersections: RelaxedCounter::new(0),
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        primitive::{Tuple, Vector},
        rtc::{Intersection, Material, Pattern, Transform},
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
                Object::new_sphere().scale(0.5, 0.5, 0.5).transform(),
            ],
            lights: vec![Light::new_point_light(
                Color::white(),
                Point::new(-10.0, 10.0, -10.0),
            )],
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
        assert_eq!(xs[0].t(), 4.0);
        assert_eq!(xs[1].t(), 4.5);
        assert_eq!(xs[2].t(), 5.5);
        assert_eq!(xs[3].t(), 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = default_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let object = &w.objects[0];
        let i = Intersection::new(4.0, object);

        let comps =
            IntersectionState::new(&Intersections::new().with_intersections(vec![i]), 0, &ray);
        let color = w.shade_hit(&comps, 1);

        assert_eq!(color, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            lights: vec![Light::new_point_light(
                Color::white(),
                Point::new(0.0, 0.25, 0.0),
            )],
            ..default_world()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let object = w.objects[1].clone();
        let i = Intersection::new(0.5, &object);

        let comps =
            IntersectionState::new(&Intersections::new().with_intersections(vec![i]), 0, &ray);

        assert_eq!(
            w.shade_hit(&comps, 1),
            Color::new(0.90498, 0.90498, 0.90498)
        );
    }

    #[test]
    fn shade_hit_is_given_an_intesection_in_shadow() {
        let s1 = Object::new_sphere();
        let s2 = Object::new_sphere().translate(0.0, 0.0, 10.0).transform();

        let w = World {
            lights: vec![Light::new_point_light(
                Color::white(),
                Point::new(0.0, 0.0, -10.0),
            )],
            objects: vec![s1],
            ..Default::default()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.5),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection::new(4.0, &s2);

        let comps =
            IntersectionState::new(&Intersections::new().with_intersections(vec![i]), 0, &ray);

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
            .scale(0.5, 0.5, 0.5)
            .transform();

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
    fn is_shadowed_tests_for_occlusion_between_two_points() {
        let w = default_world();
        let light_position = Point::new(-10.0, -10.0, -10.0);

        let tests = vec![
            (Point::new(-10.0, -10.0, -10.0), false),
            (Point::new(10.0, 10.0, 10.0), true),
            (Point::new(-20.0, -20.0, -20.0), false),
            (Point::new(-5.0, -5.0, -5.0), false),
        ];

        for (point, is_shadowed) in tests.into_iter() {
            assert_eq!(w.is_shadowed(&light_position, &point), is_shadowed);
        }
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let w = default_world();

        let obj1 = w.objects[1].clone();
        let obj1_material = obj1.material().clone();
        let object = obj1.clone().with_material(obj1_material.with_ambient(1.0));

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection::new(1.0, &object);

        let comps =
            IntersectionState::new(&Intersections::new().with_intersections(vec![i]), 0, &ray);

        assert_eq!(w.reflected_color(&comps, 1), Color::black());
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let sqrt2 = f64::sqrt(2.0);

        let mut w = default_world();

        w.objects.push(
            Object::new_plane()
                .with_material(Material::new().with_reflective(0.5))
                .translate(0.0, -1.0, 0.0)
                .transform(),
        );
        let object = &w.objects.last().unwrap();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -sqrt2 / 2.0, sqrt2 / 2.0),
        };

        let i = Intersection::new(sqrt2, object);

        let comps =
            IntersectionState::new(&Intersections::new().with_intersections(vec![i]), 0, &ray);

        assert_eq!(
            w.reflected_color(&comps, 1),
            Color::new(0.19032, 0.2379, 0.14274)
        );
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let sqrt2 = f64::sqrt(2.0);

        let mut w = default_world();

        w.objects.push(
            Object::new_plane()
                .with_material(Material::new().with_reflective(0.5))
                .translate(0.0, -1.0, 0.0)
                .transform(),
        );
        let object = &w.objects.last().unwrap();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -sqrt2 / 2.0, sqrt2 / 2.0),
        };

        let i = Intersection::new(sqrt2, &object);

        let comps =
            IntersectionState::new(&Intersections::new().with_intersections(vec![i]), 0, &ray);

        assert_eq!(
            w.shade_hit(&comps, 1),
            Color::new(0.87677, 0.92436, 0.82918)
        );
    }

    #[test]
    fn color_at_with_mutually_reflexive_surfaces() {
        let w = World {
            lights: vec![Light::new_point_light(
                Color::white(),
                Point::new(0.0, 0.0, 0.0),
            )],
            objects: vec![
                Object::new_plane()
                    .with_material(Material::new().with_reflective(1.0))
                    .translate(0.0, -1.0, 0.0)
                    .transform(),
                Object::new_plane()
                    .with_material(Material::new().with_reflective(1.0))
                    .translate(0.0, 1.0, 0.0)
                    .transform(),
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
        let object = &w.objects[0].clone();
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(4.0, object),
            Intersection::new(6.0, object),
        ]);

        let comps = IntersectionState::new(&xs, 0, &ray);

        assert_eq!(w.refracted_color(&comps, 5), Color::black());
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let w = default_world();

        let obj0 = &w.objects[0];
        let obj0_material = obj0.material().clone();
        let object = obj0.clone().with_material(
            obj0_material
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(4.0, &object),
            Intersection::new(6.0, &object),
        ]);

        let comps = IntersectionState::new(&xs, 0, &ray);

        assert_eq!(w.refracted_color(&comps, 0), Color::black());
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let w = default_world();

        let obj0 = &w.objects[0];
        let obj0_material = obj0.material().clone();
        let object = obj0.clone().with_material(
            obj0_material
                .with_transparency(1.0)
                .with_refractive_index(1.5),
        );

        let ray = Ray {
            origin: Point::new(0.0, 0.0, f64::sqrt(2.0) / 2.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(-f64::sqrt(2.0) / 2.0, &object),
            Intersection::new(f64::sqrt(2.0) / 2.0, &object),
        ]);

        let comps = IntersectionState::new(&xs, 1, &ray);

        assert_eq!(w.refracted_color(&comps, 5), Color::black());
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let (a, b) = {
            let w = default_world();

            let obj0 = &w.objects[0];
            let obj0_material = obj0.material().clone();
            let a = obj0.clone().with_material(
                obj0_material
                    .with_ambient(1.0)
                    .with_pattern(Pattern::new_test()),
            );

            let obj1 = &w.objects[1];
            let obj1_material = obj1.material().clone();
            let b = obj1.clone().with_material(
                obj1_material
                    .with_transparency(1.0)
                    .with_refractive_index(1.5),
            );

            (a, b)
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.1),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let w = default_world().with_objects(vec![a, b]);

        let a = &w.objects[0];
        let b = &w.objects[1];

        let xs = Intersections::new().with_intersections(vec![
            Intersection::new(-0.9899, &a),
            Intersection::new(-0.4899, &b),
            Intersection::new(0.4899, &b),
            Intersection::new(0.9899, &a),
        ]);

        let comps = IntersectionState::new(&xs, 2, &ray);

        assert_eq!(
            w.refracted_color(&comps, 5),
            Color::new(0.0, 0.99888, 0.04725)
        );
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = default_world();

        let floor = Object::new_plane()
            .with_material(
                Material::new()
                    .with_transparency(0.5)
                    .with_refractive_index(1.5),
            )
            .translate(0.0, -1.0, 0.0)
            .transform();
        w.objects.push(floor.clone());

        let ball = Object::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(1.0, 0.0, 0.0))
                    .with_ambient(0.5),
            )
            .translate(0.0, -3.5, -0.5)
            .transform();
        w.objects.push(ball);

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        };

        let xs = Intersections::new()
            .with_intersections(vec![Intersection::new(f64::sqrt(2.0), &floor)]);

        let comps = IntersectionState::new(&xs, 0, &ray);

        assert_eq!(
            w.shade_hit(&comps, 5),
            Color::new(0.93642, 0.68642, 0.68642)
        );
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = default_world();

        let floor = Object::new_plane()
            .with_material(
                Material::new()
                    .with_transparency(0.5)
                    .with_refractive_index(1.5)
                    .with_reflective(0.5),
            )
            .translate(0.0, -1.0, 0.0)
            .transform();
        w.objects.push(floor.clone());

        let ball = Object::new_sphere()
            .with_material(
                Material::new()
                    .with_color(Color::new(1.0, 0.0, 0.0))
                    .with_ambient(0.5),
            )
            .translate(0.0, -3.5, -0.5)
            .transform();
        w.objects.push(ball.clone());

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        };

        let xs = Intersections::new()
            .with_intersections(vec![Intersection::new(f64::sqrt(2.0), &floor)]);

        let comps = IntersectionState::new(&xs, 0, &ray);

        assert_eq!(
            w.shade_hit(&comps, 5),
            Color::new(0.93391, 0.69643, 0.69243)
        );
    }
}

/* ---------------------------------------------------------------------------------------------- */
