use crate::{Intersection, Ray, Tuple};
use crate::geometry::objects::{Object, Shape};

pub fn new() -> Object {
    Object::new(Shape::Sphere)
}

pub fn intersect(sphere: Object, ray: Ray) -> Option<[Intersection; 2]> {

    let transformed_ray = ray.transform(sphere.get_transform().invert());

    let vec_from_sphere_to_ray = transformed_ray.get_origin() - Tuple::origin();
    let a = transformed_ray.get_direction() * transformed_ray.get_direction();
    let b = 2.0 * (transformed_ray.get_direction() * vec_from_sphere_to_ray);
    let c = (vec_from_sphere_to_ray * vec_from_sphere_to_ray) - 1.0;

    let discriminant = b.powi(2) - (4.0 * a * c);
    if discriminant < 0.0 {
        return None;
    }

    let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
    let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
    if t1 < t2 {
        Some([Intersection::new(t1, sphere), Intersection::new(t2, sphere)])
    } else {
        Some([Intersection::new(t2, sphere), Intersection::new(t1, sphere)])
    }
}

pub(crate) fn hit(intersections: [Intersection; 2]) -> Option<Intersection> {
    let mut min_t_value = f64::MAX;
    let mut min_intersection = intersections[0];
    for intersection in intersections {
        if intersection.get_t() > 0.0 && intersection.get_t() < min_t_value {
            min_t_value = intersection.get_t();
            min_intersection = intersection;
        }
        if min_intersection.get_t() < 0.0 && intersection.get_t() > 0.0 {
            min_t_value = intersection.get_t();
            min_intersection = intersection;
        }
    }
    if min_intersection.get_t() > 0.0 {
        Some(min_intersection)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{Intersection, Ray, spheres, transformations, Tuple};

    #[test]
    fn test_intersect() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let xs = spheres::intersect(sphere, ray).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_object(), Intersection::new(4.0, sphere).get_object());
        assert_eq!(xs[0].get_t(), Intersection::new(4.0, sphere).get_t());
        assert_eq!(xs[1].get_object(), Intersection::new(6.0, sphere).get_object());
        assert_eq!(xs[1].get_t(), Intersection::new(6.0, sphere).get_t());
    }

    #[test]
    fn test_tangent_intersect() {
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let xs = spheres::intersect(sphere, ray).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_object(), Intersection::new(5.0, sphere).get_object());
        assert_eq!(xs[1].get_object(), Intersection::new(5.0, sphere).get_object());
        assert_eq!(xs[0].get_t(), Intersection::new(5.0, sphere).get_t());
        assert_eq!(xs[1].get_t(), Intersection::new(5.0, sphere).get_t());
    }

    #[test]
    #[should_panic]
    fn test_miss() {
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        spheres::intersect(sphere, ray).unwrap();
    }

    #[test]
    fn test_ray_originating_inside_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let xs = spheres::intersect(sphere, ray).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_object(), Intersection::new(-1.0, sphere).get_object());
        assert_eq!(xs[1].get_object(), Intersection::new(1.0, sphere).get_object());
        assert_eq!(xs[0].get_t(), Intersection::new(-1.0, sphere).get_t());
        assert_eq!(xs[1].get_t(), Intersection::new(1.0, sphere).get_t());
    }

    #[test]
    fn test_intersect_of_scaled_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));

        let mut sphere = spheres::new();
        sphere.set_transform(transformations::scaling(2.0, 2.0, 2.0));

        let xs = spheres::intersect(sphere, ray).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_object(), Intersection::new(3.0, sphere).get_object());
        assert_eq!(xs[0].get_t(), Intersection::new(3.0, sphere).get_t());
        assert_eq!(xs[1].get_object(), Intersection::new(7.0, sphere).get_object());
        assert_eq!(xs[1].get_t(), Intersection::new(7.0, sphere).get_t());
    }

    #[test]
    #[should_panic]
    fn test_intersect_of_translated_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));

        let mut sphere = spheres::new();
        sphere.set_transform(transformations::translation(5.0, 0.0, 0.0));

        spheres::intersect(sphere, ray).unwrap();
    }

    #[test]
    fn test_hit() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let xs = spheres::intersect(sphere, ray).unwrap();
        let hit = spheres::hit(xs).unwrap();
        assert_eq!(hit.get_t(), xs[0].get_t());
        assert_eq!(hit.get_object(), xs[0].get_object());
    }

    #[test]
    #[should_panic]
    fn test_no_hit() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = spheres::new();
        let xs = spheres::intersect(sphere, ray).unwrap();
        spheres::hit(xs).unwrap();
    }
}