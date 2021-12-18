use crate::Tuple;
use crate::colors::*;
use crate::LightType::PointLight;
use crate::shapes::materials::*;

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

pub fn lighting(material: Material, point: Tuple, light: Light, eyev: Tuple, normal: Tuple, in_shadow: bool) -> Color {
    let effective_color = material.get_color() * light.get_intensity();

    let lightv = (light.get_position() - point).normalize();

    let ambient = effective_color * material.get_ambient();

    if in_shadow {
        return ambient;
    }

    let mut diffuse = black();
    let mut specular = black();

    let light_dot_normal = lightv * normal;

    if light_dot_normal >= 0.0 {
        diffuse = effective_color * material.get_diffuse() * light_dot_normal;

        let reflectv = -lightv
            .reflect_vector(normal);

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

    #[test]
    fn test_lighting() {
        let m = Material::new();
        let position = origin();

        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0),
                                    color(1.0, 1.0, 1.0));
        let result = lighting(m, position, light, eyev, normalv, false);
        assert_eq!(result, color(1.9, 1.9, 1.9));

        let eyev = vector(0.0, f64::sqrt(2.0)/2.0, f64::sqrt(2.0)/-2.0);
        let result = lighting(m, position, light, eyev, normalv, false);
        assert_eq!(result, color(1.0, 1.0, 1.0));

        let eyev = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 10.0, -10.0),
                                    color(1.0, 1.0, 1.0));
        let result = lighting(m, position, light, eyev, normalv, false);
        assert_eq!(result, color(0.7364, 0.7364, 0.7364));

        let eyev = vector(0.0, f64::sqrt(2.0)/-2.0, f64::sqrt(2.0)/-2.0);
        let result = lighting(m, position, light, eyev, normalv, false);
        assert_eq!(result, color(1.6364, 1.6364, 1.6364));

        let eyev = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, 10.0),
                                    color(1.0, 1.0, 1.0));
        let result = lighting(m, position, light, eyev, normalv, false);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_shadow() {
        let m = Material::new();
        let position = origin();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), white());

        let result = lighting(m, position, light, eyev, normalv, true);
        assert_eq!(result, color(0.1, 0.1, 0.1));
    }
}