use std::{fs, error::Error, io::Write};
use image::*;
use crate::prelude::*;

pub struct Canvas {
    height: usize,
    width: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            height,
            width,
            pixels: vec![color(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn pixels(&mut self) -> &mut Vec<Color> {
        &mut self.pixels
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn xy_to_1d(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let index = self.xy_to_1d(x, y);
        self.pixels[index] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.xy_to_1d(x, y)]
    }

    pub fn canvas_to_ppm(&self, path: &str) -> Result<bool, Box<dyn Error>> {
        let mut f = fs::File::create(path)?;
        f.write_all(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())?;

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.xy_to_1d(x, y);
                let mut r = (self.pixels[index].get_red() * 255.0) as isize;
                if r > 255 {
                    r = 255;
                } else if r < 0 {
                    r = 0;
                }

                let mut g = (self.pixels[index].get_green() * 255.0) as isize;
                if g > 255 {
                    g = 255;
                } else if g < 0 {
                    g = 0;
                }

                let mut b = (self.pixels[index].get_blue() * 255.0) as isize;
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

    pub fn canvas_to_png(&self, path: &str) {
        let mut buffer: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let clr = self.pixel_at(x as usize, y as usize);
            *pixel = Rgb([(clr.get_red() * 255.999) as u8,
                (clr.get_green() * 255.999) as u8,
                (clr.get_blue() * 255.999) as u8]);
        }
        match buffer.save(path) {
            Err(e) => eprintln!("\nError: {}", e),
            Ok(()) => println!("\nRender complete: image saved as {}", path),
        };
    }

    pub fn canvas_to_buffer(&self) -> RgbaImage {
        let mut buffer: RgbaImage = ImageBuffer::new(self.width as u32, self.height as u32);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let clr = self.pixel_at(x as usize, y as usize);
            *pixel = Rgba([(clr.get_red() * 255.999) as u8,
                (clr.get_green() * 255.999) as u8,
                (clr.get_blue() * 255.999) as u8,
                255.999 as u8]);
        }
        buffer
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use super::*;

    #[test]
    fn test_initialize() {
        let c = Canvas::new(10, 10);
        assert_eq!(c.pixel_at(2, 3), color(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_write_pixels() {
        let mut c = Canvas::new(10, 10);
        c.write_pixel(2, 3, color(1.0, 0.0, 0.0));
        assert_eq!(c.pixel_at(2, 3), color(1.0, 0.0, 0.0));
        assert_eq!(c.pixel_at(0, 0), color(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_write_file() {
        let path = "./testimage.ppm";
        let mut c = Canvas::new(10, 10);

        c.write_pixel(0, 0, color(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, color(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, color(-0.5, 0.0, 1.0));

        c.canvas_to_ppm(path).unwrap();

        let mut f = fs::File::open(path).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();

        assert!(s.contains("255 0 0"));
        assert!(s.contains("0 0 0 0 0 0 0 127 0"));
        assert!(s.contains("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"));

        fs::remove_file(path).unwrap();
    }
}