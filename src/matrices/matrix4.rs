use std::ops;
use crate::matrices::matrix3::Matrix3;
use crate::matrices::transformations;
use crate::matrices::tuples::*;

#[derive(Clone, Copy, Debug)]
pub struct Matrix4 {
    vals: [[f64; 4]; 4]
}

impl Matrix4 {
    pub fn new() -> Self {
        Self {
            vals: [[0.0; 4]; 4]
        }
    }

    pub fn convert(vals: [[f64; 4]; 4]) -> Self {
        Self {
            vals,
        }
    }

    pub fn identity() -> Self {
        let mut matrix = Matrix4::new();
        for i in 0..4 {
            matrix.write(1.0, i, i);
        }
        matrix
    }

    pub fn write(&mut self, val: f64, row: usize, col: usize) {
        if row >= 4 || col >= 4 {
            panic!("Write index error. Attempted to write to row: {}, col: {}. Size is 4x4.", row, col);
        }
        self.vals[row][col] = val;
    }

    pub fn val_at(&self, row: usize, col: usize) -> f64 {
        if row >= 4 || col >= 4 {
            panic!("Read index error. Attempted to read row: {}, col: {}. Size is 4x4.", row, col);
        }
        self.vals[row][col]
    }

    pub fn transpose(&self) -> Matrix4 {
        let mut result = Matrix4::new();
        for i in 0..4 {
            for j in 0..4 {
                result.write(self.val_at(i, j), j, i);
            }
        }
        result
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        if row >= 4 || col >= 4 {
            panic!("Attempted to get submatrix by removing row {} and col {} from 4x4 matrix", row, col);
        }
        let mut new_vals = [[0.0; 3]; 3];
        let mut curr_i = 0;
        for i in 0..4 {
            if i == row {
                continue;
            }
            let mut curr_j = 0;
            for j in 0..4 {
                if j == col {
                    continue
                } else {
                    new_vals[curr_i][curr_j] = self.val_at(i, j);
                    curr_j += 1;
                }
            }
            curr_i += 1;
        }
        Matrix3::convert(new_vals)
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        let submatrix = self.submatrix(row, col);
        submatrix.determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let m = self.minor(row, col);
        if (row + col) % 2 == 1 {
            -m
        } else {
            m
        }
    }

    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        for i in 0..4 {
            determinant += self.val_at(i, 0) * self.cofactor(i, 0);
        }
        determinant
    }

    fn is_invertible(&self) -> bool {
        let d = self.determinant();
        f64::abs(d) > 0.00001
    }

    pub fn invert(&self) -> Matrix4 {
        if !self.is_invertible() {
            panic!("Attempted to invert a non-invertible matrix");
        } else {
            let determinant = self.determinant();
            let mut result = Matrix4::new();
            for i in 0..4 {
                for j in 0..4 {
                    let c = self.cofactor(i, j);
                    result.write(c / determinant, j, i);
                }
            }
            result
        }
    }

    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        transformations::translation(x, y, z) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        transformations::scaling(x, y, z) * self
    }

    pub fn rotate_x(self, rad: f64) -> Self {
        transformations::rotation_x(rad) * self
    }

    pub fn rotate_y(self, rad: f64) -> Self {
        transformations::rotation_y(rad) * self
    }

    pub fn rotate_z(self, rad: f64) -> Self {
        transformations::rotation_z(rad) * self
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        const EPSILON: f64 = 0.00001;
        for i in 0..4 {
            for j in 0..4 {
                if f64::abs(self.val_at(i, j) - other.val_at(i, j)) > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

impl ops::Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut m = Matrix4::new();
        for i in 0..self.vals.len() {
            for j in 0..other.vals[0].len() {
                let mut val = 0.0;
                for a in 0..self.vals.len() {
                    val += self.val_at(i, a) * other.val_at(a, j);
                }
                m.write(val, i, j);
            }
        }
        m
    }
}

impl ops::Mul<Tuple> for Matrix4 {
    type Output = Tuple;
    fn mul(self, tup: Tuple) -> Tuple {
        let other = [[tup.x], [tup.y], [tup.z], [tup.v as f64]];
        let mut m = Matrix4::new();
        for i in 0..self.vals.len() {
            for j in 0..other[0].len() {
                let mut val = 0.0;
                for a in 0..self.vals.len() {
                    val += self.val_at(i, a) * other[a][j];
                }
                m.write(val, i, j);
            }
        }
        if tup.is_vector() {
            vector(m.val_at(0, 0),m.val_at(1, 0),m.val_at(2, 0))
        } else {
            point(m.val_at(0, 0),m.val_at(1, 0),m.val_at(2, 0))
        }

    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;
    use super::*;

    #[test]
    fn test_init() {
        let m = Matrix4::new();
        assert_eq!(m.vals[0][1], 0.0);
    }

    #[test]
    fn test_write() {
        let mut m = Matrix4::new();
        m.write(3.3, 0, 1);
        assert_eq!(m.vals[0][1], 3.3);
        assert_ne!(m.vals[1][1], 3.3);
    }

    #[test]
    fn test_read() {
        let m = Matrix4::new();
        assert_eq!(m.val_at(0, 1), 0.0);
    }

    #[test]
    fn test_equals() {
        let mut m = Matrix4::new();
        let mut n = Matrix4::new();

        for i in 0..4 {
            for j in 0..4 {
                m.write((i + j) as f64, i, j);
                n.write((i + j) as f64, i, j);
            }
        }
        assert_eq!(m, n);

        m.write(7.631, 0, 0);
        assert_ne!(m, n);
    }

    #[test]
    fn test_mul() {
        let m_arr = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0]];
        let n_arr = [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0]];
        let q_arr = [
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0]];

        let m = Matrix4::convert(m_arr);
        let n = Matrix4::convert(n_arr);
        let p = m * n;
        let q = Matrix4::convert(q_arr);

        assert_eq!(p, q);
    }

    #[test]
    fn test_tuple_mul() {
        let m_arr = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0]];
        let m = Matrix4::convert(m_arr);
        let tup = tuple(1.0, 2.0, 3.0, 1);
        let prod = m * tup;
        assert_eq!(prod, tuple(18.0, 24.0, 33.0, 1));
    }

    #[test]
    fn test_identity_mul() {
        let id_arr = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]];

        let id_matrix = Matrix4::convert(id_arr);

        let m_arr = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0]];

        let m = Matrix4::convert(m_arr);

        assert_eq!(m * id_matrix, m);
    }

    #[test]
    fn test_transpose() {
        let m_arr = [
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0]];

        let transposed_arr = [
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0]];

        let m = Matrix4::convert(m_arr);
        let t = Matrix4::convert(transposed_arr);

        assert_eq!(m.transpose(), t);
    }

    #[test]
    fn test_4x4_determinant() {
        let vec_m = [
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0]];

        let m = Matrix4::convert(vec_m);
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn test_invert() {
        let m_arr = [
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0]];

        let m = Matrix4::convert(m_arr);
        assert!(m.is_invertible());

        let i_arr = [
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639]];

        let i = Matrix4::convert(i_arr);
        assert_eq!(m.invert(), i);
    }

    #[test]
    fn test_mul_and_invert() {
        let a_arr = [
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0]];

        let b_arr = [
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0]];

        let a = Matrix4::convert(a_arr);
        let b = Matrix4::convert(b_arr);
        let c = a * b;
        assert_eq!(a, c * b.invert());
    }

    #[test]
    fn test_concatenated_transformations() {
        let transform = Matrix4::identity()
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        let pt = point(1.0, 0.0, 1.0);
        assert_eq!(transform * pt, point(15.0, 0.0, 7.0));
    }
}