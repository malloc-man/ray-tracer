use crate::{Color, Intersection, Material, Ray, spheres, planes, Pattern, cubes, cylinders};
use crate::matrices::tuples::*;
use crate::matrix4::Matrix4;

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
        }
    }

    pub fn pattern_at_object(&self, point: Tuple) -> Color {
        let local_point = self.inverse_transform * point;
        let pattern_space_point = self.get_pattern_inverse_transform() * local_point;
        self.get_pattern().pattern_at(pattern_space_point)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::{spheres, transformations, surfaces::colors::*, surfaces::patterns::*};
    use crate::shapes::objects::Object;
    use crate::shapes::objects::Shape::Sphere;
    use crate::matrices::tuples::*;
    use crate::transformations::{scaling, translation};

    #[test]
    fn test_sphere_normal() {
        let sphere = Object::new(Sphere);

        assert_eq!(sphere.normal_at(point(1.0, 0.0, 0.0)), vector(1.0, 0.0, 0.0));
        assert_eq!(sphere.normal_at(point(0.0, 1.0, 0.0)), vector(0.0, 1.0, 0.0));
        assert_eq!(sphere.normal_at(point(0.0, 0.0, 1.0)), vector(0.0, 0.0, 1.0));

        let f = f64::sqrt(3.0) / 3.0;

        let n = sphere.normal_at(point(f, f, f));
        assert_eq!(n, vector(f, f, f));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_sphere_normal_transformed() {
        let mut sphere = Object::new(Sphere);
        sphere.set_transform(transformations::translation(0.0, 1.0, 0.0));
        assert_eq!(sphere.normal_at(point(0.0, 1.70711, -0.70711)),
                   vector(0.0, 0.70711, -0.70711));

        let m = transformations::scaling(1.0, 0.5, 1.0) * transformations::rotation_z(PI/5.0);

        sphere.set_transform(m);
        assert_eq!(sphere.normal_at(point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / -2.0)),
                   vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn test_stripe_at_object_with_obj_transformation() {
        let mut object = spheres::new();
        object.set_transform(transformations::scaling(2.0, 2.0, 2.0));
        object.set_pattern(stripe(white(), black()));

        let c = object.pattern_at_object(point(1.5, 0.0, 0.0));
        assert_eq!(c, white());
    }

    #[test]
    fn test_stripe_at_object_with_pattern_transformation() {
        let mut object = spheres::new();
        object.set_pattern(stripe(white(), black()));
        object.set_pattern_transform(scaling(2.0, 2.0, 2.0));
        let c = object.pattern_at_object(point(1.5, 0.0, 0.0));
        assert_eq!(c, white());
    }

    #[test]
    fn test_stripe_at_object_with_pattern_and_obj_transformations() {
        let mut object = spheres::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        object.set_pattern(stripe(white(), black()));
        object.set_pattern_transform(translation(0.5, 0.0, 0.0));
        let c = object.pattern_at_object(point(2.5, 0.0, 0.0));
        assert_eq!(c, white());
    }
}