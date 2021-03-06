use std::cmp::Ordering;
use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
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

    pub fn position(&self, t: f64) -> Tuple {
        let pos = self.get_origin();
        let dir = self.get_direction();
        pos + (dir * t)
    }

    pub fn transform(&self, matrix: Matrix4) -> Self {
        let new_origin = matrix * self.get_origin();
        let new_direction = matrix * self.get_direction();
        Ray::new(new_origin, new_direction)
    }

    pub fn get_origin(&self) -> Tuple {
        self.origin
    }

    pub fn get_direction(&self) -> Tuple {
        self.direction
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection {
    t_value: f64,
    object: Object,
}

impl Intersection {
    pub fn new(t_value: f64, object: Object) -> Self {
        Self {
            t_value,
            object,
        }
    }

    pub fn get_t(&self) -> f64 {
        self.t_value
    }

    pub fn get_object(&self) -> Object {
        self.object
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_t().partial_cmp(&other.get_t())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let ray = Ray::new(point(2.0, 3.0, 4.0),
                           vector(1.0, 0.0, 0.0));
        assert_eq!(ray.position(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), point(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_ray_translate() {
        let ray = Ray::new(point(1.0, 2.0, 3.0),
                           vector(0.0, 1.0, 0.0));
        let m = translation(3.0, 4.0, 5.0);
        let transformed_ray = ray.transform(m);
        assert_eq!(transformed_ray.get_origin(), point(4.0, 6.0, 8.0));
        assert_eq!(transformed_ray.get_direction(), vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_ray_scale() {
        let ray = Ray::new(point(1.0, 2.0, 3.0),
                           vector(0.0, 1.0, 0.0));
        let m = scaling(2.0, 3.0, 4.0);
        let transformed_ray = ray.transform(m);
        assert_eq!(transformed_ray.get_origin(), point(2.0, 6.0, 12.0));
        assert_eq!(transformed_ray.get_direction(), vector(0.0, 3.0, 0.0));
    }
}