use crate::{colors::*, Light, Material, spheres, transformations, tuples::*};
use crate::objects::Object;

struct World {
    objects: Vec<Object>,
    lights: Vec<Light>,
}

impl World {
    fn new(objects: Vec<Object>, lights: Vec<Light>) -> Self {
        Self {
            objects,
            lights,
        }
    }

    fn new_default() -> Self {
        let mut sphere1 = spheres::new();
        let mut material = Material::new();
        material.set_color(color(0.8, 1.0, 0.6))
            .set_ambient(0.1)
            .set_diffuse(0.9)
            .set_specular(0.9)
            .set_shininess(200.0);
        sphere1.set_material(material);

        let mut sphere2 = spheres::new();
        sphere2.set_transform(transformations::scaling(0.5, 0.5, 0.5));

        let objects = vec![sphere1, sphere2];

        let light = Light::new(
            point(-10.0, 10.0, -10.0),
            color(1.0, 1.0, 1.0));

        let lights = vec![light];

        World::new(objects, lights)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_default_world_intersect() {

    }
}