use std::ops;
use crate::utils::ApproxEq;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    Color {
        red,
        green,
        blue
    }
}

pub fn black() -> Color {
    Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    }
}

pub fn white() -> Color {
    Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    }
}

impl Color {
    pub fn get_red(&self) -> f64 {
        self.red
    }

    pub fn get_green(&self) -> f64 {
        self.green
    }

    pub fn get_blue(&self) -> f64 {
        self.blue
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        color(self.red + other.red, self.green + other.green, self.blue + other.blue)
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        *self = *self + color(self.red + other.red, self.green + other.green, self.blue + other.blue)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        color(self.red - other.red, self.green - other.green, self.blue - other.blue)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        color(self.red * other.red, self.green * other.green, self.blue * other.blue)
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, scale: f64) -> Color {
        color(self.red * scale, self.green * scale, self.blue * scale)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        if !self.red.approx_eq(other.red) ||
            !self.green.approx_eq(other.green) ||
            !self.blue.approx_eq(other.blue) {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);

        let sum = c1 + c2;
        assert_eq!(sum, color(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_sub_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);

        let diff = c1 - c2;
        assert_eq!(diff, color(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_scalar_mul() {
        let c1 = color(0.2, 0.3, 0.4);
        assert_eq!(c1 * 2.0, color(0.4, 0.6, 0.8));
    }

    #[test]
    fn test_color_prod() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
    }
}
