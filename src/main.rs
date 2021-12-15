mod canvas;

use std::f64::*;
use crate::TupleError::*;
use crate::canvas::*;

#[derive(Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    v: i8
}

#[derive(Debug)]
enum TupleError {
    UsedPointAsVector(String),
    UsedVectorAsPoint(String),
    WrongVal(String),
}

impl Tuple {
    fn new(x: f64, y: f64, z: f64, v: i8) -> Result<Tuple, TupleError> {
        match v {
            1 => Ok(Tuple::point(x, y, z)),
            0 => Ok(Tuple::vector(x, y, z)),
            _ => Err(WrongVal(format!("Expected 1 or 0 for `v`, found {}", v)))
        }
    }

    fn point(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            v: 1,
        }
    }

    fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            v: 0,
        }
    }

    fn is_vector(&self) -> bool {
        self.v == 0
    }

    fn is_point(&self) -> bool {
        self.v == 1
    }

    fn add(&self, other: &Tuple) -> Result<Tuple, TupleError> {
        Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, self.v + other.v)
    }

    fn subtract(&self, other: &Tuple) -> Result<Tuple, TupleError> {
        Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, self.v - other.v)
    }

    fn negate(&self) -> Result<Tuple, TupleError> {
        Tuple::new(-self.x, -self.y, -self.z, -self.v)
    }

    fn scalar_mult_vec(&self, scale: f64) -> Result<Tuple, TupleError> {
        if self.is_vector() {
            Ok(Tuple::vector(self.x * scale, self.y * scale, self.z * scale))
        } else {
            Err(UsedPointAsVector(String::from("Used vector-only method on point tuple")))
        }
    }

    fn scalar_div_vec(&self, scale: f64) -> Result<Tuple, TupleError> {
        if self.is_vector() {
            Ok(Tuple::vector(self.x / scale, self.y / scale, self.z / scale))
        } else {
            Err(UsedPointAsVector(String::from("Used vector-only method on point tuple")))
        }
    }

    fn vector_magnitude(&self) -> Result<f64, TupleError> {
        if self.is_vector() {
            Ok(f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.v.pow(2) as f64))
        } else {
            Err(UsedPointAsVector(String::from("Used vector-only method on point tuple")))
        }
    }

    fn normalize(&self) -> Result<Tuple, TupleError> {
        if self.is_vector() {
            let mag = self.vector_magnitude()?;
            Ok(Tuple::vector(self.x / mag, self.y / mag, self.z / mag))
        } else {
            Err(UsedPointAsVector(String::from("Used vector-only method on point tuple")))
        }
    }

    fn dot_product(&self, other: &Tuple) -> Result<f64, TupleError> {
        if self.is_vector() && other.is_vector() {
            Ok(self.x * other.x + self.y * other.y + self.z * other.z)
        } else {
            Err(UsedPointAsVector(String::from("Need two vectors to compute dot product")))
        }
    }

    fn cross_product(&self, other: &Tuple) -> Result<Tuple, TupleError> {
        if self.is_vector() && other.is_vector() {
            Ok(Tuple::vector(
                self.y * other.z - self.z * other.y,
                self.z * other.x - self.x * other.z,
                self.x * other.y - self.y * other.x))
        } else {
            Err(UsedPointAsVector(String::from("Need two vectors to compute cross product")))
        }
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

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::Tuple;

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

        let sum = point.add(&vector).unwrap();
        assert_eq!(sum, Tuple::point(6.0, 3.0, 8.0));
    }

    #[test]
    fn test_subtracting_two_points() {
        let point1 = Tuple::point(5.0, 2.0, 7.0);
        let point2 = Tuple::point(1.0, 1.0, 1.0);

        let diff = point1.subtract(&point2).unwrap();
        assert_eq!(diff, Tuple::vector(4.0, 1.0, 6.0));
    }

    #[test]
    fn test_subtracting_vector_from_point() {
        let point = Tuple::point(5.0, 2.0, 7.0);
        let vector = Tuple::vector(1.0, 1.0, 1.0);

        let diff = point.subtract(&vector).unwrap();
        assert_eq!(diff, Tuple::point(4.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(5.0, 2.0, 7.0);
        let v2 = Tuple::vector(3.0, 1.0, 2.0);

        let diff = v1.subtract(&v2).unwrap();
        assert_eq!(diff, Tuple::vector(2.0, 1.0, 5.0));
    }

    #[test]
    fn test_scalar_mult() {
        let v1 = Tuple::vector(5.0, 2.0, 7.0);
        let v2 = Tuple::scalar_mult_vec(&v1, 3.5).unwrap();

        assert_eq!(v2, Tuple::vector(17.5, 7.0, 24.5));
    }

    #[test]
    fn test_scalar_div() {
        let v1 = Tuple::vector(5.0, 2.0, 7.0);
        let v2 = Tuple::scalar_div_vec(&v1, 2.0).unwrap();

        assert_eq!(v2, Tuple::vector(2.5, 1.0, 3.5));
    }

    #[test]
    fn test_vec_mag() {
        let v1 = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v1.vector_magnitude().unwrap(), 1.0);

        let v2 = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v2.vector_magnitude().unwrap(), 1.0);

        let v3 = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v3.vector_magnitude().unwrap(), 14.0_f64.sqrt())
    }

    #[test]
    fn test_normalize() {
        let v1 = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v1.normalize().unwrap(), Tuple::vector(1.0, 0.0, 0.0));

        let v2 = Tuple::vector(1.0, 2.0, 3.0);
        let v3 = v2.normalize().unwrap();
        assert_eq!(v3, Tuple::vector(0.26726, 0.53452, 0.80178));
        assert_ne!(v3, v1);

        assert_eq!(v3.vector_magnitude().unwrap(), 1.0);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        let dot = v1.dot_product(&v2).unwrap();

        assert_eq!(dot, 20.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Tuple::vector(1.0, 2.0, 3.0);
        let v2 = Tuple::vector(2.0, 3.0, 4.0);
        let cross_1_2 = Tuple::vector(-1.0, 2.0, -1.0);
        let cross_2_1 = Tuple::vector(1.0, -2.0, 1.0);

        assert_eq!(v1.cross_product(&v2).unwrap(), cross_1_2);
        assert_eq!(v2.cross_product(&v1).unwrap(), cross_2_1);
    }
}
