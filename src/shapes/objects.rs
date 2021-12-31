use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Object {
    material: Material,
    transform: Matrix4,
    inverse_transform: Matrix4,
    inverse_transform_transposed: Matrix4,
    pub shape: Shape,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
    Cube,
    Cylinder {min: f64, max: f64, closed: bool},
    Cone {min: f64, max: f64, closed: bool},
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self {
            material: Material::new(),
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity(),
            inverse_transform_transposed: Matrix4::identity(),
            shape,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4) -> &Self {
        self.transform = transform;
        self.inverse_transform = transform.invert();
        self.inverse_transform_transposed = self.inverse_transform.transpose();
        self
    }

    pub fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    pub fn get_inverse_transform(&self) -> Matrix4 {
        self.inverse_transform
    }

    pub fn get_inverse_transform_transposed(&self) -> Matrix4 {
        self.inverse_transform_transposed
    }

    pub fn set_material(&mut self, material: Material) -> &mut Self {
        self.material = material;
        self
    }

    pub fn get_material(&self) -> Material {
        self.material
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.material.set_color(color);
        self
    }

    pub fn get_color(&self) -> Color {
        self.material.get_color()
    }

    pub fn get_ambient(&self) -> f64 {
        self.material.get_ambient()
    }

    pub fn set_ambient(&mut self, ambient: f64) -> &mut Self {
        self.material.set_ambient(ambient);
        self
    }

    pub fn get_diffuse(&self) -> f64 {
        self.material.get_diffuse()
    }

    pub fn set_diffuse(&mut self, diffuse: f64) -> &mut Self {
        self.material.set_diffuse(diffuse);
        self
    }

    pub fn get_specular(&self) -> f64 {
        self.material.get_specular()
    }

    pub fn set_specular(&mut self, specular: f64) -> &mut Self {
        self.material.set_specular(specular);
        self
    }

    pub fn get_shininess(&self) -> f64 {
        self.material.get_shininess()
    }

    pub fn set_shininess(&mut self, shininess: f64) -> &mut Self {
        self.material.set_shininess(shininess);
        self
    }

    pub fn get_pattern(&self) -> Pattern {
        self.material.get_pattern()
    }

    pub fn set_pattern(&mut self, pattern: Pattern) -> &mut Self {
        self.material.set_pattern(pattern);
        self
    }

    pub fn get_pattern_transform(&self) -> Matrix4 {
        self.material.get_pattern_transform()
    }

    pub fn set_pattern_transform(&mut self, transform: Matrix4) -> &mut Self {
        self.material.set_pattern_transform(transform);
        self
    }

    pub fn get_pattern_inverse_transform(&self) -> Matrix4 {
        self.material.get_pattern_inverse_transform()
    }

    pub fn get_reflective(&self) -> f64 {
        self.material.get_reflective()
    }

    pub fn set_reflective(&mut self, reflective: f64) -> &mut Self {
        self.material.set_reflective(reflective);
        self
    }

    pub fn get_transparency(&self) -> f64 {
        self.material.get_transparency()
    }

    pub fn set_transparency(&mut self, transparency: f64) -> &mut Self {
        self.material.set_transparency(transparency);
        self
    }

    pub fn get_refractive_index(&self) -> f64 {
        self.material.get_refractive_index()
    }

    pub fn set_refractive_index(&mut self, index: f64) -> &mut Self {
        self.material.set_refractive_index(index);
        self
    }

    pub fn casts_shadow(&self) -> bool {
        self.material.casts_shadow()
    }

    pub fn set_casts_shadow(&mut self, cs: bool) -> &mut Self {
        self.material.set_casts_shadow(cs);
        self
    }

    pub fn normal_at(&self, pt: Tuple) -> Tuple {
        let local_point = self.inverse_transform * pt;
        let local_normal = match self.shape {
            Shape::Sphere => spheres::normal_at(local_point),
            Shape::Plane => planes::normal_at(),
            Shape::Cube => cubes::normal_at(local_point),
            Shape::Cylinder {min: _, max: _, closed: _} => cylinders::normal_at(*self, local_point),
            Shape::Cone {min: _, max: _, closed: _} => cones::normal_at(*self, local_point),
        };
        let world_normal = self.inverse_transform_transposed * local_normal;
        world_normal.vectorize().normalize()
    }

    pub fn intersect(self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.get_inverse_transform());
        match self.shape {
            Shape::Sphere => spheres::intersect(self, local_ray),
            Shape::Plane => planes::intersect(self, local_ray),
            Shape::Cube => cubes::intersect(self, local_ray),
            Shape::Cylinder {min: _, max: _, closed: _} => cylinders::intersect(self, local_ray),
            Shape::Cone {min: _, max: _, closed: _} => cones::intersect(self, local_ray),
        }
    }

    pub fn pattern_at_object(&self, point: Tuple) -> Color {
        let local_point = self.inverse_transform * point;
        let pattern_space_point = self.get_pattern_inverse_transform() * local_point;
        self.get_pattern().pattern_at(pattern_space_point)
    }
}
