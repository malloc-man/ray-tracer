use crate::{Intersection, Ray, objects::*, tuples::*};
use crate::utils::{ApproxEq, EPSILON};

pub fn new(min: f64, max: f64, closed: bool) -> Object {
    Object::new(Shape::Cone {min, max, closed})
}

pub fn new_unbounded() -> Object {
    Object::new(Shape::Cone {min: f64::MIN, max: f64::MAX, closed: false})
}

fn min(cone: Object) -> f64 {
    if let Shape::Cone {min, max: _, closed: _} = cone.shape {
        min
    } else {
        f64::MIN
    }
}

fn max(cone: Object) -> f64 {
    if let Shape::Cone {min: _, max, closed: _} = cone.shape {
        max
    } else {
        f64::MAX
    }
}

pub fn intersect(cone: Object, ray: Ray) -> Vec<Intersection> {
    let mut vec = vec![];

    let a = ray.get_direction().x.powi(2) -
        ray.get_direction().y.powi(2) +
        ray.get_direction().z.powi(2);

    let b = 2.0 * ray.get_origin().x * ray.get_direction().x -
        2.0 * ray.get_origin().y * ray.get_direction().y +
        2.0 * ray.get_origin().z * ray.get_direction().z;

    if a.approx_eq(0.0) && b.approx_eq(0.0) {
        intersect_caps(cone, ray, &mut vec);
        return vec;
    }

    let c = ray.get_origin().x.powi(2) -
        ray.get_origin().y.powi(2) +
        ray.get_origin().z.powi(2);

    if a.approx_eq(0.0) {
        let t = -c / (2.0 * b);
        vec.push(Intersection::new(t, cone));
        intersect_caps(cone, ray, &mut vec);
        return vec;
    }

    let disc = b.powi(2) - 4.0 * a * c;

    if disc < 0.0 {
        return vec;
    }

    let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
    let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

    if t0 > t1 {
        let tmp = t0;
        t0 = t1;
        t1 = tmp;
    }

    let y0 = ray.get_origin().y + (t0 * ray.get_direction().y);

    if min(cone) < y0 && y0 < max(cone) {
        vec.push(Intersection::new(t0, cone));
    }

    let y1 = ray.get_origin().y + (t1 * ray.get_direction().y);
    if min(cone) < y1 && y1 < max(cone) {
        vec.push(Intersection::new(t1, cone));
    }

    intersect_caps(cone, ray, &mut vec);
    vec
}

fn intersect_caps(cone: Object, ray: Ray, intersections: &mut Vec<Intersection>) {
    if !is_closed(cone) || ray.get_direction().y.approx_eq(0.0) {
        return;
    }

    let t1 = (min(cone) - ray.get_origin().y) / ray.get_direction().y;
    if check_cap(ray, t1, min(cone)) {
        intersections.push(Intersection::new(t1, cone));
    }

    let t2 = (max(cone) - ray.get_origin().y) / ray.get_direction().y;
    if check_cap(ray, t2, max(cone)) {
        intersections.push(Intersection::new(t2, cone));
    }
}

fn check_cap(ray: Ray, t: f64, y: f64) -> bool {
    let x = ray.get_origin().x + (t * ray.get_direction().x);
    let z = ray.get_origin().z + (t * ray.get_direction().z);
    x.powi(2) + z.powi(2) <= y.abs()
}

fn is_closed(cone: Object) -> bool {
    if let Shape::Cone {min: _, max: _, closed} = cone.shape {
        closed
    } else {
        false
    }
}

pub fn normal_at(cone: Object, point: Tuple) -> Tuple {
    let dist_to_y_axis_sq = point.x.powi(2) + point.z.powi(2);
    if dist_to_y_axis_sq < 1.0 && point.y >= max(cone) - EPSILON {
        vector(0.0, 1.0, 0.0)
    } else if dist_to_y_axis_sq < 1.0 && point.y <= min(cone) + EPSILON {
        vector(0.0, -1.0, 0.0)
    } else {
        let mut y = (point.x.powi(2) + point.z.powi(2)).sqrt();
        if point.y > 0.0 {
            y = -y;
        }
        vector(point.x, y, point.z)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;
    use crate::cones;
    use crate::utils::ApproxEq;
    use super::*;

    #[test]
    fn test_intersect_cone() {
        let cone = cones::new_unbounded();

        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = cone.intersect(r);
        assert_eq!(xs[0].get_t(), 5.0);
        assert_eq!(xs[1].get_t(), 5.0);

        let r = Ray::new(point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0).normalize());
        let xs = cone.intersect(r);
        assert!((xs[0].get_t().approx_eq(8.66025)));
        assert!((xs[1].get_t().approx_eq(8.66025)));

        let r = Ray::new(point(1.0, 1.0, -5.0), vector(-0.5, -1.0, 1.0).normalize());
        let xs = cone.intersect(r);
        assert!((xs[0].get_t().approx_eq(4.55006)));
        assert!((xs[1].get_t().approx_eq(49.44994)));
    }

    #[test]
    fn test_intersect_cone_ray_parallel_to_half() {
        let cone = cones::new_unbounded();
        let direction = vector(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(point(0.0, 0.0, -1.0), direction);
        let xs = cone.intersect(r);
        assert_eq!(xs.len(), 1);
        assert!((xs[0].get_t().approx_eq(0.35355)));
    }

    #[test]
    fn test_intersect_cone_end_caps() {
        let cone = cones::new(-0.5, 0.5, true);

        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let xs = cone.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(0.0, 0.0, -0.25), vector(0.0, 1.0, 1.0).normalize());
        let xs = cone.intersect(r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(point(0.0, 0.0, -0.25), vector(0.0, 1.0, 0.0));
        let xs = cone.intersect(r);
        assert_eq!(xs.len(), 4);
    }

    #[test]
    fn test_normal() {
        let cone = cones::new_unbounded();

        let n = normal_at(cone, origin());
        assert_eq!(n, vector(0.0, 0.0, 0.0));

        let n = normal_at(cone, point(1.0, 1.0, 1.0));
        assert_eq!(n, vector(1.0, -SQRT_2, 1.0));

        let n = normal_at(cone, point(-1.0, -1.0, 0.0));
        assert_eq!(n, vector(-1.0, 1.0, 0.0));
    }
}