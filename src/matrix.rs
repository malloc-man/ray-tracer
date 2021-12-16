use crate::Tuple;

#[derive(Debug)]
struct Matrix {
    rows: usize,
    cols: usize,
    vals: Vec<Vec<f64>>,
}

#[derive(Debug)]
enum MatrixError {
    IndexError(String),
    IncompatibleMultiplication(String),
    DeterminantError(String),
    MinorError(String),
    InversionError(String),
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            vals: vec![vec![0.0_f64; cols]; rows],
        }
    }

    fn convert(vals: Vec<Vec<f64>>) -> Self {
        Self {
            rows: vals.len(),
            cols: vals[0].len(),
            vals,
        }
    }

    fn write(&mut self, val: f64, row: usize, col: usize) -> Result<(), MatrixError> {
        if row < 0 || row >= self.rows || col < 0 || col >= self.cols {
            return Err(MatrixError::IndexError(
                format!("Write index error. Attempted to write to row: {}, col: {}. Size is rows: {}, cols: {}.",
                        row, col, self.rows, self.cols)))
        }
        self.vals[row][col] = val;
        Ok(())
    }

    fn val_at(&self, row: usize, col: usize) -> Result<f64, MatrixError> {
        if row < 0 || row >= self.rows || col < 0 || col >= self.cols {
            return Err(MatrixError::IndexError(
                format!("Read index error. Attempted to read row: {}, col: {}. Size is rows: {}, cols: {}.",
                        row, col, self.rows, self.cols)))
        }
        Ok(self.vals[row][col])
    }

    fn mul(&self, other: &Matrix) -> Result<Matrix, MatrixError> {
        if self.cols != other.rows {
            return Err(MatrixError::IncompatibleMultiplication(
                format!("Attempted to multiply incompatible matrices. Rows is {} and cols is {}.",
                        self.rows, other.cols)));
        }
        let mut m = Matrix::new(self.rows, other.cols);
        for i in 0..m.rows {
            for j in 0..m.cols {
                let mut val = 0.0;
                for a in 0..self.rows {
                    val += self.val_at(i, a)? * other.val_at(a, j)?;
                }
                m.write(val, i, j);
            }
        }
        Ok(m)
    }

    fn tuple_mul(&self, other: &Tuple) -> Result<Tuple, MatrixError> {
        if self.rows != 4 {
            return Err(MatrixError::IncompatibleMultiplication(format!("Attempted to use tuple multiplication on matrix without 4 rows")));
        }
        let tuple_as_matrix = Matrix::convert(
            vec![vec![other.x], vec![other.y], vec![other.z], vec![other.v as f64]]);
        let result_matrix = self.mul(&tuple_as_matrix)?;
        Ok(Tuple::new(
            result_matrix.val_at(0, 0)?,
            result_matrix.val_at(1, 0)?,
            result_matrix.val_at(2, 0)?,
            result_matrix.val_at(3, 0)? as i8).unwrap())
    }

    fn transpose(&self) -> Result<Matrix, MatrixError> {
        let mut result = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.write(self.val_at(i, j)?, j, i);
            }
        }
        Ok(result)
    }

    fn submatrix(&self, row: usize, col: usize) -> Result<Matrix, MatrixError> {
        if row >= self.rows || col >= self.cols || row < 0 || col < 0 {
            return Err(MatrixError::IndexError(
                format!("Attempted to get submatrix by removing row {} and col {} from {}x{} matrix",
                row, col, self.rows, self.cols)
            ))
        }
        let mut new_vec = self.vals.clone();
        new_vec.remove(row);
        for vec in &mut new_vec {
            vec.remove(col);
        }
        Ok(Matrix::convert(new_vec))
    }

    fn minor(&self, row: usize, col: usize) -> Result<f64, MatrixError> {
        let submatrix = self.submatrix(row, col)?;
        submatrix.determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> Result<f64, MatrixError> {
        let m = self.minor(row, col)?;
        if (row + col) % 2 == 1 {
            Ok(-m)
        } else {
            Ok(m)
        }
    }

    fn determinant(&self) -> Result<f64, MatrixError> {
        if self.rows != self.cols {
            Err(MatrixError::DeterminantError(String::from("Cannot compute determinant of non-square matrix")))
        } else if self.rows == 2 && self.cols == 2 {
            Ok(self.val_at(0, 0).unwrap() * self.val_at(1, 1).unwrap() -
                self.val_at(1, 0).unwrap() * self.val_at(0, 1).unwrap())
        } else {
            let mut determinant = 0.0;
            for i in 0..self.rows {
                determinant += self.val_at(i, 0)? * self.cofactor(i, 0)?;
            }
            Ok(determinant)
        }
    }

    fn is_invertible(&self) -> Result<bool, MatrixError> {
        let d = self.determinant()?;
        Ok(f64::abs(d) > 0.00001)
    }

    fn invert(&self) -> Result<Matrix, MatrixError> {
        if !self.is_invertible()? {
            Err(MatrixError::InversionError(String::from("Attempted to invert a non-invertible matrix")))
        } else {
            let determinant = self.determinant()?;
            let mut result = Matrix::new(self.rows, self.cols);
            for i in 0..self.rows {
                for j in 0..self.cols {
                    let c = self.cofactor(i, j)?;
                    result.write(c / determinant, j, i)?;
                }
            }
            Ok(result)
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        const EPSILON: f64 = 0.00001;
        if self.cols != other.cols || self.rows != other.rows {
            return false;
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                if f64::abs(self.val_at(i, j).unwrap() - other.val_at(i, j).unwrap()) > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use crate::matrix::*;

    #[test]
    fn test_init() {
        let m = Matrix::new(2, 3);
        assert_eq!(m.vals[0][1], 0.0);
    }

    #[test]
    fn test_convert() {
        let m = vec![vec![0.0, 0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0, 0.0]];
        let n = Matrix::convert(m);
        assert_eq!(n, Matrix::new(2, 4));
    }

    #[test]
    fn test_write() {
        let mut m = Matrix::new(2, 3);
        m.write(3.3, 0, 1).unwrap();
        assert_eq!(m.vals[0][1], 3.3);
        assert_ne!(m.vals[1][1], 3.3);
    }

    #[test]
    fn test_read() {
        let m = Matrix::new(2, 3);
        assert_eq!(m.val_at(0, 1).unwrap(), 0.0);
    }

    #[test]
    fn test_equals() {
        let mut m = Matrix::new(4, 4);
        let mut n = Matrix::new(4, 4);

        for i in 0..m.rows {
            for j in 0..m.cols {
                m.write((i + j) as f64, i, j).unwrap();
                n.write((i + j) as f64, i, j).unwrap();
            }
        }
        assert_eq!(m, n);

        m.write(7.631, 0, 0).unwrap();
        assert_ne!(m, n);
    }

    #[test]
    fn test_mul() {
        let m_vec = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0]];
        let n_vec = vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0]];
        let q_vec = vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0]];

        let m = Matrix::convert(m_vec);
        let n = Matrix::convert(n_vec);
        let p = m.mul(&n).unwrap();
        let q = Matrix::convert(q_vec);

        assert_eq!(p, q);
    }

    #[test]
    fn test_tuple_mul() {
        let m_vec = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0]];
        let m = Matrix::convert(m_vec);
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1).unwrap();
        let prod = m.tuple_mul(&tuple).unwrap();
        assert_eq!(prod, Tuple::new(18.0, 24.0, 33.0, 1).unwrap());
    }

    #[test]
    fn test_identity_mul() {
        let id_vec = vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0]];

        let id_matrix = Matrix::convert(id_vec);

        let m_vec = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0]];

        let m = Matrix::convert(m_vec);

        assert_eq!(m.mul(&id_matrix).unwrap(), m);
    }

    #[test]
    fn test_transpose() {
        let m_vec = vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0]];

        let transposed_vec = vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0]];

        let m = Matrix::convert(m_vec);
        let t = Matrix::convert(transposed_vec);

        assert_eq!(m.transpose().unwrap(), t);
    }

    #[test]
    fn test_determinant_2x2() {
        let m_vec = vec![
            vec![1.0, 5.0],
            vec![-3.0, 2.0]];

        let m = Matrix::convert(m_vec);
        assert_eq!(m.determinant().unwrap(), 17.0)
    }

    #[test]
    fn test_submatrix() {
        let m_vec = vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0]];

        let sub_vec = vec![
            vec![-3.0, 2.0],
            vec![0.0, 6.0]];

        let m = Matrix::convert(m_vec);
        let sub = Matrix::convert(sub_vec);
        assert_eq!(m.submatrix(0, 2).unwrap(), sub);
    }

    #[test]
    fn test_minor() {
        let m_vec = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0]];

        let m = Matrix::convert(m_vec);
        assert_eq!(m.minor(1, 0).unwrap(), 25.0);
    }

    #[test]
    fn test_cofactor() {
        let m_vec = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0]];

        let m = Matrix::convert(m_vec);
        assert_eq!(m.minor(0, 0).unwrap(), -12.0);
        assert_eq!(m.cofactor(0, 0).unwrap(), -12.0);
        assert_eq!(m.minor(1, 0).unwrap(), 25.0);
        assert_eq!(m.cofactor(1, 0).unwrap(), -25.0);
    }

    #[test]
    fn test_3x3_determinant() {
        let vec_m = vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0]];

        let m = Matrix::convert(vec_m);
        assert_eq!(m.cofactor(0, 0).unwrap(), 56.0);
        assert_eq!(m.cofactor(0, 1).unwrap(), 12.0);
        assert_eq!(m.cofactor(0, 2).unwrap(), -46.0);
        assert_eq!(m.determinant().unwrap(), -196.0);
    }

    #[test]
    fn test_4x4_determinant() {
        let vec_m = vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0]];

        let m = Matrix::convert(vec_m);

        assert_eq!(m.cofactor(0, 0).unwrap(), 690.0);
        assert_eq!(m.cofactor(0, 1).unwrap(), 447.0);
        assert_eq!(m.cofactor(0, 2).unwrap(), 210.0);
        assert_eq!(m.cofactor(0, 3).unwrap(), 51.0);
        assert_eq!(m.determinant().unwrap(), -4071.0);
    }

    #[test]
    fn test_invert() {
        let m_vec = vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0]];

        let m = Matrix::convert(m_vec);

        assert!(m.is_invertible().unwrap());

        let i_vec = vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639]];

        let i = Matrix::convert(i_vec);

        assert_eq!(m.invert().unwrap(), i);
    }

    #[test]
    fn test_mul_and_invert() {
        let a_vec = vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0]];

        let b_vec = vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0]];

        let a = Matrix::convert(a_vec);
        let b = Matrix::convert(b_vec);
        let c = a.mul(&b).unwrap();

        assert_eq!(a, c.mul(&b.invert().unwrap()).unwrap())
    }
}