use crate::matrices::matrix::*;
use crate::materials::*;
use crate::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    radius: f64,
    pub(crate) origin: Tuple,
    pub(crate) transform: Matrix,
    pub(crate) material: Material,
}

impl Sphere {
    pub fn new(radius: f64, origin: Tuple) -> Self {
        if origin.v == 0 {
            panic!("Attempted to create sphere with vector origin rather than point");
        }
        Self {
            radius,
            origin,
            transform: Matrix::identity(4),
            material: Material::new_default(),
        }
    }

    pub(crate) fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub(crate) fn set_material(&mut self, material: Material) { self.material = material; }

    pub(crate) fn normal_at(&self, point: Tuple) -> Tuple {
        let object_point = &self.transform.invert().unwrap().tuple_mul(&point);
        let object_normal = object_point.subtract(&Tuple::point(0.0, 0.0, 0.0));
        let world_normal = &self.transform.invert().unwrap().transpose().tuple_mul(&object_normal);
        let result = Tuple::vector(world_normal.x, world_normal.y, world_normal.z);
        result.normalize()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::{Sphere, transformations, Tuple};
    use crate::matrix::Matrix;

    #[test]
    fn test_normal() {
        let sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(sphere.normal_at(Tuple::point(1.0, 0.0, 0.0)), Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(sphere.normal_at(Tuple::point(0.0, 1.0, 0.0)), Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(sphere.normal_at(Tuple::point(0.0, 0.0, 1.0)), Tuple::vector(0.0, 0.0, 1.0));
        let f = f64::sqrt(3.0) / 3.0;
        let n = sphere.normal_at(Tuple::point(f, f, f));
        assert_eq!(n, Tuple::vector(f, f, f));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_normal_transformed() {
        let mut sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
        sphere.set_transform(transformations::translation(0.0, 1.0, 0.0));
        assert_eq!(sphere.normal_at(Tuple::point(0.0, 1.70711, -0.70711)),
                   Tuple::vector(0.0, 0.70711, -0.70711));

        let m = transformations::scaling(1.0, 0.5, 1.0)
            .mul(&transformations::rotation_z(PI/5.0));

        sphere.set_transform(m);
        assert_eq!(sphere.normal_at(Tuple::point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / -2.0)),
                   Tuple::vector(0.0, 0.97014, -0.24254));
    }
}