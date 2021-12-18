use crate::{Color, Intersection, Material, Ray, spheres};
use crate::matrices::tuples::*;
use crate::matrix4::Matrix4;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Object {
    material: Material,
    transform: Matrix4,
    shape: Shape,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere,
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self {
            material: Material::new(),
            transform: Matrix4::identity(),
            shape,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4) -> &Self {
        self.transform = transform;
        self
    }

    pub fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    pub fn set_material(&mut self, material: Material) -> &Self {
        self.material = material;
        self
    }

    pub fn get_material(&self) -> Material {
        self.material
    }

    pub fn set_color(&mut self, color: Color) -> &Self {
        self.material.set_color(color);
        self
    }

    pub fn get_color(&self) -> Color {
        self.material.get_color()
    }

    pub fn normal_at(&self, pt: Tuple) -> Tuple {
        let object_point = self.transform.invert() * pt;
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let world_normal = self.transform.invert().transpose() * object_normal;
        world_normal.vectorize().normalize()
    }

    pub fn intersect(self, ray: Ray) -> Option<[Intersection; 2]> {
        match self.shape {
            Shape::Sphere => spheres::intersect(self, ray),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::transformations;
    use crate::shapes::objects::Object;
    use crate::shapes::objects::Shape::Sphere;
    use crate::matrices::tuples::*;

    #[test]
    fn test_normal() {
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
    fn test_normal_transformed() {
        let mut sphere = Object::new(Sphere);
        sphere.set_transform(transformations::translation(0.0, 1.0, 0.0));
        assert_eq!(sphere.normal_at(point(0.0, 1.70711, -0.70711)),
                   vector(0.0, 0.70711, -0.70711));

        let m = transformations::scaling(1.0, 0.5, 1.0) * transformations::rotation_z(PI/5.0);

        sphere.set_transform(m);
        assert_eq!(sphere.normal_at(point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / -2.0)),
                   vector(0.0, 0.97014, -0.24254));
    }
}