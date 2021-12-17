use crate::colors::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
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
}