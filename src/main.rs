use std::error::Error;
use {geometry::*, geometry::rays::*, canvas::*, tuples::*, matrices::*, materials::*, lights::*, colors::*};

mod tuples;
mod canvas;
mod matrices;
mod geometry;
mod lights;
mod materials;
mod colors;
mod world;

pub const CANVAS_WIDTH: usize = 100;
pub const CANVAS_HEIGHT: usize = 100;

fn main() -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new();
    let wall_size = 7.0;
    let pixel_size = wall_size / 100.0;
    let half = wall_size / 2.0;
    let mut sphere = spheres::new();
    sphere.set_color(Color::new(1.0, 0.2, 1.0));
    let wall_z = 10.0;
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let light = Light::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    for y in 0..100 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..100 {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            if let Some(xs) = spheres::intersect(sphere, ray) {
                if let Some(i) = spheres::hit(xs) {
                    let point = ray.position(i.get_t());
                    let normal = sphere.normal_at(point);
                    let eye = -ray.get_direction();
                    let new_color = lighting(i.get_object().get_material(), point, light,eye, normal);
                    canvas.write_pixel(x, y, new_color);
                }
            }
        }
    }
    canvas.canvas_to_ppm("./image.ppm")?;
    Ok(())
}