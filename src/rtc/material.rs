// --------------------------------------------------------------------------------------------- //

use crate::{
    float::ApproxEq,
    primitive::{Point, Vector},
    rtc::{Color, Light, Object, Pattern},
};

// --------------------------------------------------------------------------------------------- //

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub ambient: f64,
    pub pattern: Pattern,
    pub diffuse: f64,
    pub reflective: f64,
    pub refractive_index: f64,
    pub shininess: f64,
    pub specular: f64,
    pub transparency: f64,
}

// --------------------------------------------------------------------------------------------- //

impl Material {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_ambient(mut self, ambient: f64) -> Material {
        self.ambient = ambient;
        self
    }

    pub fn with_color(mut self, color: Color) -> Material {
        self.pattern = Pattern::new_plain(color);
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Material {
        self.diffuse = diffuse;
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Material {
        self.pattern = pattern;
        self
    }

    pub fn with_reflective(mut self, reflective: f64) -> Material {
        self.reflective = reflective;
        self
    }

    pub fn with_refractive_index(mut self, index: f64) -> Material {
        self.refractive_index = index;
        self
    }

    pub fn with_shininess(mut self, index: f64) -> Material {
        self.shininess = index;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Material {
        self.specular = specular;
        self
    }

    pub fn with_transparency(mut self, transparency: f64) -> Material {
        self.transparency = transparency;
        self
    }

    pub fn lighting(
        &self,
        object: &Object,
        light: &Light,
        position: &Point,
        eye_v: &Vector,
        normal_v: &Vector,
        intensity: f64,
    ) -> Color {
        let color = self.pattern.pattern_at_object(&object, &position);
        let effective_color = color * light.intensity();
        let ambient = effective_color * self.ambient;

        if intensity.approx_eq(0.0) {
            ambient
        } else {
            let mut sum = Color::black();
            let nb_samples = light.positions().len() as f64;

            for light_position in light.positions().iter() {
                let light_v = (*light_position - *position).normalize();
                let light_dot_normal = light_v ^ *normal_v;

                if light_dot_normal >= 0.0 {
                    let diffuse = effective_color * self.diffuse * light_dot_normal;
                    sum = sum + diffuse;

                    let reflect_v = (-light_v).reflect(normal_v);
                    let reflect_dot_eye = reflect_v ^ *eye_v;

                    if reflect_dot_eye > 0.0 {
                        let factor = f64::powf(reflect_dot_eye, self.shininess);
                        let specular = light.intensity() * self.specular * factor;

                        sum = sum + specular;
                    }
                }
            }

            ambient + (sum / nb_samples) * intensity
        }
    }
}

// --------------------------------------------------------------------------------------------- //

impl Default for Material {
    fn default() -> Self {
        Material {
            ambient: 0.1,
            pattern: Pattern::new_plain(Color::white()),
            diffuse: 0.9,
            reflective: 0.0,
            refractive_index: 1.0,
            shininess: 200.0,
            specular: 0.9,
            transparency: 0.0,
        }
    }
}

// --------------------------------------------------------------------------------------------- //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{primitive::Tuple, rtc::World};
    use std::sync::Arc;

    #[test]
    fn lighting_with_the_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Point::zero();
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                1.0
            ),
            Color::new(1.9, 1.9, 1.9)
        );
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let position = Point::zero();
        let eye_v = Vector::new(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                1.0
            ),
            Color::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn lighting_with_the_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let position = Point::zero();
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                1.0
            ),
            Color::new(0.7364, 0.7364, 0.7364)
        );
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = Point::zero();
        let eye_v = Vector::new(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                1.0
            ),
            Color::new(1.6364, 1.6364, 1.6364)
        );
    }

    #[test]
    fn lighting_with_the_eye_behind_the_surface() {
        let m = Material::new();
        let position = Point::zero();
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, 10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                1.0
            ),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::new();
        let position = Point::zero();
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                0.0
            ),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let m = Material::new()
            .with_pattern(Pattern::new_stripe(vec![Color::white(), Color::black()]))
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0);

        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new_point_light(Color::white(), Point::new(0.0, 0.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &Point::new(0.9, 0.0, 0.0),
                &eye_v,
                &normal_v,
                1.0
            ),
            Color::black()
        );
        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &Point::new(1.1, 0.0, 0.0),
                &eye_v,
                &normal_v,
                1.0
            ),
            Color::white()
        );
    }

    #[test]
    fn lighting_uses_light_intensity_to_attenuate_color() {
        let mut objects = crate::rtc::world::tests::default_world().objects().clone();
        let mut object = (*objects[0]).clone();
        object.material_mut().ambient = 0.1;
        object.material_mut().diffuse = 0.9;
        object.material_mut().specular = 0.0;
        object.material_mut().pattern = Pattern::new_plain(Color::white());
        objects[0] = Arc::new(object.clone());

        let w = World::new()
            .with_objects(objects)
            .with_lights(vec![Light::new_point_light(
                Color::white(),
                Point::new(0.0, 0.0, -10.0),
            )]);
        let light = &w.lights()[0];

        let point = Point::new(0.0, 0.0, -1.0);
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);

        let tests = vec![
            (1.0, Color::white()),
            (0.5, Color::new(0.55, 0.55, 0.55)),
            (0.0, Color::new(0.1, 0.1, 0.1)),
        ];

        for (intensity, result) in tests.into_iter() {
            assert_eq!(
                object
                    .material()
                    .lighting(&object, &light, &point, &eye_v, &normal_v, intensity),
                result
            );
        }
    }

    #[test]
    fn lighting_samples_the_area_light() {
        let corner = Point::new(-0.5, -0.5, -5.0);
        let v1 = Vector::new(1.0, 0.0, 0.0);
        let v2 = Vector::new(0.0, 1.0, 0.0);
        let light = Light::new_area_light(Color::white(), corner, v1, 2, v2, 2);

        let object = Object::new_sphere().with_material(
            Material::new()
                .with_ambient(0.1)
                .with_diffuse(0.9)
                .with_specular(0.0)
                .with_color(Color::white()),
        );

        let eye = Point::new(0.0, 0.0, -5.0);

        let tests = vec![
            (
                Point::new(0.0, 0.0, -1.0),
                Color::new(0.9965, 0.9965, 0.9965),
            ),
            (
                Point::new(0.0, 0.7071, -0.7071),
                Color::new(0.6232, 0.6232, 0.6232),
            ),
        ];

        for (point, result) in tests.into_iter() {
            let eye_v = (eye - point).normalize();
            let normal_v = Vector::new(point.x(), point.y(), point.z());

            assert_eq!(
                object
                    .material()
                    .lighting(&object, &light, &point, &eye_v, &normal_v, 1.0),
                result
            );
        }
    }
}

// --------------------------------------------------------------------------------------------- //
