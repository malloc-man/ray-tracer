use crate::Intersection;
use crate::objects::*;
use crate::rays::Ray;
use crate::tuples::*;

pub fn new() -> Object {
    Object::new(Shape::Cube)
}

pub fn intersect(cube: Object, ray: Ray) -> Vec<Intersection> {
    let mut vec = vec![];
    let (xtmin, xtmax) = check_axis(ray.get_origin().x, ray.get_direction().x);

    let (ytmin, ytmax) = check_axis(ray.get_origin().y, ray.get_direction().y);

    if ytmin > xtmax || xtmin > ytmax {
        return vec;
    }

    let (ztmin, ztmax) = check_axis(ray.get_origin().z, ray.get_direction().z);

    let tmin = f64::max(f64::max(xtmin, ytmin), ztmin);
    let tmax = f64::min(f64::min(xtmax, ytmax), ztmax);

    if tmin > tmax {
        return vec;
    }

    vec.push(Intersection::new(tmin, cube));
    vec.push(Intersection::new(tmax, cube));
    vec
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let mut tmin;
    let mut tmax;

    if direction.abs() >= 0.00001 {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * f64::MAX;
        tmax = tmax_numerator * f64::MAX;
    }
    if tmin > tmax {
        let tmp = tmin;
        tmin = tmax;
        tmax = tmp;
    }
    (tmin, tmax)
}

pub fn normal_at(point: Tuple) -> Tuple {
    let maxc = f64::max(f64::max(point.x.abs(), point.y.abs()), point.z.abs());

    if maxc == point.x.abs() {
        vector(point.x, 0.0, 0.0)
    } else if maxc == point.y.abs() {
        vector(0.0, point.y, 0.0)
    } else {
        vector(0.0, 0.0, point.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::cubes;
    use crate::vector;

    #[test]
    fn test_local_intersect() {
        let c = cubes::new();

        let r = Ray::new(point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 6.0);

        let r = Ray::new(point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 6.0);

        let r = Ray::new(point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 6.0);

        let r = Ray::new(point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 6.0);

        let r = Ray::new(point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 6.0);

        let r = Ray::new(point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), 4.0);
        assert_eq!(xs[1].get_t(), 6.0);

        let r = Ray::new(point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_t(), -1.0);
        assert_eq!(xs[1].get_t(), 1.0);
    }

    #[test]
    fn test_miss_intersect() {
        let c = cubes::new();

        let r = Ray::new(point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0));
        let xs = intersect(c, r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_normal_at() {
        let p = point(1.0, 0.5, -0.8);
        let normal = normal_at(p);
        assert_eq!(normal, vector(1.0, 0.0, 0.0));

        let p = point(-1.0, -0.2, 0.9);
        let normal = normal_at(p);
        assert_eq!(normal, vector(-1.0, 0.0, 0.0));

        let p = point(-0.4, 1.0, -0.1);
        let normal = normal_at(p);
        assert_eq!(normal, vector(0.0, 1.0, 0.0));

        let p = point(0.3, -1.0, -0.7);
        let normal = normal_at(p);
        assert_eq!(normal, vector(0.0, -1.0, 0.0));

        let p = point(-0.6, 0.3, 1.0);
        let normal = normal_at(p);
        assert_eq!(normal, vector(0.0, 0.0, 1.0));

        let p = point(0.4, 0.4, -1.0);
        let normal = normal_at(p);
        assert_eq!(normal, vector(0.0, 0.0, -1.0));

        let p = point(1.0, 1.0, 1.0);
        let normal = normal_at(p);
        assert_eq!(normal, vector(1.0, 0.0, 0.0));

        let p = point(-1.0, -1.0, -1.0);
        let normal = normal_at(p);
        assert_eq!(normal, vector(-1.0, 0.0, 0.0));
    }
}