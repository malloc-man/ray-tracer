use std::sync::atomic::{AtomicUsize, Ordering};
use rayon::prelude::*;
use crate::{matrix4::*, Ray};
use crate::matrices::tuples::*;
use crate::scenes::canvas::*;
use crate::scenes::world::*;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix4,
    inverse_transform: Matrix4,
    pixel_size: f64,
    half_width: f64,
    half_height: f64
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity(),
            pixel_size: 0.0,
            half_width: 0.0,
            half_height: 0.0,
        }.initialize()
    }

    fn initialize(mut self) -> Self {
        let half_view = f64::tan(self.field_of_view / 2.0);
        let aspect = self.hsize as f64 / self.vsize as f64;
        if aspect >= 1.0 {
            self.half_width = half_view;
            self.half_height = half_view / aspect;
        } else {
            self.half_width = half_view * aspect;
            self.half_height = half_view;
        }
        self.pixel_size = (self.half_width * 2.0) / self.hsize as f64;
        self
    }

    pub fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
        self.inverse_transform = transform.invert();
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.inverse_transform * point(world_x, world_y, -1.0);
        let origin = self.inverse_transform * origin();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray, DEFAULT_REFLECTION_DEPTH);
                image.write_pixel(x, y, color);
            }
        }
        image
    }

    pub fn parallel_render(&self, world: &World) -> Canvas {
        let mut pixels_rendered = AtomicUsize::new(0);

        println!("Beginning render...");

        const BAND_SIZE: usize = 10;
        let mut image = Canvas::new(self.hsize, self.vsize);

        println!("Rendering image: {} x {}", self.hsize, self.vsize);
        image
            .pixels()
            .par_chunks_mut(self.hsize * BAND_SIZE)
            .enumerate()
            .for_each(|(i, band)| {
                for row in 0..BAND_SIZE {
                    for col in 0..self.hsize {
                        let ray = self.ray_for_pixel(col, row + i * BAND_SIZE);
                        if (row * self.hsize) + col < band.len() {
                            band[(row * self.hsize) + col] =
                                world.color_at(ray, DEFAULT_REFLECTION_DEPTH);
                            pixels_rendered.fetch_add(1, Ordering::SeqCst);
                        }
                    }
                    print!("\rRender progress: {:.1}%",
                           pixels_rendered.load(Ordering::Relaxed) as f64 * 100.0 /
                               ((self.hsize * self.vsize) as f64));
                }
            });
        image
    }
}

#[cfg(test)]
mod tests {
    use crate::transformations::view_transform;
    use crate::surfaces::colors::*;
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_pixel_size_horizontal_canvas() {
        let c = Camera::new(200, 125, PI/2.0);
        assert!(f64::abs(c.pixel_size - 0.01) < 0.00001);
    }

    #[test]
    fn test_pixel_size_vertical_canvas() {
        let c = Camera::new(125, 200, PI/2.0);
        assert!(f64::abs(c.pixel_size - 0.01) < 0.00001);
    }

    #[test]
    fn test_ray_for_pixel_center() {
        let c = Camera::new(201, 101, PI/2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.get_origin(), origin());
        assert_eq!(r.get_direction(), vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_ray_for_pixel_corner() {
        let c = Camera::new(201, 101, PI/2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.get_origin(), origin());
        assert_eq!(r.get_direction(), vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_ray_for_pixel_transformed() {
        let mut c = Camera::new(201, 101, PI/2.0);
        c.set_transform(
            Matrix4::identity()
                .translate(0.0, -2.0, 5.0)
                .rotate_y(PI/4.0)
        );

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.get_origin(), point(0.0, 2.0, -5.0));
        assert_eq!(r.get_direction(), vector(f64::sqrt(2.0)/2.0, 0.0, f64::sqrt(2.0)/-2.0));
    }

    #[test]
    fn test_render_world() {
        let w = World::new_default();
        let mut c = Camera::new(11, 11, PI/2.0);

        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);

        c.set_transform(view_transform(from, to, up));

        let image = c.render(&w);
        assert_eq!(image.pixel_at(5, 5), color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_parallel_render_world() {
        let w = World::new_default();
        let mut c = Camera::new(11, 11, PI/2.0);

        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);

        c.set_transform(view_transform(from, to, up));

        let image = c.parallel_render(&w);
        assert_eq!(image.pixel_at(5, 5), color(0.38066, 0.47583, 0.2855));
    }
}