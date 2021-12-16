use std::cmp::{max, min};
use std::fs;
use std::error::Error;
use std::io::Write;
use crate::colors::Color;

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

#[cfg(test)]
mod tests {
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