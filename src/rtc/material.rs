// --------------------------------------------------------------------------------------------- //

use crate::{
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
        in_shadow: bool,
    ) -> Color {
        let color = self.pattern.pattern_at_object(&object, &position);
        let effective_color = color * light.intensity;
        let ambient = effective_color * self.ambient;

        if in_shadow {
            ambient
        } else {
            let mut diffuse = Color::black();
            let mut specular = Color::black();

            let light_v = (light.position - *position).normalize();
            let light_dot_normal = light_v ^ *normal_v;

            if light_dot_normal >= 0.0 {
                diffuse = effective_color * self.diffuse * light_dot_normal;
                let reflect_v = (-light_v).reflect(normal_v);
                let reflect_dot_eye = reflect_v ^ *eye_v;

                if reflect_dot_eye > 0.0 {
                    let factor = f64::powf(reflect_dot_eye, self.shininess);
                    specular = light.intensity * self.specular * factor
                }
            }

            ambient + diffuse + specular
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
    use crate::primitive::Tuple;

    #[test]
    fn lighting_with_the_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Point::zero();
        let eye_v = Vector::new(0.0, 0.0, -1.0);
        let normal_v = Vector::new(0.0, 0.0, -1.0);
        let light = Light::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                false
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
        let light = Light::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                false
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
        let light = Light::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                false
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
        let light = Light::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 10.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                false
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
        let light = Light::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, 10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                false
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
        let light = Light::new(Color::new(1.0, 1.0, 1.0), Point::new(0.0, 0.0, -10.0));
        let in_shadow = true;

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &position,
                &eye_v,
                &normal_v,
                in_shadow
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
        let light = Light::new(Color::white(), Point::new(0.0, 0.0, -10.0));

        assert_eq!(
            m.lighting(
                &Object::new_sphere(),
                &light,
                &Point::new(0.9, 0.0, 0.0),
                &eye_v,
                &normal_v,
                false
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
                false
            ),
            Color::white()
        );
    }
}

// --------------------------------------------------------------------------------------------- //
