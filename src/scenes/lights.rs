use crate::Tuple;
use crate::surfaces::colors::*;
use crate::LightType::PointLight;
use crate::surfaces::materials::*;
use crate::objects::*;

#[derive(Copy, Clone, Debug)]
pub struct Light {
    position: Tuple,
    intensity: Color,
    light: LightType
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LightType {
    PointLight,
}

impl Light {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
            light: PointLight
        }
    }

    pub fn set_position(&mut self, position: Tuple) -> &mut Self {
        self.position = position;
        self
    }

    pub fn get_position(&self) -> Tuple {
        self.position
    }

    pub fn set_intensity(&mut self, intensity: Color) -> &mut Self {
        self.intensity = intensity;
        self
    }

    pub fn get_intensity(&self) -> Color {
        self.intensity
    }
}

pub fn lighting(material: Material, object: Object, light: Light, point: Tuple, eyev: Tuple, normalv: Tuple, in_shadow: bool) -> Color {
    let mut clr = black();

    if let Some(pattern) = material.get_pattern() {
        clr = object.stripe_at_object(point);
    } else {
        clr = material.get_color();
    }

    let effective_color = clr * light.get_intensity();

    let lightv = (light.get_position() - point).normalize();

    let ambient = effective_color * material.get_ambient();

    if in_shadow {
        return ambient;
    }

    let mut diffuse = black();
    let mut specular = black();

    let light_dot_normal = lightv * normalv;

    if light_dot_normal >= 0.0 {
        diffuse = effective_color * material.get_diffuse() * light_dot_normal;

        let reflectv = -lightv
            .reflect_vector(normalv);

        let reflect_dot_eye = reflectv * eyev;

        if reflect_dot_eye <= 0.0 {
            specular = color(0.0, 0.0, 0.0);
        } else {
            let factor = reflect_dot_eye.powf(material.get_shininess());
            specular = light.get_intensity() * material.get_specular() * factor;
        }
    }
    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrices::tuples::*;
    use crate::surfaces::patterns::Pattern;
    use crate::spheres;

    #[test]
    fn test_lighting() {
        let m = Material::new();
        let position = origin();
        let object = spheres::new();

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0),
                                    color(1.0, 1.0, 1.0));
        let result = lighting(m, object, light, position, eyev, normalv, false);
        assert_eq!(result, color(1.9, 1.9, 1.9));

        let eyev = vector(0.0, f64::sqrt(2.0)/2.0, f64::sqrt(2.0)/-2.0);
        let result = lighting(m, object, light, position, eyev, normalv, false);
        assert_eq!(result, color(1.0, 1.0, 1.0));

        let eyev = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 10.0, -10.0),
                                    color(1.0, 1.0, 1.0));
        let result = lighting(m, object, light, position, eyev, normalv, false);
        assert_eq!(result, color(0.7364, 0.7364, 0.7364));

        let eyev = vector(0.0, f64::sqrt(2.0)/-2.0, f64::sqrt(2.0)/-2.0);
        let result = lighting(m, object, light, position, eyev, normalv, false);
        assert_eq!(result, color(1.6364, 1.6364, 1.6364));

        let eyev = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, 10.0),
                                    color(1.0, 1.0, 1.0));
        let result = lighting(m, object, light, position, eyev, normalv, false);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_shadow() {
        let m = Material::new();
        let s = spheres::new();
        let position = origin();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), white());

        let result = lighting(m, s, light, position, eyev, normalv, true);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_lighting_with_pattern() {
        let mut m = Material::new();
        m.set_specular(0.0);
        m.set_diffuse(0.0);
        m.set_ambient(1.0);
        m.set_pattern(Pattern::stripe_pattern(white(), black()));
        let mut s = spheres::new();
        s.set_material(m);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), white());
        let c1 = lighting(m, s, light, point(0.9, 0.0, 0.0), eyev, normalv, false);
        let c2 = lighting(m, s, light, point(1.1, 0.0, 0.0), eyev, normalv, false);

        assert_eq!(c1, white());
        assert_eq!(c2, black());
    }
}