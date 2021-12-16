use crate::matrices::matrix::Matrix;
use crate::Tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    radius: f64,
    pub(crate) origin: Tuple,
    pub(crate) transform: Matrix,
}

impl Sphere {
    pub fn new(radius: f64, origin: Tuple) -> Self {
        if origin.v == 0 {
            panic!("Attempted to create sphere with vector origin rather than point");
        }
        Self {
            radius,
            origin,
            transform: Matrix::identity(4),
        }
    }

    pub(crate) fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}
