use std::cmp::min;
use crate::geometry::sphere::*;
use crate::matrices::matrix::Matrix;
use crate::tuples::Tuple;

pub(crate) struct Ray {
    origin: Tuple,
    pub(crate) direction: Tuple
}

impl Ray {
    pub(crate) fn new(origin: Tuple, direction: Tuple) -> Self {
        if origin.v != 1 {
            panic!("Attempted to create ray with vector origin");
        } else if direction.v != 0 {
            panic!("Attempted to create ray with point direction");
        } else {
            Self {
                origin,
                direction,
            }
        }
    }

    pub(crate) fn position(&self, t: f64) -> Tuple {
        let pos = &self.origin;
        let dir = &self.direction;
        pos.add(&dir.scalar_mult_vec(t))
    }

    pub(crate) fn sphere_intersect<'a>(&self, other: &'a Sphere) -> Vec<Intersection<&'a Sphere>> {
        let transformed_ray = self.transform(&other.transform.invert().unwrap());
        let vec_from_sphere_to_ray = transformed_ray.origin.subtract(&other.origin);
        let a = transformed_ray.direction.dot_product(&transformed_ray.direction);
        let b = 2.0 * transformed_ray.direction.dot_product(&vec_from_sphere_to_ray);
        let c = vec_from_sphere_to_ray.dot_product(&vec_from_sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - (4.0 * a * c);

        let mut intersections: Vec<Intersection<&Sphere>> = vec![];
        if discriminant < 0.0 {
            return intersections;
        }
        let t1 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        let t2 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
        if t1 < t2 {
            intersections.push(Intersection::new(t1, &other));
            intersections.push(Intersection::new(t2, &other));
        } else {
            intersections.push(Intersection::new(t2, &other));
            intersections.push(Intersection::new(t1, &other));
        }
        intersections
    }

    fn transform(&self, matrix: &Matrix) -> Self {
        let new_origin = matrix.tuple_mul(&self.origin);
        let new_direction = matrix.tuple_mul(&self.direction);
        Ray::new(new_origin, new_direction)
    }
}

pub(crate) fn hit<T>(intersections: &Vec<Intersection<T>>) -> Option<&Intersection<T>> {
    if intersections.len() == 0 {
        return None;
    }
    let mut min_t_value = f64::MAX;
    let mut min_intersection = &intersections[0];
    for intersection in intersections {
        if intersection.t_value > 0.0 && intersection.t_value < min_t_value {
            min_t_value = intersection.t_value;
            min_intersection = intersection;
        }
        if min_intersection.t_value < 0.0 && intersection.t_value > 0.0 {
            min_t_value = intersection.t_value;
            min_intersection = intersection;
        }
    }
    if min_intersection.t_value > 0.0 {
        Some(min_intersection)
    } else {
        None
    }
}

pub struct Intersection<T> {
    pub(crate) t_value: f64,
    pub(crate) object: T,
}

impl<T> Intersection<T> {
    fn new(t_value: f64, object: T) -> Self {
        Self {
            t_value,
            object,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::Tuple;
    use super::*;
    use crate::matrices::transformations;

    #[test]
    fn test_position() {
        let ray = Ray::new(Tuple::point(2.0, 3.0, 4.0),
                           Tuple::vector(1.0, 0.0, 0.0));
        assert_eq!(ray.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_intersect() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
        let xs = ray.sphere_intersect(&sphere);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, Intersection::new(4.0, &sphere).object);
        assert_eq!(xs[0].t_value, Intersection::new(4.0, &sphere).t_value);
        assert_eq!(xs[1].object, Intersection::new(6.0, &sphere).object);
        assert_eq!(xs[1].t_value, Intersection::new(6.0, &sphere).t_value);
    }

    #[test]
    fn test_tangent_intersect() {
        let ray = Ray::new(Tuple::point(0.0, 1.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
        let xs = ray.sphere_intersect(&sphere);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, Intersection::new(5.0, &sphere).object);
        assert_eq!(xs[1].object, Intersection::new(5.0, &sphere).object);
        assert_eq!(xs[0].t_value, Intersection::new(5.0, &sphere).t_value);
        assert_eq!(xs[1].t_value, Intersection::new(5.0, &sphere).t_value);
    }

    #[test]
    fn test_miss() {
        let ray = Ray::new(Tuple::point(0.0, 2.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
        let xs = ray.sphere_intersect(&sphere);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_ray_originating_inside_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 0.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
        let xs = ray.sphere_intersect(&sphere);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, Intersection::new(-1.0, &sphere).object);
        assert_eq!(xs[1].object, Intersection::new(1.0, &sphere).object);
        assert_eq!(xs[0].t_value, Intersection::new(-1.0, &sphere).t_value);
        assert_eq!(xs[1].t_value, Intersection::new(1.0, &sphere).t_value);
    }

    #[test]
    fn test_hit() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
        let xs = ray.sphere_intersect(&sphere);
        let hit = hit(&xs).unwrap();
        assert_eq!(hit.t_value, xs[0].t_value);
        assert_eq!(hit.object, xs[0].object);
    }

    #[test]
    #[should_panic]
    fn test_no_hit() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, 5.0),
                           Tuple::vector(0.0, 0.0, 1.0));
        let sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
        let xs = ray.sphere_intersect(&sphere);
        let hit = hit(&xs).unwrap();
    }

    #[test]
    fn test_ray_translate() {
        let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0),
                           Tuple::vector(0.0, 1.0, 0.0));
        let m = transformations::translation(3.0, 4.0, 5.0);
        let transformed_ray = ray.transform(&m);
        assert_eq!(transformed_ray.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(transformed_ray.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_ray_scale() {
        let ray = Ray::new(Tuple::point(1.0, 2.0, 3.0),
                           Tuple::vector(0.0, 1.0, 0.0));
        let m = transformations::scaling(2.0, 3.0, 4.0);
        let transformed_ray = ray.transform(&m);
        assert_eq!(transformed_ray.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(transformed_ray.direction, Tuple::vector(0.0, 3.0, 0.0));
    }

    #[test]
    fn test_intersect_of_scaled_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));

        let mut sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
        sphere.set_transform(transformations::scaling(2.0, 2.0, 2.0));

        let xs = ray.sphere_intersect(&sphere);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, Intersection::new(3.0, &sphere).object);
        assert_eq!(xs[0].t_value, Intersection::new(3.0, &sphere).t_value);
        assert_eq!(xs[1].object, Intersection::new(7.0, &sphere).object);
        assert_eq!(xs[1].t_value, Intersection::new(7.0, &sphere).t_value);
    }

    #[test]
    fn test_intersect_of_translated_sphere() {
        let ray = Ray::new(Tuple::point(0.0, 0.0, -5.0),
                           Tuple::vector(0.0, 0.0, 1.0));

        let mut sphere = Sphere::new(1.0, Tuple::point(0.0, 0.0, 0.0));
        sphere.set_transform(transformations::translation(5.0, 0.0, 0.0));

        let xs = ray.sphere_intersect(&sphere);
        assert_eq!(xs.len(), 0);
    }
}