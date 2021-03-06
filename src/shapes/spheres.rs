use crate::prelude::*;

pub fn new() -> Object {
    Object::new(Shape::Sphere)
}

pub fn glass_sphere() -> Object {
    let mut sphere = Object::new(Shape::Sphere);
    sphere.set_transparency(0.9);
    sphere.set_reflective(0.9);
    sphere.set_refractive_index(1.5);
    sphere.set_color(black());
    sphere.set_casts_shadow(false);
    sphere.set_ambient(0.0);
    sphere.set_diffuse(0.0);
    sphere.set_shininess(300.0);
    sphere.set_specular(0.9);
    sphere
}
pub fn normal_at(pt: Tuple) -> Tuple {
    pt - point(0.0, 0.0, 0.0)
}

pub fn intersect(sphere: Object, ray: Ray) -> Vec<Intersection> {
    let vec_from_sphere_to_ray = ray.get_origin() - origin();
    let a = ray.get_direction() * ray.get_direction();
    let b = 2.0 * (ray.get_direction() * vec_from_sphere_to_ray);
    let c = (vec_from_sphere_to_ray * vec_from_sphere_to_ray) - 1.0;

    let discriminant = b.powi(2) - (4.0 * a * c);
    if discriminant < 0.0 {
        return vec![]
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
    if t1 < t2 {
        vec![Intersection::new(t1, sphere), Intersection::new(t2, sphere)]
    } else {
        vec![Intersection::new(t2, sphere), Intersection::new(t1, sphere)]
    }
}

pub fn hit(intersections: &Vec<Intersection>) -> Option<Intersection> {
    if intersections.is_empty() {
        None
    } else if intersections[0].get_t() >= 0.0 {
        Some(intersections[0])
    } else if intersections[1].get_t() >= 0.0 {
        Some(intersections[1])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let ray = Ray::new(point(0.0, 0.0, -5.0),
                           vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let xs = spheres::intersect(sphere, ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_object(), Intersection::new(4.0, sphere).get_object());
        assert_eq!(xs[0].get_t(), Intersection::new(4.0, sphere).get_t());
        assert_eq!(xs[1].get_object(), Intersection::new(6.0, sphere).get_object());
        assert_eq!(xs[1].get_t(), Intersection::new(6.0, sphere).get_t());
    }

    #[test]
    fn test_tangent_intersect() {
        let ray = Ray::new(point(0.0, 1.0, -5.0),
                           vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let xs = spheres::intersect(sphere, ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_object(), Intersection::new(5.0, sphere).get_object());
        assert_eq!(xs[1].get_object(), Intersection::new(5.0, sphere).get_object());
        assert_eq!(xs[0].get_t(), Intersection::new(5.0, sphere).get_t());
        assert_eq!(xs[1].get_t(), Intersection::new(5.0, sphere).get_t());
    }

    #[test]
    fn test_miss() {
        let ray = Ray::new(point(0.0, 2.0, -5.0),
                           vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let xs = spheres::intersect(sphere, ray);
        assert!(xs.is_empty());
    }

    #[test]
    fn test_ray_originating_inside_sphere() {
        let ray = Ray::new(point(0.0, 0.0, 0.0),
                           vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let xs = spheres::intersect(sphere, ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_object(), Intersection::new(-1.0, sphere).get_object());
        assert_eq!(xs[1].get_object(), Intersection::new(1.0, sphere).get_object());
        assert_eq!(xs[0].get_t(), Intersection::new(-1.0, sphere).get_t());
        assert_eq!(xs[1].get_t(), Intersection::new(1.0, sphere).get_t());
    }

    #[test]
    fn test_intersect_of_scaled_sphere() {
        let ray = Ray::new(point(0.0, 0.0, -5.0),
                           vector(0.0, 0.0, 1.0));

        let mut sphere = spheres::new();
        sphere.set_transform(scaling(2.0, 2.0, 2.0));

        let local_ray = ray.transform(sphere.get_inverse_transform());
        let xs = spheres::intersect(sphere, local_ray);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_object(), Intersection::new(3.0, sphere).get_object());
        assert_eq!(xs[0].get_t(), Intersection::new(3.0, sphere).get_t());
        assert_eq!(xs[1].get_object(), Intersection::new(7.0, sphere).get_object());
        assert_eq!(xs[1].get_t(), Intersection::new(7.0, sphere).get_t());
    }

    #[test]
    fn test_intersect_of_translated_sphere() {
        let ray = Ray::new(point(0.0, 0.0, -5.0),
                           vector(0.0, 0.0, 1.0));

        let mut sphere = spheres::new();
        sphere.set_transform(translation(5.0, 0.0, 0.0));

        let local_ray = ray.transform(sphere.get_inverse_transform());
        assert!(spheres::intersect(sphere, local_ray).is_empty());
    }

    #[test]
    fn test_hit() {
        let ray = Ray::new(point(0.0, 0.0, -5.0),
                           vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let local_ray = ray.transform(sphere.get_inverse_transform());
        let xs = spheres::intersect(sphere, local_ray);
        let hit = spheres::hit(&xs).unwrap();
        assert_eq!(hit.get_t(), xs[0].get_t());
        assert_eq!(hit.get_object(), xs[0].get_object());
    }

    #[test]
    #[should_panic]
    fn test_no_hit() {
        let ray = Ray::new(point(0.0, 0.0, 5.0),
                           vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let local_ray = ray.transform(sphere.get_inverse_transform());
        let xs = spheres::intersect(sphere, local_ray);
        spheres::hit(&xs).unwrap();
    }

    #[test]
    fn test_sphere_normal() {
        let sphere = spheres::new();

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
        let mut sphere = spheres::new();
        sphere.set_transform(translation(0.0, 1.0, 0.0));
        assert_eq!(sphere.normal_at(point(0.0, 1.70711, -FRAC_1_SQRT_2)),
                   vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));

        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI/5.0);

        sphere.set_transform(m);
        assert_eq!(sphere.normal_at(point(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)),
                   vector(0.0, 0.97014, -0.24254));
    }
}
