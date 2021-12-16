use std::cmp::{max, min};
use std::fs;
use std::error::Error;
use std::io::Write;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); width] ; height]
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y][x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        let c = &self.pixels[y][x];
        Color::new(c.red, c.green, c.blue)
    }

    pub fn canvas_to_ppm(&self, path: &str) -> Result<bool, Box<dyn Error>> {
        let mut f = fs::File::create(path)?;
        f.write_all(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())?;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut r = (self.pixels[y][x].red * 255.0) as isize;
                if r > 255 {
                    r = 255;
                } else if r < 0 {
                    r = 0;
                }

                let mut g = (self.pixels[y][x].green * 255.0) as isize;
                if g > 255 {
                    g = 255;
                } else if g < 0 {
                    g = 0;
                }

                let mut b = (self.pixels[y][x].blue * 255.0) as isize;
                if b > 255 {
                    b = 255;
                } else if b < 0 {
                    b = 0;
                }
                f.write_all(format!("{} {} {} ", r, g, b).as_bytes())?;
            }
            f.write_all("\n".as_bytes())?;
        }
        Ok(true)
    }
}

#[derive(Debug, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            red,
            green,
            blue
        }
    }

    fn add(&self, other: &Color) -> Color {
        Color::new(self.red + other.red, self.green + other.green, self.blue + other.blue)
    }

    fn subtract(&self, other: &Color) -> Color {
        Color::new(self.red - other.red, self.green - other.green, self.blue - other.blue)
    }

    fn scalar_mul(&self, scale: f64) -> Color {
        Color::new(self.red * scale, self.green * scale, self.blue * scale)
    }

    fn mul(&self, other: &Color) -> Color {
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
mod canvas_tests {
    use std::io::Read;
    use crate::canvas::*;

    #[test]
    fn test_initialize() {
        let c = Canvas::new(5, 6);
        assert_eq!(c.pixel_at(2, 3), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_write_pixels() {
        let mut c = Canvas::new(5, 6);
        c.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));
        assert_eq!(c.pixel_at(2, 3), Color::new(1.0, 0.0, 0.0));
        assert_eq!(c.pixel_at(0, 0), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_write_file() {
        let path = "./testimage.ppm";
        let mut c = Canvas::new(5, 3);

        c.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));

        c.canvas_to_ppm(path);

        let mut f = fs::File::open(path).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        assert!(s.contains("5 3\n"));
        assert!(s.contains("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"));
        assert!(s.contains("0 0 0 0 0 0 0 127 0 0 0 0 0 0 0"));
        assert!(s.contains("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));

        fs::remove_file(path);
    }
}

#[cfg(test)]
mod color_tests {
    use crate::canvas::Color;

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