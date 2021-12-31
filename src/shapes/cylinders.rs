use crate::prelude::*;

pub fn new(min: f64, max: f64, closed: bool) -> Object {
    Object::new(Shape::Cylinder {min, max, closed})
}

pub fn new_unbounded() -> Object {
    Object::new(Shape::Cylinder {min: f64::MIN, max: f64::MAX, closed: false})
}

pub fn max(cylinder: Object) -> f64 {
    if let Shape::Cylinder {min: _, max, closed: _} = cylinder.shape {
        max
    } else {
        f64::MAX
    }
}

pub fn min(cylinder: Object) -> f64 {
    if let Shape::Cylinder {min, max: _, closed: _} = cylinder.shape {
        min
    } else {
        f64::MIN
    }
}

pub fn intersect(cylinder: Object, ray: Ray) -> Vec<Intersection> {
    let mut vec = vec![];
    let a = ray.get_direction().x.powi(2) + ray.get_direction().z.powi(2);
    if a < EPSILON {
        intersect_caps(cylinder, ray, &mut vec);
        return vec;
    }
    let b = 2.0 * ray.get_origin().x * ray.get_direction().x + 2.0 * ray.get_origin().z * ray.get_direction().z;
    let c = ray.get_origin().x.powi(2) + ray.get_origin().z.powi(2) - 1.0;

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
    if min(cylinder) < y0 && y0 < max(cylinder) {
        vec.push(Intersection::new(t0, cylinder));
    }

    let y1 = ray.get_origin().y + (t1 * ray.get_direction().y);
    if min(cylinder) < y1 && y1 < max(cylinder) {
        vec.push(Intersection::new(t1, cylinder));
    }

    intersect_caps(cylinder, ray, &mut vec);
    vec
}

fn intersect_caps(cylinder: Object, ray: Ray, intersections: &mut Vec<Intersection>) {
    if !is_closed(cylinder) || ray.get_direction().y.approx_eq(0.0) {
        return;
    }

    let t1 = (min(cylinder) - ray.get_origin().y) / ray.get_direction().y;
    if check_cap(ray, t1) {
        intersections.push(Intersection::new(t1, cylinder));
    }

    let t2 = (max(cylinder) - ray.get_origin().y) / ray.get_direction().y;
    if check_cap(ray, t2) {
        intersections.push(Intersection::new(t2, cylinder));
    }
}

fn check_cap(ray: Ray, t: f64) -> bool {
    let x = ray.get_origin().x + (t * ray.get_direction().x);
    let z = ray.get_origin().z + (t * ray.get_direction().z);
    x.powi(2) + z.powi(2) <= 1.0
}

fn is_closed(cylinder: Object) -> bool {
    if let Shape::Cylinder {min: _, max: _, closed} = cylinder.shape {
        closed
    } else {
        false
    }
}

pub fn normal_at(cylinder: Object, point: Tuple) -> Tuple {
    let dist_to_y_axis_sq = point.x.powi(2) + point.z.powi(2);
    if dist_to_y_axis_sq < 1.0 && point.y >= max(cylinder) - EPSILON {
        vector(0.0, 1.0, 0.0)
    } else if dist_to_y_axis_sq < 1.0 && point.y <= min(cylinder) + EPSILON {
        vector(0.0, -1.0, 0.0)
    } else {
        vector(point.x, 0.0, point.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miss_cylinder() {
        let cyl = cylinders::new_unbounded();

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
        let cyl = cylinders::new_unbounded();

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
        assert!((xs[0].get_t().approx_eq(6.80798)));
        assert!((xs[1].get_t().approx_eq(7.08872)));
    }

    #[test]
    fn test_normal_at() {
        let cyl = cylinders::new_unbounded();

        let n = cyl.normal_at(point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));

        let n = cyl.normal_at(point(0.0, 5.0, -1.0));
        assert_eq!(n, vector(0.0, 0.0, -1.0));

        let n = cyl.normal_at(point(0.0, -2.0, 1.0));
        assert_eq!(n, vector(0.0, 0.0, 1.0));

        let n = cyl.normal_at(point(-1.0, 1.0, 0.0));
        assert_eq!(n, vector(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_cylinder_truncate() {
        let cyl = cylinders::new(1.0, 2.0, false);

        let r = Ray::new(point(0.0, 1.5, 0.0), vector(0.1, 1.0, 0.0).normalize());
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(0.0, 3.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(point(0.0, 1.5, -2.0), vector(0.0, 0.0, 1.0));
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn test_intersect_end_caps() {
        let cyl = cylinders::new(1.0, 2.0, true);

        let r = Ray::new(point(0.0, 3.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(point(0.0, 3.0, -2.0), vector(0.0, -1.0, 2.0).normalize());
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(point(0.0, 4.0, -2.0), vector(0.0, -1.0, 1.0).normalize());
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(point(0.0, 0.0, -2.0), vector(0.0, 1.0, 2.0).normalize());
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(point(0.0, -1.0, -2.0), vector(0.0, 1.0, 1.0).normalize());
        let xs = cyl.intersect(r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn test_normal_at_end_caps() {
        let cyl = cylinders::new(1.0, 2.0, true);

        let n = normal_at(cyl, point(0.0, 1.0, 0.0));
        assert_eq!(n, vector(0.0, -1.0, 0.0));

        let n = normal_at(cyl, point(0.5, 1.0, 0.0));
        assert_eq!(n, vector(0.0, -1.0, 0.0));

        let n = normal_at(cyl, point(0.0, 1.0, 0.5));
        assert_eq!(n, vector(0.0, -1.0, 0.0));

        let n = normal_at(cyl, point(0.0, 2.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));

        let n = normal_at(cyl, point(0.5, 2.0, 0.0));
        assert_eq!(n, vector(0.0, 1.0, 0.0));

        let n = normal_at(cyl, point(0.0, 2.0, 0.5));
        assert_eq!(n, vector(0.0, 1.0, 0.0));
    }
}