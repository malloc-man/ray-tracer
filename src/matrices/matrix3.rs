use crate::matrices::matrix2::Matrix2;

#[derive(Clone, Copy, Debug)]
pub struct Matrix3 {
    vals: [[f64; 3]; 3]
}

impl Matrix3 {

    pub fn convert(vals: [[f64; 3]; 3]) -> Self {
        Self {
            vals,
        }
    }

    pub fn val_at(&self, row: usize, col: usize) -> f64 {
        if row >= 3 || col >= 3 {
            panic!("Read index error. Attempted to read row: {}, col: {}. Size is 3x3.", row, col);
        }
        self.vals[row][col]
    }

    pub(crate) fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        if row >= 3 || col >= 3 {
            panic!("Attempted to get submatrix by removing row {} and col {} from 4x4 matrix", row, col);
        }
        let mut new_vals = [[0.0; 2]; 2];
        let mut curr_i = 0;
        for i in 0..3 {
            if i == row {
                continue;
            }
            let mut curr_j = 0;
            for j in 0..3 {
                if j == col {
                    continue
                } else {
                    new_vals[curr_i][curr_j] = self.val_at(i, j);
                    curr_j += 1;
                }
            }
            curr_i += 1;
        }
        Matrix2::convert(new_vals)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let submatrix = self.submatrix(row, col);
        submatrix.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let m = self.minor(row, col);
        if (row + col) % 2 == 1 {
            -m
        } else {
            m
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        for i in 0..3 {
            determinant += self.val_at(i, 0) * self.cofactor(i, 0);
        }
        determinant
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Matrix3) -> bool {
        const EPSILON: f64 = 0.00001;
        for i in 0..3 {
            for j in 0..3 {
                if f64::abs(self.val_at(i, j) - other.val_at(i, j)) > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submatrix() {
        let m_vec = [
            [1.0, 5.0, 0.0],
            [-3.0, 2.0, 7.0],
            [0.0, 6.0, -3.0]];

        let sub_vec = [
            [-3.0, 2.0],
            [0.0, 6.0]];

        let m = Matrix3::convert(m_vec);
        let sub = Matrix2::convert(sub_vec);
        assert_eq!(m.submatrix(0, 2), sub);
    }

    #[test]
    fn test_minor() {
        let m_vec = [
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]];

        let m = Matrix3::convert(m_vec);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactor() {
        let m_vec = [
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]];

        let m = Matrix3::convert(m_vec);
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_3x3_determinant() {
        let vec_m = [
            [1.0, 2.0, 6.0],
            [-5.0, 8.0, -4.0],
            [2.0, 6.0, 4.0]];

        let m = Matrix3::convert(vec_m);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }
}