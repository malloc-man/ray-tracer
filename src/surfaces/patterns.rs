use crate::Matrix4;
use crate::tuples::*;
use super::colors::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pattern {
    pattern_type: PatternType,
    transform: Matrix4,
    inverse_transform: Matrix4,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PatternType {
    Solid {a: Color},
    Stripe {a: Color, b: Color},
    Gradient {a: Color, b: Color},
    Ring {a: Color, b: Color},
    Checker3d {a: Color, b: Color},
}

impl Pattern {
    pub fn new(pattern_type: PatternType, transform: Matrix4) -> Self {
        Self {
            pattern_type,
            transform,
            inverse_transform: transform.invert(),
        }
    }

    pub fn get_pattern_type(&self) -> PatternType {
        self.pattern_type
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

    pub fn pattern_at(&self, point: Tuple) -> Color {
        match self.pattern_type {
            PatternType::Stripe {a, b} => stripe_at(a, b, point),
            PatternType::Gradient {a, b} => gradient_at(a, b, point),
            PatternType::Ring {a, b} => ring_at(a, b, point),
            PatternType::Checker3d {a, b} => checker_3d_at(a, b, point),
            PatternType::Solid{a} => a,
        }
    }
}

pub fn solid(a: Color) -> Pattern {
    Pattern::new(PatternType::Solid{a}, Matrix4::identity())
}

pub fn stripe(a: Color, b: Color) -> Pattern {
    Pattern::new(PatternType::Stripe{a,b}, Matrix4::identity())
}

pub fn gradient(a: Color, b: Color) -> Pattern {
    Pattern::new(PatternType::Gradient{a,b}, Matrix4::identity())
}

pub fn ring(a: Color, b: Color) -> Pattern {
    Pattern::new(PatternType::Ring {a, b}, Matrix4::identity())
}

pub fn checker_3d(a: Color, b: Color) -> Pattern {
    Pattern::new(PatternType::Checker3d {a, b}, Matrix4::identity())
}

fn stripe_at(a: Color, b: Color, point: Tuple) -> Color {
    if (f64::floor(point.x) as isize) % 2 == 0 {
        a
    } else {
        b
    }
}

fn gradient_at(a: Color, b: Color, point: Tuple) -> Color {
    let distance = b - a;
    let fraction = point.x - f64::floor(point.x);
    a + distance * fraction
}

fn ring_at(a: Color, b: Color, point: Tuple) -> Color {
    if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() as usize % 2 == 0 {
        a
    } else {
        b
    }
}

fn checker_3d_at(a: Color, b: Color, point: Tuple) -> Color {
    if ((point.x.floor() + point.y.floor() + point.z.floor()) as isize) % 2 == 0 {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stripe_y() {
        let pattern = stripe(white(), black());
        assert_eq!(pattern.pattern_at(origin()), white());
        assert_eq!(pattern.pattern_at(point(0.0, 1.0, 0.0)), white());
        assert_eq!(pattern.pattern_at(point(0.0, 2.0, 0.0)), white());
    }

    #[test]
    fn test_stripe_z() {
        let pattern = stripe(white(), black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 2.0)), white());
    }

    #[test]
    fn test_stripe_x() {
        let pattern = stripe(white(), black());
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), black());
        assert_eq!(pattern.pattern_at(point(-1.0, 0.0, 0.0)), black());
        assert_eq!(pattern.pattern_at(point(-1.1, 0.0, 0.0)), white());
    }

    #[test]
    fn test_gradient() {
        let pattern = gradient(white(), black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 0.0)), white());
        assert_eq!(pattern.pattern_at(point(0.25, 0.0, 0.0)), color(0.75, 0.75, 0.75));
        assert_eq!(pattern.pattern_at(point(0.5, 0.0, 0.0)), color(0.5, 0.5, 0.5));
        assert_eq!(pattern.pattern_at(point(0.75, 0.0, 0.0)), color(0.25, 0.25, 0.25));
    }

    #[test]
    fn test_ring() {
        let pattern = ring(white(), black());
        assert_eq!(pattern.pattern_at(origin()), white());
        assert_eq!(pattern.pattern_at(point(1.0, 0.0, 0.0)), black());
        assert_eq!(pattern.pattern_at(point(0.0, 0.0, 1.0)), black());
        assert_eq!(pattern.pattern_at(point(0.708, 0.0, 0.708)), black());
    }

    #[test]
    fn test_checker_3d_x() {
        let pattern = checker_3d(white(), black());
        assert_eq!(pattern.pattern_at(origin()), white());
        assert_eq!(pattern.pattern_at(point(0.99, 0.0, 0.0)), white());
        assert_eq!(pattern.pattern_at(point(1.01, 0.0, 0.0)), black());
    }

    #[test]
    fn test_checker_3d_y() {
        let pattern = checker_3d(white(), black());
        assert_eq!(pattern.pattern_at(origin()), white());
        assert_eq!(pattern.pattern_at(point(0.0, 0.99,0.0)), white());
        assert_eq!(pattern.pattern_at(point(0.0, 1.01,0.0)), black());
    }

    #[test]
    fn test_checker_3d_z() {
        let pattern = checker_3d(white(), black());
        assert_eq!(pattern.pattern_at(origin()), white());
        assert_eq!(pattern.pattern_at(point(0.0,0.0, 0.99)), white());
        assert_eq!(pattern.pattern_at(point(0.0,0.0, 1.01)), black());
    }
}
