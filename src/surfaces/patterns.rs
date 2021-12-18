use crate::Matrix4;
use crate::tuples::*;
use super::colors::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pattern {
    a: Color,
    b: Color,
    transform: Matrix4,
    inverse_transform: Matrix4,
}

impl Pattern {
    pub fn stripe_pattern(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity(),
        }
    }

    pub fn stripe_at(&self, point: Tuple) -> Color {
        if (f64::floor(point.x) as isize) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    pub fn get_inverse_transform(&self) -> Matrix4 {
        self.inverse_transform
    }

    pub fn set_transform(&mut self, transform: Matrix4) -> &mut Self {
        self.transform = transform;
        self.inverse_transform = self.transform.invert();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stripe_y() {
        let pattern = Pattern::stripe_pattern(white(), black());
        assert_eq!(pattern.stripe_at(origin()), white());
        assert_eq!(pattern.stripe_at(point(0.0, 1.0, 0.0)), white());
        assert_eq!(pattern.stripe_at(point(0.0, 2.0, 0.0)), white());
    }

    #[test]
    fn test_stripe_z() {
        let pattern = Pattern::stripe_pattern(white(), black());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 1.0)), white());
        assert_eq!(pattern.stripe_at(point(0.0, 0.0, 2.0)), white());
    }

    #[test]
    fn test_stripe_x() {
        let pattern = Pattern::stripe_pattern(white(), black());
        assert_eq!(pattern.stripe_at(point(1.0, 0.0, 0.0)), black());
        assert_eq!(pattern.stripe_at(point(-1.0, 0.0, 0.0)), black());
        assert_eq!(pattern.stripe_at(point(-1.1, 0.0, 0.0)), white());
    }
}