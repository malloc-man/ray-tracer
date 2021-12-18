use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub v: i8
}

pub fn tuple(x: f64, y: f64, z: f64, v: i8) -> Tuple {
    if v == 1 {
        point(x, y, z)
    } else if v == 0 {
        vector(x, y, z)
    } else {
        panic!("Attempted to create tuple with v: {}. Must be 0 or 1", v);
    }
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x,
        y,
        z,
        v: 1,
    }
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple {
        x,
        y,
        z,
        v: 0,
    }
}

pub fn origin() -> Tuple {
    point(0.0, 0.0, 0.0)
}

impl Tuple {
    pub fn vectorize(self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z,
            v: 0,
        }
    }

    pub fn is_vector(&self) -> bool {
        self.v == 0
    }

    pub fn magnitude(&self) -> f64 {
        if self.is_vector() {
            f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.v.pow(2) as f64)
        } else {
            panic!("Cannot get magnitude of point");
        }
    }

    pub fn normalize(&self) -> Tuple {
        if self.is_vector() {
            let mag = self.magnitude();
            vector(self.x / mag, self.y / mag, self.z / mag)
        } else {
            panic!("Cannot normalize a point");
        }
    }

    // Cross product
    pub fn xprod(&self, other: Tuple) -> Tuple {
        if self.is_vector() && other.is_vector() {
            vector(
                self.y * other.z - self.z * other.y,
                self.z * other.x - self.x * other.z,
                self.x * other.y - self.y * other.x)
        } else {
            panic!("Need two vectors to compute cross product");
        }
    }

    pub fn reflect_vector(self, normal: Tuple) -> Tuple {
        let dot = 2.0 * (self * normal);
        let nrm = normal * dot;
        self - nrm
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

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;
    fn add(self, other: Tuple) -> Tuple {
        tuple(self.x + other.x, self.y + other.y, self.z + other.z, self.v + other.v)
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;
    fn sub(self, other: Tuple) -> Tuple {
        tuple(self.x - other.x, self.y - other.y, self.z - other.z, self.v - other.v)
    }
}

// Negate vector
impl ops::Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        if self.is_vector() {
            tuple(-self.x, -self.y, -self.z, -self.v)
        } else {
            panic!("Cannot negate a point");
        }
    }
}

// Multiplication operator applied to vector and float scales the vector
impl ops::Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, scale: f64) -> Tuple {
        if self.is_vector() {
            vector(self.x * scale, self.y * scale, self.z * scale)
        } else {
            panic!("Cannot multiply a point")
        }
    }
}

// Multiplication operator applied to 2 vectors computes dot product
impl ops::Mul<Tuple> for Tuple {
    type Output = f64;
    fn mul(self, other: Tuple) -> f64 {
        if self.is_vector() && other.is_vector() {
            self.x * other.x + self.y * other.y + self.z * other.z
        } else {
            panic!("Need two vectors to compute dot product")
        }
    }
}

// Division operator applied to vector and float does scalar division
impl ops::Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, scale: f64) -> Tuple {
        if self.is_vector() {
            vector(self.x / scale, self.y / scale, self.z / scale)
        } else {
            panic!("Cannot divide a point")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_point() {
        let point = point(1.0, 1.0, 1.0);
        assert_eq!(point.v, 1);
    }

    #[test]
    fn create_vector() {
        let vector = vector(1.0, 1.0, 1.0);
        assert_eq!(vector.v, 0);
    }

    #[test]
    fn test_equality() {
        let vector1 = vector(1.0, 1.0, 1.0);
        let vector2 = vector(1.0, 1.0, 1.0);
        let point = point(1.0, 1.0, 1.0);

        assert_eq!(vector1, vector2);
        assert_ne!(vector1, point);
    }

    #[test]
    fn test_add() {
        let vc = vector(5.0, 2.0, 7.0);
        let pt = point(1.0, 1.0, 1.0);

        let sum = pt + vc;
        assert_eq!(sum, point(6.0, 3.0, 8.0));
    }

    #[test]
    fn test_subtracting_two_points() {
        let point1 = point(5.0, 2.0, 7.0);
        let point2 = point(1.0, 1.0, 1.0);

        let diff = point1 - point2;
        assert_eq!(diff, vector(4.0, 1.0, 6.0));
    }

    #[test]
    fn test_subtracting_vector_from_point() {
        let pt = point(5.0, 2.0, 7.0);
        let vc = vector(1.0, 1.0, 1.0);

        let diff = pt - vc;
        assert_eq!(diff, point(4.0, 1.0, 6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(5.0, 2.0, 7.0);
        let v2 = vector(3.0, 1.0, 2.0);

        let diff = v1 - v2;
        assert_eq!(diff, vector(2.0, 1.0, 5.0));
    }

    #[test]
    fn test_scalar_mult() {
        let v1 = vector(5.0, 2.0, 7.0);
        let v2 = v1 * 3.5;

        assert_eq!(v2, vector(17.5, 7.0, 24.5));
    }

    #[test]
    fn test_scalar_div() {
        let v1 = vector(5.0, 2.0, 7.0);
        let v2 = v1 / 2.0;

        assert_eq!(v2, vector(2.5, 1.0, 3.5));
    }

    #[test]
    fn test_vec_mag() {
        let v1 = vector(1.0, 0.0, 0.0);
        assert_eq!(v1.magnitude(), 1.0);

        let v2 = vector(0.0, 1.0, 0.0);
        assert_eq!(v2.magnitude(), 1.0);

        let v3 = vector(-1.0, -2.0, -3.0);
        assert_eq!(v3.magnitude(), 14.0_f64.sqrt())
    }

    #[test]
    fn test_normalize() {
        let v1 = vector(4.0, 0.0, 0.0);
        assert_eq!(v1.normalize(), vector(1.0, 0.0, 0.0));

        let v2 = vector(1.0, 2.0, 3.0);
        let v3 = v2.normalize();
        assert_eq!(v3, vector(0.26726, 0.53452, 0.80178));
        assert_ne!(v3, v1);

        assert_eq!(v3.magnitude(), 1.0);
    }

    #[test]
    fn test_dot_product() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        let dot = v1 * v2;

        assert_eq!(dot, 20.0);
    }

    #[test]
    fn test_cross_product() {
        let v1 = vector(1.0, 2.0, 3.0);
        let v2 = vector(2.0, 3.0, 4.0);
        let cross_1_2 = vector(-1.0, 2.0, -1.0);
        let cross_2_1 = vector(1.0, -2.0, 1.0);

        assert_eq!(v1.xprod(v2), cross_1_2);
        assert_eq!(v2.xprod(v1), cross_2_1);
    }

    #[test]
    fn test_reflect_vector() {
        let vct = vector(1.0, -1.0, 0.0);
        let normal = vector(0.0, 1.0, 0.0);
        let r = vct.reflect_vector(normal);
        assert_eq!(r, vector(1.0, 1.0, 0.0));

        let vct = vector(0.0, -1.0, 0.0);
        let p = f64::sqrt(2.0) / 2.0;
        let normal = vector(p, p, 0.0);
        let r = vct.reflect_vector(normal);
        assert_eq!(r, vector(1.0, 0.0, 0.0));
    }
}
