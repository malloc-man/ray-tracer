#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub(crate) fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            red,
            green,
            blue
        }
    }

    pub(crate) fn add(&self, other: &Color) -> Color {
        Color::new(self.red + other.red, self.green + other.green, self.blue + other.blue)
    }

    fn subtract(&self, other: &Color) -> Color {
        Color::new(self.red - other.red, self.green - other.green, self.blue - other.blue)
    }

    pub(crate) fn scalar_mul(&self, scale: f64) -> Color {
        Color::new(self.red * scale, self.green * scale, self.blue * scale)
    }

    pub(crate) fn mul(&self, other: &Color) -> Color {
        Color::new(self.red * other.red, self.green * other.green, self.blue * other.blue)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        const EPSILON: f64 = 0.00001;
        if f64::abs(self.red - other.red) > EPSILON ||
            f64::abs(self.green - other.green) > EPSILON ||
            f64::abs(self.blue - other.blue) > EPSILON {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod color_tests {
    use crate::canvas::*;
    use crate::colors::Color;

    #[test]
    fn test_add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let sum = c1.add(&c2);
        assert_eq!(sum, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_sub_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let diff = c1.subtract(&c2);
        assert_eq!(diff, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_scalar_mul() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c1.scalar_mul(2.0), Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_color_prod() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1.mul(&c2), Color::new(0.9, 0.2, 0.04));
    }
}
