use crate::{Intersection, Material, spheres, transformations};
use crate::matrices::tuples::*;
use crate::objects::Object;
use crate::rays::*;
use crate::scenes::lights::*;
use crate::surfaces::colors::*;

pub struct World {
    objects: Vec<Object>,
    lights: Vec<Light>,
}

impl World {
    pub fn new(objects: Vec<Object>, lights: Vec<Light>) -> Self {
        Self {
            objects,
            lights,
        }
    }

    pub fn new_default() -> Self {
        let mut sphere1 = spheres::new();
        let mut material = Material::new();
        material.set_color(color(0.8, 1.0, 0.6))
            .set_ambient(0.1)
            .set_diffuse(0.7)
            .set_specular(0.2)
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

    fn intersect_world(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];
        for object in &self.objects {
            if let Some(intersection_arr) = object.intersect(ray) {
                intersections.extend(intersection_arr.iter());
            }
        }
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
        intersections
    }

    fn hit_world(&self, intersections: Vec<Intersection>) -> Option<Intersection> {
        if intersections.is_empty() {
            return None;
        }
        for intersection in intersections {
            if intersection.get_t() >= 0.0 {
                return Some(intersection);
            }
        }
        None
    }

    fn shade_hit(&self, comps: Computations) -> Color {
        let mut color = black();
        for light in &self.lights {
            let shadowed = self.is_shadowed(comps.over_point);
            color += lighting(comps.object.get_material(), comps.point, *light, comps.eyev, comps.normalv, shadowed);
        }
        color
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.intersect_world(ray);
        if let Some(intersection) = self.hit_world(intersections) {
            let comps = prepare_computations(intersection, ray);
            self.shade_hit(comps)
        } else {
            black()
        }
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        let vector = self.lights[0].get_position() - point;
        let distance = vector.magnitude();
        let ray = Ray::new(point, vector.normalize());
        let intersections = self.intersect_world(ray);
        if let Some(hit) = self.hit_world(intersections) {
            if hit.get_t() < distance {
                return true;
            }
        }
        false
    }
}

struct Computations {
    object: Object,
    t_value: f64,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    inside: bool,
    over_point: Tuple,
}

impl Computations {
    fn new(intersection: Intersection, point: Tuple, eyev: Tuple, normalv: Tuple, inside: bool) -> Self {
        Self {
            object: intersection.get_object(),
            t_value: intersection.get_t(),
            point,
            eyev,
            normalv,
            inside,
            over_point: point + normalv * 0.00001,
        }
    }
}

fn prepare_computations (intersection: Intersection, ray: Ray) -> Computations {
    let point = ray.position(intersection.get_t());
    let eyev = -ray.get_direction();
    let mut normalv = intersection.get_object().normal_at(point);

    let mut inside = false;
    if normalv * eyev < 0.0 {
        inside = true;
        normalv = -normalv;
    }
    Computations::new(intersection, point, eyev, normalv, inside)
}

#[cfg(test)]
mod tests {
    use crate::{Intersection, Light, Matrix4, Ray, spheres};
    use crate::matrices::tuples::*;
    use crate::objects::Shape::Sphere;
    use crate::surfaces::colors::*;
    use crate::transformations::translation;
    use super::*;

    #[test]
    fn test_default_world_intersect() {
        let w = World::new_default();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = w.intersect_world(ray);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 4.5);
        assert_eq!(xs[2].get_t(), 5.5);
        assert_eq!(xs[3].get_t(), 6.0);
    }

    #[test]
    fn test_precompute_when_ray_inside_object() {
        let ray = Ray::new(origin(), vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let intersection = Intersection::new(1.0, sphere);

        let comps = prepare_computations(intersection, ray);

        assert_eq!(comps.point, point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_precompute_offsets_point() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut shape = spheres::new();
        shape.set_transform(translation(0.0, 0.0, 1.0));

        let i = Intersection::new(5.0, shape);

        let comps = prepare_computations(i, ray);

        assert!(comps.over_point.z < -0.00001/2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn test_shade_hit() {
        let w = World::new_default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[0];
        let i = Intersection::new(4.0, shape);

        let comps = prepare_computations(i, r);
        assert_eq!(w.shade_hit(comps), color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shade_hit_ray_inside() {
        let w_default = World::new_default();
        let light = Light::new(point(0.0, 0.25, 0.0), white());

        let w = World::new(w_default.objects, vec![light]);
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);

        let comps = prepare_computations(i, r);
        assert_eq!(w.shade_hit(comps), color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_shade_hit_in_shadow() {
        let mut w = World::new(vec![], vec![]);
        w.lights.push(Light::new(point(0.0, 0.0, -10.0), white()));
        w.objects.push(spheres::new());
        w.objects.push(spheres::new());
        w.objects[1].set_transform(Matrix4::identity().translate(0.0, 0.0, 10.0));

        let ray = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.objects[1]);

        let comps = prepare_computations(i, ray);
        let c = w.shade_hit(comps);

        assert_eq!(c, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_color_at_when_no_intersection() {
        let w = World::new_default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));

        assert_eq!(w.color_at(r), black());
    }

    #[test]
    fn test_color_at_with_intersection() {
        let w = World::new_default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        assert_eq!(w.color_at(r), color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_color_at_inside_sphere() {
        let mut w = World::new_default();
        let mut sph1_material = w.objects[0].get_material();
        sph1_material.set_ambient(1.0);
        w.objects[0].set_material(sph1_material);

        let mut sph2_material = w.objects[1].get_material();
        sph2_material.set_ambient(1.0);
        w.objects[1].set_material(sph2_material);

        let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));

        let c = w.color_at(r);
        assert_eq!(c, w.objects[1].get_color());
    }

    #[test]
    fn test_no_shadow() {
        let w = World::new_default();
        assert!(!w.is_shadowed(point(0.0, 10.0, 0.0)));
        assert!(!w.is_shadowed(point(-20.0, 20.0, -20.0)));
        assert!(!w.is_shadowed(point(-2.0, 2.0, -2.0)));
    }

    #[test]
    fn test_shadowed() {
        let w = World::new_default();
        assert!(w.is_shadowed(point(10.0, -10.0, 10.0)));
    }
}