pub const EPSILON: f64 = 0.00001;

pub trait ApproxEq {
    fn approx_eq(&self, other: f64) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: f64) -> bool {
        (self - other).abs() < EPSILON
    }
}