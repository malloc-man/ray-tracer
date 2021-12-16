#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub v: i8
}

#[derive(Debug)]
pub enum TupleError {
    UsedPointAsVector(String),
    UsedVectorAsPoint(String),
    WrongVal(String),
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, v: i8) -> Tuple {
        if v != 1 && v != 0 {
            panic!("Fourth argument must be 0 (vector) or 1 (point)")
        }
        if v == 1 {
            Tuple::point(x, y, z)
        } else {
            Tuple::vector(x, y, z)
        }
    }

    pub(crate) fn point(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            v: 1,
        }
    }

    pub(crate) fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            v: 0,
        }
    }

    pub(crate) fn origin() -> Self {
        Tuple::point(0.0, 0.0, 0.0)
    }

    fn is_vector(&self) -> bool {
        self.v == 0
    }

    fn is_point(&self) -> bool {
        self.v == 1
    }

    pub(crate) fn add(&self, other: &Tuple) -> Tuple {
        Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, self.v + other.v)
    }

    pub(crate) fn subtract(&self, other: &Tuple) -> Tuple {
        Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, self.v - other.v)
    }

    pub(crate) fn negate(&self) -> Tuple {
        Tuple::new(-self.x, -self.y, -self.z, -self.v)
    }

    pub(crate) fn scalar_mult_vec(&self, scale: f64) -> Tuple {
        if self.is_vector() {
            Tuple::vector(self.x * scale, self.y * scale, self.z * scale)
        } else {
            panic!("Used vector-only method on point tuple")
        }
    }

    fn scalar_div_vec(&self, scale: f64) -> Tuple {
        if self.is_vector() {
            Tuple::vector(self.x / scale, self.y / scale, self.z / scale)
        } else {
            panic!("Used vector-only method on point tuple")
        }
    }

    fn vector_magnitude(&self) -> f64 {
        if self.is_vector() {
            f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.v.pow(2) as f64)
        } else {
            panic!("Used vector-only method on point tuple");
        }
    }

    pub(crate) fn normalize(&self) -> Tuple {
        if self.is_vector() {
            let mag = self.vector_magnitude();
            Tuple::vector(self.x / mag, self.y / mag, self.z / mag)
        } else {
            panic!("Used vector-only method on point tuple");
        }
    }

    pub(crate) fn dot_product(&self, other: &Tuple) -> f64 {
        if self.is_vector() && other.is_vector() {
            self.x * other.x + self.y * other.y + self.z * other.z
        } else {
            panic!("Need two vectors to compute dot product")
        }
    }

    fn cross_product(&self, other: &Tuple) -> Tuple {
        if self.is_vector() && other.is_vector() {
            Tuple::vector(
                self.y * other.z - self.z * other.y,
                self.z * other.x - self.x * other.z,
                self.x * other.y - self.y * other.x)
        } else {
            panic!("Need two vectors to compute cross product");
        }
    }

    pub(crate) fn reflect_vector(&self, normal: &Tuple) -> Tuple {
        let dot = 2.0 * self.dot_product(&normal);
        let nrm = normal.scalar_mult_vec(dot);
        self.subtract(&nrm)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        const EPSILON: f64 = 0.00001;
        if f64::abs(self.x - other.x) > EPSILON ||
            f64::abs(self.y - other.y) > EPSILON ||
            f64::abs(self.z - other.z) > EPSILON ||
            self.v != other.v {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_point() {
        let point = Tuple::point(1.0, 1.0, 1.0);
        assert_eq!(point.v, 1);
    }

    #[test]
    fn create_vector() {
        let vector = Tuple::vector(1.0, 1.0, 1.0);
        assert_eq!(vector.v, 0);
    }

    #[test]
    fn test_equality() {
        let vector1 = Tuple::vector(1.0, 1.0, 1.0);
        let vector2 = Tuple::vector(1.0, 1.0, 1.0);
        let point = Tuple::point(1.0, 1.0, 1.0);

        assert_eq!(vector1, vector2);
        assert_ne!(vector1, point);
    }

    #[test]
    fn test_add() {
        let vector = Tuple::vector(5.0, 2.0, 7.0);
        let point = Tuple::point(1.0, 1.0, 1.0);

        let sum = point.add(&vector);
        assert_eq!(sum, Tuple::point(6.0, 3.0, 8.0));
    }

    #[test]
    fn test_subtracting_two_points() {
        let point1 = Tuple::point(5.0, 2.0, 7.0);
        let point2 = Tuple::point(1.0, 1.0, 1.0);

        let diff = point1.subtract(&point2);
        assert_eq!(diff, Tuple::vector(4.0, 1.0, 6.0));
    }

    #[test]
    fn test_subtracting_vector_from_point() {
        let point = Tuple::point(5.0, 2.0, 7.0);
        let vector = Tuple::vector(1.0, 1.0, 1.0);

        let diff = point.subtract(&vector);
        assert_eq!(diff, Tuple::point(4.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(5.0, 2.0, 7.0);
        let v2 = Tuple::vector(3.0, 1.0, 2.0);

        let diff = v1.subtract(&v2);
        assert_eq!(diff, Tuple::vector(2.0, 1.0, 5.0));
    }

    #[test]
    fn test_scalar_mult() {
        let v1 = Tuple::vector(5.0, 2.0, 7.0);
        let v2 = Tuple::scalar_mult_vec(&v1, 3.5);

        assert_eq!(v2, Tuple::vector(17.5, 7.0, 24.5));
    }

    #[test]
    fn test_scalar_div() {
        let v1 = Tuple::vector(5.0, 2.0, 7.0);
        let v2 = Tuple::scalar_div_vec(&v1, 2.0);

        assert_eq!(v2, Tuple::vector(2.5, 1.0, 3.5));
    }

    #[test]
    fn test_vec_mag() {
        let v1 = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v1.vector_magnitude(), 1.0);

        let v2 = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v2.vector_magnitude(), 1.0);

        let v3 = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v3.vector_magnitude(), 14.0_f64.sqrt())
    }

    #[test]
    fn test_normalize() {
        let v1 = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v1.normalize(), Tuple::vector(1.0, 0.0, 0.0));

        let v2 = Tuple::vector(1.0, 2.0, 3.0);
        let v3 = v2.normalize();
        assert_eq!(v3, Tuple::vector(0.26726, 0.53452, 0.80178));
        assert_ne!(v3, v1);

        assert_eq!(v3.vector_magnitude(), 1.0);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        let dot = v1.dot_product(&v2);

        assert_eq!(dot, 20.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        let cross_1_2 = Tuple::vector(-1.0, 2.0, -1.0);
        let cross_2_1 = Tuple::vector(1.0, -2.0, 1.0);

        assert_eq!(v1.cross_product(&v2), cross_1_2);
        assert_eq!(v2.cross_product(&v1), cross_2_1);
    }

    #[test]
    fn test_reflect_vector() {
        let vector = Tuple::vector(1.0, -1.0, 0.0);
        let normal = Tuple::vector(0.0, 1.0, 0.0);
        let r = vector.reflect_vector(&normal);
        assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));

        let vector = Tuple::vector(0.0, -1.0, 0.0);
        let p = f64::sqrt(2.0) / 2.0;
        let normal = Tuple::vector(p, p, 0.0);
        let r = vector.reflect_vector(&normal);
        assert_eq!(r, Tuple::vector(1.0, 0.0, 0.0));
    }
}
