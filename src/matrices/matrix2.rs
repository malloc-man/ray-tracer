use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Matrix2 {
    vals: [[f64; 2]; 2]
}

impl Matrix2 {

    pub fn convert(vals: [[f64; 2]; 2]) -> Self {
        Self {
            vals,
        }
    }

    pub fn val_at(&self, row: usize, col: usize) -> f64 {
        if row >= 2 || col >= 2 {
            panic!("Read index error. Attempted to read row: {}, col: {}. Size is 2x2.", row, col);
        }
        self.vals[row][col]
    }

    pub fn determinant(&self) -> f64 {
        self.val_at(0, 0) * self.val_at(1, 1) - self.val_at(1, 0) * self.val_at(0, 1)
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Matrix2) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                if !self.val_at(i, j).approx_eq(other.val_at(i, j)) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_determinant_2x2() {
        let m_arr = [
            [1.0, 5.0],
            [-3.0, 2.0]];

        let m = Matrix2::convert(m_arr);
        assert_eq!(m.determinant(), 17.0)
    }
}