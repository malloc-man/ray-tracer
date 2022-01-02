use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pattern {
    pattern_type: PatternType,
    transform: Matrix4,
    inverse_transform: Matrix4,
    color1: Color,
    color2: Color,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PatternType {
    Solid,
    Stripe,
    Gradient,
    Ring,
    Checker3d,
    Test,
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.pattern_type {
            PatternType::Solid => write!(f, "Solid Color"),
            PatternType::Stripe => write!(f, "Stripe"),
            PatternType::Gradient => write!(f, "Gradient"),
            PatternType::Ring => write!(f, "Ring"),
            PatternType::Checker3d => write!(f, "Checkers"),
            _ => write!(f, "Test"),
        }
    }
}

impl Pattern {
    pub fn new(pattern_type: PatternType, color1: Color, color2: Color) -> Self {
        Self {
            pattern_type,
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity(),
            color1,
            color2,
        }
    }

    pub fn duplicate_different_type(&self, new_type: PatternType) -> Self {
        Self {
            pattern_type: new_type,
            transform: self.transform,
            inverse_transform: self.inverse_transform,
            color1: self.color1,
            color2: self.color2,
        }
    }

    pub fn duplicate_change_color_1(&self, color: Color) -> Self {
        Self {
            pattern_type: self.pattern_type,
            transform: self.transform,
            inverse_transform: self.inverse_transform,
            color1: color,
            color2: self.color2,
        }
    }

    pub fn duplicate_change_color_2(&self, color: Color) -> Self {
        Self {
            pattern_type: self.pattern_type,
            transform: self.transform,
            inverse_transform: self.inverse_transform,
            color1: self.color1,
            color2: color,
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
            PatternType::Stripe => stripe_at(self.color1, self.color2, point),
            PatternType::Gradient => gradient_at(self.color1, self.color2, point),
            PatternType::Ring => ring_at(self.color1, self.color2, point),
            PatternType::Checker3d => checker_3d_at(self.color1, self.color2, point),
            PatternType::Solid => black(),
            PatternType::Test => color(point.x, point.y, point.z),
        }
    }

    pub fn colors (&self) -> [Color; 2] {
        [self.color1, self.color2]
    }

    pub fn set_color_1(&mut self, color: Color) -> &mut Self {
        self.color1 = color;
        self
    }

    pub fn set_color_2(&mut self, color: Color) -> &mut Self {
        self.color2 = color;
        self
    }
}

pub fn solid() -> Pattern {
    Pattern::new(PatternType::Solid, white(), black())
}

pub fn stripe(a: Color, b: Color) -> Pattern {
    Pattern::new(PatternType::Stripe, a,b)
}

pub fn gradient(a: Color, b: Color) -> Pattern {
    Pattern::new(PatternType::Gradient, a,b)
}

pub fn ring(a: Color, b: Color) -> Pattern {
    Pattern::new(PatternType::Ring,a, b)
}

pub fn checker_3d(a: Color, b: Color) -> Pattern {
    Pattern::new(PatternType::Checker3d, a, b)
}

pub fn test_pattern() -> Pattern {
    Pattern::new(PatternType::Test, black(), black())
}

fn stripe_at(a: Color, b: Color, point: Tuple) -> Color {
    if (point.x.floor() as isize) % 2 == 0 {
        a
    } else {
        b
    }
}

fn gradient_at(a: Color, b: Color, point: Tuple) -> Color {
    let distance = b - a;
    let fraction = point.x - point.x.floor();
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

    #[test]
    fn test_stripe_at_object_with_pattern_transformation() {
        let mut object = spheres::new();
        object.set_pattern(stripe(white(), black()));
        object.set_pattern_transform(scaling(2.0, 2.0, 2.0));
        let c = object.pattern_at_object(point(1.5, 0.0, 0.0));
        assert_eq!(c, white());
    }

    #[test]
    fn test_stripe_at_object_with_pattern_and_obj_transformations() {
        let mut object = spheres::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        object.set_pattern(stripe(white(), black()));
        object.set_pattern_transform(translation(0.5, 0.0, 0.0));
        let c = object.pattern_at_object(point(2.5, 0.0, 0.0));
        assert_eq!(c, white());
    }

    #[test]
    fn test_stripe_at_object_with_obj_transformation() {
        let mut object = spheres::new();
        object.set_transform(scaling(2.0, 2.0, 2.0));
        object.set_pattern(stripe(white(), black()));

        let c = object.pattern_at_object(point(1.5, 0.0, 0.0));
        assert_eq!(c, white());
    }
}
