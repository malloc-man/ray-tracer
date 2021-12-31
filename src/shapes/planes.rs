use crate::{objects::*, tuples::*, rays::*};
use crate::utils::ApproxEq;

pub fn new() -> Object {
    Object::new(Shape::Plane)
}

pub fn normal_at() -> Tuple {
    vector(0.0, 1.0, 0.0)
}

pub fn intersect(plane: Object, ray: Ray) -> Vec<Intersection> {
    if ray.get_direction().y.approx_eq(0.0) {
        return vec![]
    }
    let t = -ray.get_origin().y / ray.get_direction().y;
    vec![Intersection::new(t, plane)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_parallel_ray() {
        let p = new();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = intersect(p, r);
        assert!(xs.is_empty());
    }

    #[test]
    fn test_intersection_coplanar_ray() {
        let p = new();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = intersect(p, r);
        assert!(xs.is_empty());
    }

    #[test]
    fn test_intersection_ray_above_plane() {
        let p = new();
        let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = intersect(p, r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].get_t(), 1.0);
        assert_eq!(xs[0].get_object(), p);
    }

    #[test]
    fn test_intersection_ray_below_plane() {
        let p = new();
        let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = intersect(p, r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].get_t(), 1.0);
        assert_eq!(xs[0].get_object(), p);
    }
}