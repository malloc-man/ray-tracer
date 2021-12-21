use crate::{Intersection, Material, spheres, transformations};
use crate::matrices::tuples::*;
use crate::objects::Object;
use crate::rays::*;
use crate::scenes::lights::*;
use crate::surfaces::colors::*;
use crate::surfaces::patterns::*;

pub const DEFAULT_REFLECTION_DEPTH: usize = 5;

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
        material.set_pattern(solid(color(0.8, 1.0, 0.6)))
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

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object)
    }

    fn intersect_world(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];
        for object in &self.objects {
            let object_intersections = object.intersect(ray);
            intersections.extend(object_intersections.iter());
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

    fn shade_hit(&self, comps: Computations, remaining: usize) -> Color {
        let mut color = black();
        let mut reflections = black();
        let mut refractions = black();
        for light in &self.lights {
            let shadowed = self.is_shadowed(comps.over_point);
            color += lighting(comps.object.get_material(), comps.object, *light, comps.over_point, comps.eyev, comps.normalv, shadowed);
            reflections += self.reflected_color(comps, remaining);
            refractions += self.refracted_color(comps, remaining);
        }
        color + reflections + refractions
    }

    pub fn color_at(&self, ray: Ray, remaining: usize) -> Color {
        let intersections = self.intersect_world(ray);
        if let Some(intersection) = self.hit_world(intersections.clone()) {
            let comps = prepare_computations(intersection, ray, &intersections);
            self.shade_hit(comps, remaining)
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

    fn reflected_color(&self, comps: Computations, remaining: usize) -> Color {
        if remaining < 1 {
            return black();
        }

        let reflective = comps.object.get_reflective();
        if reflective == 0.0 {
            return black();
        }

        let ray = Ray::new(comps.over_point, comps.reflectv);
        let clr = self.color_at(ray, remaining - 1);
        clr * reflective
    }

    fn refracted_color(&self, comps: Computations, remaining: usize) -> Color {
        if comps.object.get_transparency() == 0.0 || remaining == 0 {
            return black();
        }
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev * comps.normalv;
        let sin2_t = n_ratio.powi(2) * (1.0-(cos_i.powi(2)));
        if sin2_t > 1.0 {
            return black();
        }
        let cos_t = f64::sqrt(1.0 - sin2_t);
        let direction = comps.normalv * ((n_ratio * cos_i) - cos_t) - comps.eyev * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);

        self.color_at(refract_ray, remaining - 1) * comps.object.get_transparency()
    }
}

/* ----------------------------------------------------------------------------------------- */

#[derive(Copy, Clone)]
struct Computations {
    object: Object,
    t_value: f64,
    point: Tuple,
    eyev: Tuple,
    normalv: Tuple,
    inside: bool,
    over_point: Tuple,
    under_point: Tuple,
    reflectv: Tuple,
    n1: f64,
    n2: f64,
}

impl Computations {
    fn new(intersection: Intersection, point: Tuple, eyev: Tuple, normalv: Tuple, inside: bool, reflectv: Tuple, n1: f64, n2: f64) -> Self {
        Self {
            object: intersection.get_object(),
            t_value: intersection.get_t(),
            point,
            eyev,
            normalv,
            inside,
            over_point: point + normalv * 0.00001,
            under_point: point - normalv * 0.00001,
            reflectv,
            n1,
            n2,
        }
    }
}

fn prepare_computations (intersection: Intersection, ray: Ray, intersection_list: &Vec<Intersection>) -> Computations {
    let point = ray.position(intersection.get_t());
    let eyev = -ray.get_direction();
    let mut normalv = intersection.get_object().normal_at(point);
    let mut inside = false;
    if normalv * eyev < 0.0 {
        inside = true;
        normalv = -normalv;
    }
    let reflectv = ray.get_direction().reflect_vector(normalv);

    // Compute refraction
    let mut n1 = 1.0;
    let mut n2 = 1.0;
    let mut containers: Vec<Object> = vec![];
    for i in intersection_list {
        if i == &intersection {
            if !containers.is_empty() {
                n1 = containers[containers.len()-1].get_refractive_index();
            }
        }
        if containers.contains(&i.get_object()) {
            let index = containers.iter().position(|x| x == &i.get_object()).unwrap();
            containers.remove(index);
        } else {
            containers.push(i.get_object());
        }
        if i == &intersection {
            if !containers.is_empty() {
                n2 = containers[containers.len()-1].get_refractive_index();
            }
            break;
        }
    }
    Computations::new(intersection, point, eyev, normalv, inside, reflectv, n1, n2)
}

/* ----------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use crate::{Intersection, Light, Matrix4, Ray, spheres, planes};
    use crate::matrices::tuples::*;
    use crate::surfaces::colors::*;
    use crate::transformations::{scaling, translation};
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

        let comps = prepare_computations(intersection, ray, &vec![intersection]);

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

        let comps = prepare_computations(i, ray, &vec![i]);

        assert!(comps.over_point.z < -0.00001/2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn test_shade_hit() {
        let w = World::new_default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[0];
        let i = Intersection::new(4.0, shape);

        let comps = prepare_computations(i, r, &vec![i]);
        assert_eq!(w.shade_hit(comps, DEFAULT_REFLECTION_DEPTH), color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shade_hit_ray_inside() {
        let w_default = World::new_default();
        let light = Light::new(point(0.0, 0.25, 0.0), white());

        let w = World::new(w_default.objects, vec![light]);
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);

        let comps = prepare_computations(i, r, &vec![i]);
        assert_eq!(w.shade_hit(comps, DEFAULT_REFLECTION_DEPTH), color(0.90498, 0.90498, 0.90498));
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

        let comps = prepare_computations(i, ray, &vec![i]);
        let c = w.shade_hit(comps, DEFAULT_REFLECTION_DEPTH);

        assert_eq!(c, color(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_color_at_when_no_intersection() {
        let w = World::new_default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));

        assert_eq!(w.color_at(r, DEFAULT_REFLECTION_DEPTH), black());
    }

    #[test]
    fn test_color_at_with_intersection() {
        let w = World::new_default();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));

        assert_eq!(w.color_at(r, DEFAULT_REFLECTION_DEPTH), color(0.38066, 0.47583, 0.2855));
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

        let c = w.color_at(r, DEFAULT_REFLECTION_DEPTH);
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

    #[test]
    fn test_compute_reflection_vector() {
        let shape = planes::new();
        let ray = Ray::new(point(0.0, 1.0, -1.0),
                           vector(0.0, -f64::sqrt(2.0)/2.0, f64::sqrt(2.0)/2.0));
        let i = Intersection::new(f64::sqrt(2.0), shape);
        let comps = prepare_computations(i, ray, &vec![i]);
        assert_eq!(comps.reflectv, vector(0.0, f64::sqrt(2.0)/2.0, f64::sqrt(2.0)/2.0));
    }

    #[test]
    fn test_reflected_color_of_nonreflective_surface() {
        let mut w = World::new_default();
        let r = Ray::new(origin(), vector(0.0, 0.0, 1.0));
        let shape = &mut w.objects[1];
        shape.set_ambient(1.0);

        let i = Intersection::new(1.0, *shape);
        let comps = prepare_computations(i, r, &vec![i]);
        let color = w.reflected_color(comps, DEFAULT_REFLECTION_DEPTH);

        assert_eq!(color, black());
    }

    #[test]
    fn test_reflected_color_of_reflective_surface() {
        let mut w = World::new_default();
        let mut shape = planes::new();
        shape.set_reflective(0.5);
        shape.set_transform(translation(0.0, -1.0, 0.0));
        w.add_object(shape);

        let r = Ray::new(point(0.0, 0.0, -3.0),
                         vector(0.0, -f64::sqrt(2.0)/2.0, f64::sqrt(2.0)/2.0));
        let i = Intersection::new(f64::sqrt(2.0), shape);

        let comps = prepare_computations(i, r, &vec![i]);
        let clr = w.reflected_color(comps, DEFAULT_REFLECTION_DEPTH);

        assert_eq!(clr, color(0.19033, 0.23791, 0.14275));
    }

    #[test]
    fn test_shade_hit_with_reflection() {
        let mut w = World::new_default();
        let mut shape = planes::new();
        shape.set_reflective(0.5);
        shape.set_transform(translation(0.0, -1.0, 0.0));
        w.add_object(shape);

        let r = Ray::new(point(0.0, 0.0, -3.0),
                         vector(0.0, f64::sqrt(2.0)/-2.0, f64::sqrt(2.0)/2.0));
        let i = Intersection::new(f64::sqrt(2.0), shape);

        let comps = prepare_computations(i, r, &vec![i]);
        let clr = w.shade_hit(comps, DEFAULT_REFLECTION_DEPTH);

        assert_eq!(clr, color(0.87676, 0.92434, 0.82918));
    }

    #[test]
    fn test_reflection_avoids_infinite_recursion() {
        let mut w = World::new_default();
        let mut shape = planes::new();
        shape.set_reflective(0.5);
        shape.set_transform(translation(0.0, -1.0, 0.0));
        w.add_object(shape);

        let r = Ray::new(point(0.0, 0.0, -3.0),
                         vector(0.0, -f64::sqrt(2.0)/2.0, f64::sqrt(2.0)/2.0));
        let i = Intersection::new(f64::sqrt(2.0), shape);

        let comps = prepare_computations(i, r, &vec![i]);
        let clr = w.reflected_color(comps, 0);

        assert_eq!(clr, black());
    }

    #[test]
    fn test_computing_refractive_indices() {
        let mut a = spheres::glass_sphere();
        a.set_transform(scaling(2.0, 2.0, 2.0));
        a.set_refractive_index(1.5);

        let mut b = spheres::glass_sphere();
        b.set_transform(translation(0.0, 0.0, -0.25));
        b.set_refractive_index(2.0);

        let mut c = spheres::glass_sphere();
        c.set_transform(translation(0.0, 0.0, 0.25));
        c.set_refractive_index(2.5);

        let r = Ray::new(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(2.0, a),
            Intersection::new(2.75, b),
            Intersection::new(3.25, c),
            Intersection::new(4.75, b),
            Intersection::new(5.25, c),
            Intersection::new(6.0, a)];

        let comps = prepare_computations(xs[0], r, &xs);
        assert_eq!(comps.n1, 1.0);
        assert_eq!(comps.n2, 1.5);
        let comps = prepare_computations(xs[1], r, &xs);
        assert_eq!(comps.n1, 1.5);
        assert_eq!(comps.n2, 2.0);
        let comps = prepare_computations(xs[2], r, &xs);
        assert_eq!(comps.n1, 2.0);
        assert_eq!(comps.n2, 2.5);
        let comps = prepare_computations(xs[3], r, &xs);
        assert_eq!(comps.n1, 2.5);
        assert_eq!(comps.n2, 2.5);
        let comps = prepare_computations(xs[4], r, &xs);
        assert_eq!(comps.n1, 2.5);
        assert_eq!(comps.n2, 1.5);
        let comps = prepare_computations(xs[5], r, &xs);
        assert_eq!(comps.n1, 1.5);
        assert_eq!(comps.n2, 1.0);
    }

    #[test]
    fn test_refractive_color_opaque() {
        let w = World::new_default();
        let shape = w.objects[0];
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(4.0, shape),
            Intersection::new(6.0, shape)];
        let comps = prepare_computations(xs[0], r, &xs);
        let c = w.refracted_color(comps, 5);
        assert_eq!(c, black());
    }

    #[test]
    fn test_refracted_color_max_recursive_depth() {
        let w = World::new_default();
        let mut shape = w.objects[0];
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(4.0, shape),
            Intersection::new(6.0, shape)];
        let comps = prepare_computations(xs[0], r, &xs);
        let c = w.refracted_color(comps, 0);
        assert_eq!(c, black());
    }

    #[test]
    fn test_total_internal_refraction() {
        let w = World::new_default();
        let mut shape = w.objects[0];
        let r = Ray::new(point(0.0, 0.0, f64::sqrt(2.0)/2.0), vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(f64::sqrt(2.0)/-2.0, shape),
            Intersection::new(f64::sqrt(2.0)/2.0, shape)];
        let comps = prepare_computations(xs[1], r, &xs);
        let c = w.refracted_color(comps, 5);
        assert_eq!(c, black());
    }

    #[test]
    fn test_refraction() {
        let mut w = World::new_default();
        w.objects[0].set_ambient(1.0);
        w.objects[0].set_pattern(test_pattern());
        w.objects[1].set_transparency(1.0);
        w.objects[1].set_refractive_index(1.5);
        let r = Ray::new(point(0.0, 0.0, 0.1), vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-0.9899, w.objects[0]),
            Intersection::new(-0.4899, w.objects[1]),
            Intersection::new(0.4899, w.objects[1]),
            Intersection::new(0.9899, w.objects[0])];
        let comps = prepare_computations(xs[2], r, &xs);
        let c = w.refracted_color(comps, 5);
        assert_eq!(c, color(0.0, 0.99887, 0.04722));
    }

    #[test]
    fn test_shade_hit_with_refraction() {
        let mut w = World::new_default();

        let mut floor = planes::new();
        floor.set_transform(translation(0.0, -1.0, 0.0));
        floor.set_transparency(0.5);
        floor.set_refractive_index(1.5);
        w.add_object(floor);

        let mut ball = spheres::new();
        ball.set_pattern(solid(color(1.0, 0.0, 0.0)));
        ball.set_ambient(0.5);
        ball.set_transform(translation(0.0, -3.5, -0.5));
        w.add_object(ball);

        let sqrt2 = f64::sqrt(2.0);
        let r = Ray::new(point(0.0, 0.0, -3.0),
                         vector(0.0, -sqrt2/2.0, sqrt2/2.0));
        let xs = vec![Intersection::new(sqrt2, floor)];

        let comps = prepare_computations(xs[0], r, &xs);
        let clr = w.shade_hit(comps, 5);

        assert_eq!(clr, color(0.93642, 0.68642, 0.68642));
    }
}