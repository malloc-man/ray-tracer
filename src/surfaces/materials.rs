use crate::{Matrix4, solid};
use crate::surfaces::colors::*;
use crate::surfaces::patterns::Pattern;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    pattern: Pattern,
    reflective: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern: solid(white()),
            reflective: 0.0,
        }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn get_ambient(&self) -> f64 {
        self.ambient
    }

    pub fn set_ambient(&mut self, ambient: f64) -> &mut Self {
        self.ambient = ambient;
        self
    }

    pub fn get_diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn set_diffuse(&mut self, diffuse: f64) -> &mut Self {
        self.diffuse = diffuse;
        self
    }

    pub fn get_specular(&self) -> f64 {
        self.specular
    }

    pub fn set_specular(&mut self, specular: f64) -> &mut Self {
        self.specular = specular;
        self
    }

    pub fn get_shininess(&self) -> f64 {
        self.shininess
    }

    pub fn set_shininess(&mut self, shininess: f64) -> &mut Self {
        self.shininess = shininess;
        self
    }

    pub fn get_pattern(&self) -> Pattern {
        self.pattern
    }

    pub fn set_pattern(&mut self, pattern: Pattern) -> &mut Self {
        self.pattern = pattern;
        self
    }

    pub fn get_pattern_transform(&self) -> Matrix4 {
        self.pattern.get_transform()
    }

    pub fn set_pattern_transform(&mut self, transform: Matrix4) -> &mut Self {
        self.pattern.set_transform(transform);
        self
    }

    pub fn get_pattern_inverse_transform(&self) -> Matrix4 {
        self.pattern.get_inverse_transform()
    }

    pub fn get_reflective(&self) -> f64 {
        self.reflective
    }

    pub fn set_reflective(&mut self, reflective: f64) -> &mut Self {
        self.reflective = reflective;
        self
    }
}
