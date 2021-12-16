use crate::Tuple;
use crate::colors::Color;
use crate::materials::*;

pub struct PointLight {
    position: Tuple,
    intensity: Color,
}

impl PointLight {
    pub(crate) fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

pub fn lighting(material: &Material, point: &Tuple, light: &PointLight, eyev: &Tuple, normal: &Tuple) -> Color {
    let effective_color = material.color.mul(&light.intensity);
    let lightv = (light.position.subtract(point)).normalize();
    let ambient = effective_color.scalar_mul(material.ambient);
    let mut diffuse = Color::new(0.0, 0.0, 0.0);
    let mut specular = Color::new(0.0, 0.0, 0.0);

    let light_dot_normal = lightv.dot_product(normal);
    if light_dot_normal >= 0.0 {
        diffuse = effective_color
            .scalar_mul(material.diffuse)
            .scalar_mul(light_dot_normal);

        let reflectv = lightv.negate().reflect_vector(normal);
        let reflect_dot_eye = reflectv.dot_product(eyev);
        if reflect_dot_eye <= 0.0 {
            specular = Color::new(0.0, 0.0, 0.0);
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.intensity.scalar_mul(material.specular).scalar_mul(factor);
        }
    }
    ambient.add(&diffuse).add(&specular)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lighting() {
        let m = Material::new_default();
        let position = Tuple::origin();

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0),
                                    Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &position, &light, &eyev, &normalv);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));

        let eyev = Tuple::vector(0.0, f64::sqrt(2.0)/2.0, f64::sqrt(2.0)/-2.0);
        let result = lighting(&m, &position, &light, &eyev, &normalv);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0),
                                    Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &position, &light, &eyev, &normalv);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));

        let eyev = Tuple::vector(0.0, f64::sqrt(2.0)/-2.0, f64::sqrt(2.0)/-2.0);
        let result = lighting(&m, &position, &light, &eyev, &normalv);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));

        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0),
                                    Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &position, &light, &eyev, &normalv);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}