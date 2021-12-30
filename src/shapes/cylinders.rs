use crate::{Intersection, Ray, objects::*, tuples::*};

pub fn new() -> Object {
    Object::new(Shape::Cylinder)
}

pub fn intersect(cylinder: Object, ray: Ray) -> Vec<Intersection> {
    let mut vec = vec![];
    let a = ray.get_direction().x.powi(2) + ray.get_direction().z.powi(2);
    if a.abs() < 0.00001 {
        return vec;
    }
    let b = 2.0 * ray.get_origin().x * ray.get_direction().x + 2.0 * ray.get_origin().z * ray.get_direction().z;
    let c = ray.get_origin().x.powi(2) + ray.get_origin().z.powi(2) - 1.0;

    let disc = b.powi(2) - 4.0 * a * c;

    if disc < 0.0 {
        return vec;
    }

    let t0 = (-b - disc.sqrt()) / (2.0 * a);
    let t1 = (-b + disc.sqrt()) / (2.0 * a);

    vec.push(Intersection::new(t0, cylinder));
    vec.push(Intersection::new(t1, cylinder));
    vec
}

pub fn normal_at(point: Tuple) -> Tuple {
    vector(point.x, 0.0, point.z)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cylinders;

    #[test]
    fn test_miss_cylinder() {
        let cyl = cylinders::new();

        let r = Ray::new(point(1.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(origin(), vector(0.0, 1.0, 0.0));
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0).normalize());
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_hit_cylinder() {
        let cyl = cylinders::new();

        let r = Ray::new(point(1.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = cyl.intersect(r);
        assert_eq!(xs[0].get_t(), 5.0);
        assert_eq!(xs[1].get_t(), 5.0);

        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = cyl.intersect(r);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 6.0);

        let r = Ray::new(point(0.5, 0.0, -5.0), vector(0.1, 1.0, 1.0).normalize());
        let xs = cyl.intersect(r);
        assert!((xs[0].get_t() - 6.80798).abs() < 0.00001);
        assert!((xs[1].get_t() - 7.08872).abs() < 0.00001);
    }

    #[test]
    fn test_normal_at() {
        let cyl = cylinders::new();

        let n = cyl.normal_at(point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));

        let n = cyl.normal_at(point(0.0, 5.0, -1.0));
        assert_eq!(n, vector(0.0, 0.0, -1.0));

        let n = cyl.normal_at(point(0.0, -2.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));

        let n = cyl.normal_at(point(-1.0, 1.0, 0.0));
        assert_eq!(n, vector(-1.0, 0.0, 0.0));
    }
}